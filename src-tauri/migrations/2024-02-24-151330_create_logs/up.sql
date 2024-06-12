-- Your SQL goes here
CREATE TABLE logs (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  log_type INTEGER DEFAULT 0 NOT NULL, -- 操作类型 0:更新,1:删除,2:增加,-1:未知
  content TEXT NOT NULL,  -- JSON数据存储为文本
  created_at TEXT NOT NULL, -- 创建时间
  created_by INTEGER,  -- 操作人员，默认为当前操作人员，system
  FOREIGN KEY (created_by) REFERENCES users(id)  -- 外键约束引用users表的id
);
