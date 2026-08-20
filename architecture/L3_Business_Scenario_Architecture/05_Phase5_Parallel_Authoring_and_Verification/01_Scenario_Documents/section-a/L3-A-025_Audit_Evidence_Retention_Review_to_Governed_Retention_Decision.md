# Audit Evidence Retention Review to Governed Retention Decision

# 1. Scenario Metadata

**Scenario ID:** L3-A-025  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Audit Infrastructure  
**Participating Domains:** A  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A04 — Authorization & Policy Workflows; L2-A07 — Audit Infrastructure Workflows; L3-A-022 — Canonical Business Event to Immutable Audit Evidence Availability  
**Referenced By:** L3-A-026 — Retention Decision to Governed Audit Evidence Archival and evidence-governance activities

---

# 2. Scenario Purpose

Evaluate immutable Audit Evidence under applicable institutional Retention Policy and record a governed decision for continued retention, archival eligibility or retention completion without altering evidence.

---

# 3. Business Outcome

A governed retention decision determines continued retention, archival eligibility, or retention completion while preserving evidence immutability and applicable preservation obligations.

The locked L2 workflow, policies and independent lifecycle dimensions support the outcome. This Draft is classified `PASS WITH ISSUES` because L2-A07 defines no canonical retention-decision event except the later `Retention Completed` event.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when audit evidence reaches a point requiring evaluation under applicable institutional retention policy.

## 4.2 End Boundary

The scenario ends when an immutable Retention Governance Decision records continued retention, archival eligibility or retention completion and initiates only the corresponding governed follow-on handling.

## 4.3 In Scope

- Evidence scope, Tenant, institution, classification and policy resolution.
- Preservation, investigation, legal and governance obligations.
- Continued-retention, archival-eligibility or retention-completion decision.
- Independent Governance Status handling and decision traceability.

## 4.4 Out of Scope

- Evidence content modification or deletion.
- Performing archival; that belongs to A026.
- Restoring archived access; that belongs to A027.
- Technical storage tiers, schedules, jobs or disposal implementation.
- Inventing a legal authority or retention period.

## 4.5 Related Scenarios

| Relationship | Scenario | Reason |
|---|---|---|
| Upstream | L3-A-022 — Canonical Business Event to Immutable Audit Evidence Availability | Supplies evidence under active retention. |
| Followed by | L3-A-026 — Retention Decision to Governed Audit Evidence Archival | Executes an archival-eligibility decision. |
| Related | L3-A-024 — Audit Evidence Verification Request to Integrity Verification Outcome | Verification results may inform governance where policy requires. |

---

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Audit Infrastructure / Section A | Owns Retention Governance workflow, decision record and evidence lifecycle dimensions. |
| Authorization & Policy / Section A | Owns Retention, Archival, Access and legal-preservation policy evaluation. |
| Platform & Tenant / Section A | Supplies Tenant and institutional governance context. |

The frozen Participating Domains value remains exactly `A`.

---

# 6. Business Actors, Roles and Responsibilities

L2-A07 defines a policy-driven retention workflow but no universal human Retention Reviewer or Approver. The decision is governed through A04 policy; no placeholder role is invented.

---

# 7. Preconditions and Entry Criteria

- Immutable Audit Evidence exists with Governance Status `Under Retention` unless policy is reviewing an already completed obligation.
- Tenant, institution, Audit Classification and applicable Retention Policy are known.
- Legal/institutional preservation obligations and investigation holds, where applicable, can be resolved by policy.
- Evidence relationships and governance history remain traceable.

---

# 8. Trigger

Audit evidence reaches a point requiring evaluation under applicable institutional retention policy.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Identify the evidence scope requiring retention review. | Audit Infrastructure | A governed review scope is established. |
| 2 | Resolve Tenant, institution, classification, current Storage/Availability/Governance Status and policy references. | Audit Infrastructure; Platform & Tenant | Current lifecycle context is explicit. |
| 3 | Evaluate the applicable Retention Policy and active preservation obligations. | Authorization & Policy | Governing preservation requirements are determined. |
| 4 | Consider investigation, legal, compliance and relationship-preservation constraints required by policy. | Authorization & Policy; Audit Infrastructure | No protected evidence is released prematurely. |
| 5 | Determine continued retention, archival eligibility or retention completion. | Authorization & Policy supplies policy decision; Audit Infrastructure applies it | One authoritative lifecycle outcome is selected. |
| 6 | Record an immutable Retention Governance Decision with evidence scope and policy reference. | Audit Infrastructure | The decision becomes traceable without changing evidence. |
| 7 | For continued retention, retain Governance Status `Under Retention`. | Audit Infrastructure | Preservation continues. |
| 8 | For archival eligibility, hand off the governed decision to A026 without changing evidence content. | Audit Infrastructure | Archival may begin separately. |
| 9 | For retention completion, update Governance Status to `Retention Completed` and publish `Retention Completed`. | Audit Infrastructure | The active obligation ends; evidence remains preserved. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Path | Outcome | Owner |
|---|---|---|---|---|
| Continued retention | Policy requires ongoing preservation. | Keep evidence under retention and record the next governed review condition where policy defines it. | No archival/completion transition. | Audit Infrastructure |
| Archival eligible | Active access is no longer required but preservation continues. | Record eligibility and hand off to A026. | Evidence remains unchanged pending archival. | Audit Infrastructure |
| Retention completed | Active preservation obligation concludes. | Update only Governance Status and publish the exact completion event. | Evidence remains preserved under governed storage/access controls. | Audit Infrastructure |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Resolution | Owner |
|---|---|---|---|
| Applicable policy cannot be resolved. | No governed decision can be made. | Continue current preservation and defer the decision. | Authorization & Policy; Audit Infrastructure |
| Tenant, institution or classification is inconsistent. | Policy application may be invalid. | Stop review until canonical context is resolved. | Platform & Tenant; Audit Infrastructure |
| Active preservation obligation prohibits archival/completion. | Proposed outcome is ineligible. | Continue retention. | Authorization & Policy |
| Decision would delete or mutate evidence. | Audit invariants would be violated. | Reject that outcome; evidence remains immutable and preserved. | Audit Infrastructure |
| Decision record is incomplete. | Governance is not traceable. | Do not initiate archival or retention completion. | Audit Infrastructure |

---

# 12. Business Rules and Decision Constraints

| Rule | Owner | Effect | Source |
|---|---|---|---|
| Retention is policy driven and Tenant/institution aware. | Authorization & Policy | No implementation-specific or cross-boundary decision is permitted. | L2-A07 §§28, 31 |
| Evidence, relationships, accountability and historical ordering remain preserved. | Audit Infrastructure | Retention never mutates the historical record. | L2-A07 §31.1 |
| Storage, Availability and Governance Status are independent. | Audit Infrastructure | Retention decision changes only the applicable dimension. | L2-A07 §21.2 |
| Retention completion does not delete, tombstone or rewrite evidence. | Audit Infrastructure | Evidence remains governed after obligation completion. | L2-A07 §21.9 |
| Policy changes never rewrite historical evidence or prior decisions. | Authorization & Policy; Audit Infrastructure | Governance history remains traceable. | L2-A07 §28.2 |

---

# 13. State and Lifecycle Impact

| Concept | Entry | Impact | Result | Owner |
|---|---|---|---|---|
| Audit Evidence | Created Once; immutable | Referenced by decision only. | Unchanged. | Audit Infrastructure |
| Governance Status | `Under Retention` | Continue obligation. | Remains `Under Retention`. | Audit Infrastructure |
| Governance Status | `Under Retention` | Complete active obligation when policy permits. | `Retention Completed` | Audit Infrastructure |
| Storage/Availability Status | Any governed value | No direct change in A025. | Unchanged pending A026/A027. | Audit Infrastructure |
| Retention Governance Decision | Does not exist | Record one policy-derived outcome. | Immutable decision. | Audit Infrastructure |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Events

| Event | Publisher | Condition | Effect |
|---|---|---|---|
| `Retention Completed` | Audit Infrastructure | Governance Status successfully changes to `Retention Completed`. | Authorized governance consumers learn active obligation completion. |

L2-A07 defines no exact event for continued-retention or archival-eligibility decisions.

## 14.2 Contracts

| Provider | Consumer | Hand-off |
|---|---|---|
| Platform & Tenant | Audit Infrastructure | Tenant/institution context. |
| Authorization & Policy | Audit Infrastructure | Retention/archival/legal-preservation policy decision. |
| Audit Infrastructure | A026 Evidence Archival | Archival-eligibility decision and evidence references. |
| Audit Infrastructure | Governance consumers | Retention Governance Decision and applicable final status. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

Retention decisions require applicable policy authority; A07 applies but does not evaluate policy.

## 15.2 Approval and Separation of Duty

No universal retention approver, override or SoD rule is defined. Legal preservation, where applicable, remains an A04 policy condition and cannot be bypassed.

## 15.3 Audit Obligations

Evidence scope, Tenant/institution, classification, current statuses, policy/version reference, preservation constraints, decision, authority reference, time and follow-on hand-off must remain traceable.

---

# 16. Completion Criteria and Postconditions

Completion requires authoritative context and policy evaluation, preservation-obligation consideration, one immutable continued-retention/archival-eligibility/retention-completion decision, and initiation only of its governed follow-on path.

Postconditions: evidence remains immutable; no archival occurs in A025; retention completion does not delete evidence; prior governance remains traceable.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Impact | Resolution |
|---|---|---|
| **A025-ISSUE-01:** L2-A07 defines `Retention Completed` but no canonical events for continued-retention or archival-eligibility decisions. | Those decisions are available through the synchronous Retention Governance Decision only. | Define exact decision events and publisher if downstream event notification is required. |
| **A025-ISSUE-02:** L2-A07 defines no universal retention reviewer/approver. | Exact human authority cannot inform later RBAC. | Map policy decision authority to canonical business roles where manual review is required. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
