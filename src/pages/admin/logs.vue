<script lang="ts" setup>
import { useI18n } from 'vue-i18n'
import { formatTimeV2, sendLog } from '@/utils'
import { getAllLogs } from '@/apis'
import { h, onMounted, reactive } from 'vue'
import { type DataTableColumns, NTag } from 'naive-ui'

defineOptions({
  name: 'AdminLogs',
})

interface Logs {
  id: number
  content: string
  log_type: number
  user_email: string
  user_name: string
  user_avatar_url: string
  password: string
  created_at: number
  created_by: number
}
const { t } = useI18n()

function createColumns(): DataTableColumns<Logs> {
  return [
    {
      type: 'expand',
      renderExpand: (rowData) => {
        return rowData.content
      },
    },
    {
      title: 'ID',
      key: 'id',
    },
    {
      title: '日志类型',
      key: 'log_type',
      render(rowData, _) {
        const getTag = (type: number) => {
          let tagType = 'default'
          let text = ''
          switch (type) {
            case 0:
              tagType = 'success'
              text = '更新'
              break
            case -1:
              tagType = 'warning'
              text = '未知'

              break
            case 1:
              tagType = 'error'
              text = '删除'
              break
            case 2:
              tagType = 'success'
              text = '增加'
              break
          }
          return {
            tagType,
            text,
          }
        }
        const { tagType, text } = getTag(rowData.log_type)
        return h(NTag, {
          bordered: false,
          type: tagType as typeof NTag['type'],
        }, () => t(text))
      },
    },
    {
      title: t('用户名'),
      key: 'user_name',
    },
    {
      title: t('头像'),
      key: 'user_avatar_url',
      render(rowData, _) {
        return h('img', {
          src: rowData.user_avatar_url,
          style: 'width: 32px; height: 32px; border-radius: 50%;',
        })
      },
    },
    {
      title: t('邮箱'),
      key: 'user_email',
    },
    {
      title: t('创建时间'),
      key: 'created_at',
      render(rowData, _) {
        return h('span', {}, formatTimeV2(Number(rowData.created_at), 'YYYY-MM-DD HH:mm:ss'))
      },
    },
  ]
}

const data = reactive({
  list: [] as Logs[],
  columns: createColumns(),
  loading: true,
  pagination: {
    page: 1,
    pageSize: 5,
    itemCount: 0,
    showSizePicker: true,
    pageSizes: [5, 10, 15, 20, 30],
    onChange: (page: number) => {
      data.pagination.page = page
      getData()
    },
    onUpdatePageSize: (pageSize: number) => {
      data.pagination.pageSize = pageSize
      data.pagination.page = 1
      getData()
    },
  },
})

function getData() {
  data.loading = true
  getAllLogs({
    page: data.pagination.page,
    pageSize: data.pagination.pageSize,
  }).then((res: any) => {
    console.log('e12e', res)
    data.list = res.data.list
    data.pagination.itemCount = res.data.total
  }).catch((err) => {
    data.list = []
    data.pagination.itemCount = 0
    // 发送错误日志
    sendLog({
      msg: JSON.stringify({
        error: {
          msg: err.message,
          stack: err.stack,
        },
      }),
      level: 'error',
    })
  }).finally(() => {
    data.loading = false
  })
}

onMounted(() => {
  getData()
})
</script>

<template>
  <n-data-table
    :columns="data.columns"
    :data="data.list"
    :loading="data.loading"
    :row-key="(row) => row.id"
    :pagination="data.pagination"
    bordered
    remote
  />
</template>
