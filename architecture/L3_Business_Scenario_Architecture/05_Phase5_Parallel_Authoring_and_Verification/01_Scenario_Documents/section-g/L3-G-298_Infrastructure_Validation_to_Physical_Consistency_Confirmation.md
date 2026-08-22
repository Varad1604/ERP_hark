# Infrastructure Validation to Physical Consistency Confirmation

# 1. Scenario Metadata

**Scenario ID:** L3-G-298  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-21  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-A04 — Authorization & Policy Workflows  
**Referenced By:** Authorized business processes that require confirmation of authoritative physical-infrastructure consistency before a governed change or use

# 2. Scenario Purpose

Validate an authoritative Infrastructure record or hierarchy against the physical-identity, hierarchy, classification, capacity, availability and relationship constraints owned by Infrastructure Management so that physical consistency is confirmed without transferring ownership or silently correcting source records.

# 3. Business Outcome

Physical identity, hierarchy, classification, capacity, availability, and relationship rules are validated

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an infrastructure record or hierarchy requires validation.

## 4.2 End Boundary

The scenario ends when the applicable Infrastructure record or hierarchy has been evaluated against the authoritative G10 validation rules and either physical consistency is confirmed or a validation failure is identified for a separately governed correction or change path.

## 4.3 In Scope

- Validation of the authoritative Infrastructure identity and required identifying information.
- Validation of parent-child physical hierarchy and relationship consistency.
- Validation of applicable infrastructure classification and lifecycle compatibility.
- Validation of source-defined capacity and availability constraints where applicable to the record being checked.
- Validation that physical-infrastructure relationships remain consistent with canonical ownership and hierarchy rules.
- Authorization of the validation business operation before it proceeds.

## 4.4 Out of Scope

- Creation, administrative update, correction, decommissioning or archival of Infrastructure records.
- Reclassification, capacity redefinition, availability change or other lifecycle mutation as part of the validation itself.
- Creation of a new validation-result lifecycle, state or event not defined by the locked L2 architecture.
- Redefinition of physical hierarchy or ownership by a consuming domain.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-G-299 — Administrative Infrastructure Change to Governed Update | A separate governed change may be required when validation identifies information that must be updated. |
| Related | L3-G-286 — Space Classification to Controlled Physical Classification | Classification changes remain governed by their dedicated Infrastructure Management outcome. |
| Related | L3-G-288 — Infrastructure Capacity Definition to Capacity Profile Activation | Capacity definition remains a separately governed Infrastructure Management outcome. |
| Related | L3-G-289 — Infrastructure Availability Definition to Availability Profile Activation | Availability definition remains a separately governed Infrastructure Management outcome. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns the authoritative physical Infrastructure record, hierarchy, identity, classification, capacity, availability and relationship rules being validated. |
| Platform Foundation / Section A | Owns the authorization decision and policy-evaluation capability used before the validation business operation; it does not own or alter Infrastructure facts. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Performs or initiates governed validation of Infrastructure within authorized scope. | May validate Infrastructure where the canonical authorization decision permits the business operation. |
| Campus Administrator | Participates in validation for Infrastructure within the governed campus scope. | May validate campus-scoped Infrastructure where authorized. |
| Auditor | Reviews source-defined historical evidence where available. | Read-only participation; does not change Infrastructure facts. |

L2-G10 defines common Infrastructure roles but does not assign one unique role as the mandatory initiator of every standalone infrastructure-validation scenario. No additional actor is invented here.

# 7. Preconditions and Entry Criteria

- The Infrastructure record or hierarchy to be validated exists under Infrastructure Management ownership.
- The validation scope identifies the relevant Infrastructure aggregate, hierarchy segment or relationship set.
- Applicable Infrastructure configuration, classification, capacity and availability facts needed for the validation are available from their canonical G10 source.
- The validation business operation is authorized before execution.

# 8. Trigger

An infrastructure record or hierarchy requires validation

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the authoritative Infrastructure record, hierarchy segment and relationships that require validation. | Infrastructure Management | The exact physical-infrastructure scope is established. |
| 2 | Confirm that the validation business operation is authorized for the applicable Infrastructure scope. | Infrastructure Management using the canonical authorization decision owned by Section A | The validation is permitted to proceed. |
| 3 | Validate Infrastructure identity and required identifying information without changing the immutable aggregate identity. | Infrastructure Management | Identity consistency is evaluated. |
| 4 | Validate the parent-child hierarchy and relationship structure, including that required parents exist and the physical hierarchy remains valid and acyclic. | Infrastructure Management | Hierarchy and relationship consistency are evaluated. |
| 5 | Validate the applicable classification and lifecycle constraints for the Infrastructure being checked. | Infrastructure Management | Classification and lifecycle consistency are evaluated against G10 rules. |
| 6 | Validate applicable capacity and availability constraints using the authoritative G10 capacity and availability facts. | Infrastructure Management | Capacity and availability consistency are evaluated. |
| 7 | Conclude the validation only when all applicable source-defined checks have been evaluated. | Infrastructure Management | Physical consistency is confirmed, or a validation failure is identified without silently changing the source Infrastructure. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Validation scope does not require every category | The Infrastructure record does not carry a particular capacity, availability or classification concern applicable to the requested validation. | Apply only the source-defined validations relevant to the identified record or hierarchy while preserving the mandatory identity, hierarchy and ownership boundaries. | The validation remains limited to applicable authoritative rules and does not invent irrelevant requirements. | Infrastructure Management |
| Specialized correction is required | A validation failure identifies information that must be changed through a dedicated Infrastructure workflow. | End the validation with the failure identified and use the separately governed change, classification, capacity, availability or correction path as applicable. | Validation does not become an unauthorized mutation workflow. | Infrastructure Management |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required parent or relationship is inconsistent | Physical consistency cannot be confirmed. | Preserve the validation result and resolve the authoritative hierarchy or relationship issue through the applicable governed Infrastructure change path before a later validation. | Infrastructure Management |
| Authorization does not permit the validation operation | The validation does not proceed. | Obtain a later valid authorization decision if the business need remains; no Infrastructure fact is changed by this scenario. | Platform Foundation / Authorization; Infrastructure Management retains ownership of the Infrastructure fact. |
| Required canonical Infrastructure information is unavailable or inconsistent | The requested validation cannot reach a trustworthy consistency conclusion. | Resolve the canonical source information before the validation is repeated; do not infer missing physical facts. | Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Infrastructure Management is the single canonical owner of physical institutional infrastructure. | Infrastructure Management | Validation uses G10 facts and cannot accept a downstream duplicate as authoritative. | L2-G10 §§1–6; SutraERP L1 Business Domain Architecture |
| Every Infrastructure transaction shall execute enterprise validation before state mutation. | Infrastructure Management | Validation is a prerequisite to governed mutation and is kept distinct from the mutation itself. | L2-G10 §29 Infrastructure Validation; DIP-05 |
| Structural validation includes parent existence, valid hierarchy, duplicate prevention and Infrastructure activity. | Infrastructure Management | Hierarchy checks cannot silently accept missing parents, duplicates or invalid structure. | L2-G10 §29 Structural Validation |
| The physical hierarchy shall remain acyclic and circular references shall be rejected. | Infrastructure Management | Relationship validation must reject circular hierarchy relationships. | L2-G10 hierarchy rules and Hierarchy Validation |
| Aggregate identity never changes; Business Codes and Names may change only according to institutional policy. | Infrastructure Management | Validation confirms identity consistency without rewriting immutable identity. | L2-G10 §8 Aggregate Identity |
| Capacity and availability validation use their authoritative G10 rules and current Infrastructure facts. | Infrastructure Management | A consuming domain cannot substitute its own capacity or availability interpretation. | L2-G10 Capacity Validation; Availability Validation |
| Every business operation shall be authorized before execution. | Platform Foundation — Authorization | Unauthorized validation does not proceed. | L2-A04 AP-01 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Infrastructure aggregate / hierarchy | Existing source-defined lifecycle state applicable to the record under validation | The scenario evaluates consistency only; it does not change the Infrastructure lifecycle. | Unchanged by this scenario. | Infrastructure Management | L2-G10 canonical lifecycle and §29 validation rules |
| Capacity Profile / Availability Profile / Classification facts | Existing authoritative G10 facts where applicable | Referenced for validation; not re-owned or mutated by the validation. | Unchanged by this scenario. | Infrastructure Management | L2-G10 aggregate ownership and validation rules |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

L2-G10 does not define a canonical standalone business event named for successful Infrastructure validation or physical-consistency confirmation. This Draft therefore does not invent one.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Section A participates through the canonical authorization decision required before the validation operation. No synchronous hand-off transfers ownership of the physical Infrastructure or of the validation rules from Infrastructure Management.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Validate an Infrastructure record or hierarchy | Infrastructure Administrator or Campus Administrator within applicable scope | The operation proceeds only when the canonical authorization decision permits it. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific approval, separation-of-duty, override or exception authority for standalone Infrastructure validation is defined by L2-G10. This Draft does not add one.

## 15.3 Business Audit and Evidence Obligations

The authorization decision remains auditable under the Platform Authorization architecture. L2-G10 does not define a standalone Infrastructure-validation event or a separate validation-result aggregate from which this Draft could assert additional canonical audit evidence without inventing architecture.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The authoritative Infrastructure scope has been identified.
- All applicable source-defined identity, hierarchy, classification, capacity, availability and relationship checks have been evaluated.
- Physical consistency is either confirmed or a validation failure is explicitly identified.
- No Infrastructure state has been silently changed as part of the validation.

## 16.2 Postconditions

- A successful validation confirms physical consistency for the evaluated scope at that business point in time.
- A failed validation requires a separately governed correction or change path before a later successful validation can be asserted.
- Infrastructure Management remains the owner of all physical Infrastructure facts evaluated by the scenario.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-G10 defines the §29 Infrastructure validation rules and their validation categories, but it does not define a standalone validation-result aggregate, lifecycle or canonical event for “Physical Consistency Confirmation.” | The frozen outcome can be completed as a governed validation result, but this Draft cannot invent a new persistent business concept or event for the confirmation. | Upstream G10 architecture may define a canonical validation-result concept/event if later cross-domain traceability requires one. |

**Architecture assessment:** `PASS WITH ISSUES`

The frozen outcome is fully expressible using existing G10 validation rules; the missing standalone result/event is non-blocking because the scenario does not require ownership transfer or a lifecycle mutation.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions beyond the recorded upstream event/result gap.
