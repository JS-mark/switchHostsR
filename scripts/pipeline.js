#!/usr/bin/env node
const path = require('node:path')
const global = path.join(require('node:os').homedir(), '.claude/plugins/agent-pipeline/cli.js')

try {
  require('node:fs').accessSync(global)
}
catch {
  console.error('Agent Pipeline plugin not found: ' + global)
  process.exit(1)
}
process.env.PIPELINE_ROOT = process.env.PIPELINE_ROOT || path.resolve(__dirname, '..')
require(global)
