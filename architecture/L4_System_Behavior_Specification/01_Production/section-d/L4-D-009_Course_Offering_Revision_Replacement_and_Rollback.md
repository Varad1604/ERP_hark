# Course Offering Revision, Replacement and Rollback

## 1. Specification Metadata

**Specification ID:** L4-D-009  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Academic Delivery — Course & Course Offering  
**Realizes Scenario:** L3-D-009  
**Primary Business Owner:** Course & Course Offering  
**Version:** 1.0.0  
**Status:** Architecture Approved  
**Owner:** Pallav  
**Last Updated:** 2026-09-02  
**Depends On:** `L3-D-009_Course_Offering_Revision_Replacement_and_Rollback.md`; `L2-D01_Course_and_Course_Offering_Workflows.md`; Locked L4 governance/template/validation baseline  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

Govern version-based Course Offering revision/replacement or business rollback while retaining exactly one current version and immutable lineage.

**Start boundary:** An authorized successor-version or rollback requirement exists for a non-Archived Course Offering with preserved version history.

**End boundary:** An authorized successor becomes current or an approved historical target is restored, with all intervening history preserved.

This specification is limited to observable system behavior required by the Locked parent. It does not absorb adjacent domain outcomes or prescribe implementation mechanisms.

---

## 3. Required Business Context

- Non-Archived Course Offering
- Immutable version lineage
- Approved rollback target where rollback is requested
- Version/lifecycle policy and effective dates

All source-owned facts remain owned by their canonical business domains.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-D-009-BH-001 | The system shall classify the request as successor revision/replacement or rollback. | L3-D-009 §§3–13 |
| L4-D-009-BH-002 | The system shall prevent version action when the offering is Archived or the lifecycle prohibits the change. | L3-D-009 §§3–13 |
| L4-D-009-BH-003 | The system shall validate version policy, effective dates and historical integrity. | L3-D-009 §§3–13 |
| L4-D-009-BH-004 | The system shall for revision/replacement, create a new Draft successor linked to its predecessor. | L3-D-009 §§3–13 |
| L4-D-009-BH-005 | The system shall permit a successor to become current only after the required validation, authorization and publication conditions are satisfied. | L3-D-009 §§3–13 |
| L4-D-009-BH-006 | The system shall preserve the previous current version as immutable history when a successor becomes current. | L3-D-009 §§3–13 |
| L4-D-009-BH-007 | The system shall for rollback, require an existing Approved target and reject an invalid target. | L3-D-009 §§3–13 |
| L4-D-009-BH-008 | The system shall restore the approved historical target as the governed current version without deleting intervening history. | L3-D-009 §§3–13 |
| L4-D-009-BH-009 | The system shall maintain exactly one designated current version after a completed successor or rollback outcome. | L3-D-009 §§3–13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-D-009-BH-010 | The system shall publish `CourseOfferingVersionCreated` when a successor Draft is established. | L3-D-009 §14; L2-D01_Course_and_Course_Offering_Workflows.md exact event semantics |
| L4-D-009-BH-011 | The system shall publish `CourseOfferingVersionPublished` when the authorized successor becomes current. | L3-D-009 §14; L2-D01_Course_and_Course_Offering_Workflows.md exact event semantics |
| L4-D-009-BH-012 | The system shall publish `CourseOfferingRollbackRequested` when a governed rollback request is established. | L3-D-009 §14; L2-D01_Course_and_Course_Offering_Workflows.md exact event semantics |
| L4-D-009-BH-013 | The system shall publish `CourseOfferingRollbackCompleted` when the approved target is restored. | L3-D-009 §14; L2-D01_Course_and_Course_Offering_Workflows.md exact event semantics |

### Ownership-Preserving Collaboration

No synchronous cross-domain business hand-off materially participates beyond the parent-defined facts and ownership boundaries.

Event names and publisher direction remain exactly those established by the parent/upstream architecture. No message schema, broker, queue, delivery or retry design is defined here.

---

## 6. Failure / Rejection / Continuation

- Archived/lifecycle-ineligible offering preserves current version.
- Invalid/unapproved rollback target rejects rollback.
- Invalid successor remains outside the published/current position.

In every failure or interruption path, the system shall preserve the last valid business position and expose only the source-supported business continuation/rejection outcome. Technical rollback, retry and infrastructure recovery are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to demonstrate this specification: Preserve complete predecessor/successor lineage, current-version designation, validation/authority basis, rollback rationale and all exact version events.

Evidence behavior remains implementation-independent and does not prescribe tables, logs, storage engines or retention technology.

---

## 8. Observable Completion Conditions

- exactly one governed current version exists
- all other versions remain immutable historical versions
- the applicable exact version/rollback facts are published

The specification is not complete merely because a technical action occurred; the applicable governed business outcome above must be objectively observable.

---

## 9. Traceability & Provenance

**Parent:** `L3-D-009` — Course Offering Revision, Replacement and Rollback  
**Primary verification source:** `L2-D01_Course_and_Course_Offering_Workflows.md`  
**Architecture boundary:** Locked L4 Governance and Architecture Standard; Locked L4 Validation Standard  

Every BH is derived from the Locked L3 parent. The L2 source is used only to verify exact owner, rule, lifecycle and event semantics already represented by the parent; this L4 specification does not silently repair or expand the L3 business scenario.

---

## 10. Open Issues / Deferred Design

Rollback here is a business version-restoration outcome, not a technical transaction rollback.

The following remain deferred to L5/L6: API/interface design, persistence/data model, service/component design, event transport, technical access control, transaction/retry/rollback mechanics, infrastructure/deployment and UI/UX design.

---

**Canonical Validation Note:** This specification is **Architecture Approved** under the final Architecture Approval issued by Dhruv / Lead Architect on 2026-09-02, as reported for this Section D batch. The specification is not `Locked`; movement to `Locked` requires separate explicit governance authorization.
