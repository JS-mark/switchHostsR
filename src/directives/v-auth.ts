import type { DirectiveBinding, FunctionDirective, VNode } from 'vue'

import { isArray, isFunction, isObject, omit } from 'lodash-es'

import type { User } from '@/apis/system/user'

import { useUserStore } from '@/store'

type AuthCallBack = (event: { user: User, el: HTMLElement, vnode: VNode }) => boolean | void
type AuthPromiseCallBack = (event: { user: User, el: HTMLElement, vnode: VNode }, ...args: any[]) => Promise<boolean | void>

interface AuthValue {
  auth: AuthCallBack | AuthPromiseCallBack
}

type AuthPermisson = (string | number)[]

type AuthBindingValue = AuthValue | (AuthCallBack | AuthPromiseCallBack) | AuthPermisson

/**
 * 权限校验指令
 */
export const Auth: FunctionDirective<HTMLElement, DirectiveBinding<AuthBindingValue>> = async (el, binding, VNode) => {
  const userStore = useUserStore()
  const { custom } = binding.modifiers
  let authFn: AuthCallBack | AuthPromiseCallBack | null = null
  let authFnParams: Record<string, any> = {}

  if (isFunction(binding.value)) {
    authFn = binding.value as unknown as (AuthCallBack | AuthPromiseCallBack)
  }

  if (!isFunction(binding.value) && isObject(binding.value)) {
    authFn = (binding.value as unknown as AuthValue).auth
    authFnParams = omit(binding.value, 'auth')
  }

  if (!isFunction(binding.value) && isArray(binding.value)) {
    authFn = () => {
      const role = userStore.getRole
      // 用户角色
      return (binding.value as unknown as AuthPermisson).includes(role)
    }
  }

  if (!authFn)
    return

  // 所有函数均等待
  const flag = await authFn({ user: userStore.getUserData, el, vnode: VNode }, ...Object.values((authFnParams || {})))

  // 非自定义直接判断 flag
  if (!custom) {
    // 删除
    !flag && el.parentNode?.removeChild(el)
  }
}

export function isAuth(value: AuthPermisson) {
  const userStore = useUserStore()
  // 用户角色
  return value.includes(userStore.getRole)
}
