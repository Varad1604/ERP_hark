-- Local bootstrap (runs once on an empty volume).
-- Keycloak gets its own database; SutraERP domain schemas are created by migrations.
CREATE ROLE keycloak LOGIN PASSWORD 'keycloak';
CREATE DATABASE keycloak OWNER keycloak;

-- Per-owner runtime roles (ADR-L5-0036 §3.1). Batch 1 switches the API to per-schema roles;
-- Batch 0 runs migrations and the API as the `sutra` owner for simplicity.
DO $$
DECLARE s text;
BEGIN
  FOREACH s IN ARRAY ARRAY['platform','institution','student','academic','people','finance','campus','services','exams','resources'] LOOP
    EXECUTE format('CREATE ROLE %I NOLOGIN', 'sutra_' || s);
  END LOOP;
END $$;
