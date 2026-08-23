# Infrastructure Correction to Audited Historical Amendment

# 1. Scenario Metadata

**Scenario ID:** L3-G-319  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Authorised governance, audit, investigation and historical-reconstruction activities

# 2. Scenario Purpose

Govern an authorised correction to a completed G10 Infrastructure record so the current authoritative fact is amended through source-owned procedures while the original historical action and immutable audit evidence remain traceable.

# 3. Business Outcome

The correction is recorded through governed procedures while the original historical action remains traceable

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorised correction is required for a completed infrastructure record.

## 4.2 End Boundary

The scenario ends when the valid corrected Infrastructure fact is established under G10 ownership, the original completed action remains preserved, and additional immutable audit evidence records the correction lineage.

## 4.3 In Scope

- Identification of the completed G10-owned record and fact requiring correction.
- Validation of correction authority and applicable G10 business rules.
- Preservation of immutable Infrastructure identity and original history.
- Establishment of the corrected current G10 business fact.
- Additional immutable audit evidence describing the correction/amendment.
- Publication of an existing canonical G10 event where the corrected fact maps to a source-defined event.

## 4.4 Out of Scope

- Deleting, rewriting or obscuring the original historical action.
- Modifying committed audit evidence.
- Inventing a correction aggregate, correction lifecycle, generic correction event or universal approval role.
- Correcting business facts owned by another domain.
- Technical rollback, data repair mechanics, database updates, retries or implementation recovery.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns the Infrastructure record, corrected business fact, applicable business validation, lifecycle/attribute consequences and any source-defined change event. |
| Platform Foundation / Section A | A07 owns immutable audit evidence generated from canonical business events; A04 supplies authorization/policy capability where applicable. |

Audit participation never transfers ownership of the Infrastructure business fact from G10.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Initiates or applies an authorised correction within Infrastructure scope. | Must satisfy applicable G10 business and authorization rules. |
| Campus Administrator | May correct campus-scoped Infrastructure information within authorised responsibility. | Authority is limited to applicable scope. |
| Auditor | Reviews original evidence and correction lineage where authorised. | Read-only audit participation. |

G10 recognises Administrative Data Corrections as governance-sensitive operations, but no universal correction approver is named in the reviewed locked sources.

# 7. Preconditions and Entry Criteria

- The target completed record exists and is canonically owned by G10.
- The original completed action/history remains available and traceable.
- The correction request is authorised.
- The proposed corrected value satisfies applicable G10 rules and does not require mutation of immutable identity.

# 8. Trigger

An authorised correction is required for a completed infrastructure record

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the completed G10 Infrastructure record and the precise business fact requiring correction. | Infrastructure Management | Correction scope is unambiguous. |
| 2 | Confirm that the fact is G10-owned and that immutable Infrastructure identity will not be changed. | Infrastructure Management | Canonical ownership and identity are preserved. |
| 3 | Validate the correction against applicable authorization, business rules, lifecycle constraints and any configured governance approval requirement. | Infrastructure Management / Platform authorization capability | Only an authorised valid correction may proceed. |
| 4 | Establish the corrected value as the current authoritative G10 fact while preserving the original completed action and prior value in history. | Infrastructure Management | Current truth is corrected without erasing historical truth. |
| 5 | Publish the existing source-defined G10 event corresponding to the corrected fact where one is defined. | Infrastructure Management | The corrected completed fact is communicated without inventing a generic correction event. |
| 6 | Generate additional immutable audit evidence for the correction, preserving actor, business entity, operation, change attribution, correlation/causation and historical traceability. | Audit Infrastructure | The correction is auditable while prior evidence remains immutable. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Proposed correction violates G10 rules | Corrected value is invalid. | Reject the correction and preserve the current authoritative record. | No amendment occurs. | Infrastructure Management |
| Correction targets immutable identity | Request would replace the canonical Infrastructure identifier. | Reject the identity mutation and resolve the business issue without changing canonical identity. | Identity remains unchanged. | Infrastructure Management |
| Corrected fact belongs to another domain | The target fact is only referenced by G10. | Route the correction to the canonical source owner; G10 retains only the permitted reference. | No ownership transfer occurs. | Infrastructure Management / source-owning domain |
| Configured governance approval is required | Applicable upstream policy requires approval for the Administrative Data Correction. | Await the authoritative approval outcome before applying the correction. | Correction proceeds only when approved. | Infrastructure Management / governance authority |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Original history cannot be reconstructed | The amendment cannot preserve required historical truth. | Do not complete the correction until the original authoritative history is available. | Infrastructure Management / Audit Infrastructure |
| Required audit evidence cannot be established | Accountability for the correction would be incomplete. | Do not claim completion until immutable correction evidence can be generated. | Audit Infrastructure / Infrastructure Management |
| Correction is later found to be wrong | A second governed amendment is required. | Preserve all existing history and apply a new authorised correction rather than rewriting previous evidence. | Infrastructure Management / Audit Infrastructure |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Infrastructure identity is immutable. | Infrastructure Management | Correction cannot replace canonical aggregate identity. | L2-G10 §§8, 47 |
| Infrastructure history shall never be deleted and historical reconstruction shall remain possible. | Infrastructure Management | Original completed actions remain traceable. | L2-G10 §6 Complete Auditability; §47 |
| Administrative Data Corrections are governance-sensitive operations; governance approval requirements are configurable. | Infrastructure Management / Governance | L3 applies only source-configured approval requirements and does not invent a universal approver. | L2-G10 governance operations section |
| Once audit evidence is committed it is immutable; correction, amendment, supersession or invalidation is represented by additional audit evidence. | Audit Infrastructure | The original evidence is never modified. | L2-A07 §7.3 |
| Business domains own business transactions/state while Audit owns business evidence. | Infrastructure Management / Audit Infrastructure | Corrected Infrastructure truth remains G10-owned; A07 owns evidence only. | L2-A07 §§3–4 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Infrastructure aggregate / completed record | Any source-defined context in which the affected fact may validly be amended | The applicable business fact is corrected without changing immutable identity. | Existing lifecycle remains unchanged unless the source-defined correction itself validly changes a lifecycle fact. | Infrastructure Management | L2-G10 |
| Audit Evidence | Existing immutable evidence | Additional correction/amendment evidence is created. | Prior evidence remains immutable; new evidence records the correction. | Audit Infrastructure | L2-A07 §7.3 |

No separate `Correction` lifecycle is defined upstream and none is introduced.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

G10 defines source-specific update events, including `InfrastructureUpdated` for the generic Update Infrastructure workflow and more specific events for particular changed facts. The scenario publishes only the canonical event that corresponds to the actual corrected fact. No `InfrastructureCorrected` or `HistoricalAmendmentRecorded` event is invented.

A07 consumes canonical business events to generate immutable audit evidence; it does not own the corrected Infrastructure state.

## 14.2 Business Contracts and Cross-Domain Hand-offs

The source-owned G10 business fact remains available through approved Infrastructure contracts/events. A07 preserves immutable evidence describing the completed correction. No synchronous hand-off transfers Infrastructure ownership to Section A.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate/apply Infrastructure correction | Infrastructure Administrator or applicable scoped administrator | Must satisfy applicable G10 authorization, validation and configured governance requirements. | Infrastructure Management |
| Review immutable correction evidence | Auditor | Read-only evidence access under applicable policy. | Audit Infrastructure |

## 15.2 Separation of Duties, Overrides and Exceptions

The reviewed G10 source does not define one universal approving actor or separation-of-duty model for every Administrative Data Correction. No local approval role is invented.

## 15.3 Business Audit and Evidence Obligations

Correction evidence must preserve sufficient context to reconstruct who performed the correction, what changed, when it occurred, under which authority, which business entity was affected, and how the correction relates to prior evidence. The original evidence remains immutable.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The target record and corrected fact are confirmed as G10-owned.
- The correction is authorised and valid under applicable source rules.
- Immutable Infrastructure identity is preserved.
- The corrected fact becomes the current authoritative G10 value.
- The original historical action/value remains traceable.
- Any applicable canonical G10 event for the corrected fact is published.
- Additional immutable A07 audit evidence records the amendment lineage.

## 16.2 Postconditions

- G10 remains authoritative for the corrected current Infrastructure fact.
- Original Infrastructure history and committed audit evidence remain preserved.
- The correction is represented by additional governed business history/evidence rather than destructive rewriting.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| G10 does not define one universal correction/amendment event; event selection depends on the actual corrected fact. | Consumers cannot rely on a single generic correction event. | Non-blocking: use the source-defined event for the corrected fact when one exists; do not invent a generic event. |
| G10 identifies Administrative Data Corrections as potentially subject to configurable governance approval but does not name a universal approving business role. | Approval authority cannot be made universally mandatory in this L3 scenario. | Upstream governance may define correction-specific approval authority where required. |

**Architecture assessment:** `PASS WITH ISSUES`

The frozen correction and historical-traceability outcome is fully grounded by G10's history obligations and A07's immutable-evidence amendment rule. The remaining event/approver granularity gaps are non-blocking.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
