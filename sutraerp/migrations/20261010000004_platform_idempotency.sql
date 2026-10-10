-- ADR-L5-0034 §3.6 owner-scoped idempotency records. Retention is profile-driven; expires_at is
-- set by the writer and swept by a background job (Batch 1).
CREATE TABLE platform.idempotency_key (
    scope_key        text        NOT NULL,
    idempotency_key  text        NOT NULL,
    fingerprint      text        NOT NULL,
    state            text        NOT NULL CHECK (state IN ('in_flight', 'completed')),
    response_status  smallint,
    response_type    text,
    response_body    bytea,
    created_at       timestamptz NOT NULL DEFAULT now(),
    completed_at     timestamptz,
    expires_at       timestamptz NOT NULL DEFAULT now() + interval '24 hours',
    PRIMARY KEY (scope_key, idempotency_key)
);

CREATE INDEX idempotency_key_expiry_idx ON platform.idempotency_key (expires_at);
