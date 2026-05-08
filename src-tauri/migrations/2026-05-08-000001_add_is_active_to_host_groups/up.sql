ALTER TABLE host_groups ADD COLUMN is_active INTEGER NOT NULL DEFAULT 1;
CREATE INDEX IF NOT EXISTS idx_host_groups_is_active ON host_groups(is_active);
