<template>
  <n-drawer
    v-model:show="data.isShow"
    placement="right"
    :width="502"
    :on-after-leave="onClose"
  >
    <n-drawer-content>
      <template #header>
        {{ title }}
      </template>
      <!-- 表单 -->
      <n-form
        ref="formRef"
        :model="data.model"
        :rules="rules"
        label-placement="left"
        label-width="auto"
        require-mark-placement="right-hanging"
        size="medium"
      >
        <!-- hosts 类型 -->
        <n-form-item :label="$t('Hosts 类型')" path="hostsType">
          <n-select
            v-model:value="data.model.hostsType"
            :placeholder="$t('请选择 Hosts 类型')"
            :options="hostsTypeOptions"
          />
        </n-form-item>

        <!-- 名称 -->
        <n-form-item :label="$t('名称')" path="title">
          <n-input
            v-model:value="data.model.title"
            :placeholder="$t('请输入名称')"
          />
        </n-form-item>

        <!-- url -->
        <n-form-item v-if="isShowRemote" :label="$t('远程地址')" path="url">
          <n-input
            v-model:value="data.model.url"
            :placeholder="$t('请输入远程地址')"
          />
        </n-form-item>

        <!-- refresh 类型 -->
        <n-form-item
          path="refreshType"
          v-if="isShowRemote"
          :label="$t('刷新类型')"
        >
          <n-select
            v-model:value="data.model.refreshType"
            :placeholder="$t('请选择刷新类型')"
            :options="refreshTypeOptions"
          />
        </n-form-item>
      </n-form>

      <template #footer>
        <n-button round type="dander" @click="onCancel">
          {{ $t("cancel") }}
        </n-button>
        <n-button round type="primary" @click="onSubmit">
          {{ $t("confirm") }}
        </n-button>
      </template>
    </n-drawer-content>
  </n-drawer>
</template>

<script lang="ts">
import { useI18n } from "vue-i18n";
import { storeToRefs } from "pinia";
import { useRouter } from "vue-router";
import { useHostsStore } from "@/store/index";
import { addLocalHostsRoute } from "./utils";
import { type FormInst, useMessage, type FormItemRule } from "naive-ui";
import {
  ref,
  reactive,
  computed,
  watch,
  defineComponent,
  onMounted,
} from "vue";

export default defineComponent({
  name: "ListDraw",
  setup() {
    const { t } = useI18n();
    const message = useMessage();
    const router = useRouter();
    const store = useHostsStore();
    const { isShowEditor, mode } = storeToRefs(store);
    const formRef = ref<FormInst | null>(null);

    const data = reactive<{
      isShow: boolean;
      model: {
        title: string;
        url: string;
        hostsType: string | null;
        refreshType: number | null;
      };
    }>({
      isShow: false,
      model: {
        title: "",
        url: "",
        hostsType: null,
        refreshType: null,
      },
    });

    const isShowRemote = computed(() => {
      return data.model.hostsType === "remote";
    });

    const rules = computed(() => {
      let rule: { [key: string]: any } = {
        title: {
          required: true,
          trigger: ["blur", "input"],
          message: t("请输入名称"),
        },
        hostsType: {
          required: true,
          trigger: ["blur", "change"],
          message: t("请选择 Hosts 类型"),
        },
      };
      switch (data.model.hostsType) {
        case "remote":
          rule.url = {
            required: true,
            trigger: ["blur", "input"],
            message: t("请输入远程 Url 地址"),
            validator(rule: FormItemRule, value: any) {
              const reg = /(http|https):\/\/([\w.]+\/?)\S*/;
              if (!value) return new Error(t("请输入远程 Url 地址"));
              return reg.test(value)
                ? ""
                : new Error(t("请输入正确的远程 Url 地址"));
            },
          };
          rule.refreshType = {
            required: true,
            trigger: ["blur", "change"],
            message: t("请选择刷新类型"),
            validator(rule: FormItemRule, value: any) {
              if (!value) return new Error(t("请选择刷新类型"));
            },
          };
          break;

        default:
          break;
      }
      return rule;
    });
    const title = computed(() => {
      return mode.value === "create" ? t("创建 Hosts") : t("编辑 Hosts");
    });

    const refreshTypeOptions = computed(() => [
      {
        label: t("never"),
        value: 0,
      },
      {
        label: t("分钟", { m: 1 }),
        value: 1 * 60 * 1000,
      },
      {
        label: t("分钟", { m: 5 }),
        value: 5 * 60 * 1000,
      },
      {
        label: t("分钟", { m: 15 }),
        value: 15 * 60 * 1000,
      },
      {
        label: t("小时", { s: 1 }),
        value: 60 * 3600 * 1000,
      },
      {
        label: t("小时", { s: 24 }),
        value: 24 * 60 * 3600 * 1000,
      },
      {
        label: t("天", { d: 7 }),
        value: 7 * 24 * 60 * 3600 * 1000,
      },
    ]);

    const hostsTypeOptions = computed(() => {
      return [
        {
          label: t("local"),
          value: "local",
        },
        {
          label: t("remote"),
          value: "remote",
        },
      ];
    });

    const onClose = () => {
      formRef.value?.restoreValidation();
      data.model = {
        title: "",
        url: "",
        hostsType: null,
        refreshType: null,
      };
      store.hide();
    };

    const onSubmit = (event: MouseEvent) => {
      event.preventDefault();
      formRef.value?.validate((errors) => {
        if (!errors) {
          type modeType = "remote" | "local";
          addLocalHostsRoute(
            {
              name: data.model.title,
              title: data.model.title,
              type: data.model.hostsType as modeType,
            },
            (data) => {
              if (data) {
                message.success(t("创建成功"));
                router.addRoute(data);
                setTimeout(() => {
                  router.push({ name: data.name });
                }, 0);
                onClose();
              } else {
                message.success(t("创建失败"));
              }
            },
          );
        } else {
          message.error(t("创建失败"));
        }
      });
    };

    const onCancel = () => {
      onClose();
    };

    onMounted(() => {
      watch(
        () => isShowEditor.value,
        () => {
          data.isShow = isShowEditor.value;
        },
        { immediate: true },
      );
    });

    return {
      data,
      title,
      rules,
      formRef,
      onClose,
      onSubmit,
      onCancel,
      isShowRemote,
      hostsTypeOptions,
      refreshTypeOptions,
    };
  },
});
</script>
