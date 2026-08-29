# Accreditation Evidence Request to Approved Evidence Record

## 1. Scenario Metadata

**Scenario ID:** L3-H-064  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accreditation & Compliance  
**Participating Domains:** H, VAR:Evidence-Owning Domain  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP_L1_Business_Domain_Architecture_Locked.md; DEPENDENCY_MAP.md; L2-H09_Accreditation_and_Compliance_Workflows.md  
**Referenced By:** None yet

## 2. Scenario Purpose

Govern source-domain evidence for accreditation use while preserving source ownership, traceability, validation, approval/version history and H09 evidence governance.

## 3. Business Outcome

A source-owned reference is collected, validated, approved or rejected, versioned, and retained without copying canonical business ownership into H09

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An active criterion or compliance requirement needs source-domain evidence.

### 4.2 End Boundary

A source-owned reference is collected, validated, approved or rejected, versioned, and retained without copying canonical business ownership into H09.

### 4.3 In Scope

- H09-owned governance behaviour materially required to reach the frozen business outcome.
- Applicable source-domain references, evidence and governed hand-offs without ownership transfer.
- Only upstream-defined rules, lifecycle positions, events, approvals and evidence obligations relevant to this scenario.

### 4.4 Out of Scope

- Operational business-state ownership belonging to participating source domains.
- Adjacent downstream institutional outcomes after this scenario's End Boundary.
- Later architecture-level implementation concerns.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accreditation & Compliance (H) | Owns the H09 governance outcome and applicable H09 lifecycles. |
| VAR:Evidence-Owning Domain | Retains ownership of authoritative source evidence/business facts while supplying governed references or readiness evidence. |

Participation and evidence contribution do not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| IQAC | Reviews evidence sufficiency for accreditation governance. | Reviews |
| Accreditation Coordinator | Coordinates evidence collection and mapping. | Performs |
| Department Coordinator | Provides and updates departmental evidence. | Performs |
| Evidence Reviewer | Reviews evidence metadata, completeness and validity. | Reviews |
| Compliance Officer | Participates in compliance evidence governance. | Participates |

## 7. Preconditions and Entry Criteria

- An active criterion or Compliance Requirement requires evidence.
- The source evidence remains owned by `VAR:Evidence-Owning Domain`.

## 8. Trigger

An active criterion or compliance requirement needs source-domain evidence

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Request and identify the authoritative source reference. | Accreditation & Compliance / VAR:Evidence-Owning Domain | The source-owned evidence reference is identified. |
| 2 | Link the source record or approved institutional document and record H09 evidence metadata. | Accreditation & Compliance | Traceable Evidence is Created/Uploaded without duplicating source facts. |
| 3 | Validate ownership, source traceability, completeness, authenticity and version. | Evidence Reviewer / IQAC | Evidence reaches the governed validation/review position. |
| 4 | Approve evidence when validation succeeds. | Accreditation & Compliance | `Evidence Approved` is established. |
| 5 | Preserve version history; subsequent modification after approval creates a new version. | Accreditation & Compliance | The approved evidence remains read-only and historical versions are preserved. |

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Evidence is rejected | Validation/review does not support approval | Publish the canonical `Evidence Rejected` business event and return the evidence for governed correction or successor evidence. | No unsupported `Rejected` lifecycle state is asserted. | Accreditation & Compliance |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Source traceability or completeness is insufficient | Evidence cannot be approved for accreditation use. | Request corrections and reassess the updated evidence. | Accreditation & Compliance |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| BR-H09-009 — every Evidence Repository Item references its originating record or approved institutional document. | Accreditation & Compliance | Requires traceability. | L2-H09_Accreditation_and_Compliance_Workflows.md §8 |
| BR-H09-012 — approved evidence becomes read-only. | Accreditation & Compliance | Protects approved evidence. | L2-H09_Accreditation_and_Compliance_Workflows.md §8 |
| BR-H09-013 — modifications after approval require version creation. | Accreditation & Compliance | Preserves approved history. | L2-H09_Accreditation_and_Compliance_Workflows.md §8 |
| BR-H09-036/037 — H09 does not duplicate source transactional data; it stores governed references/metadata/approved snapshots and supporting documents. | Accreditation & Compliance | Preserves source-domain ownership. | L2-H09_Accreditation_and_Compliance_Workflows.md §8 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Evidence Repository Item | Created / Uploaded | Validation, review and approval. | Approved; may subsequently be Published, Locked and Archived | Accreditation & Compliance | L2-H09_Accreditation_and_Compliance_Workflows.md §6 H09-A04 / §7 |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| Evidence Linked | Evidence Repository aggregate | Readiness Assessment; Compliance Review; Audit | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Evidence Uploaded | Evidence Repository aggregate | Readiness Assessment; Compliance Review; Audit | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Evidence Updated | Evidence Repository aggregate | Readiness Assessment; Compliance Review; Audit | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Evidence Validation Requested | Evidence Repository aggregate | Readiness Assessment; Compliance Review; Audit | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Evidence Approved | Evidence Repository aggregate | Readiness Assessment; Compliance Review; Audit | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Evidence Rejected | Evidence Repository aggregate | Readiness Assessment; Compliance Review; Audit | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| VAR:Evidence-Owning Domain | Accreditation & Compliance | Source Evidence Reference Contract | Provide governed source reference and evidence context while retaining source lifecycle ownership. | L2-H09_Accreditation_and_Compliance_Workflows.md §23.1 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Evidence review / approval | Evidence Reviewer / IQAC | Only governed H09 evidence review may establish accreditation approval. | Accreditation & Compliance |

### 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty rule is defined for evidence approval.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Evidence traceability | Source reference, governed metadata and validation context | Accreditation & Compliance |
| Approved evidence history | Approval and preserved version history | Accreditation & Compliance |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- A traceable source-owned evidence reference is governed in H09.
- The evidence is validated and receives an approval or rejection decision.
- Approved evidence is version-preserved without duplicating source-domain ownership.

### 16.2 Postconditions

- Approved evidence is read-only; later modification requires a new version.
- `VAR:Evidence-Owning Domain` remains authoritative for the source business fact.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| H09 defines the exact `Evidence Rejected` event but does not define a `Rejected` state/transition in the Evidence Repository Item lifecycle. | The rejection decision is authoritative as an event, but L3 cannot assign a resulting rejection lifecycle state. | Upstream H09 should reconcile the rejection event with the evidence lifecycle vocabulary before locking. |

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
