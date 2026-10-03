# Offline A06/J05 registry evidence checker

**Status:** evidence structural gate ONLY. This utility does not connect to a registry, certify signatures, authorize activation, generate active definitions, create events or approve institutional policy.

Run after A06 supplies an independently authenticated read-only registry export AND the owners provide source-linked consumer/approval evidence:
`python3 tools/a06_j05_registry_evidence_check.py /secure/path/to/a06-evidence-bundle.json`

Run unit tests without CI:
`python3 -m unittest discover -s tools/tests -p 'test_a06_j05_registry.py'`

The input is a reviewed JSON *evidence mapping* (not an active schema) with `provenance` (registryId, environment, capturedAt, operator, authorityReference, exportDigest), `definitions` (exactly both candidate names; status ACTIVE, INACTIVE or NOT_FOUND; for ACTIVE require definitionId, owner J05, defaultVersion true, immutable version/schemaId/schemaSha256/schemaReference, approvalReference, activatedAt and businessMeaning), `consumerChecks` (actual J06/J08/F14 subscription or NOT_SUBSCRIBED decisions, references and compatibility for SUPPORTED), and `approval` (a06DecisionReference, j05DecisionReference, securityReviewReference).

A structural PASS means *only that all claimed fields are present*. Do not enter placeholder strings outside controlled unit tests. Someone must independently compare every field to the authentic external registry, verify digest/signatures and review actual compatibility/authorization. This utility deliberately does **not** establish that the external source exists, that either named event is active, that the event can carry finalised-correction lineage or that positive J05 correction/replacement is approved. If neither/both names are Active, reroute under L3-A-018/L3-A-060; do not loosen these fail-closed checks to pass an unsupported case.

Current repositories checked: current Architecture + frontend, plus separately discovered older Rust `DhruvNaik26/SutraERP` (legacy in-memory bus only). No authentic A06 export is yet available in those repositories. See Architecture #152 and merged documentary PRs #151/#153.
