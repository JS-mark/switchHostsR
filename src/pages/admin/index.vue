<script lang="ts" setup>
import Logs from './logs.vue'
import Users from './user.vue'
</script>

<template>
  <n-card :bordered="false">
    <n-tabs
      class="card-tabs"
      default-value="users"
      size="large"
      animated
      pane-wrapper-style="margin: 0 -4px"
      pane-style="padding-left: 4px; padding-right: 4px; box-sizing: border-box;"
    >
      <n-tab-pane name="users" tab="用户管理">
        <Users />
      </n-tab-pane>
      <n-tab-pane name="logs" tab="操作日志">
        <Logs />
      </n-tab-pane>
    </n-tabs>
  </n-card>
</template>
