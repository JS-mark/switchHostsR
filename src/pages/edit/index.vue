<script lang="ts" setup>
import { useMessage } from 'naive-ui'
import { computed, onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ArrowLeft, Download, FileText, Save, Upload } from '@vicons/tabler'

import type { Hosts } from '@/apis'

import { getHostById, updateHost } from '@/apis'

defineOptions({
  name: 'EditPage',
})

const route = useRoute()
const router = useRouter()
const message = useMessage()
const content = ref('')
const hostData = ref<Hosts | null>(null)
const isLoading = ref(true)
const isModified = ref(false)
const isSaving = ref(false)
const useFormat = ref(true)

const hostId = computed(() => {
  const id = route.params.id
  return typeof id === 'string' ? Number.parseInt(id, 10) : Number.NaN
})

const fileName = computed(() => {
  return hostData.value?.name || 'hosts'
})

const isReadonly = computed(() => {
  return hostData.value?.is_system === 1
})

// 监听内容变化
function onContentChange() {
  isModified.value = true
}

// 保存文件
function saveFile() {
  if (!content.value.trim()) {
    message.warning('内容不能为空')
    return
  }

  if (!hostData.value) {
    message.error('未加载 Host 数据')
    return
  }

  isSaving.value = true
  updateHost(hostData.value.id, {
    content: content.value,
  }).then((res) => {
    if (res.code === 200) {
      hostData.value = res.data
      message.success('保存成功')
      isModified.value = false
    }
    else {
      message.error(res.msg || '保存失败')
    }
  }).catch((err) => {
    message.error(err.msg || '保存失败')
  }).finally(() => {
    isSaving.value = false
  })
}

// 导出文件
function exportFile() {
  if (!content.value.trim()) {
    message.warning('内容不能为空')
    return
  }

  const blob = new Blob([content.value], { type: 'text/plain' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = fileName.value || 'hosts'
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
  URL.revokeObjectURL(url)

  message.success('导出成功')
}

// 导入文件
function importFile() {
  if (isReadonly.value) {
    message.warning('系统 Hosts 为只读，无法导入')
    return
  }
  const input = document.createElement('input')
  input.type = 'file'
  input.accept = '.txt,.hosts'
  input.onchange = (e) => {
    const file = (e.target as HTMLInputElement).files?.[0]
    if (file) {
      const reader = new FileReader()
      reader.onload = (evt) => {
        content.value = evt.target?.result as string
        isModified.value = true
        message.success('导入成功')
      }
      reader.readAsText(file)
    }
  }
  input.click()
}

// 返回列表
function goBack() {
  router.push({ name: 'HostsList' })
}

// 切换格式化
function toggleFormat() {
  useFormat.value = !useFormat.value
  message.info(`格式化已${useFormat.value ? '开启' : '关闭'}`)
}

// 加载 host 数据
function loadHostData() {
  if (Number.isNaN(hostId.value)) {
    message.error('无效的 Host ID')
    isLoading.value = false
    return
  }

  isLoading.value = true
  getHostById(hostId.value).then((res) => {
    if (res.code === 200) {
      hostData.value = res.data
      content.value = res.data.content
    }
    else {
      message.error(res.msg || '加载 Host 数据失败')
    }
  }).catch((err) => {
    message.error(err.msg || '加载 Host 数据失败')
  }).finally(() => {
    isLoading.value = false
  })
}

onMounted(() => {
  loadHostData()
})
</script>

<template>
  <n-card :bordered="false" class="h-full">
    <template #header>
      <div class="flex justify-between items-center">
        <div class="flex items-center gap-3">
          <n-button text size="small" @click="goBack">
            <template #icon>
              <n-icon :component="ArrowLeft" />
            </template>
          </n-button>
          <h2 class="text-sm font-semibold m-0">
            Hosts 编辑器
          </h2>
          <div class="flex items-center gap-2 text-xs text-gray-500">
            <n-icon :component="FileText" />
            <span>{{ fileName }}</span>
            <n-tag v-if="isModified" type="warning" size="small">
              未保存
            </n-tag>
            <n-tag v-if="isReadonly" type="info" size="small">
              只读
            </n-tag>
          </div>
        </div>
        <div class="flex items-center gap-2">
          <n-button size="small" :disabled="isReadonly" @click="importFile">
            <template #icon>
              <n-icon :component="Upload" />
            </template>
            导入
          </n-button>
          <n-button size="small" @click="exportFile">
            <template #icon>
              <n-icon :component="Download" />
            </template>
            导出
          </n-button>
          <n-button
            size="small"
            type="primary"
            :disabled="!isModified || isReadonly"
            :loading="isSaving"
            @click="saveFile"
          >
            <template #icon>
              <n-icon :component="Save" />
            </template>
            保存
          </n-button>
        </div>
      </div>
    </template>

    <n-spin :show="isLoading">
      <div class="editor-container">
        <div class="editor-info">
          <div class="flex justify-between items-center text-xs text-gray-500 mb-2">
            <span>文件名: {{ fileName }}</span>
            <span>字符数: {{ content.length }}</span>
          </div>
        </div>

        <editor
          v-model="content"
          :format="useFormat"
          class="editor"
          language="hosts"
          :options="{
            readOnly: isReadonly,
          }"
          @on-change="onContentChange"
        />
      </div>
    </n-spin>
  </n-card>
</template>

<style lang="less" scoped>
.editor-container {
  height: calc(100vh - 200px);
  display: flex;
  flex-direction: column;
}

.editor-info {
  background: #fafafa;
  border: 1px solid #e0e0e0;
  border-radius: 4px 4px 0 0;
  padding: 6px 12px;
}

.editor {
  flex: 1;
  border: 1px solid #e0e0e0;
  border-top: none;
  border-radius: 0 0 4px 4px;
  overflow: hidden;
}

:deep(.n-card-header) {
  padding: 10px 16px;
  border-bottom: 1px solid #e0e0e0;
}

:deep(.n-card__content) {
  padding: 10px 16px;
}
</style>
