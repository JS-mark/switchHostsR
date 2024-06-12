-- Your SQL goes here
CREATE TABLE permission_list (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL, -- 权限名称
  menu_list TEXT NOT NULL, -- 权限菜单
  status INTEGER NOT NULL, -- 状态是否已启用
  is_del INTEGER DEFAULT 0 NOT NULL, -- 是否已删除, 0: 未删除, 1: 已删除
  created_by INTEGER,  -- 操作人员，默认为当前操作人员，system
  created_at TEXT NOT NULL, -- 创建时间
  updated_at TEXT NOT NULL, -- 更新时间
  FOREIGN KEY (created_by) REFERENCES users(id)  -- 外键约束引用users表的id
);
