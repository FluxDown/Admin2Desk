DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'auth_method') THEN
        CREATE TYPE auth_method AS ENUM ('password', 'key');
    END IF;
END $$;

CREATE TABLE IF NOT EXISTS ssh_credentials (
    id                   CHAR(36)     NOT NULL PRIMARY KEY,
    target_id            CHAR(36)     NOT NULL REFERENCES ssh_targets(id) ON DELETE CASCADE,
    username             VARCHAR(128) NOT NULL,
    auth_type            auth_method  NOT NULL,
    encrypted_credential BYTEA        NOT NULL,
    created_at           TIMESTAMP    NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_credentials_target ON ssh_credentials (target_id);
