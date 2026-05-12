import { useBridgeFunc } from '@/utils'

import type { Result } from './public'

export interface BackupInfo {
  id: number
  name: string
  description: string | null
  created_at: number
  file_name: string
  app_version?: string
  schema_version?: number
}

export function createBackup(request: { name: string, description?: string }) {
  return useBridgeFunc<Result<BackupInfo>>(() => {
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('create_backup', {
      request: {
        name: request.name,
        description: request.description ?? null,
      },
    })
      .then(res => resolve(res as Result<BackupInfo>))
      .catch(err => reject(err))
  })
}

export function getBackups() {
  return useBridgeFunc<Result<BackupInfo[]>>(() => {
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('get_backups')
      .then(res => resolve(res as Result<BackupInfo[]>))
      .catch(err => reject(err))
  })
}

export function getBackupDir() {
  return useBridgeFunc<Result<string>>(() => {
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('get_backup_dir')
      .then(res => resolve(res as Result<string>))
      .catch(err => reject(err))
  })
}

export function restoreBackup(backupId: number) {
  return useBridgeFunc<Result<void>>(() => {
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('restore_backup', {
      request: {
        backup_id: backupId,
      },
    })
      .then(res => resolve(res as Result<void>))
      .catch(err => reject(err))
  })
}
