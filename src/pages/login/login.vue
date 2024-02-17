<script lang="ts">
import { useMessage } from 'naive-ui'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import { storeToRefs } from 'pinia'
import { getUser } from '@/apis/user'
import { invoke } from '@tauri-apps/api'
import { mixins } from './mixins'
import { handerUserInfoByGithub } from '@/utils'
import { computed, defineComponent, reactive, ref } from 'vue'
import { type User, type UserMode, useUserStore } from '@/store'
import type { FormInst, FormItemRule } from 'naive-ui'

export default defineComponent({
  name: 'Login',
  emits: ['onCallback'],
  setup(props, ctx) {
    const data = reactive({
      email: '',
      userMode: 'email' as UserMode,
      account: '',
      password: '',
    })
    const { t, locale } = useI18n()
    const router = useRouter()
    const videoStyle = ref({})
    const store = useUserStore()
    const message = useMessage()
    const formRef = ref<FormInst | null>(null)
    const { setMode, setLogin, setUserInfo } = store
    const { isLogin, info: userInfo } = storeToRefs(store)

    const labelWidth = computed(() => {
      return locale.value
    })

    const placeholder = computed(() => {
      const mode = data.userMode
      let str = ''
      switch (mode) {
        case 'github':
          str = t('请输入github账号')
          break
        case 'weibo':
          str = t('请输入微博UID')
          break
        default:
          str = t('请输入账号')
          break
      }
      return str
    })

    const login = (event: MouseEvent) => {
      mixins.loading = true
      event.preventDefault()
      formRef.value?.validate((errors) => {
        if (!errors) {
          if (isLogin.value)
            return
          const getUserInfo = Reflect.get(getUser, data.userMode)
          // 自有邮箱登录
          if (data.userMode === 'email') {
            invoke('user_login', {
              email: data.email,
              password: data.password,
            }).then((res: any) => {
              if (res.code === 10000) {
                debugger
                setLogin(true)
                setMode('email')
                setUserInfo(res.data)
                message.success(t('登录成功'))
              }
              else {
                return Promise.reject(res)
              }
            }).catch((err) => {
              console.error('login err', err)
              message.error(err.msg || t('登录失败'))
            }).finally(() => {
              mixins.loading = false
            })
            return
          }

          getUserInfo(data.account)
            .then((res: any) => {
              message.success('登录成功')
              let resData: User = {}
              switch (data.userMode) {
                case 'github':
                  resData = handerUserInfoByGithub(res)
                  break
                case 'weibo':
                  resData = handerUserInfoByGithub(res)
                  break

                default:
                  break
              }
              setLogin(true)
              setMode(data.userMode)
              setUserInfo(resData)

              setTimeout(() => {
                router.replace({
                  name: 'Home',
                })
              }, 16)
            })
            .finally(() => {
              mixins.loading = false
            })
        }
        else {
          mixins.loading = false
          message.error('请输入账号！')
        }
      })
    }

    const onBack = () => {
      ctx.emit('onCallback', { reset: true })
    }

    return {
      login,
      onBack,
      userInfo,
      isLogin,
      labelWidth,
      placeholder,
      videoStyle,
      formRef,
      data,
      userModeList: [
        {
          label: 'Github',
          value: 'github',
          disabled: true,
        },
        {
          label: t('微博'),
          value: 'weibo',
          disabled: true,
        },
        {
          label: t('email'),
          value: 'email',
          disabled: false,
        },
      ],
      rules: {
        account: {
          required: true,
          trigger: ['blur', 'input'],
          message: t('请输入账号'),
        },
        email: {
          required: true,
          trigger: ['blur', 'input'],
          message: t('请输入正确邮箱'),
          validator: (rule: FormItemRule, value: string) => {
            return /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/.test(value)
          },
        },
        password: {
          required: true,
          trigger: ['blur', 'input'],
          message: t('请输入密码'),
        },
        userMode: {
          required: true,
          trigger: ['blur', 'change'],
          message: t('请选择账号来源'),
        },
      },
    }
  },
})
</script>

<template>
  <n-form
    ref="formRef"
    :key="labelWidth"
    :model="data"
    :rules="rules"
    label-placement="left"
    label-width="auto"
    size="medium"
    require-mark-placement="right-hanging"
  >
    <n-form-item v-if="data.userMode !== 'email'" :label="$t('login.account')" path="account">
      <n-input
        v-model:value="data.account"
        :placeholder="placeholder"
      />
    </n-form-item>
    <n-form-item v-if="data.userMode === 'email'" :label="$t('login.email')" path="email">
      <n-input
        v-model:value="data.email"
        :placeholder="$t('login.email')"
      />
    </n-form-item>
    <n-form-item v-if="data.userMode === 'email'" :label="$t('login.password')" path="password">
      <n-input
        v-model:value="data.password"
        type="password"
        show-password-on="mousedown"
        :placeholder="$t('login.password')"
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
      block
      class="!w-full btn"
      size="large"
      type="primary"
      dashed
      @click="login"
      @keydown.enter="login"
    >
      {{ isLogin ? $t("Logged") : $t("login.btn") }}
    </n-button>
  </n-form>
</template>

<style lang="stylus" scoped>
.btn-container

  &:deep(div:first-child)
    flex: 1
</style>
