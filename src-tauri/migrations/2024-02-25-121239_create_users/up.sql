-- Your SQL goes here
CREATE TABLE users (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL, -- 用户名称
  email TEXT NOT NULL, -- 用户邮箱
  avatar_url TEXT NOT NULL, -- 用户头像
  status INTEGER DEFAULT 0 NOT NULL, -- 用户状态, 0: 未启用, 1: 已启用, -1: 已禁用
  user_level INTEGER DEFAULT 0 NOT NULL, -- 操作类型 0: 系统用户, 1: 普通用户, 99: 超管用户
  password TEXT NOT NULL, -- 用户密码
  is_del INTEGER DEFAULT 0 NOT NULL, -- 是否已删除, 0: 未删除, 1: 已删除
  is_third INTEGER DEFAULT 0 NOT NULL, -- 是否是三方账户, 0: 是, 1: 不是
  third_account_uid TEXT, -- 三方账户UID
  created_at TEXT DEFAULT CURRENT_TIMESTAMP NOT NULL, -- 创建时间
  updated_at TEXT NOT NULL -- 更新时间
);
