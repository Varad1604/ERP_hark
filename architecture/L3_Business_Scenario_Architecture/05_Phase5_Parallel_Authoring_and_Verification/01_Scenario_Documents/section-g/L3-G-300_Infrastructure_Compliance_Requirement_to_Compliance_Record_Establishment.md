# Infrastructure Compliance Requirement to Compliance Record Establishment

# 1. Scenario Metadata

**Scenario ID:** L3-G-300  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-21  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Authorized business processes requiring the current compliance position of physical Infrastructure

# 2. Scenario Purpose

Establish a Compliance Record against authoritative physical Infrastructure when a statutory, regulatory, institutional, safety or other configured compliance requirement applies, while preserving G10 ownership of the physical asset and the Compliance Record and using Section A only for its canonical platform capabilities.

# 3. Business Outcome

A Compliance Record is established against the applicable infrastructure

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a physical infrastructure asset requires a compliance record.

## 4.2 End Boundary

The scenario ends when the applicable physical Infrastructure has passed the source-defined registration validations, a G10-owned Compliance Record has been established, and the completed registration fact has been published as `ComplianceRegistered`.

## 4.3 In Scope

- Identification of the authoritative physical Infrastructure requiring compliance governance.
- Selection or recognition of the applicable source-defined/configured compliance category and policy context.
- Authorization of Compliance Record registration.
- Infrastructure and policy validation before record establishment.
- Establishment of the G10-owned Compliance Record against the applicable Infrastructure.
- Publication of the canonical `ComplianceRegistered` event.
- Preservation of immutable audit evidence from the completed business fact.

## 4.4 Out of Scope

- Regulatory interpretation or creation of new regulatory obligations outside the configured G10 compliance governance.
- Compliance renewal, expiry, status change or failure handling after the Compliance Record has been established.
- Infrastructure inspection execution.
- Transfer of ownership of the physical Infrastructure or Compliance Record to Platform Foundation.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Followed by | L3-G-301 — Compliance Status Change to Infrastructure Compliance Update | Governs later changes to the current compliance position while retaining history. |
| Related | L3-G-302 — Compliance Failure to Controlled Infrastructure Restriction | Governs the configured restriction or review path when compliance becomes invalid or expires. |
| Related | L3-G-303 — Infrastructure Inspection Requirement to Closed Risk-Reviewed Outcome | Inspection may provide evidence relevant to Infrastructure condition or compliance but remains a distinct governed outcome. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns the physical Infrastructure, Compliance Record aggregate, compliance lifecycle, registration workflow, compliance policy application and completed compliance-registration business fact. |
| Platform Foundation / Section A | Owns authorization decisions, event-governance capability and immutable audit evidence derived from the completed business fact; it does not own Infrastructure compliance semantics. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Administers Infrastructure compliance information within authorized scope. | May initiate or complete governed Compliance Record registration where authorized. |
| Compliance Officer | Participates in Infrastructure compliance governance according to applicable institutional responsibility. | May provide or review compliance information within authorized scope; this scenario does not invent a universal approval authority. |
| Campus Administrator | Participates for Infrastructure within the governed campus scope. | May administer campus-scoped compliance information where authorized. |
| Auditor | Reviews preserved compliance history and evidence. | Read-only participation; cannot alter the Compliance Record. |

# 7. Preconditions and Entry Criteria

- The applicable physical Infrastructure exists under G10 ownership.
- A compliance requirement or configured compliance category applies to the Infrastructure.
- Applicable compliance policy information needed for registration is available.
- The compliance-registration business operation is authorized before execution.

# 8. Trigger

A physical infrastructure asset requires a compliance record

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the authoritative physical Infrastructure requiring a Compliance Record. | Infrastructure Management | The exact Infrastructure target is established. |
| 2 | Identify the applicable configured compliance category and policy context. | Infrastructure Management | The compliance requirement is placed within the G10 compliance governance. |
| 3 | Confirm that Compliance Record registration is authorized for the applicable Infrastructure scope. | Infrastructure Management using the canonical authorization decision owned by Section A | Registration is permitted to proceed. |
| 4 | Validate the Infrastructure reference and applicable Infrastructure rules. | Infrastructure Management | The Compliance Record cannot be established against an invalid or non-authoritative physical reference. |
| 5 | Validate the registration against the applicable configured compliance policy. | Infrastructure Management | The compliance requirement is eligible for registration under G10 policy. |
| 6 | Establish the Compliance Record against the applicable Infrastructure in the source-defined compliance lifecycle. | Infrastructure Management | The required Compliance Record exists under G10 ownership. |
| 7 | Publish `ComplianceRegistered` as the completed compliance-registration fact. | Infrastructure Management | The completed G10 business fact is available through the canonical event-governance capability. |
| 8 | Preserve immutable audit evidence from the canonical completed business fact. | Platform Foundation — Audit Infrastructure | Historical evidence is preserved without becoming the operational compliance record. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Institution-specific compliance category | The applicable requirement is not one of the example compliance categories named by G10 but is configured by the institution. | Use the configured G10 compliance category and policy without creating a new ownership model. | The Compliance Record is established within the same G10 compliance architecture. | Infrastructure Management |
| Registration cannot satisfy policy | Required compliance policy information is incomplete or the proposed record violates applicable policy. | Do not establish the Compliance Record; resolve the business information or policy condition before a new registration attempt. | No unsupported compliance fact is asserted. | Infrastructure Management |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Authorization denies registration | The Compliance Record is not established. | A later registration may proceed only after a valid authorization decision. | Platform Foundation / Authorization; Infrastructure Management retains compliance ownership. |
| Infrastructure validation fails | The record cannot be reliably associated with authoritative physical Infrastructure. | Resolve the authoritative Infrastructure issue before a later registration attempt. | Infrastructure Management |
| Compliance policy validation fails | The proposed compliance registration cannot be accepted. | Resolve the policy or business-information deficiency and resubmit through the governed registration path. | Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Infrastructure Management owns Compliance Status and Compliance Records for physical Infrastructure. | Infrastructure Management | The compliance record remains part of G10 and cannot be re-owned by Section A or a consuming domain. | L2-G10 §§1, 3, 8 |
| Compliance is a regulatory, statutory, institutional or safety requirement applicable to Infrastructure. | Infrastructure Management | Registration must represent an Infrastructure compliance requirement within the G10 meaning of Compliance. | L2-G10 §7 Domain Terminology |
| Infrastructure Compliance supports institution-specific compliance governance, multi-country regulatory compliance, version-controlled compliance, effective dating, expiry management, auditability, workflow-driven approvals and configurable compliance policies. | Infrastructure Management | The scenario uses configured compliance governance rather than inventing a fixed institutional rule set. | L2-G10 Compliance Principles |
| Compliance Registration follows Authorization Validation → Infrastructure Validation → Policy Validation → Compliance Created → Publish Domain Event. | Infrastructure Management / Platform Authorization for the authorization decision | The record is not established before the source-defined validations succeed. | L2-G10 Compliance Registration Workflow |
| The canonical Compliance lifecycle is Draft → Active → Expiring → Expired → Renewed. | Infrastructure Management | The newly established record enters and remains governed by the source-defined Compliance lifecycle; no new state is introduced. | L2-G10 Canonical Lifecycle States |
| Every business operation shall be authorized before execution. | Platform Foundation — Authorization | Unauthorized registration does not proceed. | L2-A04 AP-01 |
| Events represent completed business facts and business domains retain ownership of those facts. | Infrastructure Management / Platform Foundation — Event Infrastructure | `ComplianceRegistered` is published only after the Compliance Record is established and remains a G10 fact. | L2-A06 EIP-01, EIP-02 |
| Audit evidence is derived from canonical business events and remains distinct from operational business data. | Platform Foundation — Audit | Compliance business history can be evidenced without duplicating the Compliance Record into Audit Infrastructure. | L2-A07 §§7.2–7.4 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Compliance Record | Not yet established | A new Compliance Record is created through the canonical registration workflow and becomes governed by the canonical Compliance lifecycle. | The lifecycle begins at the source-defined `Draft` position before later permitted progression to `Active`, `Expiring`, `Expired` and `Renewed`. | Infrastructure Management | L2-G10 Compliance Registration Workflow; Canonical Lifecycle States |
| Infrastructure | Existing source-defined lifecycle state capable of carrying the applicable Compliance Record | The Infrastructure is referenced by the new compliance record; this scenario does not alter its lifecycle solely because a record is established. | Unchanged by this scenario unless a separate source-defined Infrastructure workflow applies. | Infrastructure Management | L2-G10 aggregate ownership and lifecycle rules |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `ComplianceRegistered` | Infrastructure Management | Authorized consumers of Infrastructure compliance facts; Audit Infrastructure consumes canonical business events for evidence generation. | Publishes the completed fact that a Compliance Record has been created. | L2-G10 Compliance Events; L2-A06 EIP-02; L2-A07 §7.2 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

Section A provides canonical authorization, event-governance and audit capabilities. The Compliance Record, its lifecycle and its compliance meaning remain owned by Infrastructure Management throughout the collaboration.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Register an Infrastructure Compliance Record | Infrastructure Administrator, Compliance Officer or Campus Administrator within applicable authorized scope | Registration must pass the canonical authorization, Infrastructure and policy validations. L2-G10 does not define one universal mandatory approver for every compliance category. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

G10 states that Infrastructure Compliance supports workflow-driven approvals and configurable compliance policies, but it does not define one universal approval or separation-of-duty pattern for every Compliance Record. Any category-specific approval requirement must therefore come from the applicable authoritative policy rather than this L3 scenario.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve the established compliance fact | The G10 Compliance Record and `ComplianceRegistered` business event. | Infrastructure Management |
| Preserve immutable evidence of registration | Audit evidence generated from the canonical completed business event with available actor and context information. | Platform Foundation — Audit Infrastructure |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The authoritative physical Infrastructure has been identified.
- The applicable compliance requirement is within the configured G10 compliance governance.
- Authorization, Infrastructure validation and policy validation have succeeded.
- A Compliance Record has been established against the applicable Infrastructure.
- `ComplianceRegistered` has been published.
- Immutable audit evidence is available through the canonical audit architecture.

## 16.2 Postconditions

- Infrastructure Management owns the established Compliance Record and its subsequent lifecycle.
- Authorized consumers may rely on the existence of the current G10 compliance record.
- Later status changes, expiry, renewal or compliance failure remain separate governed outcomes.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No blocking architecture issue was identified. The locked G10 registration workflow, aggregate ownership, compliance lifecycle and canonical registration event are sufficient to express the frozen scenario outcome without inventing new architecture.

**Architecture assessment:** `PASS`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
