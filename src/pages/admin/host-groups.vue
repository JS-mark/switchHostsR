<script lang="ts" setup>
import type { DataTableColumns } from 'naive-ui'

import { h, onMounted, reactive, ref } from 'vue'
import {

  NButton,
  NSpace,
  NSwitch,
  NTag,
  useDialog,
  useMessage,
} from 'naive-ui'

import type { HostGroup } from '@/apis'

import { formatTimeV2 } from '@/utils'
import {
  createHostGroup,
  deleteHostGroup,
  getHostGroups,

  toggleGroupActive,
  updateHostGroup,
} from '@/apis'

defineOptions({
  name: 'AdminHostGroups',
})

const message = useMessage()
const dialogInstance = useDialog()

// 编辑弹窗
const isEditModalVisible = ref(false)
const isCreateMode = ref(true)
const editingGroupId = ref<number | null>(null)
const editForm = reactive({
  name: '',
  description: '',
})

function openCreateModal() {
  isCreateMode.value = true
  editingGroupId.value = null
  editForm.name = ''
  editForm.description = ''
  isEditModalVisible.value = true
}

function openEditModal(group: HostGroup) {
  isCreateMode.value = false
  editingGroupId.value = group.id
  editForm.name = group.name
  editForm.description = group.description || ''
  isEditModalVisible.value = true
}

function submitForm() {
  if (!editForm.name.trim()) {
    message.warning('请输入组名')
    return
  }

  if (isCreateMode.value) {
    createHostGroup({
      name: editForm.name,
      description: editForm.description || undefined,
    }).then((res) => {
      if (res.code === 200) {
        message.success('创建成功')
        isEditModalVisible.value = false
        getData()
      }
      else {
        message.error(res.msg || '创建失败')
      }
    }).catch((err) => {
      message.error(err.msg || '创建失败')
    })
  }
  else if (editingGroupId.value !== null) {
    updateHostGroup(editingGroupId.value, {
      name: editForm.name,
      description: editForm.description || undefined,
    }).then((res) => {
      if (res.code === 200) {
        message.success('更新成功')
        isEditModalVisible.value = false
        getData()
      }
      else {
        message.error(res.msg || '更新失败')
      }
    }).catch((err) => {
      message.error(err.msg || '更新失败')
    })
  }
}

function handleToggleActive(group: HostGroup) {
  toggleGroupActive(group.id).then((res) => {
    if (res.code === 200) {
      const isNowActive = res.data.is_active === 1
      message.success(isNowActive ? '已启用' : '已禁用')
      getData()
    }
    else {
      message.error(res.msg || '操作失败')
    }
  }).catch((err) => {
    message.error(err.msg || '操作失败')
  })
}

function handleDelete(group: HostGroup) {
  dialogInstance.error({
    title: '删除主机组',
    content: `确定要删除主机组 "${group.name}" 吗？此操作不可恢复。`,
    positiveText: '确定删除',
    negativeText: '取消',
    onPositiveClick: () => {
      deleteHostGroup(group.id).then((res) => {
        if (res.code === 200) {
          message.success('删除成功')
          getData()
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

function createColumns(): DataTableColumns<HostGroup> {
  return [
    {
      title: 'ID',
      key: 'id',
      width: 60,
    },
    {
      title: '组名',
      key: 'name',
    },
    {
      title: '描述',
      key: 'description',
      render(rowData) {
        return h('span', { class: 'text-gray-500' }, rowData.description || '-')
      },
    },
    {
      title: '状态',
      key: 'is_active',
      width: 100,
      render(rowData) {
        return h(NTag, {
          type: rowData.is_active === 1 ? 'success' : 'default',
          size: 'small',
        }, {
          default: () => rowData.is_active === 1 ? '启用' : '禁用',
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
      title: '激活',
      key: 'toggle',
      width: 80,
      render(rowData) {
        return h(NSwitch, {
          'value': rowData.is_active === 1,
          'round': false,
          'on-update:value': () => handleToggleActive(rowData),
        })
      },
    },
    {
      title: '操作',
      key: 'actions',
      width: 160,
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
            { default: () => '编辑' },
          ),
          h(
            NButton,
            {
              strong: true,
              tertiary: true,
              type: 'error',
              size: 'small',
              onClick: () => handleDelete(row),
            },
            { default: () => '删除' },
          ),
        ])
      },
    },
  ]
}

const data = reactive({
  list: [] as HostGroup[],
  columns: createColumns(),
  loading: true,
})

function getData() {
  data.loading = true
  getHostGroups().then((res) => {
    if (res.code === 200) {
      data.list = res.data
    }
    else {
      message.error(res.msg || '获取主机组失败')
      data.list = []
    }
  }).catch((err) => {
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
  <div class="host-groups-page">
    <div class="mb-4 flex justify-end">
      <n-button size="small" type="primary" @click="openCreateModal">
        新建主机组
      </n-button>
    </div>

    <n-data-table
      :columns="data.columns"
      :data="data.list"
      :loading="data.loading"
      bordered
    />

    <!-- 创建/编辑弹窗 -->
    <n-modal
      v-model:show="isEditModalVisible"
      preset="dialog"
      :title="isCreateMode ? '新建主机组' : '编辑主机组'"
      positive-text="保存"
      negative-text="取消"
      @positive-click="submitForm"
    >
      <n-form label-placement="left" label-width="60">
        <n-form-item label="组名">
          <n-input v-model:value="editForm.name" placeholder="请输入主机组名称" />
        </n-form-item>
        <n-form-item label="描述">
          <n-input
            v-model:value="editForm.description"
            type="textarea"
            placeholder="可选，输入描述"
            :rows="3"
          />
        </n-form-item>
      </n-form>
    </n-modal>
  </div>
</template>

<style lang="less" scoped>
.host-groups-page {
  padding: 10px;
}

.mb-4 {
  margin-bottom: 10px;
}
</style>
