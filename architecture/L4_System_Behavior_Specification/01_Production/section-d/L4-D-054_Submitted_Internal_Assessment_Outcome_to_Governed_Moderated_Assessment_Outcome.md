# Submitted Internal Assessment Outcome to Governed Moderated Assessment Outcome

## 1. Specification Metadata

**Specification ID:** L4-D-054  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Academic Delivery — Internal Assessment  
**Realizes Scenario:** L3-D-054  
**Primary Business Owner:** Internal Assessment  
**Version:** 1.0.0  
**Status:** Architecture Approved  
**Owner:** Pallav  
**Last Updated:** 2026-09-02  
**Depends On:** `L3-D-054_Submitted_Internal_Assessment_Outcome_to_Governed_Moderated_Assessment_Outcome.md`; `L2-D07_Internal_Assessment_Workflows.md`; Locked L4 governance/template/validation baseline  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

Govern moderation of a recorded internal-assessment submission while preserving the original submission/mark history.

**Start boundary:** A recorded Assessment Submission requires moderation under assessment policy or authorized review.

**End boundary:** The Moderation Case reaches a governed Approved or Rejected decision; approved moderated outcome becomes available to D07 finalization.

This specification is limited to observable system behavior required by the Locked parent. It does not absorb adjacent domain outcomes or prescribe implementation mechanisms.

---

## 3. Required Business Context

- Recorded immutable original submission
- Blueprint/policy/evidence
- Applicable independent academic review
- Authoritative moderation decision

All source-owned facts remain owned by their canonical business domains.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-D-054-BH-001 | The system shall establish a moderation case linked to the immutable original submission. | L3-D-054 §§3–13 |
| L4-D-054-BH-002 | The system shall validate moderation evidence, blueprint component and policy before review. | L3-D-054 §§3–13 |
| L4-D-054-BH-003 | The system shall require the governed independent academic review whenever D07 policy requires such review. | L3-D-054 §§3–13 |
| L4-D-054-BH-004 | The system shall require an authoritative D07 moderation decision before adopting or rejecting a proposed moderated outcome. | L3-D-054 §§3–13 |
| L4-D-054-BH-005 | The system shall for an approved moderation, record the moderated outcome as a new governed fact while preserving the original submission. | L3-D-054 §§3–13 |
| L4-D-054-BH-006 | The system shall for a rejected moderation, retain the prior valid submission outcome as authoritative. | L3-D-054 §§3–13 |
| L4-D-054-BH-007 | The system shall make an approved moderated outcome available to D07 finalization without finalizing the full assessment in this scenario. | L3-D-054 §§3–13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-D-054-BH-008 | The system shall publish `AssessmentModerationRequested` when the moderation case is established. | L3-D-054 §14; L2-D07_Internal_Assessment_Workflows.md exact event semantics |
| L4-D-054-BH-009 | The system shall publish `AssessmentModerationCompleted` when an approved moderated outcome is completed. | L3-D-054 §14; L2-D07_Internal_Assessment_Workflows.md exact event semantics |

### Ownership-Preserving Collaboration

No synchronous cross-domain business hand-off materially participates beyond the parent-defined facts and ownership boundaries.

Event names and publisher direction remain exactly those established by the parent/upstream architecture. No message schema, broker, queue, delivery or retry design is defined here.

---

## 6. Failure / Rejection / Continuation

- Incomplete evidence/policy blocks moderation decision.
- Missing valid moderation authority preserves the current business position.
- Interrupted moderation does not create a completed moderated outcome.

In every failure or interruption path, the system shall preserve the last valid business position and expose only the source-supported business continuation/rejection outcome. Technical rollback, retry and infrastructure recovery are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to demonstrate this specification: Preserve original submission, moderation request/evidence, independent review, authority/decision/reason, moderated outcome and completion fact.

Evidence behavior remains implementation-independent and does not prescribe tables, logs, storage engines or retention technology.

---

## 8. Observable Completion Conditions

- moderation has a governed Approved or Rejected decision
- original submission history remains unchanged
- approved moderated outcome is available to finalization where applicable

The specification is not complete merely because a technical action occurred; the applicable governed business outcome above must be objectively observable.

---

## 9. Traceability & Provenance

**Parent:** `L3-D-054` — Submitted Internal Assessment Outcome to Governed Moderated Assessment Outcome  
**Primary verification source:** `L2-D07_Internal_Assessment_Workflows.md`  
**Architecture boundary:** Locked L4 Governance and Architecture Standard; Locked L4 Validation Standard  

Every BH is derived from the Locked L3 parent. The L2 source is used only to verify exact owner, rule, lifecycle and event semantics already represented by the parent; this L4 specification does not silently repair or expand the L3 business scenario.

---

## 10. Open Issues / Deferred Design

No L4-specific open issue remains under the current Locked parent baseline.

The following remain deferred to L5/L6: API/interface design, persistence/data model, service/component design, event transport, technical access control, transaction/retry/rollback mechanics, infrastructure/deployment and UI/UX design.

---

**Canonical Validation Note:** This specification is **Architecture Approved** under the final Architecture Approval issued by Dhruv / Lead Architect on 2026-09-02, as reported for this Section D batch. The specification is not `Locked`; movement to `Locked` requires separate explicit governance authorization.
