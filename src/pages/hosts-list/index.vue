<script lang="ts" setup>
import { debounce } from 'lodash-es'
import { useDialog, useMessage } from 'naive-ui'
import { computed, onBeforeMount, onBeforeUnmount, reactive, ref } from 'vue'
import { Eye, EyeOff, FileText, Plus, Refresh, Search, Trash, Upload } from '@vicons/tabler'

import type { Hosts } from '@/apis'

import { sendLog } from '@/utils'
import { useHostsStore } from '@/store'
import { globalEventEmitter } from '@/utils/event'
import {
  applyHostsToSystem,
  deleteHost,
  getAllHosts,
  readSystemHostsFile,
  toggleHostActive,
  updateHost,
} from '@/apis'

defineOptions({
  name: 'HostsList',
})

const message = useMessage()
const dialog = useDialog()
const hostsStore = useHostsStore()
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

// 系统 hosts 文件查看器
const isSystemHostsVisible = ref(false)
const systemHostsContent = ref('')
const isLoadingSystemHosts = ref(false)

function viewSystemHosts() {
  isLoadingSystemHosts.value = true
  readSystemHostsFile().then((res) => {
    console.log('[viewSystemHosts] response:', res)
    if (res.code === 200) {
      systemHostsContent.value = res.data
      isSystemHostsVisible.value = true
    }
    else {
      message.error(res.msg || '读取系统 hosts 文件失败')
    }
  }).catch((err) => {
    console.error('[viewSystemHosts] error:', err)
    message.error(err.msg || '读取系统 hosts 文件失败')
  }).finally(() => {
    isLoadingSystemHosts.value = false
  })
}

// 过滤后的 hosts 列表
const filteredHostsList = computed(() => {
  if (!searchKeyword.value)
    return data.hostsList
  return data.hostsList.filter(hosts =>
    hosts.name.toLowerCase().includes(searchKeyword.value.toLowerCase()),
  )
})

// 统计信息
const stats = computed(() => {
  const total = data.hostsList.length
  const active = data.hostsList.filter(h => h.is_active === 1).length
  const system = data.hostsList.filter(h => h.is_system === 1).length
  return { total, active, system }
})

function getData() {
  data.loading = true
  getAllHosts().then((res) => {
    if (res.code === 200) {
      data.hostsList = res.data || []
      data.pageConfig.total = data.hostsList.length

      if (data.hostsList.length > 0)
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

function handleToggleHostStatus(hosts: Hosts) {
  const isCurrentlyActive = hosts.is_active === 1
  const action = isCurrentlyActive ? '禁用' : '启用'

  dialog.warning({
    title: `${action} Hosts`,
    content: `确定要${action} "${hosts.name}" 吗？`,
    positiveText: '确定',
    negativeText: '取消',
    onPositiveClick: () => {
      toggleHostActive(hosts.id).then((res) => {
        if (res.code === 200) {
          hosts.is_active = res.data.is_active
          message.success(`${action}成功`)
          sendLog({
            msg: `${action}了 ${hosts.name}`,
            level: 'system',
          })
        }
        else {
          message.error(res.msg || `${action}失败`)
        }
      }).catch((err) => {
        message.error(err.msg || `${action}失败`)
      })
    },
  })
}

function handleDeleteHost(hosts: Hosts) {
  dialog.error({
    title: '删除 Hosts',
    content: `确定要删除 "${hosts.name}" 吗？此操作不可恢复。`,
    positiveText: '确定删除',
    negativeText: '取消',
    onPositiveClick: () => {
      deleteHost(hosts.id).then((res) => {
        if (res.code === 200) {
          message.success('删除成功')
          getData()
          sendLog({
            msg: `删除了 ${hosts.name}`,
            level: 'system',
          })
        }
        else {
          message.error(res.msg || '删除失败')
        }
      }).catch((err) => {
        message.error(err.msg || '删除失败')
      })
    },
  })
}

function addNewHosts() {
  hostsStore.show({ mode: 'create' })
}

function handleApplyToSystem() {
  dialog.warning({
    title: '应用到系统',
    content: '将所有已激活的 Hosts 规则写入系统 hosts 文件，需要管理员权限。确认继续？',
    positiveText: '确定',
    negativeText: '取消',
    onPositiveClick: () => {
      applyHostsToSystem().then((res) => {
        if (res.code === 200 && res.data.success) {
          message.success(res.data.message)
          sendLog({
            msg: `应用 hosts 到系统：${res.data.entries_count} 个规则组`,
            level: 'system',
          })
        }
        else {
          message.error(res.msg || '应用失败')
        }
      }).catch((err) => {
        message.error(err.msg || '应用失败，请确认是否有管理员权限')
      })
    },
  })
}

const onEditorChange = debounce((event: {
  originValue: string
  newValue: string
}, hosts: Hosts, _: number) => {
  sendLog({
    msg: `更新了${hosts.name}`,
    level: 'system',
  })
  updateHost(hosts.id, {
    content: event.newValue,
  }).then((res) => {
    if (res.code === 200) {
      hosts.content = res.data.content
      hosts.updated_at = res.data.updated_at
    }
  }).catch((err) => {
    message.error(err.msg || '保存失败')
  })
}, 300)

onBeforeMount(() => {
  getData()
  globalEventEmitter.on('hosts-created', getData)
})

onBeforeUnmount(() => {
  globalEventEmitter.off('hosts-created', getData)
})
</script>

<template>
  <div class="hosts-page">
    <!-- 顶部工具栏 -->
    <div class="toolbar">
      <div class="toolbar__left">
        <h2 class="toolbar__title">
          Hosts 管理
        </h2>
        <div class="toolbar__stats">
          <span class="toolbar__stat">
            <span class="toolbar__stat-value">{{ stats.total }}</span>
            <span class="toolbar__stat-label">总计</span>
          </span>
          <span class="toolbar__stat">
            <span class="toolbar__stat-value toolbar__stat-value--active">{{ stats.active }}</span>
            <span class="toolbar__stat-label">活跃</span>
          </span>
          <span class="toolbar__stat">
            <span class="toolbar__stat-value toolbar__stat-value--system">{{ stats.system }}</span>
            <span class="toolbar__stat-label">系统</span>
          </span>
        </div>
      </div>
      <div class="toolbar__right">
        <n-input
          v-model:value="searchKeyword"
          placeholder="搜索..."
          size="small"
          class="toolbar__search"
          clearable
        >
          <template #prefix>
            <n-icon :component="Search" />
          </template>
        </n-input>
        <n-button-group size="small">
          <n-button :loading="data.loading" @click="refreshData">
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
        </n-button-group>
        <n-button-group size="small">
          <n-button :loading="isLoadingSystemHosts" @click="viewSystemHosts">
            <template #icon>
              <n-icon :component="FileText" />
            </template>
            系统 Hosts
          </n-button>
          <n-button type="warning" @click="handleApplyToSystem">
            <template #icon>
              <n-icon :component="Upload" />
            </template>
            应用到系统
          </n-button>
        </n-button-group>
      </div>
    </div>

    <!-- 主内容区 -->
    <div class="hosts-content">
      <n-spin :show="data.loading">
        <!-- 有数据：编辑器视图 -->
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
                  <div class="flex justify-between items-center min-w-0">
                    <div class="flex items-center gap-2 min-w-0 overflow-hidden">
                      <h3 class="text-sm font-medium m-0 truncate">
                        {{ hosts.name }}
                      </h3>
                      <n-tag :type="hosts.is_active === 1 ? 'success' : 'default'" size="small">
                        {{ hosts.is_active === 1 ? '已启用' : '已禁用' }}
                      </n-tag>
                      <n-tag v-if="hosts.is_system === 1" type="warning" size="small">
                        系统
                      </n-tag>
                    </div>
                    <div class="flex items-center gap-2">
                      <n-button
                        size="small"
                        :type="hosts.is_active === 1 ? 'default' : 'primary'"
                        :disabled="hosts.is_system === 1"
                        @click="handleToggleHostStatus(hosts)"
                      >
                        <template #icon>
                          <n-icon :component="hosts.is_active === 1 ? EyeOff : Eye" />
                        </template>
                        {{ hosts.is_active === 1 ? '禁用' : '启用' }}
                      </n-button>
                      <n-button
                        size="small"
                        type="error"
                        :disabled="hosts.is_system === 1"
                        @click="handleDeleteHost(hosts)"
                      >
                        <template #icon>
                          <n-icon :component="Trash" />
                        </template>
                        删除
                      </n-button>
                    </div>
                  </div>
                  <div v-if="hosts.description" class="text-xs op-60 mt-1">
                    {{ hosts.description }}
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
                    readOnly: hosts.is_system === 1,
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
                            :class="hosts.is_active === 1 ? 'bg-green-500' : 'bg-gray-400'"
                          ></div>
                          <span class="tab__name">{{ hosts.name }}</span>
                        </div>
                        <div class="flex gap-1">
                          <n-tag v-if="hosts.is_system === 1" :bordered="false" type="warning" size="tiny">
                            系统
                          </n-tag>
                          <n-tag
                            :bordered="false"
                            :type="hosts.is_active === 1 ? 'success' : 'default'"
                            size="tiny"
                          >
                            {{ hosts.is_active === 1 ? '启用' : '禁用' }}
                          </n-tag>
                        </div>
                      </div>
                    </template>
                    <div>
                      <div>{{ hosts.name }}</div>
                      <div v-if="hosts.description" class="text-xs op-60">
                        {{ hosts.description }}
                      </div>
                      <div class="text-xs">
                        状态: {{ hosts.is_active === 1 ? '已启用' : '已禁用' }}
                        {{ hosts.is_system === 1 ? ' | 系统' : '' }}
                      </div>
                    </div>
                  </n-tooltip>
                </template>
              </n-tab-pane>
            </template>
          </n-tabs>
        </div>

        <!-- 搜索无结果 -->
        <div v-else-if="searchKeyword && data.hostsList.length > 0" class="empty-state">
          <div class="empty-state__icon">
            <n-icon :component="Search" :size="48" />
          </div>
          <h3 class="empty-state__title">
            未找到匹配的 Hosts
          </h3>
          <p class="empty-state__desc">
            没有找到包含 "{{ searchKeyword }}" 的配置文件
          </p>
          <n-button size="small" @click="searchKeyword = ''">
            清除搜索
          </n-button>
        </div>

        <!-- 空状态 -->
        <div v-else class="empty-state">
          <div class="empty-state__visual">
            <svg class="empty-state__svg" viewBox="0 0 200 160" fill="none" xmlns="http://www.w3.org/2000/svg">
              <!-- 文件图标 -->
              <rect x="55" y="20" width="90" height="110" rx="8" fill="currentColor" opacity="0.06" stroke="currentColor" stroke-opacity="0.15" stroke-width="1.5" />
              <rect x="65" y="45" width="50" height="4" rx="2" fill="currentColor" opacity="0.12" />
              <rect x="65" y="57" width="65" height="4" rx="2" fill="currentColor" opacity="0.12" />
              <rect x="65" y="69" width="40" height="4" rx="2" fill="currentColor" opacity="0.12" />
              <rect x="65" y="81" width="55" height="4" rx="2" fill="currentColor" opacity="0.12" />
              <rect x="65" y="93" width="30" height="4" rx="2" fill="currentColor" opacity="0.12" />
              <!-- 加号圆圈 -->
              <circle cx="145" cy="110" r="24" fill="var(--primary-color, #1677ff)" opacity="0.12" />
              <line x1="145" y1="100" x2="145" y2="120" stroke="var(--primary-color, #1677ff)" stroke-width="2.5" stroke-linecap="round" opacity="0.6" />
              <line x1="135" y1="110" x2="155" y2="110" stroke="var(--primary-color, #1677ff)" stroke-width="2.5" stroke-linecap="round" opacity="0.6" />
              <!-- 折角 -->
              <path d="M120 20L145 20L145 45L120 20Z" fill="currentColor" opacity="0.04" stroke="currentColor" stroke-opacity="0.1" stroke-width="1" />
            </svg>
          </div>
          <h3 class="empty-state__title">
            还没有 Hosts 配置
          </h3>
          <p class="empty-state__desc">
            创建你的第一个 Hosts 文件，开始管理域名解析规则
          </p>
          <div class="empty-state__actions">
            <n-button type="primary" @click="addNewHosts">
              <template #icon>
                <n-icon :component="Plus" />
              </template>
              创建 Hosts
            </n-button>
            <n-button :loading="isLoadingSystemHosts" @click="viewSystemHosts">
              <template #icon>
                <n-icon :component="FileText" />
              </template>
              查看系统 Hosts
            </n-button>
          </div>
        </div>
      </n-spin>
    </div>

    <!-- 系统 hosts 文件查看器 -->
    <n-modal
      v-model:show="isSystemHostsVisible"
      preset="card"
      title="系统 Hosts 文件（只读）"
      :style="{ width: '720px' }"
    >
      <editor
        v-model="systemHostsContent"
        :format="true"
        language="hosts"
        :options="{ readOnly: true }"
        style="height: 480px; border-radius: 6px; overflow: hidden;"
      />
    </n-modal>
  </div>
</template>

<style lang="less" scoped>
.hosts-page {
  display: flex;
  flex-direction: column;
  height: 100%;
}

// 工具栏
.toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 16px;
  border-bottom: 1px solid var(--n-border-color, rgba(255, 255, 255, 0.09));
  flex-shrink: 0;
  gap: 8px;

  &__left {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  &__title {
    font-size: 14px;
    font-weight: 600;
    margin: 0;
    white-space: nowrap;
  }

  &__stats {
    display: flex;
    gap: 12px;
  }

  &__stat {
    display: flex;
    align-items: baseline;
    gap: 3px;
  }

  &__stat-value {
    font-size: 14px;
    font-weight: 600;
    font-variant-numeric: tabular-nums;

    &--active {
      color: var(--n-color-success, #18a058);
    }

    &--system {
      color: var(--n-color-warning, #f0a020);
    }
  }

  &__stat-label {
    font-size: 11px;
    opacity: 0.5;
  }

  &__right {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-shrink: 1;
    min-width: 0;
    flex-wrap: wrap;
    justify-content: flex-end;
  }

  &__search {
    width: 120px;
  }
}

// 主内容
.hosts-content {
  flex: 1;
  overflow: hidden;

  :deep(.n-spin-container),
  :deep(.n-spin-content) {
    height: 100%;
  }
}

// Tab 编辑器视图
.container-tab {
  height: 100%;
}

.tab {
  :deep(.n-tabs-nav) {
    width: 170px;
  }

  :deep(.n-tab-pane) {
    overflow: hidden;
  }

  :deep(.n-tabs-tab) {
    padding: 8px 6px;
    min-height: 44px;
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
    max-width: 110px;
    font-size: 12px;
    font-weight: 500;
  }
}

.editor-header {
  border: 1px solid var(--n-border-color, rgba(255, 255, 255, 0.09));
  border-radius: 4px;
  padding: 10px 12px;
  margin-bottom: 10px;
  overflow: hidden;
  min-width: 0;
}

.editor {
  border: 1px solid var(--n-border-color, rgba(255, 255, 255, 0.09));
  border-radius: 4px;
  overflow: hidden;
}

// 空状态
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  min-height: 400px;
  padding: 40px 20px;

  &__visual {
    margin-bottom: 24px;
  }

  &__svg {
    width: 180px;
    height: 144px;
    color: var(--n-text-color, currentColor);
  }

  &__icon {
    width: 80px;
    height: 80px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    background: var(--n-color-primary, #1677ff);
    opacity: 0.08;
    margin-bottom: 20px;
    color: var(--n-text-color, currentColor);
  }

  &__title {
    font-size: 18px;
    font-weight: 600;
    margin: 0 0 8px;
  }

  &__desc {
    font-size: 13px;
    opacity: 0.5;
    margin: 0 0 24px;
    max-width: 320px;
    text-align: center;
    line-height: 1.6;
  }

  &__actions {
    display: flex;
    gap: 12px;
  }
}
</style>
