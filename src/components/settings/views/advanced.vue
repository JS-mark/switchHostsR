<script lang="ts" setup>
import { useDialog, useMessage } from 'naive-ui'
import { computed, onMounted, reactive, ref, watchEffect } from 'vue'

import type { BackupInfo } from '@/apis'
import type { SettingSpace } from '@/store/useSettings'

import { APP_NAME } from '@/utils/constant'
import { useSettingsStore } from '@/store/useSettings'
import { createBackup, getBackups, restoreBackup } from '@/apis'
import { formatTimeV2, openDirectory, openFile, sendLog } from '@/utils'

import AdvancedCell from './components/advanced-cell.vue'

defineOptions({
  name: 'Advanced',
})
const store = useSettingsStore()
const message = useMessage()
const dialog = useDialog()
const model = reactive<SettingSpace.Advanced>({
  canSendData: false,
  hostsPath: '',
  SwitchHostsRPath: '',
})

const backups = ref<BackupInfo[]>([])
const isLoadingBackups = ref(false)
const isCreatingBackup = ref(false)
const selectedBackupId = ref<number | null>(null)

const backupOptions = computed(() => {
  return backups.value.map(b => ({
    label: `${b.name} (${formatTimeV2(b.created_at * 1000, 'YYYY-MM-DD HH:mm:ss')})`,
    value: b.id,
  }))
})

const backupDirTip = computed(() => {
  return `${model.SwitchHostsRPath}/backups`
})

function loadBackups() {
  isLoadingBackups.value = true
  getBackups().then((res) => {
    if (res.code === 200) {
      backups.value = res.data || []
      if (selectedBackupId.value === null && backups.value.length > 0)
        selectedBackupId.value = backups.value[0].id
    }
    else {
      message.error(res.msg || '获取备份列表失败')
    }
  }).catch((err) => {
    message.error(err.msg || '获取备份列表失败')
  }).finally(() => {
    isLoadingBackups.value = false
  })
}

function handleCreateBackup() {
  isCreatingBackup.value = true
  const name = `手动备份 ${formatTimeV2(Date.now(), 'YYYY-MM-DD HH:mm:ss')}`
  createBackup({ name }).then((res) => {
    if (res.code === 200) {
      message.success('备份创建成功')
      loadBackups()
    }
    else {
      message.error(res.msg || '备份创建失败')
    }
  }).catch((err) => {
    message.error(err.msg || '备份创建失败')
  }).finally(() => {
    isCreatingBackup.value = false
  })
}

function handleRestoreBackup() {
  if (selectedBackupId.value === null) {
    message.warning('请选择要恢复的备份')
    return
  }
  const target = backups.value.find(b => b.id === selectedBackupId.value)
  if (!target) {
    message.warning('备份不存在')
    return
  }
  dialog.warning({
    title: '恢复备份',
    content: `确定要恢复备份 "${target.name}" 吗？此操作会覆盖当前数据库内容。`,
    positiveText: '确定恢复',
    negativeText: '取消',
    onPositiveClick: () => {
      restoreBackup(target.id).then((res) => {
        if (res.code === 200) {
          message.success('恢复成功')
          message.info('建议重启应用以确保数据与登录态刷新')
        }
        else {
          message.error(res.msg || '恢复失败')
        }
      }).catch((err) => {
        message.error(err.msg || '恢复失败')
      })
    },
  })
}

function onChecked(event: boolean) {
  store.setCanSendData(event)
  model.canSendData = event
}

function openFileByPath(file?: string) {
  openFile(file)
}

function changeFile(file?: string) {
  // 打开文件夹
  openDirectory(file).then((res) => {
    res
    && store.setSettingsByData({
      advanced: {
        canSendData: model.canSendData,
        hostsPath: model.hostsPath,
        SwitchHostsRPath: `${res}/.${APP_NAME}`,
      },
    })
  })
  sendLog({
    msg: JSON.stringify({
      msg: '改变储存文件',
      file,
    }),
  })
}

watchEffect(() => {
  for (const [key, value] of Object.entries(store.advanced))
    Reflect.set(model, key, value)
})

onMounted(() => {
  loadBackups()
})
</script>

<template>
  <section class="main pt-10px">
    <!-- 标题 -->
    <AdvancedCell :title="$t('settings.advanced.title')">
      <!-- 提示 -->
      <span class="tip">
        {{ $t("settings.advanced.tip") }}
      </span>
      <p>
        <!-- 确认按钮提示 -->
        <n-checkbox :checked="model.canSendData" :on-update:checked="onChecked">
          {{ $t("好的，发送匿名的使用数据") }}
        </n-checkbox>
      </p>
    </AdvancedCell>

    <AdvancedCell :title="$t('我的 Hosts 文件在哪里？')">
      <div class="row a-center">
        <span class="tip">{{ $t("你的 Hosts 文件在：") }}</span>
        <n-tooltip placement="bottom" trigger="hover">
          <template #trigger>
            <n-button size="small" quaternary type="primary" @click="openFile(model.hostsPath)">
              {{ model.hostsPath }}
            </n-button>
          </template>
          <span>{{ $t("点击打开") }}</span>
        </n-tooltip>
      </div>
    </AdvancedCell>

    <AdvancedCell :title="$t('我的数据储存在哪里？')">
      <div class="row a-center">
        <span class="tip">
          {{ $t("你的数据文件在：") }}
        </span>
        <n-tooltip placement="bottom" trigger="hover">
          <template #trigger>
            <n-button size="small" quaternary type="primary" @click="openFileByPath(model.SwitchHostsRPath)">
              {{ model.SwitchHostsRPath }}
            </n-button>
          </template>
          <span>{{ $t("点击打开") }}</span>
        </n-tooltip>
        <n-button size="small" quaternary type="primary" @click="changeFile(model.SwitchHostsRPath)">
          {{ $t("change") }}
        </n-button>
      </div>
    </AdvancedCell>

    <AdvancedCell title="备份与恢复">
      <div class="row a-center">
        <n-space align="center">
          <n-button size="small" type="primary" :loading="isCreatingBackup" @click="handleCreateBackup">
            创建备份
          </n-button>
          <n-button size="small" :loading="isLoadingBackups" @click="loadBackups">
            刷新列表
          </n-button>
          <n-select
            v-model:value="selectedBackupId"
            size="small"
            :options="backupOptions"
            placeholder="选择备份"
            style="width: 260px"
            :loading="isLoadingBackups"
            clearable
          />
          <n-button size="small" type="warning" @click="handleRestoreBackup">
            恢复
          </n-button>
        </n-space>
      </div>
      <div class="row a-center mt-8px">
        <span class="tip">备份目录：{{ backupDirTip }}</span>
      </div>
      <div class="row a-center mt-4px">
        <span class="tip">备份包含用户与配置等数据，请妥善保管备份文件。</span>
      </div>
    </AdvancedCell>
  </section>
</template>

<style lang="stylus" scoped>
.main
  height 100%

  & .tip
    color #ccc
    font-size 12px
</style>
