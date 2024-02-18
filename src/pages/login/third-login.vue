<script lang="ts">
import { useMessage } from 'naive-ui'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import { storeToRefs } from 'pinia'
import { getUser } from '@/apis/user'
import { invoke } from '@tauri-apps/api'
import { mixins } from './mixins'
import { computed, defineComponent, reactive, ref } from 'vue'
import { type UserMode, useUserStore } from '@/store'
import type { FormInst } from 'naive-ui'

export default defineComponent({
  name: 'ThirdLogin',
  emits: ['onCallback'],
  setup(props, ctx) {
    const data = reactive({
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

    const placeholder = computed(() => {
      const mode = data.userMode
      let str = ''
      switch (mode) {
        case 'github':
          str = t('请输入github账号')
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
          // 获取三方用户信息
          getUserInfo(data.account)
            .then(async (res: any) => {
              const params = {
                email: res.email,
                name: res.name,
                uid: `${res.id}`,
                account: data.account,
                avatarUrl: res.avatar_url,
                password: data.password,
                createdAt: res.created_at,
                updatedAt: res.updated_at,
              }
              // github 用户信息
              await invoke('third_account_login', params).then((resData: any) => {
                setLogin(true)
                setMode(data.userMode)
                setUserInfo(resData)
                message.success('登录成功')

                setTimeout(() => {
                  router.replace({
                    name: 'Home',
                  })
                }, 16)
              }).catch((err) => {
                return Promise.reject(err)
              })
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
      placeholder,
      videoStyle,
      formRef,
      data,
      userModeList: [
        {
          label: 'Github',
          value: 'github',
          disabled: false,
        },
        {
          label: t('微博'),
          value: 'weibo',
          disabled: true,
        },
      ],
      rules: {
        account: {
          required: true,
          trigger: ['blur', 'input'],
          message: t('请输入账号'),
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
    <n-form-item
      :label="$t('login.account')"
      path="account"
    >
      <n-input v-model:value="data.account" :placeholder="placeholder" />
    </n-form-item>
    <!-- 登录密码 -->
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
