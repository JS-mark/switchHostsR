<script lang="ts">
import { useMessage } from 'naive-ui'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import { storeToRefs } from 'pinia'
import { invoke } from '@tauri-apps/api'
import { mixins } from './mixins'
import { useUserStore } from '@/store'
import { debounce } from 'lodash-es'
import type { FormInst, FormItemRule } from 'naive-ui'
import { computed, defineComponent, reactive, ref } from 'vue'

export default defineComponent({
  name: 'Register',
  emits: ['onCallback'],
  setup(props, ctx) {
    const data = reactive({
      nickname: '',
      email: '',
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

    const onRegister = (event: MouseEvent) => {
      mixins.loading = true
      event.preventDefault()
      formRef.value?.validate((errors) => {
        if (!errors) {
          if (isLogin.value)
            return
          invoke('add_user', {
            name: data.nickname,
            email: data.email,
            password: data.password,
          }).then((res: any) => {
            if (res.code !== 10000)
              return Promise.reject(new Error('注册失败'))

            message.success(t('注册成功'))
            setMode('email')
            setLogin(true)
            setUserInfo(res)

            router.replace({
              name: 'Home',
            })
          }).catch((err) => {
            message.error(t('注册失败'))
            console.error('err', err)
          }).finally(() => {
            mixins.loading = false
          })
        }
        else {
          mixins.loading = false
          message.error(t('请正确填写表单'))
        }
      })
    }

    const onBack = () => {
      ctx.emit('onCallback', { reset: true })
    }

    return {
      onRegister: debounce(onRegister, 300),
      onBack: debounce(onBack, 300),
      userInfo,
      isLogin,
      labelWidth,
      videoStyle,
      formRef,
      data,
      rules: {
        nickname: {
          required: true,
          trigger: ['blur', 'input'],
          message: t('请输入用户名称'),
        },
        password: {
          required: true,
          trigger: ['blur', 'input'],
          message: t('请输入密码'),
        },
        email: {
          required: true,
          trigger: ['blur', 'input'],
          message: t('请输入正确邮箱'),
          validator: (rule: FormItemRule, value: string) => {
            return /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/.test(value)
          },
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
    <n-form-item :label="$t('login.nickname')" path="nickname">
      <n-input
        v-model:value="data.nickname"
        :placeholder="$t('login.nickname')"
      />
    </n-form-item>
    <n-form-item :label="$t('login.email')" path="email">
      <n-input
        v-model:value="data.email"
        :placeholder="$t('login.email')"
      />
    </n-form-item>
    <n-form-item :label="$t('login.password')" path="password">
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
      @click="onRegister"
      @keydown.enter="onRegister"
    >
      {{ isLogin ? $t("Logged") : $t("login.register") }}
    </n-button>
  </n-form>
</template>

<style lang="stylus" scoped>
.btn-container

  &:deep(div:first-child)
    flex: 1
</style>
