<script lang="ts" setup>
import type { DataTableColumns } from 'naive-ui'

import { NTag } from 'naive-ui'
import { useI18n } from 'vue-i18n'
import { h, onMounted, reactive } from 'vue'

import { getAllLogs } from '@/apis'
import { formatTimeV2, sendLog } from '@/utils'

defineOptions({
  name: 'AdminLogs',
})

interface Logs {
  id: number
  user_id: number
  action: string
  target_type: string
  target_id: number | null
  details: string | null
  username: string
  created_at: number
}
const { t } = useI18n()

function createColumns(): DataTableColumns<Logs> {
  return [
    {
      type: 'expand',
      renderExpand: (rowData) => {
        return rowData.details || '-'
      },
    },
    {
      title: 'ID',
      key: 'id',
    },
    {
      title: t('操作'),
      key: 'action',
    },
    {
      title: '目标类型',
      key: 'target_type',
      render(rowData) {
        const typeMap: Record<string, string> = {
          system: '系统',
          hosts: 'Hosts',
          user: '用户',
        }
        return h(NTag, {
          bordered: false,
          type: 'info',
          size: 'small',
        }, () => typeMap[rowData.target_type] || rowData.target_type)
      },
    },
    {
      title: t('用户名'),
      key: 'username',
    },
    {
      title: t('创建时间'),
      key: 'created_at',
      render(rowData) {
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
    pageSize: 10,
    showSizePicker: true,
    pageSizes: [5, 10, 15, 20, 30],
  },
})

function getData() {
  data.loading = true
  getAllLogs().then((res: any) => {
    if (res.code === 200) {
      data.list = res.data || []
    }
    else {
      data.list = []
    }
  }).catch((err) => {
    data.list = []
    sendLog({
      msg: `获取日志失败: ${err.message || err.msg || '未知错误'}`,
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
  />
</template>
