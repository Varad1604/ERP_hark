# SutraERP L4 Controlled Pilot Workspace

**Architecture Level:** L4 — System Behavior Specification  
**Status:** Draft  
**Purpose:** Controlled pilot only  
**Authority:** User-authorized L4 pilot on 2026-08-26  

This folder contains provisional L4 System Behavior Specification pilot artifacts used to validate the proposed L4 method before any naming, final template, physical granularity, workload model, or broad authoring rule is locked.

## Pilot rules

- Every pilot artifact must trace to an eligible approved L3 parent.
- Pilot IDs and filenames are provisional and do not establish final repository naming governance.
- One parent file with multiple `BH` obligations is the current pilot default, not a locked corpus rule.
- Every `BH` must have defensible upstream provenance.
- L4 must not redefine L3 business meaning, ownership, rules, lifecycle, authority, event names, or publisher direction.
- APIs, database schemas, services, DTOs, queues, UI design, technical RBAC, deployment, retries, rollback, and technology decisions are prohibited.
- Pilot findings may require L3 remediation, template refinement, or L4 methodology change before scale.
- No pilot artifact may be promoted to `Locked` through this workspace.

## Provisional pilot structure

1. Specification Metadata
2. Behavioral Objective & Boundary
3. Required Business Context
4. System Behavioral Obligations
5. Cross-Domain & Event Behavior
6. Failure / Rejection / Continuation
7. Audit & Evidence Behavior
8. Observable Completion Conditions
9. Traceability & Provenance
10. Open Issues / Deferred Design
