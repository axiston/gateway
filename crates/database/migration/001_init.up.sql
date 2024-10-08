CREATE TABLE runtimes
(
    id           UUID PRIMARY KEY   DEFAULT gen_random_uuid(),
    displayed_id UUID      NOT NULL DEFAULT gen_random_uuid(),

    name         TEXT               DEFAULT NULL,
    description  TEXT               DEFAULT NULL,
    is_enabled   BOOLEAN   NOT NULL DEFAULT FALSE,
    is_secured   BOOLEAN   NOT NULL DEFAULT FALSE,

    created_at   TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at   TIMESTAMP NOT NULL DEFAULT current_timestamp
);

-- TODO: Settings

CREATE TABLE invites
(
    id               SERIAL PRIMARY KEY,
    invitation_token CHARACTER VARYING(32),

    created_at       TIMESTAMP NOT NULL               DEFAULT now(),
    activated_at     TIMESTAMP                        DEFAULT NULL,
    activated_by     INTEGER REFERENCES accounts (id) DEFAULT NULL,
    deleted_at       TIMESTAMP                        DEFAULT NULL
);

CREATE TABLE accounts
(
    id             SERIAL PRIMARY KEY,

    account_name   TEXT      NOT NULL UNIQUE,
    displayed_name TEXT               DEFAULT NULL,
    pwd_hash       TEXT      NOT NULL,

    -- Ensures the account name is alphanumeric and has 4 to 12 chars.
    CONSTRAINT valid_account_name CHECK (account_name ~ '^[a-zA-Z0-9]{4,12}$'),

--     recovery_token       CHARACTER VARYING(255),
--     recovery_sent_at     TIMESTAMP WITH TIME ZONE,
--     email_change_token   CHARACTER VARYING(255),
--     email_change         CHARACTER VARYING(255),
--     email_change_sent_at TIMESTAMP WITH TIME ZONE,

    created_at     TIMESTAMP NOT NULL DEFAULT now(),
    updated_at     TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at     TIMESTAMP NOT NULL DEFAULT now(),

    -- Ensure that the account was updated only after it was created.
    CONSTRAINT updated_after_created CHECK (updated_at >= created_at),
    -- Ensure that the account was deleted only after it was created.
    CONSTRAINT deleted_after_created CHECK (deleted_at IS NULL OR deleted_at >= created_at),
    -- Ensure that the account was deleted only after it was updated.
    CONSTRAINT deleted_after_updated CHECK (deleted_at IS NULL OR deleted_at >= updated_at)
);

CREATE TABLE account_emails
(
    id                 SERIAL PRIMARY KEY,
    account_id         INT REFERENCES accounts (id),

    email_address      TEXT      NOT NULL UNIQUE,
    is_primary         BOOLEAN   NOT NULL DEFAULT FALSE,

    -- account_id + confirmation_token = unique
    confirmation_token CHARACTER VARYING(255),
--     confirmation_sent_at TIMESTAMP NOT NULL DEFAULT now(),
--     confirmed_at         TIMESTAMP          DEFAULT NULL,

    -- confirmation_sent_at
    created_at         TIMESTAMP NOT NULL DEFAULT now(),
    -- confirmed_at, mb default null
    updated_at         TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at         TIMESTAMP          DEFAULT NULL,

    -- Ensure that the account email was updated only after it was created.
    CONSTRAINT updated_after_created CHECK (updated_at >= created_at),
    -- Ensure that the account email was deleted only after it was created.
    CONSTRAINT deleted_after_created CHECK (deleted_at IS NULL OR deleted_at >= created_at),
    -- Ensure that the account email was deleted only after it was updated.
    CONSTRAINT deleted_after_updated CHECK (deleted_at IS NULL OR deleted_at >= updated_at)
);

-- Includes only accounts with elevated permissions
CREATE TABLE account_permissions
(
    account_id   INT REFERENCES accounts (id),

    is_readonly  BOOLEAN NOT NULL DEFAULT FALSE,
    is_writeonly BOOLEAN NOT NULL DEFAULT FALSE,
    is_super     BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE projects
(
    id           SERIAL PRIMARY KEY,

    -- Account of the project's owner.
    account_id   INT REFERENCES accounts (id),
    -- Attached project's name (unique for a scope of a single account).
    project_name TEXT      NOT NULL DEFAULT 'project',
    -- Attached project's description.
    project_note TEXT               DEFAULT NULL,

    -- Ensures the unique project path, i.e. ./account/project.
    CONSTRAINT unique_project_path UNIQUE (account_id, project_name),
    -- Ensures the project name is alphanumeric and has 4 to 12 chars.
    CONSTRAINT valid_project_name CHECK (project_name ~ '^[a-zA-Z0-9]{4,12}$'),

    created_at   TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at   TIMESTAMP NOT NULL DEFAULT current_timestamp,
    deleted_at   TIMESTAMP          DEFAULT NULL
);

CREATE TABLE project_members
(
    id          SERIAL PRIMARY KEY,

    account_id  INT REFERENCES accounts (id),
    project_id  INT REFERENCES projects (id),

    invited_by  INT REFERENCES accounts (id),
    is_accepted BOOLEAN   NOT NULL DEFAULT FALSE,

    invited_at  TIMESTAMP NOT NULL DEFAULT current_timestamp,
    accepted_at TIMESTAMP          DEFAULT NULL,
    evicted_at  TIMESTAMP          DEFAULT NULL

);

CREATE TABLE account_projects
(
    id         SERIAL PRIMARY KEY,
    account_id INT       NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    project_id INT       NOT NULL REFERENCES projects (id) ON DELETE CASCADE,

    is_hidden  BOOLEAN   NOT NULL DEFAULT FALSE,

    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);

-- TODO: project/workflow timezone

CREATE TABLE workflows
(
    id                  SERIAL PRIMARY KEY,
    project_id          INT REFERENCES projects (id),

    name                TEXT      NOT NULL,
    description         TEXT      NOT NULL,
    tags                TEXT      NOT NULL,

    -- TODO: is_enabled
    -- TODO: is_verified

    -- Enables the manual trigger execution via the web or mobile application.
    is_manual_enabled   BOOLEAN            DEFAULT TRUE,
    -- Enabled the webhook trigger execution.
    is_webhook_enabled  BOOLEAN            DEFAULT TRUE,
    is_schedule_enabled BOOLEAN            DEFAULT TRUE,

    is_archived         BOOLEAN   NOT NULL DEFAULT FALSE,
    is_private          BOOLEAN   NOT NULL DEFAULT TRUE,
    -- Enables the additional security checks to confirm workflow changes.
    is_secured          BOOLEAN   NOT NULL DEFAULT FALSE,

    -- Enables the periodic trigger executions controlled by the trigger interval.
    is_interval_enabled BOOLEAN            DEFAULT TRUE,
    -- Controls the interval between two consecutive trigger checks.
    -- Expressed in whole seconds as a positive integer.
    -- Defaults to 15 minutes (15 secs * 60 secs per min).
    interval_seconds    BIGINT             DEFAULT 15 * 60,

    CONSTRAINT lower_bound_interval CHECK ( interval_seconds IS NULL OR interval_seconds > 30 ),
    CONSTRAINT upper_bound_interval CHECK ( interval_seconds IS NULL OR interval_seconds < 31536000 ),

    created_at          TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at          TIMESTAMP NOT NULL DEFAULT current_timestamp,
    deleted_at          TIMESTAMP NOT NULL DEFAULT NULL,

    -- Ensure that the workflow was updated after it was created.
    CONSTRAINT updated_after_created CHECK (updated_at >= created_at),
    -- Ensure that the workflow was deleted after it was created.
    CONSTRAINT deleted_after_created CHECK (deleted_at IS NULL OR deleted_at >= created_at),
    -- Ensure that the workflow was deleted after it was updated.
    CONSTRAINT deleted_after_updated CHECK (deleted_at IS NULL OR deleted_at >= updated_at)
);

-- Git-like version control
CREATE TABLE workflow_revisions
(
    id          SERIAL PRIMARY KEY,
    workflow_id INT REFERENCES workflows (id),

    -- Deserialized, validated and again serialized workflow graph in a JSON format.
    graph_data  JSONB     NOT NULL,

    created_at  TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at  TIMESTAMP NOT NULL DEFAULT current_timestamp,
    deleted_at  TIMESTAMP          DEFAULT NULL,

    -- Ensure that the workflow revision was updated only after it was created.
    CONSTRAINT updated_after_created CHECK (updated_at >= created_at),
    -- Ensure that the workflow revision was deleted only after it was created.
    CONSTRAINT deleted_after_created CHECK (deleted_at IS NULL OR deleted_at >= created_at),
    -- Ensure that the workflow revision was deleted only after it was updated.
    CONSTRAINT deleted_after_updated CHECK (deleted_at IS NULL OR deleted_at >= updated_at)
);

CREATE TABLE workflow_executions
(
    id          SERIAL PRIMARY KEY,
    revision_id INT REFERENCES workflows (id),

    updated_at  TIMESTAMP NOT NULL DEFAULT current_timestamp,
    deleted_at  TIMESTAMP NOT NULL DEFAULT NULL

--     workflow_id
--     is_automatic or reason
--     started_at
--     ended_at
);


CREATE TABLE workflow_checks
(
    manual_enabled BOOLEAN DEFAULT TRUE
);

CREATE TABLE workflow_webhooks
(
    id            SERIAL PRIMARY KEY,
    workflow_id   INT REFERENCES workflows (id),
    unique_path   TEXT      NOT NULL UNIQUE,

    is_enabled    BOOLEAN            DEFAULT TRUE,

    -- TODO: Option to override the error code: No webhook, no workflow(?), no auth.

    response_code INT       NOT NULL DEFAULT 200,
    response_text TEXT               DEFAULT NULL,
    response_json JSONB              DEFAULT NULL,

    --- Return the error back to the webhook caller.
    emit_error    BOOLEAN            DEFAULT FALSE,
    error_code    INT                DEFAULT NULL,
    error_text    TEXT               DEFAULT NULL,
    error_json    JSONB              DEFAULT NULL,

    created_at    TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at    TIMESTAMP NOT NULL DEFAULT current_timestamp,
    deleted_at    TIMESTAMP NOT NULL DEFAULT NULL,

    -- Ensure that the webhook was updated only after it was created.
    CONSTRAINT updated_after_created CHECK (updated_at >= created_at),
    -- Ensure that the webhook was deleted only after it was created.
    CONSTRAINT deleted_after_created CHECK (deleted_at IS NULL OR deleted_at >= created_at),
    -- Ensure that the webhook was deleted only after it was updated.
    CONSTRAINT deleted_after_updated CHECK (deleted_at IS NULL OR deleted_at >= updated_at)
);

-- TODO: separate runtimes/registries db
