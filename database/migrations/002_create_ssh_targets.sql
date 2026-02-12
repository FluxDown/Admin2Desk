CREATE TABLE IF NOT EXISTS ssh_targets (
    id          CHAR(36)    NOT NULL PRIMARY KEY,
    name        VARCHAR(128) NOT NULL,
    hostname    VARCHAR(255) NOT NULL,
    port        SMALLINT    NOT NULL DEFAULT 22,
    description TEXT,
    created_by  CHAR(36)    NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    created_at  TIMESTAMP   NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMP   NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_targets_name ON ssh_targets (name);
CREATE INDEX IF NOT EXISTS idx_targets_created_by ON ssh_targets (created_by);
