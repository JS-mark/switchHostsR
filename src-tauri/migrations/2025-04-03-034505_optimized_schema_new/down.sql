-- This file should undo anything in `up.sql`
-- 删除索引
DROP INDEX IF EXISTS idx_host_groups_user_id;
DROP INDEX IF EXISTS idx_logs_created_at;
DROP INDEX IF EXISTS idx_logs_user_id;
DROP INDEX IF EXISTS idx_hosts_is_active;
DROP INDEX IF EXISTS idx_hosts_user_id;
DROP INDEX IF EXISTS idx_users_username;

-- 删除表
DROP TABLE IF EXISTS host_group_relations;
DROP TABLE IF EXISTS host_groups;
DROP TABLE IF EXISTS logs;
DROP TABLE IF EXISTS hosts;
DROP TABLE IF EXISTS users;
