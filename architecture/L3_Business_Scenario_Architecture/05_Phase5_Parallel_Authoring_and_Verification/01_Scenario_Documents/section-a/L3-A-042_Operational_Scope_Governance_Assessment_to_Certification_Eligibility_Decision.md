# Operational Scope Governance Assessment to Certification Eligibility Decision

# 1. Scenario Metadata

**Scenario ID:** L3-A-042  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Operational Readiness Governance & Certification  
**Participating Domains:** A, VAR:Assessed Domain  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** undefined  
**Referenced By:** Certification issuance, remediation governance, reassessment and operational governance reporting  

---

# 2. Scenario Purpose

Create or continue a policy-governed assessment for one operational scope and determine certification eligibility without producing a runtime readiness flag.

---

# 3. Business Outcome

A new or successor assessment is initialized and linked to its predecessor where applicable; the governed operational scope is then assessed through policy-driven requirement evaluation, evidence verification and governance consolidation, resulting in a recorded certification eligibility decision without producing a runtime `READY` or `NOT READY` result, while prior assessments remain immutable and traceable

Source assessment: `PASS`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

Institution go-live, campus go-live, operational capability activation, scheduled review, regulatory review, accreditation review, major change, manual governance request, or a policy-determined reassessment trigger requires governance assessment.

## 4.2 End Boundary

Certification eligibility is deterministically recorded for one governed scope after requirement and scope consolidation; a certificate has not yet been issued and no runtime readiness result exists.

## 4.3 In Scope

- New or successor assessment initialization.
- Scope, policy, initiation source and predecessor lineage.
- Requirement evaluation and evidence verification.
- Domain and operational-scope consolidation.
- Certification eligibility decision and events.

## 4.4 Out of Scope

- Dynamic operation-specific Capability Readiness Evaluation and runtime `READY`/`NOT READY` outcomes.
- Operational remediation work owned by collaborating business domains.
- Authentication, authorization, event transport or audit-storage implementation.
- Technical APIs, databases, UI, deployment and infrastructure design.

## 4.5 Related Scenarios

| Relationship | Scenario | Boundary |
|---|---|---|

| Followed by | L3-A-043 | Obtains missing evidence. |
| Followed by | L3-A-044 | Governs failed requirements. |
| Followed by | L3-A-046 | Issues a certificate after approved eligibility. |
| Related | L3-A-048 | May initiate successor reassessment. |

---

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|

| Operational Readiness Governance & Certification / Section A | Owns assessments, requirements, verification outcomes, findings, improvement plans, governance decisions and certificates. |
| Platform & Tenant / Section A | Supplies Tenant and operational-scope context. |
| Authorization & Policy / Section A | Supplies authority and policy context without owning readiness artifacts. |
| Workflow Engine / Section A | Coordinates workflow execution without owning governance decisions. |
| Event Infrastructure / Section A | Distributes canonical readiness events. |
| Audit Infrastructure / Section A | Preserves immutable governance evidence. |
| VAR:Assessed Domain | Owns operational facts and evidence for the assessed scope. |

The frozen Participating Domains value remains exactly `A, VAR:Assessed Domain`.

---

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Boundary |
|---|---|---|

| Governance Authority | Authorizes assessments, verifies evidence, acknowledges findings and governs certificates. | Owns governance decisions, not operational remediation. |
| Responsible Domain | Owns operational evidence and remediation execution and reports completion. | Does not modify readiness governance artifacts directly. |
| System | Applies approved policy, monitors significant events and initiates governed reassessment triggers. | Does not create unsupported business decisions. |

---

# 7. Preconditions and Entry Criteria

- Exactly one operational scope exists.
- A governing readiness policy is available.
- The Governance Authority is authorized.
- Required collaborating domains can supply their owned evidence.
- For reassessment, a predecessor Assessment and valid trigger exist.

---

# 8. Trigger

Institution go-live, campus go-live, operational capability activation, scheduled review, regulatory review, accreditation review, major change, manual governance request, or a policy-determined reassessment trigger requires governance assessment

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|

| 1 | Receive and classify the assessment trigger. | Governance Authority; System | The initiation source is fixed. |
| 2 | Resolve Tenant, governed operational scope and readiness policy. | Assessment Management Service | The assessment boundary is established. |
| 3 | Create the Assessment in `Draft`; for reassessment, link the immutable predecessor. | Operational Readiness Assessment aggregate | A new governance record exists. |
| 4 | Initialize Readiness Requirements and record initiation metadata. | Assessment Management Service | Policy requirements belong to the Assessment. |
| 5 | Authorize initiation and transition `Draft → Initiated → Assessing`. | Governance Authority; Assessment aggregate | Evaluation begins. |
| 6 | Evaluate every mandatory requirement independently under policy. | Readiness Evaluation Service | Requirement outcomes are recorded. |
| 7 | Request and verify evidence where needed without transferring ownership. | Evidence Verification Service; Responsible Domain | Evidence-based requirement outcomes become available. |
| 8 | Create Findings for policy-defined failures. | Improvement Governance Service | Deficiencies enter governance. |
| 9 | Consolidate domain readiness and then operational-scope readiness. | Readiness Evaluation Service | A policy-derived scope result exists. |
| 10 | Evaluate unresolved findings and mandatory governance requirements. | Certification Service | Eligibility conditions are assessed. |
| 11 | Record the certification eligibility decision and publish `CertificationEligibilityDetermined`. | Operational Readiness Assessment aggregate | The frozen governance outcome completes. |

---

# 10. Alternate Business Flows

| Case | Condition | Path | Outcome |
|---|---|---|---|

| New assessment | No predecessor applies. | Create a new Assessment and record the independent initiation source. | Normal lifecycle begins. |
| Successor reassessment | A prior Assessment exists and reassessment policy is satisfied. | Create and link a successor; preserve the predecessor unchanged. | Governance continuity is established. |
| Additional evidence required | A requirement cannot complete. | Invoke A043 and pause affected evaluation in Awaiting Evidence. | Eligibility waits for governed evidence outcome. |
| Requirement fails | Policy produces a deficiency. | Invoke A044 finding/improvement governance. | Eligibility reflects unresolved findings. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Governed Resolution | Owner |
|---|---|---|---|

| Scope or policy unresolved | Assessment cannot be governed deterministically. | Reject or defer initiation. | Assessment Management Service |
| Unauthorized authority | Governance commencement is invalid. | Do not initiate. | Authorization & Policy |
| Mandatory domain omitted | Scope consolidation is invalid. | Block consolidation. | Readiness Evaluation Service |
| Evidence inaccessible | Requirement cannot complete. | Request additional evidence under A043. | Evidence Verification Service |
| Historical assessment modification attempted | Immutability would be violated. | Reject and create a successor if policy permits. | Assessment aggregate |

No technical retry, rollback, persistence or deployment mechanism is defined.

---

# 12. Business Rules and Decision Constraints

| Rule / Constraint | Effect | Source |
|---|---|---|

| One Assessment governs exactly one operational scope. | Scope mixing is prohibited. | L2-A10 §§9.3, 36.1 |
| Every requirement belongs to one Assessment and is policy-driven. | Mandatory requirements cannot be skipped. | L2-A10 §§17, 36.2 |
| Scope consolidation follows completed mandatory-domain evaluation. | Eligibility cannot precede consolidation. | L2-A10 §§24.3–24.4 |
| Reassessment creates a successor. | Prior assessments remain immutable. | L2-A10 §§28.1, 36.7 |
| Governance certification is not runtime readiness. | No `READY`/`NOT READY` flag is produced. | L2-A10 §1.4 |

---

# 13. State and Lifecycle Impact

| Concept | Entry | Impact | Result |
|---|---|---|---|

| Assessment | Absent or predecessor available | Create `Draft → Initiated → Assessing`; may use Awaiting Evidence/Verifying; record eligibility. | Active assessment with eligibility decision. |
| Requirement | Defined | Pending Evaluation → Awaiting Evidence/Under Verification → Passed/Failed/Waived. | Terminal policy outcome. |
| Prior Assessment | Existing where reassessment | Reference only; successor linkage added. | Immutable. |
| Certificate | Absent | No issuance in A042. | Unchanged. |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Business Events

| Event | Canonical Publisher | Condition |
|---|---|---|

| `OperationalReadinessAssessmentCreated` | Operational Readiness Assessment aggregate | New assessment is created. |
| `OperationalReadinessAssessmentInitiated` | Operational Readiness Assessment aggregate | Assessment reaches Initiated. |
| `ReadinessRequirementEvaluated` | Operational Readiness Assessment aggregate | A requirement evaluation completes. |
| `DomainReadinessCalculated` | Operational Readiness Assessment aggregate | Domain consolidation completes. |
| `OperationalScopeReadinessCalculated` | Operational Readiness Assessment aggregate | Scope consolidation completes. |
| `CertificationEligibilityDetermined` | Operational Readiness Assessment aggregate | Eligibility decision is recorded. |
| `OperationalReadinessReassessmentCreated` | Operational Readiness Assessment aggregate | A successor assessment is created where applicable. |

## 14.2 Synchronous Contracts and Hand-offs

| Contract | Provider | Consumer | Information / Outcome |
|---|---|---|---|

| Operational scope facts/evidence | VAR:Assessed Domain | Assessment aggregate | Owned evidence references and domain readiness inputs. |
| Authorization decision | Authorization & Policy | Assessment aggregate | Governance actor authority. |
| Governing policy | Readiness policy repository | Readiness services | Requirements and deterministic rules. |
| Event distribution | Event Infrastructure | Governance consumers | Completed readiness facts. |
| Audit input | Assessment aggregate | Audit Infrastructure | Assessment and decision history. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Assessment, verification, finding, certification and lifecycle actions require authorized governance actors.
- Authorization decisions remain owned by L2-A04.
- A certificate is governance evidence; it does not authorize every operation.

## 15.2 Approval and Separation of Duty

- Governance Authority provides the approvals explicitly required by L2-A10.
- Responsible Domains may submit evidence and remediation completion but do not approve their own governance outcome.
- Policy-approved waivers and certificate transitions must remain traceable.

## 15.3 Audit Obligations

- Record Tenant, governed scope, actor, policy, command, prior/resulting state, decision, timestamps and correlation.
- Preserve assessment lineage, evidence references, findings, plans, certificate lineage and every governance outcome.
- A10 owns readiness facts; A07 owns immutable audit evidence.

---

# 16. Completion Criteria and Postconditions

Completion requires:

- One governed scope, policy and initiation source were recorded.
- A new or linked successor Assessment was initialized.
- Mandatory requirements were evaluated with evidence governance.
- Domain and scope readiness were consolidated.
- Certification eligibility was deterministically recorded.
- Prior assessments remained immutable.
- Canonical events and audit evidence are available.
- No runtime readiness or certificate issuance was produced.

Postconditions:

- A046 may issue a certificate only after approved eligibility.
- Unresolved Findings remain governed through A044/A045.
- Business evidence remains owned by the assessed domains.

---

# 17. Open Issues, Assumptions and Deferred Questions


No open issue prevents this scenario from proceeding to Architecture Approval. No unsupported actor, state, transition, event, publisher or implementation mechanism is assumed.


