import os from 'node:os'
import path from 'node:path'
import fs from 'node:fs'
import { spawn, spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'

const __dirname = fileURLToPath(new URL('.', import.meta.url))

let tauriDriver
let exit = false
let e2eEnv = {}

function getAppBinaryPath() {
  const base = path.resolve(__dirname, '..', 'src-tauri', 'target', 'debug')
  const appName = os.platform() === 'win32' ? 'switch-hosts-r.exe' : 'switch-hosts-r'
  return path.join(base, appName)
}

function closeTauriDriver() {
  exit = true
  tauriDriver?.kill()
}

function onShutdown(fn) {
  let cleaned = false
  const cleanup = () => {
    if (cleaned)
      return
    cleaned = true
    try {
      fn()
    } catch {
    }
  }
  process.once('beforeExit', cleanup)
  process.once('SIGINT', cleanup)
  process.once('SIGTERM', cleanup)
  process.once('SIGHUP', cleanup)
  process.once('SIGBREAK', cleanup)
}

onShutdown(() => {
  closeTauriDriver()
})

export const config = {
  host: '127.0.0.1',
  port: 4444,
  specs: ['./tests/specs/**/*.e2e.js'],
  maxInstances: 1,
  capabilities: [
    {
      maxInstances: 1,
      'tauri:options': {
        application: getAppBinaryPath(),
      },
    },
  ],
  reporters: ['spec'],
  framework: 'mocha',
  mochaOpts: {
    ui: 'bdd',
    timeout: 120000,
  },
  onPrepare: () => {
    spawnSync(
      'pnpm',
      ['tauri', 'build', '--', '--debug', '--no-bundle'],
      {
        cwd: path.resolve(__dirname, '..'),
        stdio: 'inherit',
        shell: true,
      },
    )
  },
  beforeSession: () => {
    const tmpRoot = path.resolve(__dirname, '.tmp')
    fs.mkdirSync(tmpRoot, { recursive: true })
    const runId = `${Date.now()}_${Math.random().toString(16).slice(2)}`
    const homeDir = path.join(tmpRoot, `home_${runId}`)
    fs.mkdirSync(homeDir, { recursive: true })
    const dbPath = path.join(tmpRoot, `db_${runId}.db`)

    e2eEnv = {
      ...process.env,
      DATABASE_URL: dbPath,
      HOME: homeDir,
      USERPROFILE: homeDir,
    }

    const driverBinName = os.platform() === 'win32' ? 'tauri-driver.exe' : 'tauri-driver'
    const driverBinPath = path.resolve(os.homedir(), '.cargo', 'bin', driverBinName)

    tauriDriver = spawn(driverBinPath, [], {
      stdio: [null, process.stdout, process.stderr],
      env: e2eEnv,
    })
    tauriDriver.on('error', (error) => {
      process.stderr.write(`tauri-driver error: ${String(error)}\n`)
      process.exit(1)
    })
    tauriDriver.on('exit', (code) => {
      if (!exit) {
        process.stderr.write(`tauri-driver exited with code: ${String(code)}\n`)
        process.exit(1)
      }
    })
  },
  afterSession: () => {
    closeTauriDriver()
  },
}
