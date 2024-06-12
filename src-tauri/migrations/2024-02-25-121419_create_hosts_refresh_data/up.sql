-- Your SQL goes here
CREATE TABLE hosts_refresh_data (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  hosts_id INTEGER NOT NULL,
  created_by INTEGER NOT NULL, -- 操作用户
  created_at TEXT NOT NULL, -- 创建时间
  updated_at TEXT NOT NULL, -- 更新时间
  hosts_refresh_time INTEGER DEFAULT 15000 NOT NULL, -- 刷新时间
  last_refresh_time TEXT NOT NULL, -- 最后更新时间
  FOREIGN KEY (created_by) REFERENCES users(id),
  FOREIGN KEY (hosts_id) REFERENCES hosts_data(id)
)
