<script lang="ts" setup>
import { sendLog } from '@/utils'
import { debounce } from 'lodash-es'
import { useMessage, useDialog } from 'naive-ui'
import { onBeforeMount, reactive, computed, ref } from 'vue'
import { getAllHosts, type Hosts, updateHostsData } from '@/apis'
import { Search, Plus, Refresh, Settings, Eye, EyeOff } from '@vicons/tabler'

defineOptions({
  name: 'HostsList',
})

const message = useMessage()
const dialog = useDialog()
const searchKeyword = ref('')

const data = reactive({
  loading: true,
  curId: -1,
  curHostsData: {} as Hosts,
  hostsList: [] as Hosts[],
  pageConfig: {
    page: 1,
    size: 10,
    total: 0,
  },
})

// 过滤后的 hosts 列表
const filteredHostsList = computed(() => {
  if (!searchKeyword.value) return data.hostsList
  return data.hostsList.filter(hosts =>
    hosts.name.toLowerCase().includes(searchKeyword.value.toLowerCase())
  )
})

// 统计信息
const stats = computed(() => {
  const total = data.hostsList.length
  const active = data.hostsList.filter(h => h.status === 1).length
  const readonly = data.hostsList.filter(h => h.is_readonly).length
  return { total, active, readonly }
})

function getData() {
  data.loading = true
  // 调用接口获取数据
  getAllHosts({ page: 1, pageSize: 1000 }).then((res) => {
    if (res.code === 200) {
      data.hostsList = res.data.list
      data.pageConfig.total = res.data.total

      if (res.data)
        data.curId = data.hostsList[0].id
    }

    else { return Promise.reject(res) }
  }).catch((err) => {
    message.error(err.msg || '获取失败')
  }).finally(() => {
    data.loading = false
  })
}

function onChangeTab(value: number) {
  data.curId = value
}

function refreshData() {
  getData()
  message.success('数据已刷新')
}

function toggleHostStatus(hosts: Hosts) {
  const newStatus = hosts.status === 1 ? 0 : 1
  const action = newStatus === 1 ? '启用' : '禁用'

  dialog.warning({
    title: `${action} Hosts`,
    content: `确定要${action} "${hosts.name}" 吗？`,
    positiveText: '确定',
    negativeText: '取消',
    onPositiveClick: () => {
      updateHostsData(hosts.id, {
        ...hosts,
        status: newStatus
      }).then(() => {
        hosts.status = newStatus
        message.success(`${action}成功`)
        sendLog({
          msg: `${action}了 ${hosts.name}`,
          level: 'system',
        })
      }).catch((err) => {
        message.error(err.msg || `${action}失败`)
      })
    }
  })
}

function addNewHosts() {
  message.info('添加新 Hosts 功能开发中...')
}

function openSettings() {
  message.info('设置功能开发中...')
}

const onEditorChange = debounce((event: {
  originValue: string
  newValue: string
}, hosts: Hosts, _: number) => {
  // 更新数据
  sendLog({
    msg: `更新了${hosts.name}, ${JSON.stringify({
      hosts_id: hosts.id,
      hosts_name: hosts.name,
      hosts_type: hosts.hosts_type,
      hosts_path: hosts.hosts_path,
    })}`,
    level: 'system',
  })
  updateHostsData(hosts.id, {
    name: hosts.name,
    content: event.newValue,
    status: 1,
    hosts_type: hosts.hosts_type,
    hosts_path: hosts.hosts_path,
  })
}, 300)

onBeforeMount(() => {
  getData()
})
</script>

<template>
  <n-card :bordered="false" class="h-full">
    <template #header>
      <div class="flex justify-between items-center">
        <div class="flex items-center gap-4">
          <h2 class="text-lg font-semibold m-0">Hosts 管理</h2>
          <div class="flex gap-2 text-sm text-gray-500">
            <span>总计: {{ stats.total }}</span>
            <span>活跃: {{ stats.active }}</span>
            <span>只读: {{ stats.readonly }}</span>
          </div>
        </div>
        <div class="flex items-center gap-2">
          <n-input
            v-model:value="searchKeyword"
            placeholder="搜索 Hosts..."
            class="w-48"
            clearable
          >
            <template #prefix>
              <n-icon :component="Search" />
            </template>
          </n-input>
          <n-button @click="refreshData" :loading="data.loading">
            <template #icon>
              <n-icon :component="Refresh" />
            </template>
          </n-button>
          <n-button type="primary" @click="addNewHosts">
            <template #icon>
              <n-icon :component="Plus" />
            </template>
            新建
          </n-button>
          <n-button @click="openSettings">
            <template #icon>
              <n-icon :component="Settings" />
            </template>
          </n-button>
        </div>
      </div>
    </template>

    <n-spin :show="data.loading">
      <div v-if="filteredHostsList.length > 0" class="container-tab">
        <n-tabs
          type="line"
          animated
          placement="left"
          class="tab h-full"
          :on-update:value="onChangeTab"
        >
          <template v-for="(hosts, index) in filteredHostsList" :key="`hosts__${hosts.id}`">
            <n-tab-pane :name="hosts.id" :tab="hosts.id">
              <!-- 编辑器头部信息 -->
              <div v-if="hosts.id === data.curId" class="editor-header">
                <div class="flex justify-between items-center mb-4">
                  <div class="flex items-center gap-2">
                    <h3 class="text-base font-medium m-0">{{ hosts.name }}</h3>
                    <n-tag :type="hosts.status === 1 ? 'success' : 'default'" size="small">
                      {{ hosts.status === 1 ? '已启用' : '已禁用' }}
                    </n-tag>
                    <n-tag v-if="hosts.is_readonly" type="warning" size="small">
                      只读
                    </n-tag>
                  </div>
                  <div class="flex items-center gap-2">
                    <n-button
                      size="small"
                      :type="hosts.status === 1 ? 'default' : 'primary'"
                      @click="toggleHostStatus(hosts)"
                      :disabled="Boolean(hosts.is_readonly)"
                    >
                      <template #icon>
                        <n-icon :component="hosts.status === 1 ? EyeOff : Eye" />
                      </template>
                      {{ hosts.status === 1 ? '禁用' : '启用' }}
                    </n-button>
                  </div>
                </div>
                <div class="text-xs text-gray-500 mb-2">
                  路径: {{ hosts.hosts_path }}
                </div>
              </div>

              <!-- shell 编辑器 -->
              <editor
                v-if="hosts.id === data.curId"
                id="hosts-editor"
                v-model="hosts.content"
                :format="true"
                class="editor"
                language="hosts"
                :options="{
                  readOnly: hosts.is_readonly,
                }"
                @on-change="onEditorChange($event, hosts, index)"
              />
              <template #tab>
                <n-tooltip placement="right" trigger="hover">
                  <template #trigger>
                    <div class="w-full flex flex-col items-center gap-1">
                      <div class="flex items-center gap-1">
                        <div
                          class="w-2 h-2 rounded-full"
                          :class="hosts.status === 1 ? 'bg-green-500' : 'bg-gray-400'"
                        ></div>
                        <span class="tab__name">{{ hosts.name }}</span>
                      </div>
                      <div class="flex gap-1">
                        <n-tag v-if="hosts.is_readonly" :bordered="false" type="warning" size="tiny">
                          只读
                        </n-tag>
                        <n-tag
                          :bordered="false"
                          :type="hosts.status === 1 ? 'success' : 'default'"
                          size="tiny"
                        >
                          {{ hosts.status === 1 ? '启用' : '禁用' }}
                        </n-tag>
                      </div>
                    </div>
                  </template>
                  <div>
                    <div>{{ hosts.name }}</div>
                    <div class="text-xs text-gray-500">{{ hosts.hosts_path }}</div>
                    <div class="text-xs">
                      状态: {{ hosts.status === 1 ? '已启用' : '已禁用' }}
                      {{ hosts.is_readonly ? ' | 只读' : '' }}
                    </div>
                  </div>
                </n-tooltip>
              </template>
            </n-tab-pane>
          </template>
        </n-tabs>
      </div>
      <n-result v-else-if="searchKeyword && data.hostsList.length > 0" status="404" title="未找到匹配的 Hosts" description="尝试使用其他关键词搜索">
        <template #footer>
          <n-button @click="searchKeyword = ''">
            清除搜索
          </n-button>
        </template>
      </n-result>
      <n-result v-else status="404" title="暂无 Hosts 文件" description="开始创建您的第一个 Hosts 文件">
        <template #footer>
          <n-button type="primary" @click="addNewHosts">
            <template #icon>
              <n-icon :component="Plus" />
            </template>
            创建 Hosts
          </n-button>
        </template>
      </n-result>
    </n-spin>
  </n-card>
</template>

<style lang="less" scoped>
.container-tab {
  height: calc(100vh - 200px);
}

.tab {
  :deep(.n-tabs-nav) {
    width: 200px;
  }

  :deep(.n-tabs-tab) {
    padding: 12px 8px;
    min-height: 60px;
  }

  :deep(.n-tabs-tab-wrapper) {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  & .tab__name {
    display: inline-block;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 120px;
    font-size: 12px;
    font-weight: 500;
  }
}

.editor-header {
  background: #fafafa;
  border: 1px solid #e0e0e0;
  border-radius: 6px;
  padding: 16px;
  margin-bottom: 16px;
}

.editor {
  border: 1px solid #e0e0e0;
  border-radius: 6px;
  overflow: hidden;
}

:deep(.n-card-header) {
  padding: 16px 24px;
  border-bottom: 1px solid #e0e0e0;
}

:deep(.n-card__content) {
  padding: 0;
}

.w-48 {
  width: 12rem;
}

.bg-green-500 {
  background-color: #10b981;
}

.bg-gray-400 {
  background-color: #9ca3af;
}

.text-gray-500 {
  color: #6b7280;
}

.gap-1 {
  gap: 0.25rem;
}

.gap-2 {
  gap: 0.5rem;
}

.gap-4 {
  gap: 1rem;
}

.text-xs {
  font-size: 0.75rem;
  line-height: 1rem;
}

.text-sm {
  font-size: 0.875rem;
  line-height: 1.25rem;
}

.text-base {
  font-size: 1rem;
  line-height: 1.5rem;
}

.text-lg {
  font-size: 1.125rem;
  line-height: 1.75rem;
}

.font-medium {
  font-weight: 500;
}

.font-semibold {
  font-weight: 600;
}

.m-0 {
  margin: 0;
}

.mb-2 {
  margin-bottom: 0.5rem;
}

.mb-4 {
  margin-bottom: 1rem;
}

.w-2 {
  width: 0.5rem;
}

.h-2 {
  height: 0.5rem;
}

.rounded-full {
  border-radius: 9999px;
}

.flex {
  display: flex;
}

.flex-col {
  flex-direction: column;
}

.items-center {
  align-items: center;
}

.justify-between {
  justify-content: space-between;
}
</style>
