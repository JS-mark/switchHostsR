<script lang="ts" setup>
import { useI18n } from 'vue-i18n'
import { formatTimeV2 } from '@/utils'
import { invoke } from '@tauri-apps/api'
import { h, onMounted, reactive } from 'vue'

import { type DataTableColumns, NButton, NSpace, NSwitch, NTag, useMessage } from 'naive-ui'

defineOptions({
  name: 'AdminHome',
})

interface User {
  id: number
  name: string
  email: string
  status: number
  user_level: number
  password: string
  is_del: number
  is_third: number
  third_account_uid: string
  created_at: string
  updated_at: string
}
const { t } = useI18n()
const message = useMessage()

const editUser = (user: User) => {
  message.info(`编辑用户：${user.name}`)
}

const switchUser = (user: User, status: boolean) => {
  message.info(`删除用户：${user.name}, ${status}`)
}

const delUser = (user: User) => {
  message.info(`删除用户：${user.name}`)
}

function createColumns(): DataTableColumns<User> {
  return [
    {
      title: 'ID',
      key: 'id',
    },
    {
      title: '用户名',
      key: 'name',
    },
    {
      title: '邮箱',
      key: 'email',
    },
    {
      title: '三方账户',
      key: ' is_third',
      render(rowData, _) {
        const type = rowData.is_third ? 'success' : 'warning'
        return h(NTag, {
          type,
        }, rowData.is_third ? '是' : '否')
      },
    },
    {
      title: '创建时间',
      key: 'created_at',
      render(rowData, _) {
        return h('span', {}, formatTimeV2(rowData.created_at, 'YYYY-MM-DD HH:mm:ss'))
      },
    },
    {
      title: '修改时间',
      key: 'updated_at',
      render(rowData, _) {
        return h('span', {}, formatTimeV2(rowData.updated_at, 'YYYY-MM-DD HH:mm:ss'))
      },
    },
    {
      title: '状态',
      key: 'status',
      render(rowData, _) {
        return h(NSwitch, {
          'modelValue': rowData.status === 1,
          'round': false,
          'checkedValue': t('启用'),
          'uncheckedValue': t('禁用'),
          'on-update:value': (val: boolean) => {
            switchUser(rowData, val)
          },
        })
      },
    },
    {
      title: 'Action',
      key: 'actions',
      render(row) {
        return h(NSpace, {}, () => [
          h(
            NButton,
            {
              strong: true,
              tertiary: true,
              size: 'small',
              type: 'primary',
              onClick: () => editUser(row),
            },
            { default: () => t('edit_user') },
          ),
          h(
            NButton,
            {
              strong: true,
              tertiary: true,
              type: 'error',
              size: 'small',
              onClick: () => delUser(row),
            },
            { default: () => t('del_user') },
          ),
        ])
      },
    },
  ]
}

const data = reactive({
  list: [] as User[],
  columns: createColumns(),
  loading: true,
  pagination: {
    page: 1,
    pageSize: 10,
    itemCount: 0,
    showSizePicker: true,
    pageSizes: [15, 20, 30],
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
  invoke('get_all_users', {
    page: data.pagination.page,
    pageSize: data.pagination.pageSize,
  }).then((res: any) => {
    data.list = res.data.list
    data.pagination.itemCount = res.data.total
  }).catch((err) => {
    console.error(err)
    data.list = []
    data.pagination.itemCount = 0
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
    :pagination="data.pagination"
    bordered
  />
</template>
