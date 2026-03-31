export interface GetPage {
  page: number
  pageSize: number
  [key: string]: any
}

export interface Result<T = any> {
  msg: string
  data: T
  code: number
}

export interface ListResult<T> {
  list: T[]
  total: number
}

/** 后端 User 模型对应的前端接口（统一定义，各组件复用） */
export interface User {
  id: number
  username: string
  email: string | null
  avatar: string | null
  is_admin: boolean | null
  created_at: number
  updated_at: number
}
