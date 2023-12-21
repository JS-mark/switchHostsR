import type { AxiosRequestHeaders, AxiosResponse } from 'axios'
import { mock, online, test } from './config'
import { getRandID, getType } from '@/utils'

export interface ReqOptions {
  method: 'get' | 'post' | 'put' | 'delete'
  options: {
    url: string
    method: 'get' | 'post' | 'put' | 'delete'
    data: { [key: string]: any }
    params?: { [key: string]: any }
    headers?: AxiosRequestHeaders
  }
  config?: {
    online: boolean | string
    test: boolean | string
    mock: boolean | string
  }
}

interface GetUrlOption {
  url: string
  config: ReqOptions['config']
}

type reqFn = () => Promise<AxiosResponse>
/**
 * timeout 3s
 * options { retry: 3, timeout: 3 }
 */
abstract class Request {
  private retryCount: number
  public timeout: number
  private retryQueue: Map<string, Array<reqFn>>
  constructor(
    options = {
      retry: 3,
      timeout: 15,
    },
  ) {
    this.retryCount = options.retry || 3
    this.timeout = options.timeout || 15
    this.retryQueue = new Map()
  }

  getRid() {
    return getRandID()
  }

  retry(id: string, cb: reqFn) {
    return new Promise((resolve, reject) => {
      if (this.retryQueue.get(id))
        return
      const queue = Array.from(new Array(this.retryCount)).map(_ => cb)
      this.retryQueue.set(id, [...queue])
      let errCount = 0
      const oLen = queue.length
      let len = queue.length
      while (len > 0) {
        len--
        const fn = queue.pop() as reqFn
        this.retryQueue.set(id, queue)
        fn()
          .then((res) => {
            // 如果 resolve 直接清空执行栈
            len = 0
            this.retryQueue.delete(id)
            resolve(res)
          })
          .catch((err) => {
            // 原失败次数达到 errCount 才 reject
            errCount++
            errCount === oLen && reject(err)
          })
      }
    })
  }

  /**
   * 拼接 url
   * @param {*} config
   */
  getURl(config: ReqOptions['config'], url: GetUrlOption['url']) {
    if (/http(s)\:\/\//gi.test(url))
      return url

    if (!config)
      return `${online}${url}`

    let _ulr = `${online}${url}`
    if (config.online) {
      const type = getType(config.online)
      type === 'boolean' && (_ulr = `${online}${url}`)
      type === 'string' && (_ulr = `${config.online}${url}`)
    }

    if (config.test) {
      const type = getType(config.test)
      type === 'boolean' && (_ulr = `${test}${url}`)
      type === 'string' && (_ulr = `${config.test}${url}`)
    }

    if (config.mock) {
      const type = getType(config.mock)
      type === 'boolean' && (_ulr = `${mock}${url}`)
      type === 'string' && (_ulr = `${config.mock}${url}`)
    }

    return _ulr
  }

  /**
   * request
   * @param {*} method
   * @param {*} options
   * @example
   * this.$request('get', { url: 'xxx', data: {} }, config: { test: false, online: true })
   * @returns { void }
   */
  public request(
    method: ReqOptions['method'],
    options: ReqOptions['options'],
    config: ReqOptions['config'],
  ) {
    console.warn('request', method, options, config)
  }
}

export default Request
