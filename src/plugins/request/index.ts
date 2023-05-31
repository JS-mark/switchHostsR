/*
 * @Author: Mark
 * @Date: 2021-11-08 16:30:38
 * @LastEditors: Mark
 * @LastEditTime: 2023-05-31 10:54:11
 * @Description: fetch 类
 */
import {
  darkTheme,
  lightTheme,
  createDiscreteApi,
  ConfigProviderProps,
} from "naive-ui";
import axios from "axios";
import { computed, ref } from "vue";
import Request, { type ReqOptions } from "./request";

const themeRef = ref<"light" | "dark">("light");
const configProviderPropsRef = computed<ConfigProviderProps>(() => ({
  theme: themeRef.value === "light" ? lightTheme : darkTheme,
}));

const { notification, loadingBar } = createDiscreteApi(
  ["message", "notification", "loadingBar"],
  {
    configProviderProps: configProviderPropsRef,
  },
);

axios.interceptors.response.use(
  (response) => {
    const rid = response.config.headers["sn-reqid"] || "";

    if (response.data) {
      if (typeof response.data === "object") {
        response.data._rid = rid;
      }
    }
    loadingBar.finish();
    return response;
  },
  (error) => {
    loadingBar.error();
    return Promise.reject(error.response || error);
  },
);

axios.interceptors.request.use(
  (configData) => {
    loadingBar.start();
    return configData;
  },
  (error) => {
    loadingBar.error();
    return Promise.reject(error);
  },
);

/**
 * timeout 3s
 * options { retry: 3, timeout: 3 }
 */
class H5Request extends Request {
  constructor(
    options = {
      retry: 3,
      timeout: 15,
    },
  ) {
    super(options);
  }

  /**
   * request
   * @param {*} method
   * @param {*} options
   * @example
   * this.$request('get', { url: 'xxx', data: {} }, config: { test: false, online: true })
   * @returns
   */
  request(
    method: ReqOptions["method"],
    options: ReqOptions["options"],
    config: ReqOptions["config"] = { online: true, test: false, mock: false },
  ) {
    if (!method) {
      throw new Error("请填写请求方法");
    }
    const method_ = method.toLowerCase();
    if (!["post", "get"].includes(method_)) {
      throw new Error("不支持的请求类型！");
    }
    const _rid = this.getRid();
    // 重写请求方法
    options.method = method_ as ReqOptions["method"];
    options.url = this.getURl(config, options.url);
    if (options.data) {
      options.data._rid = _rid;
    } else {
      options.data = { _rid };
    }
    if (method_ === "get") {
      options.params = options.data;
      // @ts-ignore
      delete options.data;
    }

    if (method_ === "post") {
      options.data = Object.keys(options.data).reduce((obj, key) => {
        obj.append(key, options.data[key]);
        return obj;
      }, new URLSearchParams());
    }

    return axios({
      ...options,
      timeout: this.timeout * 1000,
    })
      .then((res) => {
        if (Number(res.status) === 200 && res.data) {
          return Promise.resolve(res.data);
        } else {
          notification.error(res.data.msg || "请求错误");
          return Promise.reject({ retry: true, res });
        }
      })
      .catch((err) => {
        if (err.retry || err.status !== 200) {
          return this.retry(_rid, () =>
            axios({
              ...options,
              timeout: this.timeout * 1000,
            }),
          );
        } else return Promise.reject(err);
      });
  }
}

export default new H5Request();
