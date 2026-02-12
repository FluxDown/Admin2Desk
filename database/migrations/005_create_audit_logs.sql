CREATE TABLE IF NOT EXISTS audit_logs (
    id          CHAR(36)     NOT NULL PRIMARY KEY,
    user_id     CHAR(36)     NULL REFERENCES users(id) ON DELETE SET NULL,
    action      VARCHAR(128) NOT NULL,
    details     JSONB        NULL,
    ip_address  VARCHAR(45)  NOT NULL,
    created_at  TIMESTAMP    NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_audit_user ON audit_logs (user_id);
CREATE INDEX IF NOT EXISTS idx_audit_action ON audit_logs (action);
CREATE INDEX IF NOT EXISTS idx_audit_created ON audit_logs (created_at);
