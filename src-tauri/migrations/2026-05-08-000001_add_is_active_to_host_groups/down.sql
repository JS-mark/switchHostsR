DROP INDEX IF EXISTS idx_host_groups_is_active;

CREATE TABLE host_groups__rollback (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id INTEGER NOT NULL,
  name TEXT NOT NULL,
  description TEXT,
  created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now') * 1000),
  updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now') * 1000),
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

INSERT INTO host_groups__rollback (id, user_id, name, description, created_at, updated_at)
SELECT id, user_id, name, description, created_at, updated_at
FROM host_groups;

DROP TABLE host_groups;
ALTER TABLE host_groups__rollback RENAME TO host_groups;

CREATE INDEX IF NOT EXISTS idx_host_groups_user_id ON host_groups(user_id);
