export const errorMessage: {
  [key: number]: string;
} = {
  199: "不支持的网络请求",
  200: "请求成功",
  201: "未知错误",
  202: "参数错误",
  203: "没有配置数据",
  204: "配置重复",
  205: "三方网络请求错误",
  206: "数据已经存在",
  207: "数据库出错",
  208: "页数出现问题",
};

export const networkConfig = {
  baseURL: "",
  contentType: "application/json;charset=UTF-8",
  //消息框消失时间
  messageDuration: 3000,
  //最长请求时间
  requestTimeout: 5000,
};

export const online = "";
export const test = "";
export const mock = "";
