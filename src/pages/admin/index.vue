<script lang="ts" setup>
import Logs from './logs.vue'
import Users from './user.vue'
import HostGroups from './host-groups.vue'
</script>

<template>
  <n-card :bordered="false">
    <n-tabs
      class="card-tabs"
      default-value="users"
      size="medium"
      animated
      pane-wrapper-style="margin: 0 -4px"
      pane-style="padding-left: 4px; padding-right: 4px; box-sizing: border-box;"
    >
      <n-tab-pane name="users" tab="用户管理">
        <Users />
      </n-tab-pane>
      <n-tab-pane name="host-groups" tab="主机组管理">
        <HostGroups />
      </n-tab-pane>
      <n-tab-pane name="logs" tab="操作日志">
        <Logs />
      </n-tab-pane>
    </n-tabs>
  </n-card>
</template>
