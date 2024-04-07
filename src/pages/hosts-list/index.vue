<script lang="ts" setup>
import { onBeforeMount, reactive } from 'vue'
import { useMessage } from 'naive-ui'
import { type Hosts, getAllHosts } from '@/apis'

defineOptions({
  name: 'HostsList',
})

const message = useMessage()
const data = reactive({
  loading: true,
  curId: -1,
  curHostsData: {} as Hosts,
  hostsList: [] as Hosts[],
})

const getData = () => {
  data.loading = true
  // 调用接口获取数据
  getAllHosts({ page: 1, pageSize: 1000 }).then((res) => {
    if (res.code === 10000)
      data.hostsList = res.data
    else return Promise.reject(res)
  }).catch((err) => {
    data.hostsList = new Array(100).fill('').map((_, index) => {
      return {
        id: index,
        name: `demo2${index}`,
        hosts_type: 0,
        content: `/etc/hosts${index}`,
        status: 0,
        is_del: 0,
        is_readonly: Number(index % 2 === 0),
        created_at: '2023-03-29 10:50:25',
        updated_at: '2023-03-29 10:50:25',
        hosts_refresh_time: 3600,
        last_refresh_time: '2023-03-29 10:50:25',
      }
    })
    data.curId = data.hostsList[0].id
    message.error(err.msg || '获取失败')
  }).finally(() => {
    data.loading = false
  })
}

const onChangeTab = (value: number) => {
  data.curId = value
}

const onEditorChange = (event: { originValue: string, newValue: string }, hosts: Hosts, index: number) => {
  // 更新数据

}

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
