# Course Offering Reactivation

## 1. Specification Metadata

**Specification ID:** L4-D-003  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Academic Delivery — Course & Course Offering  
**Realizes Scenario:** L3-D-003  
**Primary Business Owner:** Course & Course Offering  
**Version:** 1.0.0  
**Status:** Architecture Approved  
**Owner:** Pallav  
**Last Updated:** 2026-09-02  
**Depends On:** `L3-D-003_Course_Offering_Reactivation.md`; `L2-D01_Course_and_Course_Offering_Workflows.md`; Locked L4 governance/template/validation baseline  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

Restore a Suspended Course Offering to Active only after the suspension reason is resolved and authoritative policy/readiness validation succeeds.

**Start boundary:** A Course Offering is Suspended and its suspension reason has been reported as resolved.

**End boundary:** The offering reaches Active and the completed reactivation fact is published.

This specification is limited to observable system behavior required by the Locked parent. It does not absorb adjacent domain outcomes or prescribe implementation mechanisms.

---

## 3. Required Business Context

- Suspended Course Offering
- Resolved suspension reason
- Applicable reactivation policy
- Authoritative readiness evidence

All source-owned facts remain owned by their canonical business domains.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-D-003-BH-001 | The system shall verify that the offering is Suspended and the recorded suspension reason is resolved. | L3-D-003 §§3–13 |
| L4-D-003-BH-002 | The system shall evaluate the applicable reactivation policy and authoritative readiness results. | L3-D-003 §§3–13 |
| L4-D-003-BH-003 | The system shall prevent reactivation while the suspension reason remains unresolved. | L3-D-003 §§3–13 |
| L4-D-003-BH-004 | The system shall prevent reactivation when policy or readiness validation fails and retain Suspended. | L3-D-003 §§3–13 |
| L4-D-003-BH-005 | The system shall recognize the authoritative reactivation decision without becoming the business approver. | L3-D-003 §§3–13 |
| L4-D-003-BH-006 | The system shall transition Suspended → Active only when all required conditions are satisfied. | L3-D-003 §§3–13 |
| L4-D-003-BH-007 | The system shall preserve the prior suspension history and the same Course Offering identity. | L3-D-003 §§3–13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-D-003-BH-008 | The system shall publish `CourseOfferingReactivated` when restoration to Active completes. | L3-D-003 §14; L2-D01_Course_and_Course_Offering_Workflows.md exact event semantics |

### Ownership-Preserving Collaboration

No synchronous cross-domain business hand-off materially participates beyond the parent-defined facts and ownership boundaries.

Event names and publisher direction remain exactly those established by the parent/upstream architecture. No message schema, broker, queue, delivery or retry design is defined here.

---

## 6. Failure / Rejection / Continuation

- Unresolved suspension reason retains Suspended.
- Failed policy/readiness validation retains Suspended with the validation basis.

In every failure or interruption path, the system shall preserve the last valid business position and expose only the source-supported business continuation/rejection outcome. Technical rollback, retry and infrastructure recovery are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to demonstrate this specification: Preserve suspension resolution evidence, policy/readiness basis, authoritative decision, prior Suspended state, resulting Active state and reactivation fact.

Evidence behavior remains implementation-independent and does not prescribe tables, logs, storage engines or retention technology.

---

## 8. Observable Completion Conditions

- the suspension reason is resolved
- required policy/readiness validation succeeds
- the offering is Active
- `CourseOfferingReactivated` is published

The specification is not complete merely because a technical action occurred; the applicable governed business outcome above must be objectively observable.

---

## 9. Traceability & Provenance

**Parent:** `L3-D-003` — Course Offering Reactivation  
**Primary verification source:** `L2-D01_Course_and_Course_Offering_Workflows.md`  
**Architecture boundary:** Locked L4 Governance and Architecture Standard; Locked L4 Validation Standard  

Every BH is derived from the Locked L3 parent. The L2 source is used only to verify exact owner, rule, lifecycle and event semantics already represented by the parent; this L4 specification does not silently repair or expand the L3 business scenario.

---

## 10. Open Issues / Deferred Design

No L4-specific open issue remains under the current Locked parent baseline.

The following remain deferred to L5/L6: API/interface design, persistence/data model, service/component design, event transport, technical access control, transaction/retry/rollback mechanics, infrastructure/deployment and UI/UX design.

---

**Canonical Validation Note:** This specification is **Architecture Approved** under the final Architecture Approval issued by Dhruv / Lead Architect on 2026-09-02, as reported for this Section D batch. The specification is not `Locked`; movement to `Locked` requires separate explicit governance authorization.
