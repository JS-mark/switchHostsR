// import api from '@/plugins/request'
import { Octokit } from '@octokit/core'

export function getUserInfoByGithub(name: string) {
  const octokit = new Octokit({
    // 个人 token
    auth: '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',
  })

  return octokit.request('GET /users/{username}', {
    username: name,
    headers: {
      'X-GitHub-Api-Version': '2022-11-28',
    },
  }).then((res) => {
    if (res.status === 200) {
      if (!res.data.email)
        return Promise.reject(res)
      else return Promise.resolve(res.data)
    }
    else { return Promise.reject(res) }
  }).catch((err) => {
    return Promise.reject(err)
  })
}

export const getUser = {
  github: getUserInfoByGithub,
}
