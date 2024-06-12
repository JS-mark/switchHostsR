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
