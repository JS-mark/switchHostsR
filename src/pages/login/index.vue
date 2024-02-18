<script lang="ts">
import { storeToRefs } from 'pinia'
import { defineComponent, reactive, ref } from 'vue'
import { useUserStore } from '@/store'
import Login from './login.vue'
import Register from './register.vue'
import ThirdLogin from './third-login.vue'
import { mixins } from './mixins'

export default defineComponent({
  name: 'RegisterAndLogin',
  components: {
    Login,
    Register,
    ThirdLogin,
  },
  setup() {
    const data = reactive({
      mode: '',
    })
    const store = useUserStore()
    const videoStyle = ref({})
    const { isLogin, info: userInfo } = storeToRefs(store)

    const onRegister = (event: any) => {
      event.reset && (data.mode = '')
    }

    const onLogin = (event: any) => {
      event.reset && (data.mode = '')
    }

    const onThirdLogin = (event: any) => {
      event.reset && (data.mode = '')
    }

    const switchMode = (mode: 'register' | 'login') => {
      data.mode = mode
    }

    return {
      onLogin,
      onRegister,
      switchMode,
      onThirdLogin,
      data,
      isLogin,
      mixins,
      videoStyle,
      userInfo,
    }
  },
})
</script>

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
      <source src="../../assets/videos/bj.mp4" type="video/mp4">
    </video>
    <n-spin :show="mixins.loading" class="content">
      <n-card>
        <div class="login">
          <template v-if="isLogin">
            <n-avatar round :size="120" :src="userInfo.avatar_url" />
            <span class="nickname">{{ userInfo.name }}</span>
          </template>
          <template v-else>
            <n-tabs
              class="card-tabs"
              default-value="login"
              size="large"
              animated
              pane-wrapper-style="margin: 0 -4px"
              pane-style="padding-left: 4px; padding-right: 4px; box-sizing: border-box;"
            >
              <n-tab-pane name="login" tab="登录">
                <Login @on-callback="onLogin" />
              </n-tab-pane>
              <n-tab-pane name="signup" tab="注册">
                <Register @on-callback="onRegister" />
              </n-tab-pane>
              <n-tab-pane name="third-login" tab="三方登录">
                <ThirdLogin @on-callback="onThirdLogin" />
              </n-tab-pane>
            </n-tabs>
          </template>
        </div>
      </n-card>
    </n-spin>
  </div>
</template>

<style lang="stylus" scoped>
.main-login
  width 100%
  height 100%
  position relative
  background-color rgba(0, 0, 0, .3)

  & .content
    width 350px
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

    source
      min-width: 100%;
      min-height: 100%;
      height: auto;
      width: auto;
</style>
