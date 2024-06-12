-- Your SQL goes here
CREATE TABLE hosts_data (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL, -- hosts 名称
  hosts_type INTEGER DEFAULT 0 NOT NULL, -- hosts 类型, 0: 本地, 1: 远程
  hosts_path TEXT, -- hosts路径, 本地和远程
  content TEXT NOT NUll,  -- JSON数据存储为文本
  status INTEGER DEFAULT 0 NOT NULL, -- 是否已开启
  is_del INTEGER DEFAULT 0 NOT NULL, -- 是否已删除, 0: 未删除, 1: 已删除
  is_readonly INTEGER DEFAULT 0 NOT NULL, -- 是否只读, 0: 非只读, 1: 只读
  created_by INTEGER NOT NULL, -- 操作用户
  updated_at TEXT NOT NULL, -- 更新时间
  created_at TEXT NOT NULL, -- 创建时间
  FOREIGN KEY (created_by) REFERENCES users(id)
)
