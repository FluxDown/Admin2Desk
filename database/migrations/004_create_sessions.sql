DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'session_status') THEN
        CREATE TYPE session_status AS ENUM ('active', 'completed', 'failed', 'terminated');
    END IF;
END $$;

CREATE TABLE IF NOT EXISTS sessions (
    id              CHAR(36)       NOT NULL PRIMARY KEY,
    user_id         CHAR(36)       NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    target_id       CHAR(36)       NOT NULL REFERENCES ssh_targets(id) ON DELETE RESTRICT,
    credential_id   CHAR(36)       NOT NULL REFERENCES ssh_credentials(id) ON DELETE RESTRICT,
    started_at      TIMESTAMP      NOT NULL DEFAULT NOW(),
    ended_at        TIMESTAMP      NULL,
    status          session_status NOT NULL DEFAULT 'active',
    recording_path  VARCHAR(512)   NULL
);

CREATE INDEX IF NOT EXISTS idx_sessions_user ON sessions (user_id);
CREATE INDEX IF NOT EXISTS idx_sessions_target ON sessions (target_id);
CREATE INDEX IF NOT EXISTS idx_sessions_status ON sessions (status);
CREATE INDEX IF NOT EXISTS idx_sessions_started ON sessions (started_at);
