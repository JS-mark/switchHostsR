import api from '@/plugins/request'

export function getUserInfoByGithub(name: string) {
  return api.request('get', {
    method: 'get',
    url: `https://api.github.com/users/${name}`,
    data: {},
  })
}

export function getUserInfoByWeibo(uid: string) {
  return api.request('get', {
    method: 'get',
    url: 'https://weibo.com/ajax/profile/info',
    data: {
      uid,
    },
  })
}

export function getUserInfoByGoogle(name: string) {
  return api.request('get', {
    method: 'get',
    url: `https://api.github.com/users/${name}`,
    data: {},
  })
}

export const getUser = {
  github: getUserInfoByGithub,
  weibo: getUserInfoByWeibo,
  googel: getUserInfoByGoogle,
}
