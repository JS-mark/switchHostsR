import { defineConfig, mergeConfig } from 'vite'
import { getBaseConfig, getTauriConfig, getUtoolsConfig } from './vite-config/index'

// https://vitejs.dev/config/
export default defineConfig((env) => {
  const { mode } = env
  let config = {}
  if (mode === 'tauri')
    config = getTauriConfig(env)

  if (mode === 'utool')
    config = getUtoolsConfig(env)

  return mergeConfig(getBaseConfig(env), config, true)
})
