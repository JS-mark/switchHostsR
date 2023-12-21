<template>
  <div class="main-login">
    <video
      class="video"
      :controls="false"
      autoplay
      loop
      muted
      :style="videoStyle"
    >
      <source src="../../assets/videos/bj.mp4" type="video/mp4" />
    </video>
    <n-spin :show="loading" class="content">
      <n-card>
        <div class="login">
          <template v-if="isLogin">
            <n-avatar round :size="120" :src="userInfo.avatar" />
            <span class="nickname">{{ userInfo.nickname }}</span>
          </template>
          <n-form
            ref="formRef"
            :model="data"
            :rules="rules"
            label-placement="left"
            label-width="auto"
            :key="labelWidth"
            size="medium"
            require-mark-placement="right-hanging"
          >
            <n-form-item :label="$t('login.account')" path="account">
              <n-input
                v-model:value="data.account"
                :placeholder="placeholder"
                @keydown.enter="login"
              />
            </n-form-item>
            <n-form-item :label="$t('来源')" path="userMode">
              <n-select
                v-model:value="data.userMode"
                :placeholder="placeholder"
                :options="userModeList"
              />
            </n-form-item>
            <n-button
              style="width: 100%"
              size="large"
              type="primary"
              dashed
              @click="login"
              @keydown.enter="login"
            >
              {{ isLogin ? $t("Logged") : $t("login.btn") }}
            </n-button>
          </n-form>
        </div>
      </n-card>
    </n-spin>
  </div>
</template>

<script lang="ts">
import { useI18n } from "vue-i18n";
import { storeToRefs } from "pinia";
import { getUser } from "@/apis/user";
import { useRouter } from "vue-router";
import { User, UserMode, useUserStore } from "@/store";
import { useMessage, type FormInst } from "naive-ui";
import { ref, computed, reactive, defineComponent } from "vue";
import { handerUserInfoByGithub, handerUserInfoByWeibo } from "@/utils";

export default defineComponent({
  name: "Login",
  setup() {
    const data = reactive<{
      userMode: UserMode;
      account: string;
    }>({
      userMode: "github",
      account: "",
    });
    const { t, locale } = useI18n();
    const router = useRouter();
    const loading = ref(false);
    const videoStyle = ref({});
    const store = useUserStore();
    const message = useMessage();
    const formRef = ref<FormInst | null>(null);
    const { setMode, setLogin, setUserInfo } = store;
    const { isLogin, info: userInfo } = storeToRefs(store);

    const labelWidth = computed(() => {
      return locale.value;
    });

    const placeholder = computed(() => {
      const mode = data.userMode
      switch (mode) {
        case "github":
          return t('请输入github账号')
        case "weibo":
          return t('请输入微博UID')

        default:
          break;
      }
    })

    const login = (event: MouseEvent) => {
      loading.value = true;
      event.preventDefault();
      formRef.value?.validate((errors) => {
        if (!errors) {
          if (isLogin.value) return;
          const getUserInfo = Reflect.get(getUser, data.userMode);
          getUserInfo(data.account)
            .then((res: any) => {
              message.success("登录成功");
              let resData: User = {};
              switch (data.userMode) {
                case "github":
                  resData = handerUserInfoByGithub(res);
                  break;
                case "weibo":
                  resData = handerUserInfoByWeibo(res);
                  break;

                default:
                  break;
              }
              setLogin(true);
              setMode(data.userMode);
              setUserInfo(resData);

              setTimeout(() => {
                router.replace({
                  name: "Home",
                });
              }, 16);
            })
            .finally(() => {
              loading.value = false;
            });
        } else {
          loading.value = false;
          message.error("请输入账号！");
        }
      });
    };

    return {
      login,
      loading,
      userInfo,
      isLogin,
      labelWidth,
      placeholder,
      videoStyle,
      formRef,
      data,
      userModeList: [
        {
          label: "Github",
          value: "github",
          disabled: false,
        },
        {
          label: t("微博"),
          value: "weibo",
          disabled: false,
        },
        {
          label: "Google",
          value: "google",
          disabled: true,
        },
      ],
      rules: {
        account: {
          required: true,
          trigger: ["blur", "input"],
          message: "请输入账号",
        },
        userMode: {
          required: true,
          trigger: ["blur", "change"],
          message: "请选择账号来源",
        },
      },
    };
  },
});
</script>

<style lang="stylus" scoped>
.main-login
  width 100%
  height 100%
  position relative
  background-color rgba(0, 0, 0, .3)

  & .content
    width 600px
    left 50%
    top 50%
    transform translate(-50%, -50%)
    box-shadow 0 0 10px rgba(0, 0, 0, .35)

    & .login
      display flex
      flex-direction column
      justify-content center
      align-items center

      & .nickname
        color #fff
        font-weight 600
        font-size 16px
        padding 10px 0

  & .video
    position absolute
    top 0
    left 0
    width 100%
    height 100%
    object-fit cover
    // filter: blur(2px) //背景模糊设置

    source
      min-width: 100%;
      min-height: 100%;
      height: auto;
      width: auto;
</style>
