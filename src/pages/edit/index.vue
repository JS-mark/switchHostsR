<script lang="ts" setup>
import { ref, onMounted } from 'vue'
import { useMessage } from 'naive-ui'
import { Save, Download, Upload, FileText, Settings } from '@vicons/tabler'

defineOptions({
  name: 'EditPage',
})

const message = useMessage()
const content = ref('')
const fileName = ref('hosts')
const isModified = ref(false)
const useFormat = ref(true)

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

  // 这里应该调用保存 API
  message.success('保存成功')
  isModified.value = false
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
  const input = document.createElement('input')
  input.type = 'file'
  input.accept = '.txt,.hosts'
  input.onchange = (e) => {
    const file = (e.target as HTMLInputElement).files?.[0]
    if (file) {
      const reader = new FileReader()
      reader.onload = (e) => {
        content.value = e.target?.result as string
        fileName.value = file.name.replace(/\.[^/.]+$/, '')
        isModified.value = true
        message.success('导入成功')
      }
      reader.readAsText(file)
    }
  }
  input.click()
}

// 新建文件
function newFile() {
  if (isModified.value) {
    // 这里应该弹出确认对话框
    if (!confirm('当前文件已修改，是否放弃修改？')) {
      return
    }
  }

  content.value = ''
  fileName.value = 'hosts'
  isModified.value = false
  message.info('已创建新文件')
}

// 切换格式化
function toggleFormat() {
  useFormat.value = !useFormat.value
  message.info(`格式化已${useFormat.value ? '开启' : '关闭'}`)
}

onMounted(() => {
  // 初始化示例内容
  content.value = `# Hosts 文件示例\n# 这是一个注释\n\n# 本地主机\n127.0.0.1 localhost\n::1 localhost\n\n# 自定义域名映射\n# 192.168.1.100 example.local\n# 10.0.0.1 api.local`
})
</script>

<template>
  <n-card :bordered="false" class="h-full">
    <template #header>
      <div class="flex justify-between items-center">
        <div class="flex items-center gap-4">
          <h2 class="text-lg font-semibold m-0">Hosts 编辑器</h2>
          <div class="flex items-center gap-2 text-sm text-gray-500">
            <n-icon :component="FileText" />
            <span>{{ fileName }}</span>
            <n-tag v-if="isModified" type="warning" size="small">
              未保存
            </n-tag>
          </div>
        </div>
        <div class="flex items-center gap-2">
          <n-button @click="newFile">
            <template #icon>
              <n-icon :component="FileText" />
            </template>
            新建
          </n-button>
          <n-button @click="importFile">
            <template #icon>
              <n-icon :component="Upload" />
            </template>
            导入
          </n-button>
          <n-button @click="exportFile">
            <template #icon>
              <n-icon :component="Download" />
            </template>
            导出
          </n-button>
          <n-button type="primary" @click="saveFile" :disabled="!isModified">
            <template #icon>
              <n-icon :component="Save" />
            </template>
            保存
          </n-button>
          <n-button @click="toggleFormat">
            <template #icon>
              <n-icon :component="Settings" />
            </template>
            {{ useFormat ? '关闭格式化' : '开启格式化' }}
          </n-button>
        </div>
      </div>
    </template>

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
        @on-change="onContentChange"
      />
    </div>
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
  border-radius: 6px 6px 0 0;
  padding: 8px 16px;
}

.editor {
  flex: 1;
  border: 1px solid #e0e0e0;
  border-top: none;
  border-radius: 0 0 6px 6px;
  overflow: hidden;
}

:deep(.n-card-header) {
  padding: 16px 24px;
  border-bottom: 1px solid #e0e0e0;
}

:deep(.n-card__content) {
  padding: 16px 24px;
}
</style>
