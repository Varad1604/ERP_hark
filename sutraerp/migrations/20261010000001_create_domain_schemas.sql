-- ADR-L5-0005 / ADR-L5-0036 §3.1: one PostgreSQL schema per canonical owner (domain module).
-- A schema name never creates or renames a business domain; it is the physical realisation of
-- owner-scoped logical persistence. Cross-owner reads/writes against these schemas are prohibited.
-- Runtime roles per schema are provisioned by the deployment (see deploy/postgres/init.sql).
CREATE SCHEMA IF NOT EXISTS platform;    -- A Platform
CREATE SCHEMA IF NOT EXISTS institution; -- B Institution
CREATE SCHEMA IF NOT EXISTS student;     -- C Student lifecycle
CREATE SCHEMA IF NOT EXISTS academic;    -- D Academic
CREATE SCHEMA IF NOT EXISTS people;      -- E People & workforce
CREATE SCHEMA IF NOT EXISTS finance;     -- F Finance
CREATE SCHEMA IF NOT EXISTS campus;      -- G Campus
CREATE SCHEMA IF NOT EXISTS services;    -- H Services
CREATE SCHEMA IF NOT EXISTS exams;       -- I Examinations
CREATE SCHEMA IF NOT EXISTS resources;   -- J Resources

COMMENT ON SCHEMA platform IS 'Owner: A Platform (audit, outbox, idempotency, sessions)';
COMMENT ON SCHEMA institution IS 'Owner: B Institution';
COMMENT ON SCHEMA student IS 'Owner: C Student lifecycle';
COMMENT ON SCHEMA academic IS 'Owner: D Academic';
COMMENT ON SCHEMA people IS 'Owner: E People & workforce';
COMMENT ON SCHEMA finance IS 'Owner: F Finance';
COMMENT ON SCHEMA campus IS 'Owner: G Campus';
COMMENT ON SCHEMA services IS 'Owner: H Services';
COMMENT ON SCHEMA exams IS 'Owner: I Examinations';
COMMENT ON SCHEMA resources IS 'Owner: J Resources';
