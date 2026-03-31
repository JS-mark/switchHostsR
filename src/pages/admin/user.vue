<script lang="ts" setup>
import type { DataTableColumns } from 'naive-ui'

import { useI18n } from 'vue-i18n'
import { h, onMounted, reactive, ref } from 'vue'
import {

  NButton,
  NForm,
  NFormItem,
  NInput,
  NSpace,
  NSwitch,
  NTag,
  useDialog,
  useMessage,
} from 'naive-ui'

import type { User } from '@/apis/public'

import { formatTimeV2 } from '@/utils'
import { deleteUserById, getAllUsers, updateUserInfo } from '@/apis'

defineOptions({
  name: 'AdminHome',
})

const { t } = useI18n()
const message = useMessage()
const dialogInstance = useDialog()
const isEditModalVisible = ref(false)
const editingUser = ref<User | null>(null)
const editForm = reactive({
  username: '',
  email: '',
  avatar: '',
})

function openEditModal(user: User) {
  editingUser.value = user
  editForm.username = user.username
  editForm.email = user.email || ''
  editForm.avatar = user.avatar || ''
  isEditModalVisible.value = true
}

function submitEdit() {
  if (!editingUser.value)
    return

  updateUserInfo(editingUser.value.id, {
    username: editForm.username || undefined,
    email: editForm.email || undefined,
    avatar: editForm.avatar || undefined,
  }).then((res: { code: number, msg: string }) => {
    if (res.code === 200) {
      message.success('用户信息更新成功')
      isEditModalVisible.value = false
      getData()
    }
    else {
      message.error(res.msg || '更新失败')
    }
  }).catch((err: { msg?: string }) => {
    message.error(err.msg || '更新失败')
  })
}

function handleSwitchAdmin(user: User, isAdmin: boolean) {
  updateUserInfo(user.id, {
    role: isAdmin ? 'admin' : 'user',
  }).then((res: { code: number, msg: string }) => {
    if (res.code === 200) {
      message.success(isAdmin ? '已设置为管理员' : '已取消管理员权限')
      getData()
    }
    else {
      message.error(res.msg || '操作失败')
    }
  }).catch((err: { msg?: string }) => {
    message.error(err.msg || '操作失败')
  })
}

function handleDeleteUser(user: User) {
  dialogInstance.error({
    title: '删除用户',
    content: `确定要删除用户 "${user.username}" 吗？此操作不可恢复。`,
    positiveText: '确定删除',
    negativeText: '取消',
    onPositiveClick: () => {
      deleteUserById(user.id).then((res: { code: number, msg: string }) => {
        if (res.code === 200) {
          message.success('删除成功')
          getData()
        }
        else {
          message.error(res.msg || '删除失败')
        }
      }).catch((err: { msg?: string }) => {
        message.error(err.msg || '删除失败')
      })
    },
  })
}

function createColumns(): DataTableColumns<User> {
  return [
    {
      title: 'ID',
      key: 'id',
      width: 60,
    },
    {
      title: '用户名',
      key: 'username',
    },
    {
      title: '邮箱',
      key: 'email',
      render(rowData) {
        return h('span', {}, rowData.email || '-')
      },
    },
    {
      title: '角色',
      key: 'is_admin',
      render(rowData) {
        const isAdmin = rowData.is_admin === true
        return h(NTag, {
          type: isAdmin ? 'warning' : 'info',
        }, {
          default: () => isAdmin ? '管理员' : '普通用户',
        })
      },
    },
    {
      title: '创建时间',
      key: 'created_at',
      render(rowData) {
        return h('span', {}, formatTimeV2(rowData.created_at, 'YYYY-MM-DD HH:mm:ss'))
      },
    },
    {
      title: '修改时间',
      key: 'updated_at',
      render(rowData) {
        return h('span', {}, formatTimeV2(rowData.updated_at, 'YYYY-MM-DD HH:mm:ss'))
      },
    },
    {
      title: '管理员',
      key: 'admin_switch',
      width: 100,
      render(rowData) {
        return h(NSwitch, {
          'value': rowData.is_admin === true,
          'round': false,
          'on-update:value': (val: boolean) => {
            handleSwitchAdmin(rowData, val)
          },
        })
      },
    },
    {
      title: '操作',
      key: 'actions',
      width: 180,
      render(row) {
        return h(NSpace, {}, () => [
          h(
            NButton,
            {
              strong: true,
              tertiary: true,
              size: 'small',
              type: 'primary',
              onClick: () => openEditModal(row),
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
              onClick: () => handleDeleteUser(row),
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
})

function getData() {
  data.loading = true
  getAllUsers().then((res: { code: number, data: User[], msg: string }) => {
    if (res.code === 200) {
      data.list = res.data
    }
    else {
      message.error(res.msg || '获取用户列表失败')
      data.list = []
    }
  }).catch((err: { msg?: string }) => {
    console.error(err)
    data.list = []
  }).finally(() => {
    data.loading = false
  })
}

onMounted(() => {
  getData()
})
</script>

<template>
  <div class="admin-user-page">
    <n-data-table
      :columns="data.columns"
      :data="data.list"
      :loading="data.loading"
      bordered
    />

    <!-- 编辑用户弹窗 -->
    <n-modal
      v-model:show="isEditModalVisible"
      preset="dialog"
      title="编辑用户"
      positive-text="保存"
      negative-text="取消"
      @positive-click="submitEdit"
    >
      <n-form label-placement="left" label-width="80">
        <n-form-item label="用户名">
          <n-input v-model:value="editForm.username" placeholder="请输入用户名" />
        </n-form-item>
        <n-form-item label="邮箱">
          <n-input v-model:value="editForm.email" placeholder="请输入邮箱" />
        </n-form-item>
        <n-form-item label="头像">
          <n-input v-model:value="editForm.avatar" placeholder="请输入头像 URL" />
        </n-form-item>
      </n-form>
    </n-modal>
  </div>
</template>

<style lang="less" scoped>
.admin-user-page {
  padding: 10px;
}
</style>
