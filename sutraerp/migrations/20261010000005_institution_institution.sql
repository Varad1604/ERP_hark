-- Reference aggregate demonstrating the persistence conventions end to end:
-- UUIDv7 technical id, tenant-scoped uniqueness, optimistic concurrency `version`,
-- timestamptz instants. Business attributes are deliberately minimal until Batch 2.
CREATE TABLE institution.institution (
    id          uuid        PRIMARY KEY DEFAULT uuidv7(),
    tenant_id   uuid        NOT NULL,
    code        text        NOT NULL,
    name        text        NOT NULL,
    timezone    text        NOT NULL DEFAULT 'Asia/Kolkata',          -- business-local tz (ADR-L5-0036 §3.4)
    version     bigint      NOT NULL DEFAULT 1 CHECK (version >= 1),  -- ADR-L5-0006
    created_at  timestamptz NOT NULL DEFAULT now(),
    updated_at  timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT institution_code_unique UNIQUE (tenant_id, code)
);
