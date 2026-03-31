<script lang="ts">
import type { FormInst, FormItemRule } from 'naive-ui'

import { useI18n } from 'vue-i18n'
import { debounce } from 'lodash-es'
import { useMessage } from 'naive-ui'
import { computed, defineComponent, reactive, ref } from 'vue'

import { checkEmailAvailability, resetPassword } from '@/apis'

import { mixins } from './mixins'

export default defineComponent({
  name: 'ForgotPassword',
  emits: ['onCallback'],
  setup(props, ctx) {
    const data = reactive({
      email: '',
      newPassword: '',
      confirmPassword: '',
    })
    const { t, locale } = useI18n()
    const message = useMessage()
    const formRef = ref<FormInst | null>(null)
    const step = ref(1)
    const submitting = ref(false)

    const labelWidth = computed(() => {
      return locale.value
    })

    const verifyEmail = (event: MouseEvent) => {
      event.preventDefault()
      formRef.value?.validate((errors) => {
        if (!errors) {
          submitting.value = true
          checkEmailAvailability(data.email).then((res: any) => {
            if (res.code === 200) {
              // 返回 true 表示邮箱可用（即未注册），false 表示已注册
              if (res.data === false) {
                step.value = 2
              }
              else {
                message.error(t('login.emailNotRegistered'))
              }
            }
            else {
              return Promise.reject(res)
            }
          }).catch((err) => {
            const errMsg = typeof err === 'string' ? err : (err?.msg || err?.message || t('login.emailNotRegistered'))
            message.error(errMsg)
          }).finally(() => {
            submitting.value = false
          })
        }
      }, (rule) => {
        return rule?.key === 'email'
      })
    }

    const onReset = (event: MouseEvent) => {
      event.preventDefault()
      formRef.value?.validate((errors) => {
        if (!errors) {
          submitting.value = true
          mixins.loading = true
          resetPassword({
            email: data.email,
            new_password: data.newPassword,
          }).then((res: any) => {
            if (res.code === 200) {
              message.success(t('login.resetSuccess'))
              // 重置表单并切回登录 tab
              data.email = ''
              data.newPassword = ''
              data.confirmPassword = ''
              step.value = 1
              ctx.emit('onCallback', { reset: true })
            }
            else {
              return Promise.reject(res)
            }
          }).catch((err) => {
            const errMsg = typeof err === 'string' ? err : (err?.msg || err?.message || t('login.resetFailed'))
            message.error(errMsg)
          }).finally(() => {
            submitting.value = false
            mixins.loading = false
          })
        }
      })
    }

    const goBack = () => {
      step.value = 1
    }

    return {
      verifyEmail: debounce(verifyEmail, 300),
      onReset: debounce(onReset, 300),
      goBack,
      labelWidth,
      formRef,
      data,
      step,
      submitting,
      rules: {
        email: {
          key: 'email',
          required: true,
          trigger: ['blur', 'input'],
          message: t('请输入正确邮箱'),
          validator: (rule: FormItemRule, value: string) => {
            return /^[\w.%+-]+@[a-z0-9.-]+\.[a-z]{2,}$/i.test(value)
          },
        },
        newPassword: {
          required: true,
          trigger: ['blur', 'input'],
          message: t('login.pleaseEnterNewPassword'),
        },
        confirmPassword: {
          required: true,
          trigger: ['blur', 'input'],
          validator: (rule: FormItemRule, value: string) => {
            if (!value) {
              return new Error(t('login.pleaseConfirmPassword'))
            }
            if (value !== data.newPassword) {
              return new Error(t('login.passwordsNotMatch'))
            }
            return true
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
    <template v-if="step === 1">
      <n-form-item :label="$t('login.email')" path="email">
        <n-input
          v-model:value="data.email"
          :placeholder="$t('login.email')"
        />
      </n-form-item>
      <n-button
        block
        class="!w-full btn"
        size="large"
        type="primary"
        dashed
        :loading="submitting"
        @click="verifyEmail"
      >
        {{ $t('login.nextStep') }}
      </n-button>
    </template>
    <template v-else>
      <n-form-item :label="$t('login.newPassword')" path="newPassword">
        <n-input
          v-model:value="data.newPassword"
          type="password"
          show-password-on="mousedown"
          :placeholder="$t('login.pleaseEnterNewPassword')"
        />
      </n-form-item>
      <n-form-item :label="$t('login.confirmPassword')" path="confirmPassword">
        <n-input
          v-model:value="data.confirmPassword"
          type="password"
          show-password-on="mousedown"
          :placeholder="$t('login.pleaseConfirmPassword')"
        />
      </n-form-item>
      <div class="flex gap-2">
        <n-button
          class="flex-1"
          size="large"
          dashed
          @click="goBack"
        >
          {{ $t('login.goBack') }}
        </n-button>
        <n-button
          class="flex-1"
          size="large"
          type="primary"
          dashed
          :loading="submitting"
          @click="onReset"
        >
          {{ $t('login.resetPassword') }}
        </n-button>
      </div>
    </template>
  </n-form>
</template>

<style lang="stylus" scoped>
.btn-container

  &:deep(div:first-child)
    flex: 1
</style>
