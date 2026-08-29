# Active Readiness Certificate Suspension Condition to Controlled Suspension and Restoration

# 1. Scenario Metadata

**Scenario ID:** L3-A-047  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Operational Readiness Governance & Certification  
**Participating Domains:** A  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** undefined  
**Referenced By:** Governance consumers, reassessment, certificate supersession and revocation  

---

# 2. Scenario Purpose

Govern the intended temporary suspension and restoration of an Active Certificate while preserving authority, evidence and complete lifecycle history.

---

# 3. Business Outcome

The certificate is suspended with authority and evidence preserved and, when the suspension condition is governed as resolved, returns to Active without rewriting its prior lifecycle history

Source assessment: `BLOCKED`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A policy, regulatory or governance condition requires an active readiness certificate to be temporarily suspended.

## 4.2 End Boundary

The intended end is `Active → Suspended → Active`, but locked L2-A10 defines no resume/restore command or canonical restoration event, so exact completion cannot be specified.

## 4.3 In Scope

- Suspension-condition assessment and approval.
- Certificate `Active → Suspended`.
- Authority, evidence and history preservation.
- Resolution assessment.
- Intended return to Active, pending upstream contract.

## 4.4 Out of Scope

- Dynamic operation-specific Capability Readiness Evaluation and runtime `READY`/`NOT READY` outcomes.
- Operational remediation work owned by collaborating business domains.
- Authentication, authorization, event transport or audit-storage implementation.
- Technical APIs, databases, UI, deployment and infrastructure design.

## 4.5 Related Scenarios

| Relationship | Scenario | Boundary |
|---|---|---|

| Upstream | L3-A-046 | Supplies an issued/active Certificate. |
| Related | L3-A-061 | Owns permanent supersession. |
| Related | L3-A-062 | Owns permanent revocation. |

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

The frozen Participating Domains value remains exactly `A`.

---

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Boundary |
|---|---|---|

| Governance Authority | Authorizes assessments, verifies evidence, acknowledges findings and governs certificates. | Owns governance decisions, not operational remediation. |
| Responsible Domain | Owns operational evidence and remediation execution and reports completion. | Does not modify readiness governance artifacts directly. |
| System | Applies approved policy, monitors significant events and initiates governed reassessment triggers. | Does not create unsupported business decisions. |

---

# 7. Preconditions and Entry Criteria

- The Certificate exists and is Active.
- A policy, regulatory or governance condition is evidenced.
- Governance approval for suspension exists.
- The requested transition is permitted.
- A resolution condition can later be evaluated.

---

# 8. Trigger

A policy, regulatory or governance condition requires an active readiness certificate to be temporarily suspended

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|

| 1 | Receive and classify the suspension condition. | Governance Authority; System | A certificate lifecycle assessment begins. |
| 2 | Resolve the Active Certificate, Assessment, scope and current policy. | Certification Service | The exact artifact is fixed. |
| 3 | Validate evidence, authority and permitted transition. | Governance Authority | Suspension eligibility is established. |
| 4 | Execute `Active → Suspended`. | Assessment aggregate | The Certificate becomes temporarily invalid. |
| 5 | Record the governance decision and preserve lifecycle history. | Certification Service | Authority and evidence are traceable. |
| 6 | Publish `OperationalReadinessCertificateSuspended`. | Assessment aggregate | Consumers receive the suspension fact. |
| 7 | Receive evidence that the suspension condition is resolved. | Governance Authority | Restoration can be assessed. |
| 8 | Validate current policy and intended `Suspended → Active` transition. | Certification Service | The lifecycle model permits restoration. |
| 9 | Do not invent a resume command or restoration event. | Operational Readiness | Exact restoration execution remains blocked. |
| 10 | Retain the Certificate in its governed current state until L2-A10 defines the missing contract. | Operational Readiness | No false completion is asserted. |

---

# 10. Alternate Business Flows

| Case | Condition | Path | Outcome |
|---|---|---|---|

| Condition unresolved | Evidence remains insufficient. | Keep Certificate Suspended. | No restoration occurs. |
| Permanent invalidity | Governance determines the certificate cannot return. | Use A062 revocation. | Suspension does not substitute for revocation. |
| Successor certification | A replacement certificate is issued. | Use A061 supersession. | Prior certificate becomes terminal Superseded. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Governed Resolution | Owner |
|---|---|---|---|

| Certificate not Active | Suspension precondition fails. | Reject request. | Certification Service |
| Approval/evidence absent | Transition is unauthorized. | Do not suspend. | Governance Authority |
| Restoration contract invoked | No canonical command/event exists. | Stop and obtain upstream definition. | Operational Readiness |
| History modification attempted | Immutability is violated. | Reject and append a governed decision. | Assessment aggregate |

No technical retry, rollback, persistence or deployment mechanism is defined.

---

# 12. Business Rules and Decision Constraints

| Rule / Constraint | Effect | Source |
|---|---|---|

| Suspension is policy-driven and approved. | Only a permitted lifecycle transition executes. | L2-A10 §27.2 |
| Suspended is non-terminal. | The lifecycle model permits return to Active. | L2-A10 §20.3 |
| Certificate history is immutable. | Suspension/restoration cannot rewrite prior states. | L2-A10 §§20.4, 27.2 |
| Revocation and supersession remain separate terminal outcomes. | Temporary suspension cannot replace them. | L2-A10 §20 |

---

# 13. State and Lifecycle Impact

| Concept | Entry | Impact | Result |
|---|---|---|---|

| Certificate | Active | Defined transition `Active → Suspended`; intended `Suspended → Active` lacks command/event. | Suspended; restoration completion blocked. |
| Assessment | Historical/associated | Referenced only. | Unchanged. |
| Suspension evidence | Available | Record authority, condition and resolution evidence. | Immutable governance history. |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Business Events

| Event | Canonical Publisher | Condition |
|---|---|---|

| `OperationalReadinessCertificateSuspended` | Operational Readiness Assessment aggregate | `Active → Suspended` completes. |

## 14.2 Synchronous Contracts and Hand-offs

| Contract | Provider | Consumer | Information / Outcome |
|---|---|---|---|

| Suspension directive/evidence | Governance Authority | Certification Service | Condition and authority. |
| Certificate lifecycle request | Certification Service | Assessment aggregate | Permitted suspension transition. |
| Resolution evidence | Governance Authority | Certification Service | Basis for intended restoration. |
| Audit input | Assessment aggregate | Audit Infrastructure | Suspension and attempted resolution facts. |

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

- The suspension condition and Active Certificate were validated.
- Governance approved suspension.
- The Certificate reached Suspended and the event was published.
- Authority, evidence and history were preserved.
- Resolution evidence can be assessed.
- Exact restoration cannot complete until the missing upstream command/event is defined.

Postconditions:

- No Certificate history is rewritten.
- The Certificate remains Suspended in this blocked Draft.
- Revocation/supersession are not inferred.

---

# 17. Open Issues, Assumptions and Deferred Questions


| ID | Severity | Issue | Treatment / Required Resolution |
|---|---|---|---|

| A047-BLOCKER-01 | Blocking | L2-A10 permits `Suspended → Active` in lifecycle prose but defines no resume/restore Certificate command. | Define the canonical restoration command and owning aggregate behavior. |
| A047-BLOCKER-02 | Blocking | The Certificate event catalogue defines Suspended, Superseded, Revoked and Archived but no Activated/Restored event. | Define the exact restoration event, publication condition and canonical publisher before Architecture Approval. |

No missing contract is supplied by inference.


