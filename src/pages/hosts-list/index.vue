<script lang="ts" setup>
import { sendLog } from '@/utils'
import { debounce } from 'lodash-es'
import { useMessage } from 'naive-ui'
import { onBeforeMount, reactive } from 'vue'
import { getAllHosts, type Hosts, updateHostsData } from '@/apis'

defineOptions({
  name: 'HostsList',
})

const message = useMessage()
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

function getData() {
  data.loading = true
  // 调用接口获取数据
  getAllHosts({ page: 1, pageSize: 1000 }).then((res) => {
    if (res.code === 10000) {
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
    <n-spin :show="data.loading">
      <div v-if="data.hostsList.length > 0" class="container-tab">
        <n-tabs
          type="line"
          animated
          placement="left"
          class="tab h-full"
          :on-update:value="onChangeTab"
        >
          <template v-for="(hosts, index) in data.hostsList" :key="`hosts__${hosts.id}`">
            <n-tab-pane :name="hosts.id" :tab="hosts.id">
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
                    <div class="w-full flex justify-center items-end">
                      <span class="tab__name">{{ hosts.name }}</span>
                      <n-tag v-if="hosts.is_readonly" :bordered="false" type="warning" size="small">
                        只读
                      </n-tag>
                    </div>
                  </template>
                  {{ hosts.name }}
                </n-tooltip>
              </template>
            </n-tab-pane>
          </template>
        </n-tabs>
      </div>
      <n-result v-else status="404" title="404 资源不存在" description="生活总归带点荒谬">
        <template #footer>
          <n-button>找点乐子吧</n-button>
        </template>
      </n-result>
    </n-spin>
  </n-card>
</template>

<style lang="less" scoped>
.container-tab {
  height: calc(100vh - 142px);
}
.tab {

  & .tab__name {
    display: inline-block;
    margin-right: 10px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 80px;
  }
}

.editor {
  margin-top: 20px;
}
</style>
