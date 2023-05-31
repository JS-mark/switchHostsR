<template>
  <section class="main">
    <!-- 标题 -->
    <h4>
      {{ $t("settings.advanced.title") }}
    </h4>
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
    <h4>{{ $t("我的 Hosts 文件在哪里？") }}</h4>
    <div class="row a-center">
      <span class="tip">{{ $t("你的 Hosts 文件在：") }}</span>
      <n-tooltip placement="bottom" trigger="hover">
        <template #trigger>
          <n-button
            size="small"
            quaternary
            type="primary"
            @click="openFile(model.hostsPath)"
          >
            {{ model.hostsPath }}
          </n-button>
        </template>
        <span>{{ $t("点击打开") }}</span>
      </n-tooltip>
    </div>
    <h4>{{ $t("我的数据储存在哪里？") }}</h4>
    <div class="row a-center">
      <span class="tip">
        {{ $t("你的数据文件在：") }}
      </span>
      <n-tooltip placement="bottom" trigger="hover">
        <template #trigger>
          <n-button
            size="small"
            quaternary
            type="primary"
            @click="openFileByPath(model.SwitchHostsRPath)"
          >
            {{ model.SwitchHostsRPath }}
          </n-button>
        </template>
        <span>{{ $t("点击打开") }}</span>
      </n-tooltip>
      <n-button
        size="small"
        quaternary
        type="primary"
        @click="changeFile(model.SwitchHostsRPath)"
      >
        {{ $t("change") }}
      </n-button>
    </div>
  </section>
</template>

<script lang="ts">
/**
 * 高级设置
 */
export default {
  name: "Advanced",
};
</script>

<script lang="ts" setup>
import { sendLog } from "@/utils/sendLog";
import { APP_NAME } from "@/utils/constant";
import { reactive, watchEffect } from "vue";
import { openFile, openDirectory } from "@/utils";
import { useSettingsStore, SettingSpace } from "@/store/useSettings";

const store = useSettingsStore();
const model = reactive<SettingSpace.Advanced>({
  canSendData: false,
  hostsPath: "",
  SwitchHostsRPath: "",
});

const onChecked = (event: boolean) => {
  store.setCanSendData(event);
  model.canSendData = event;
};

const openFileByPath = (file?: string) => {
  openFile(file);
};

const changeFile = (file?: string) => {
  // 打开文件夹
  openDirectory(file).then((res) => {
    res &&
      store.setSettingsByData({
        advanced: {
          canSendData: model.canSendData,
          hostsPath: model.hostsPath,
          SwitchHostsRPath: `${res}/.${APP_NAME}`,
        },
      });
  });
  sendLog({
    msg: JSON.stringify({
      msg: "改变储存文件",
      file,
    }),
  });
};

watchEffect(() => {
  for (const [key, value] of Object.entries(store.advanced)) {
    Reflect.set(model, key, value);
  }
});
</script>

<style lang="stylus" scoped>
.main
  height 100%

  & .tip
    color #ccc
    font-size 12px
</style>
