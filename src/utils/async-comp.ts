import type { AsyncComponentLoader, AsyncComponentOptions } from 'vue'

import { NEmpty, NSpin } from 'naive-ui'
import { defineAsyncComponent, h } from 'vue'

export function useAsyncComp(comp: AsyncComponentLoader, options?: Omit<AsyncComponentOptions, 'loader'>) {
  return defineAsyncComponent({
    loader: comp,
    // 加载异步组件时要使用的组件
    loadingComponent: h(NSpin, { description: '加载中...' }),
    // 加载失败时要使用的组件
    errorComponent: h(NEmpty, { description: '暂无数据' }),
    // 在显示 loadingComponent 之前的延迟 | 默认值：200（单位 ms）
    delay: 200,
    suspensible: true,
    // 如果提供了 timeout ，并且加载组件的时间超过了设定值，将显示错误组件
    // 默认值：Infinity（即永不超时，单位 ms）
    timeout: 3000,
    ...(options || {}),
  }) as ReturnType<typeof defineAsyncComponent>
}
