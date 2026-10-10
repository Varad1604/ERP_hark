-- Append-only audit evidence. Tamper-evident chaining/anchoring (ADR-L5-0012) arrives in Batch 9.
CREATE TABLE platform.audit_log (
    id              uuid        PRIMARY KEY DEFAULT uuidv7(),          -- ADR-L5-0036 §3.3 (PG18 native)
    occurred_at     timestamptz NOT NULL DEFAULT now(),               -- ADR-L5-0036 §3.4 (UTC instant)
    tenant_id       uuid        NOT NULL,                             -- ADR-L5-0004: tenant mandatory
    institution_id  uuid,
    actor_kind      text        NOT NULL CHECK (actor_kind IN ('user', 'service', 'system', 'anonymous')),
    actor_id        text,
    action          text        NOT NULL,
    target_type     text,
    target_id       text,
    outcome         text        NOT NULL CHECK (outcome IN ('succeeded', 'rejected', 'failed')),
    correlation_id  text,
    details         jsonb       NOT NULL DEFAULT '{}'::jsonb
);

CREATE INDEX audit_log_tenant_time_idx ON platform.audit_log (tenant_id, occurred_at DESC);
CREATE INDEX audit_log_target_idx ON platform.audit_log (tenant_id, target_type, target_id);

CREATE FUNCTION platform.audit_log_immutable() RETURNS trigger
    LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION 'platform.audit_log is append-only';
END;
$$;

CREATE TRIGGER audit_log_no_update_delete
    BEFORE UPDATE OR DELETE ON platform.audit_log
    FOR EACH ROW EXECUTE FUNCTION platform.audit_log_immutable();
