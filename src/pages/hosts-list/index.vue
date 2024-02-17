<script lang="ts" setup>
import { reactive } from 'vue'

defineOptions({
  name: 'HostsList',
})

const data = reactive({
  localList: [],
  remoteList: [],
  list: [],
})

const getListData = () => {
  
}
</script>

<template>
  <n-card :bordered="false" style="margin-bottom: 16px">
    <n-tabs default-value="local" size="large" justify-content="space-evenly">
      <n-tab-pane name="local" tab="本地 Hosts">
        <n-tabs
          type="line"
          animated
          placement="left"
          class="tab"
        >
          <template v-for="local in data.localList">
            <n-tab-pane :name="local.name" :tab="Oasis">
              Wonderwall
            </n-tab-pane>
          </template>

          <n-tab-pane name="the beatles" tab="the Beatles">
            Hey Jude
          </n-tab-pane>
          <n-tab-pane name="jay chou" tab="Jay Chou">
            Qilixiang
          </n-tab-pane>
          <n-tab-pane name="oasis1" tab="Oasis1">
            Wonderwall
          </n-tab-pane>
          <n-tab-pane name="the beatles1" tab="the Beatles1">
            Hey Jude
          </n-tab-pane>
          <n-tab-pane name="jay chou1" tab="Jay Chou1">
            Qilixiang
          </n-tab-pane>
          <n-tab-pane name="oasis2" tab="Oasis2">
            Wonderwall
          </n-tab-pane>
          <n-tab-pane name="the beatles2" tab="the Beatles2">
            Hey Jude
          </n-tab-pane>
          <n-tab-pane name="jay chou2" tab="Jay Chou2">
            Qilixiang
          </n-tab-pane>
          <n-tab-pane name="oasis3" tab="Oasis3">
            Wonderwall
          </n-tab-pane>
          <n-tab-pane name="the beatles3" tab="the Beatles3">
            Hey Jude
          </n-tab-pane>
          <n-tab-pane name="jay chou3" tab="Jay Chou3">
            Qilixiang
          </n-tab-pane>
          <n-tab-pane name="oasis4" tab="Oasis4">
            Wonderwall
          </n-tab-pane>
          <n-tab-pane name="the beatles4" tab="the Beatles4">
            Hey Jude
          </n-tab-pane>
          <n-tab-pane name="jay chou4" tab="Jay Chou4">
            Qilixiang
          </n-tab-pane>
          <n-tab-pane name="oasis5" tab="Oasis5">
            Wonderwall
          </n-tab-pane>
          <n-tab-pane name="the beatles5" tab="the Beatles5">
            Hey Jude
          </n-tab-pane>
          <n-tab-pane name="jay chou5" tab="Jay Chou5">
            Qilixiang
          </n-tab-pane>
        </n-tabs>
      </n-tab-pane>
      <n-tab-pane name="remote" tab="远程 Hosts">
        <span>dd</span>
      </n-tab-pane>
    </n-tabs>
  </n-card>
</template>

<style lang="less" scoped>
.tab {
  background-color: #fff;

  & :deep(.n-tabs-nav) {
    width: 120px;
  }
}
</style>
