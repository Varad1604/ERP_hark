# Examination Policy Revision to Authoritative Superseding Policy

## 1. Specification Metadata


**Specification ID:** ** `L4-I-002`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Examination Policy & Configuration (I01)  
**Realizes Scenario:** ** `L3-I-002` — Examination Policy Revision to Authoritative Superseding Policy  
**Primary Business Owner:** Examination Policy & Configuration (I01)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-I-002_Examination_Policy_Revision_to_Authoritative_Superseding_Policy.md`; `L3-I-001`; `L2-I01_Examination_Policy_and_Configuration_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** ** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed revision of an active Examination Policy by creating a superseding policy version, preserving the prior version, and sealing cohort transition/grandfathering rules.

The behavior begins when the Academic Council enacts a statutory amendment requiring revision of the active Examination Policy. It ends when the prior Examination Policy Aggregate reaches `POLICY_SUPERSEDED` and the superseding policy is established as the authoritative policy for eligible future use.

This specification does not alter an activated policy in place, apply a revision retroactively to protected in-flight examination sessions, or define technical version-control implementation.

---

## 3. Required Business Context

- Existing active Examination Policy in `POLICY_ACTIVE_LOCKED`.
- Statutory amendment requiring policy revision.
- Academic Council.
- Controller of Examinations (CoE).
- Dean of Academic Affairs.
- Registrar.
- Required amendment content and cohort-transition/grandfathering rules.
- Parent-defined in-flight-session protection, immutable-versioning, and record-preservation rules.
- Examination Policy & Configuration ownership of the Examination Policy Aggregate.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-002-BH-001` | When a valid statutory amendment requires policy revision, the system shall recognize the current Examination Policy Aggregate in `POLICY_ACTIVE_LOCKED` as the source version to be superseded. | L3-I-002 §§4, 9, 13 |
| `L4-I-002-BH-002` | The system shall require the revised policy to be represented as a new version rather than modifying the locked active version in place. | L3-I-002 §§2–3, 12 |
| `L4-I-002-BH-003` | The system shall preserve the amendment clauses and the cohort-transition/grandfathering rules required by the parent scenario. | L3-I-002 §§2–3 |
| `L4-I-002-BH-004` | The system shall prevent the revised policy from changing the governing policy of an in-flight examination session consistent with the parent-defined in-flight examination session protection requirement from L3-I-002 §3 and BR-I01-001. | L3-I-002 §12 |
| `L4-I-002-BH-005` | The system shall require academic/regulatory scrutiny and valid Academic Council ratification before the superseding policy may become authoritative. | L3-I-002 §§9, 12, 15 |
| `L4-I-002-BH-006` | The system shall prevent the policy drafter or Controller of Examinations from unilaterally altering Academic Council-approved provisions. | L3-I-002 §15.2 |
| `L4-I-002-BH-007` | On valid supersession, the system shall transition the prior Examination Policy Aggregate from `POLICY_ACTIVE_LOCKED` to `POLICY_SUPERSEDED`. | L3-I-002 §§9, 13, 16 |
| `L4-I-002-BH-008` | The system shall preserve an explicit superseded-reference relationship between the prior version and the authoritative successor. | L3-I-002 §§3–4 |
| `L4-I-002-BH-009` | The system shall preserve the prior version and its legislative history for perpetual auditability. | L3-I-002 §§12, 15.3 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-002-BH-010` | The system shall recognize the canonical `Academic_Council_Resolution_Passed` event from Governance as the authorization basis for policy supersession without transferring Governance ownership. | L3-I-002 §14.1 |
| `L4-I-002-BH-011` | The system shall preserve the Governance quorum/assent handoff used to validate the superseding policy without prescribing a technical contract implementation. | L3-I-002 §14.2 |
| `L4-I-002-BH-012` | When the prior policy is authoritatively superseded, the system shall produce the canonical `Examination_Policy_Revised_and_Superseded` business event on behalf of Examination Policy & Configuration. | L3-I-002 §§14, 16 |

No API, event-transport, version-store, or technical notification mechanism is defined.

---

## 6. Failure / Rejection / Continuation

- If Academic Council rejects the proposed revision, the system shall prevent supersession and preserve the parent-defined `DRAFT_REJECTED` outcome.
- If a statutory conflict is identified, the system shall preserve the parent-defined `STATUTORY_HOLD` condition.
- If cohort-transition rules are incomplete, the revision shall not be treated as fully authoritative for future examination cycles.
- In-flight examination sessions shall remain bound to their protected governing policy unless higher architecture explicitly permits otherwise.
- Expedited executive ordinance handling may be recognized only with the parent-defined ex-post-facto ratification obligation.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- source active policy version;
- statutory amendment;
- revised policy version;
- cohort transition/grandfathering rules;
- academic/regulatory review;
- Academic Council resolution and quorum;
- Controller of Examinations endorsement;
- prior-to-successor relationship;
- preserved prior-version legislative history;
- resulting `POLICY_SUPERSEDED` position;
- publication of `Examination_Policy_Revised_and_Superseded`.

---

## 8. Observable Completion Conditions

L4 behavior is complete when:

- the prior Examination Policy Aggregate is `POLICY_SUPERSEDED`;
- the superseding policy version is authoritative for its approved applicability;
- protected in-flight sessions remain governed by their valid prior policy;
- cohort transition/grandfathering rules are sealed;
- historical evidence is preserved;
- `Examination_Policy_Revised_and_Superseded` has been produced.

---

## 9. Traceability & Provenance

**Parent:** `L3-I-002` — Examination Policy Revision to Authoritative Superseding Policy  
**Upstream parent dependency:** `L3-I-001`  
**Primary verification source:** `L2-I01_Examination_Policy_and_Configuration_Workflows.md`  
**Cross-domain authority source:** Section B — Governance  
**Higher-level verification:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`

All BHs derive from the Locked L3 parent. The specification preserves immutable versioning and does not introduce in-place policy mutation.

---

## 10. Open Issues / Deferred Design

No material L4 architecture issue remains for this draft.

Policy version-store implementation, cohort-resolution algorithms, signature technology, persistence, event transport, notification mechanisms, and UI/UX remain deferred.
