<template>
  <div class="form"></div>
  <!-- footer -->
  <section class="row f-end a-center">
    <n-space class="user-control a-center">
      <slot name="control"></slot>
      <!-- 取消 -->
      <n-button strong secondary type="error" @click="cancel">
        {{ $t("cancel") }}
      </n-button>
      <!-- 确认 -->
      <n-button @click="confirm" strong secondary type="primary">
        {{ $t("confirm") }}
      </n-button>
    </n-space>
  </section>
</template>

<script lang="ts">
/**
 * 命令设置
 */
export default {
  name: "CMD",
};
</script>

<script lang="ts" setup>
import { useMessage } from "naive-ui";
import { useI18n } from "vue-i18n";
import { reactive, onBeforeMount } from "vue";
import { useSettingsStore, SettingSpace } from "@/store/useSettings";
const store = useSettingsStore();
const message = useMessage();
const { t } = useI18n();
const model = reactive<SettingSpace.Cmd>({
  cmd: "",
});

onBeforeMount(() => {
  for (const [key, value] of Object.entries(store.cmd)) {
    Reflect.set(model, key, value);
  }
});

const confirm = () => {
  cancel();
  store.setSettingsByData({ cmd: model });
  message.success(t("cmd.successTip"));
};
const cancel = () => {
  store.hide();
};
</script>

<style lang="stylus" scoped>
.form
  height calc(100% - 34px)

.tip
  color #718096
  font-size 12px
  margin-top 6px
</style>
