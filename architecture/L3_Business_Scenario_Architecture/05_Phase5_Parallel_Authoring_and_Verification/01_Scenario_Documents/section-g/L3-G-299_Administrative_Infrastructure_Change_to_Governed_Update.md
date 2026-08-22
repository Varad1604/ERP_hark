# Administrative Infrastructure Change to Governed Update

# 1. Scenario Metadata

**Scenario ID:** L3-G-299  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-21  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Authorized consumers of current Infrastructure information and historical Infrastructure change evidence

# 2. Scenario Purpose

Govern an authorized administrative change to Infrastructure information so that the applicable authoritative Infrastructure record is updated under G10 rules, canonical identity and ownership remain intact, and the completed change remains traceable through the source-defined business event and audit architecture.

# 3. Business Outcome

The approved infrastructure change is applied with authorization and audit history preserved

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorised administrative process requires infrastructure information to be changed.

## 4.2 End Boundary

The scenario ends when the approved administrative Infrastructure change has been validated and applied to the authoritative G10 record, the completed update fact has been published using the applicable canonical event, and historical accountability is preserved.

## 4.3 In Scope

- Identification of the authoritative Infrastructure record and requested administrative change.
- Authorization of the Infrastructure update business operation.
- Validation of the requested change against G10 identity, hierarchy, lifecycle and applicable policy rules.
- Application of the approved Infrastructure information change without changing canonical ownership.
- Publication of the canonical `InfrastructureUpdated` event for the generic Infrastructure update outcome.
- Preservation of business history and immutable audit evidence from the completed canonical event.

## 4.4 Out of Scope

- Creation of new Infrastructure.
- Decommissioning, archival or other dedicated Infrastructure lifecycle outcomes.
- Silent modification of immutable Infrastructure identity.
- Transfer of physical Infrastructure ownership to another domain.
- Redefinition of specialized classification, capacity, accessibility, sustainability or other governed outcomes where a dedicated G10 workflow and event applies instead of the generic update path.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-G-298 — Infrastructure Validation to Physical Consistency Confirmation | A governed validation may identify a change that subsequently requires this administrative update outcome. |
| Related | L3-G-286 — Space Classification to Controlled Physical Classification | Classification-specific changes retain their dedicated G10 governance. |
| Related | L3-G-288 — Infrastructure Capacity Definition to Capacity Profile Activation | Capacity-specific changes retain their dedicated G10 governance. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns the authoritative Infrastructure record, the administrative update, validation of the requested change, and the completed business fact. |
| Platform Foundation / Section A | Owns authorization decisions, event-governance capability and immutable audit evidence generated from the completed G10 business fact; it does not own or modify Infrastructure. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Administers authoritative Infrastructure information within authorized scope. | May perform a governed Infrastructure update when authorization and G10 validation permit it. |
| Campus Administrator | Administers Infrastructure information within the governed campus scope. | May perform campus-scoped administrative updates when authorized. |
| Auditor | Reviews preserved business evidence and historical change traceability. | Read-only review; cannot alter the Infrastructure update outcome. |

# 7. Preconditions and Entry Criteria

- The Infrastructure record to be changed exists under Infrastructure Management ownership.
- The requested change identifies the authoritative Infrastructure record and the information proposed for change.
- The requested operation is authorized before execution.
- Applicable G10 business rules, hierarchy constraints and policy requirements are available for validation.

# 8. Trigger

An authorised administrative process requires infrastructure information to be changed

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the authoritative Infrastructure record and the administrative information requested for change. | Infrastructure Management | The exact update scope is established. |
| 2 | Confirm that the requested update is authorized for the applicable Infrastructure scope. | Infrastructure Management using the canonical authorization decision owned by Section A | The update is permitted to proceed. |
| 3 | Validate the proposed change against G10 identity, hierarchy, lifecycle, configuration and other applicable Infrastructure rules. | Infrastructure Management | The proposed change is either eligible for the generic update path or identified as requiring a different governed G10 outcome. |
| 4 | Apply the approved administrative change while retaining immutable Infrastructure identity and G10 canonical ownership. | Infrastructure Management | The authoritative Infrastructure information reflects the approved business change. |
| 5 | Publish `InfrastructureUpdated` as the completed generic Infrastructure update fact. | Infrastructure Management | The completed business fact becomes available through the canonical event-governance capability. |
| 6 | Preserve immutable audit evidence from the canonical completed business fact. | Platform Foundation — Audit Infrastructure | Historical accountability for the approved update is preserved without taking ownership of the Infrastructure record. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Requested change belongs to a dedicated G10 workflow | The change is specifically governed as classification, capacity, accessibility, sustainability, lifecycle or another dedicated Infrastructure outcome. | Use the applicable dedicated G10 workflow and canonical event rather than forcing the change through the generic administrative-update path. | Specialized governance and event semantics remain authoritative. | Infrastructure Management |
| Requested information is not permitted to change | The request attempts to alter immutable aggregate identity or violates a G10 invariant. | Reject the requested administrative change and preserve the existing authoritative Infrastructure fact. | No invalid Infrastructure mutation occurs. | Infrastructure Management |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Authorization denies the update | No Infrastructure information is changed. | The request ends without mutation unless a later separately authorized request is made. | Platform Foundation / Authorization; Infrastructure Management retains the source fact. |
| G10 validation fails | The approved outcome cannot be established for the requested values. | Correct the business request or resolve the authoritative Infrastructure inconsistency before a new governed update is attempted. | Infrastructure Management |
| A specialized governed workflow is required | The generic administrative-update path is not authoritative for the requested change. | Route the business need to the applicable G10 specialized outcome without duplicating or bypassing its rules. | Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Infrastructure Management is the authoritative source of truth for physical institutional infrastructure. | Infrastructure Management | The change updates only the canonical G10 Infrastructure record. | L2-G10 §§1–6; SutraERP L1 Business Domain Architecture |
| Aggregate identity shall never change. | Infrastructure Management | Administrative update cannot replace or rewrite the immutable Infrastructure identifier. | L2-G10 §8 Aggregate Identity |
| Business Codes and Names may change according to institutional policy. | Infrastructure Management | Mutable business information may be updated only under applicable policy and validation. | L2-G10 §8 Aggregate Identity |
| `WF-INF-006 — Update Infrastructure` governs modification of Infrastructure metadata and publishes `InfrastructureUpdated`. | Infrastructure Management | The generic administrative update uses the exact source-defined completed event. | L2-G10 WF-INF-006; event catalogue |
| Validation shall occur before state mutation. | Infrastructure Management | The approved values are validated before the authoritative Infrastructure fact is changed. | L2-G10 DIP-05; §29 Infrastructure Validation |
| Every business operation shall be authorized before execution. | Platform Foundation — Authorization | An unauthorized Infrastructure update cannot proceed. | L2-A04 AP-01 |
| Business domains own business facts; Event Infrastructure owns transport/governance only. | Infrastructure Management / Platform Foundation — Event Infrastructure | `InfrastructureUpdated` remains a G10 business fact. | L2-A06 EIP-01, EIP-02 |
| Audit evidence is generated from canonical business events and does not become the operational business record. | Platform Foundation — Audit | Historical evidence is preserved without transferring business ownership from G10. | L2-A07 §§4, 7.2, 7.4 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Infrastructure | Existing source-defined lifecycle state in which the requested administrative update is permitted | Administrative information is changed after authorization and validation; immutable identity is retained. | Lifecycle state remains unchanged unless the requested business need belongs to a separate source-defined lifecycle workflow. | Infrastructure Management | L2-G10 aggregate identity, WF-INF-006, canonical lifecycle rules |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `InfrastructureUpdated` | Infrastructure Management | Authorized consumers of current Infrastructure facts; Audit Infrastructure consumes canonical business events for evidence generation. | Publishes the completed generic Infrastructure update fact. | L2-G10 WF-INF-006 and event catalogue; L2-A06 EIP-02; L2-A07 §7.2 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

Section A provides canonical authorization, event-governance and audit capabilities. No hand-off transfers ownership of the Infrastructure aggregate or update decision from G10.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Perform an administrative Infrastructure update | Infrastructure Administrator or Campus Administrator within applicable scope | The operation proceeds only when the canonical authorization decision permits it and G10 validation succeeds. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

No universal scenario-specific approval, separation-of-duty or override rule for the generic Infrastructure update is defined by L2-G10. Where a specialized G10 workflow defines stronger governance, that specialized workflow remains authoritative.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve the authoritative Infrastructure change | The changed Infrastructure fact and its completed `InfrastructureUpdated` business event. | Infrastructure Management |
| Preserve immutable historical evidence | Audit evidence derived from the canonical completed business event, including available actor and business context. | Platform Foundation — Audit Infrastructure |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The requested administrative change has been authorized.
- The proposed values have passed the applicable G10 validation rules.
- The authoritative Infrastructure information has been updated without changing immutable identity or canonical ownership.
- `InfrastructureUpdated` has been published for the completed generic update.
- Historical evidence is preserved through the canonical audit architecture.

## 16.2 Postconditions

- Infrastructure Management remains the canonical owner of the updated physical Infrastructure fact.
- Authorized consumers may rely on the current Infrastructure information.
- Prior business evidence remains traceable and is not replaced by the new current fact.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No blocking architecture issue was identified for the frozen scenario outcome. L2-G10 contains both a generic Infrastructure update workflow and dedicated workflows for specialized changes; this Draft preserves those boundaries by requiring the dedicated workflow where it is authoritative.

**Architecture assessment:** `PASS`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
