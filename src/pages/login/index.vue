<template>
  <div class="main">
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
            size="medium"
            require-mark-placement="right-hanging"
          >
            <n-form-item :label="$t('login.account')" path="account">
              <n-input
                v-model:value="data.account"
                :placeholder="$t('请输入账号')"
              />
            </n-form-item>
            <n-form-item :label="$t('来源')" path="userMode">
              <n-select
                v-model:value="data.userMode"
                :placeholder="$t('请输入账号')"
                :options="userModeList"
              />
            </n-form-item>
            <n-button
              style="width: 100%"
              size="large"
              type="primary"
              dashed
              @click="login"
            >
              {{ isLogin ? $t("logined") : $t("login.btn") }}
            </n-button>
          </n-form>
        </div>
      </n-card>
    </n-spin>
  </div>
</template>

<script lang="ts">
import {
  ref,
  reactive,
  defineComponent,
  onBeforeMount,
  onBeforeUnmount,
} from "vue";
import { useI18n } from "vue-i18n";
import { storeToRefs } from "pinia";
import { getUser } from "@/apis/user";
import { useRouter } from "vue-router";
import { UserMode, useUserStore } from "@/store";
import { useMessage, type FormInst } from "naive-ui";

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
    const { t } = useI18n();
    const router = useRouter();
    const loading = ref(false);
    const videoStyle = ref({});
    const store = useUserStore();
    const message = useMessage();
    const formRef = ref<FormInst | null>(null);
    const { setMode, setLogin, setUserInfo } = store;
    const { isLogin, info: userInfo } = storeToRefs(store);

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
              const resData = {
                nickname: res.name,
                messageNum: res.followers,
                avatar: res.avatar_url,
                email: res.email,
                home: res.html_url,
              };
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

    const onResize = () => {
      const windowWidth = document.body.clientWidth;
      const windowHeight = document.body.clientHeight;
      const windowAspectRatio = windowHeight / windowWidth;
      let videoWidth;
      let videoHeight;
      if (windowAspectRatio < 0.5625) {
        videoWidth = windowWidth;
        videoHeight = videoWidth * 0.5625;
        videoStyle.value = {
          height: windowWidth * 0.5625 + "px",
          width: windowWidth + "px",
          "margin-bottom": (windowHeight - videoHeight) / 2 + "px",
          "margin-left": "initial",
        };
      } else {
        videoHeight = windowHeight;
        videoWidth = videoHeight / 0.5625;
        videoStyle.value = {
          height: windowHeight + "px",
          width: windowHeight / 0.5625 + "px",
          "margin-left": (windowWidth - videoWidth) / 2 + "px",
          "margin-bottom": "initial",
        };
      }
    };
    onBeforeMount(() => {
      window.addEventListener("resize", onResize);
    });

    onBeforeUnmount(() => {
      window.removeEventListener("resize", onResize);
    });

    return {
      login,
      loading,
      userInfo,
      isLogin,
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
          disabled: true,
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
.main
  width 100%
  height 100%
  background-color rgba(0, 0, 0, .3)

  & .content
    width 500px
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
    right 0
    bottom 0
    min-width 100%
    min-height 100%
    height auto
    width auto
    filter: blur(15px) //背景模糊设置
    -webkit-filter: grayscale(100%)
    filter:grayscale(100%); //背景灰度设置

    source
      min-width: 100%;
      min-height: 100%;
      height: auto;
      width: auto;
</style>
