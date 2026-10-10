-- ADR-L5-0007 Durable Publication Intent (transactional outbox). Rows are inserted in the same
-- owner-local transaction as the state change; the relay (Batch 1, NATS JetStream adapter per
-- ADR-L5-0022) publishes them at-least-once and stamps published_at.
CREATE TABLE platform.outbox (
    id              uuid        PRIMARY KEY,                          -- = CloudEvents id (UUIDv7)
    tenant_id       uuid        NOT NULL,
    institution_id  uuid,
    source          text        NOT NULL,                             -- publishing owner
    event_type      text        NOT NULL,                             -- canonical event contract name
    dataschema      text        NOT NULL,                             -- released schema/version ref
    subject         text,
    envelope        jsonb       NOT NULL,                             -- full CloudEvents envelope
    created_at      timestamptz NOT NULL DEFAULT now(),
    published_at    timestamptz,
    attempts        integer     NOT NULL DEFAULT 0,
    last_error      text,
    next_attempt_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX outbox_pending_idx ON platform.outbox (next_attempt_at, created_at)
    WHERE published_at IS NULL;
