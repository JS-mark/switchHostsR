<script lang="ts">
import type { FormInst, FormItemRule } from 'naive-ui'

import { useI18n } from 'vue-i18n'
import { storeToRefs } from 'pinia'
import { useMessage } from 'naive-ui'
import { useRouter } from 'vue-router'
import { loginPlatform } from '@/apis'
import { type UserMode, useUserStore } from '@/store'
import { computed, defineComponent, reactive, ref } from 'vue'

import { mixins } from './mixins'

export default defineComponent({
  name: 'Login',
  emits: ['onCallback'],
  setup(props, ctx) {
    const data = reactive({
      email: '',
      userMode: 'github' as UserMode,
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

    const login = (event: MouseEvent) => {
      mixins.loading = true
      event.preventDefault()
      formRef.value?.validate((errors) => {
        if (!errors) {
          if (isLogin.value)
            return
          // 登录平台
          loginPlatform({
            email: data.email,
            password: data.password,
          }).then((res: any) => {
            if (res.code === 10000) {
              setLogin(true)
              setMode('email')
              setUserInfo(res.data)
              // 跳转首页
              setTimeout(() => {
                router.replace({
                  name: 'Home',
                })
              }, 16)
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
      videoStyle,
      formRef,
      data,
      rules: {
        email: {
          required: true,
          trigger: ['blur', 'input'],
          message: t('请输入正确邮箱'),
          validator: (rule: FormItemRule, value: string) => {
            return /^[\w.%+-]+@[a-z0-9.-]+\.[a-z]{2,}$/i.test(value)
          },
        },
        password: {
          required: true,
          trigger: ['blur', 'input'],
          message: t('请输入密码'),
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
    <n-form-item :label="$t('login.email')" path="email">
      <n-input v-model:value="data.email" :placeholder="$t('login.email')" />
    </n-form-item>
    <n-form-item
      :label="$t('login.password')"
      path="password"
    >
      <n-input
        v-model:value="data.password"
        type="password"
        show-password-on="mousedown"
        :placeholder="$t('login.password')"
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
