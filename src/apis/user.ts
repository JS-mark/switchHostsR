import api from "@/plugins/request";

export const getUserInfoByGithub = (name: string) => {
  return api.request("get", {
    method: "get",
    url: `https://api.github.com/users/${name}`,
    data: {},
  });
};

export const getUserInfoByWeibo = (name: string) => {
  return api.request("get", {
    method: "get",
    url: `https://api.github.com/users/${name}`,
    data: {},
  });
};

export const getUserInfoByGoogle = (name: string) => {
  return api.request("get", {
    method: "get",
    url: `https://api.github.com/users/${name}`,
    data: {},
  });
};

export const getUser = {
  github: getUserInfoByGithub,
  weibo: getUserInfoByWeibo,
  googel: getUserInfoByGoogle,
};
