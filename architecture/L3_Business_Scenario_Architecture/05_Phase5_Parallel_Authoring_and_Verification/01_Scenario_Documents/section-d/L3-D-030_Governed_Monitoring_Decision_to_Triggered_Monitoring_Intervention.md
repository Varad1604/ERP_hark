# Governed Monitoring Decision to Triggered Monitoring Intervention

# 1. Scenario Metadata

**Scenario ID:** L3-D-030  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Monitoring  
**Participating Domains:** C, D  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Review  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** `L2-D05_Academic_Monitoring_Workflows.md`  
**Governance Basis:** `L2-D05_Business_Architecture_Purity_Amendment_2026-08-24.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0751 / CAN-L3-D-031 (final L3-D-030)

---

# 2. Scenario Purpose

Convert an approved monitoring governance decision requiring corrective action into a governed Monitoring Intervention with explicit ownership and traceable source evidence.

# 3. Business Outcome

A Monitoring Intervention is created, assigned to an Intervention Owner and formally initiated from the approved governance decision without modifying originating monitoring or source-domain facts.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An approved D05 governance decision requires a monitoring intervention.

## 4.2 End Boundary

The intervention reaches Assigned or In Progress and `MonitoringInterventionInitiated` is published with decision/evidence traceability.

## 4.3 In Scope

- Validate decision approval, action requirement and evidence.
- Establish intervention and link it to the decision/case/risk/exception.
- Assign Intervention Owner and initiate governed corrective work.

## 4.4 Out of Scope

- Automatic modification of Section C or D01–D04 records.
- Treating an unapproved decision as authority.
- Completion/validation beyond the intervention-initiation boundary.
- Application-service, queue, notification-transport or retry design.

## 4.5 Related Scenarios — Optional

The intervention may later continue through D05 monitoring and closure scenarios without ownership transfer.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D05 | Owns governance-decision linkage, Monitoring Intervention, assignment and progress lifecycle. |
| Section C and D01–D04 | Retain ownership of source records referenced by the intervention. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Governance Authority | Approves the decision and authorizes the intervention. | Decision authority. |
| Monitoring Authority | Validates monitoring context and tracks the triggered action. | Monitoring oversight. |
| Intervention Owner | Accepts assignment and performs corrective action. | Manages assigned intervention only. |

Intervention creation/initiation is a D05 business responsibility; `D05 Application Service` is not an L3 business actor.

# 7. Preconditions and Entry Criteria

- Governance Decision exists and is Approved.
- Decision explicitly requires corrective action.
- Evidence, policy version, authority and proposed owner are valid.

# 8. Trigger

`GovernanceDecisionApproved` records a decision whose outcome requires intervention.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish the approved governance decision and linked monitoring context. | D05 | Authoritative trigger basis is established. |
| 2 | Validate approval, policy, evidence, action requirement and absence of an already-governed duplicate institutional intervention outcome. | D05 | Intervention is authorized or blocked. |
| 3 | Establish the Monitoring Intervention linked to the decision and case. | D05 | Intervention reaches Created. |
| 4 | Assign the accountable Intervention Owner. | Governance Authority / D05 | Created → Assigned. |
| 5 | Intervention Owner accepts and begins the approved corrective action. | Intervention Owner / D05 | Assigned → In Progress; `MonitoringInterventionInitiated`. |
| 6 | Make intervention progress available to authorized monitoring/governance consumers. | D05 | Tracking begins without source-domain mutation. |

# 10. Alternate Business Flows

No distinct alternate path reaches the initiated intervention outcome. Missing approval/evidence or duplicate outcome conditions block initiation and are handled under Section 11.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Owner |
|---|---|---|---|
| Decision approval/evidence cannot be validated | Intervention cannot validly be initiated. | Resolve the authoritative D05 decision/evidence condition before continuation. | D05 |
| Intervention Owner cannot be validly assigned | Intervention cannot progress to Assigned/In Progress. | Obtain a valid governed owner assignment. | D05 |
| Intervention initiation is interrupted | Initiated outcome has not occurred. | Preserve last authoritative business position/evidence/context and continue when blocking condition is resolved. | D05 |

# 12. Business Rules and Decision Constraints

- D05 owns Monitoring Intervention. Source: L2-D05 §8.10.
- Only an approved governance decision may authorize this intervention path. Source: L2-D05 §§13.5–13.7.
- Same business fact must not create duplicate institutional intervention outcomes; technical idempotency/deduplication is deferred. Source: locked D05 Purity Amendment.
- Source-domain facts remain owned by their canonical owners.

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Governance Decision | Evaluated | Required authority approval. | Approved | D05 | L2-D05 §§13.5–13.7 |
| Monitoring Intervention | No intervention | Establish, assign and initiate. | Created → Assigned → In Progress | D05 | L2-D05 §8.10 |

# 14. Business Events and Cross-Domain Collaboration

`GovernanceDecisionApproved` and `MonitoringInterventionInitiated` remain D05-published business events under L2-D05 §§13.7 and 6.2/8.10.

Cross-domain source records are referenced only; no participant gains mutation authority over another domain.

# 15. Business Authorization, Approval and Audit Obligations

Governance Authority approves the triggering decision, Monitoring Authority oversees monitoring context, and Intervention Owner performs assigned corrective action. Automated execution has no governance discretion.

Audit evidence includes decision, authority, policy/evidence references, intervention identity, owner assignment, initiation fact and source references.

# 16. Completion Criteria and Postconditions

- Approved decision is traceable.
- Intervention is Assigned/In Progress with accountable owner.
- `MonitoringInterventionInitiated` is published by D05.
- Source-domain ownership remains unchanged.

# 17. Open Issues, Assumptions and Deferred Questions

No new open issue or assumption is introduced. Application-service design, queues, retries, notification transport, persistence and technical deduplication are deferred to L4/L5.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** L2-D05; locked D05 Business Architecture Purity Amendment; Template v2.0; Architecture Level Validation Standards.
