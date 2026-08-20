# Duplicate Person Identification to Canonical Identity Reconciliation

# 1. Scenario Metadata

**Scenario ID:** L3-A-005  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Person & Identity  
**Participating Domains:** A  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A02 — Person & Identity Workflows; L2-A04 — Authorization & Policy Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** None yet  

---

# 2. Scenario Purpose

Govern the reconciliation of multiple Person records confirmed to represent the same real individual so one correct canonical Person survives while identity history, business references and auditability are preserved.

---

# 3. Business Outcome

Duplicate identities are governed and reconciled into the correct canonical Person while historical references and auditability are preserved.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when multiple Person records are determined to represent the same real individual.

## 4.2 End Boundary

The intended end boundary is reached when one Person has been selected as the canonical survivor, the duplicate Person records have become preserved historical merge sources, applicable references have been reconciled to the survivor, `PersonMerged` has been published by the Person aggregate, and the complete merge history remains auditable.

The exact end-boundary treatment of associated Identity aggregates and downstream-owned business references is unresolved upstream and is recorded in Section 17.

## 4.3 In Scope

- Validation that the confirmed duplicate Persons belong to the same Tenant.
- Selection of the canonical surviving Person.
- Dependency, legal-restriction and merge-policy validation.
- Administrative merge approval.
- Governed reconciliation of references to the surviving Person.
- Preservation of historical references and business history.
- Archival of duplicate Person records as historical merge sources where a valid lifecycle transition exists.
- Publication of `PersonMerged`.
- Preservation of complete identity-governance auditability.

## 4.4 Out of Scope

- Preliminary duplicate-candidate detection or advisory duplicate-warning handling before duplicate confirmation.
- Establishment, verification or activation of a new Person and associated Identity under L3-A-004.
- Reversal of an incorrect merge or restoration of separated identities under L3-A-006.
- Creation of User Accounts.
- Credential, authentication-factor, Authentication Provider or Authentication Session lifecycles owned by L2-A03.
- Role, Role Assignment, Delegation or authorization-policy lifecycle changes owned by L2-A04.
- Redesign of downstream business records or transfer of their canonical ownership.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-A-004 — Person Establishment to Active Canonical Identity | Establishes a new canonical Person and Identity; confirmed duplicate reconciliation is excluded from its successful establishment outcome. |
| Related | L3-A-006 — Erroneous Person Consolidation to Restored Identity Separation | Governs the separate later outcome when an approved administrative review determines that a completed consolidation must be reversed. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Person & Identity / Section A | Owns duplicate governance, survivor selection, Person merge, Person lifecycle effects, canonical identity references, `PersonMerged`, merge history and the final reconciliation outcome. |
| Authorization & Policy / Section A | Owns the authorization policies that determine whether an actor may review or approve the merge; it does not own the merge decision or Person lifecycle. |
| Audit Infrastructure / Section A | Owns immutable platform audit evidence generated from canonical business events; it does not own the Person merge or its business decision. |

Business domains retain ownership of their own records. Reference consumption or synchronization does not transfer ownership of those records to Person & Identity.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Person | The real individual represented by the duplicate Person records and the resulting canonical Person. | Subject of reconciliation; no review or approval authority is assigned by this scenario. |
| Authorized administrators | Perform governed duplicate review, survivor selection and permitted administrative merge actions. | May act only under applicable authorization and merge policies. |

L2-A02 requires administrative approval for Person merge, but neither L2-A02 nor L2-A04 maps that approval to a specific canonical business role. The gap is recorded in Section 17.

---

# 7. Preconditions and Entry Criteria

- A valid `Active` Tenant context has been resolved under L2-A01.
- All Person records proposed for reconciliation belong to the same Tenant.
- The information required for survivor selection, dependency validation, legal-restriction validation and merge-policy evaluation is available.
- The proposed reconciliation has not already been completed as an erroneous consolidation requiring the separate L3-A-006 outcome.

---

# 8. Trigger

Multiple Person records are determined to represent the same real individual.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Accept the confirmed duplicate determination and identify the Person records proposed for reconciliation. | Person & Identity | The governed merge scope is established. |
| 2 | Confirm that all proposed Person records belong to the same Tenant. | Platform & Tenant; Person & Identity | Cross-Tenant consolidation is excluded. |
| 3 | Select the Person that will remain the canonical survivor. | Authorized administrators; Person & Identity | One surviving canonical Person is identified. |
| 4 | Validate dependencies, legal restrictions, the applicable merge policy and the proposed survivor. | Person & Identity | The proposed merge is eligible for approval or is blocked. |
| 5 | Obtain the required administrative approval under the applicable authorization policy. | Person & Identity; Authorization & Policy | The merge is approved for governed execution. |
| 6 | Reconcile applicable current references to the selected canonical Person while preserving ownership of downstream business records and leaving historical audit references unchanged. | Person & Identity | The intended canonical-reference outcome is established, subject to the unresolved collaboration contract in Section 17. |
| 7 | Preserve the non-surviving Person records as historical merge sources and archive them where an authoritative lifecycle transition permits archival. | Person & Identity — Person aggregate | Duplicate Person records cease to act as independent canonical Persons while their history is retained. |
| 8 | Publish `PersonMerged` from the Person aggregate. | Person & Identity — Person aggregate | Interested consumers are informed of the completed canonical Person merge. |
| 9 | Preserve the complete merge history and required immutable audit evidence. | Person & Identity; Audit Infrastructure | The reconciliation remains historically reconstructable and auditable. |

The flow ends at canonical Person reconciliation. It does not include reversal or Person separation under L3-A-006.

---

# 10. Alternate Business Flows

No scenario-specific alternate business flow toward the same reconciliation outcome is completely defined by the authoritative L2 architecture.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| The Person records belong to different Tenants. | The proposed consolidation violates Tenant isolation and cannot proceed. | Reject the merge; L2-A02 prohibits Person migration or consolidation across Tenant boundaries. | Platform & Tenant; Person & Identity |
| Administrative approval is not granted. | The Person merge cannot proceed. | Leave the Person records unchanged unless a later governed review produces an approved outcome. | Person & Identity |
| A conflicting legal restriction exists. | The proposed merge is ineligible. | Block the merge until the restriction is authoritatively resolved. | Person & Identity |
| Dependency validation is incomplete or fails. | Referential integrity cannot be confirmed. | Block the merge until dependency validation succeeds. | Person & Identity |
| No canonical survivor can be validly identified or the merge policy is not satisfied. | The reconciliation outcome cannot be determined correctly. | Block the merge pending a governed administrative decision supported by the applicable policy. | Person & Identity |
| A non-surviving Person is not in a state with an authoritative transition to `Archived`. | The duplicate cannot be archived through the current Person lifecycle definition. | Stop lifecycle progression and obtain an upstream lifecycle decision; do not invent a transition. | Person & Identity |
| An associated Identity requires merge, reassignment, archival, disabling or another lifecycle effect. | The required canonical identity reconciliation cannot be completed from the currently defined merge workflow. | Stop the affected Identity lifecycle action pending an authoritative L2-A02 decision. | Person & Identity |
| A completed merge is later determined to be incorrect. | The A005 outcome must not be locally reversed within this scenario. | Initiate the separately governed L3-A-006 identity-separation outcome after the required approved administrative review. | Person & Identity |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every operation executes inside one Tenant boundary, and cross-Tenant execution is prohibited. | Platform & Tenant | Only Person records belonging to the same resolved Tenant may be considered for reconciliation. | L2-A01 — Tenant Foundation; Tenant Isolation |
| One canonical Person represents a real individual within a Tenant. | Person & Identity | The successful outcome retains one canonical surviving Person. | L2-A02 — Domain Design Principles; Identity Governance Business Rules |
| PersonID is immutable. | Person & Identity | The survivor retains its immutable identifier; L3 does not redefine identifiers. | L2-A02 — Person Identifier; Person Business Rules |
| Person merge requires administrative approval. | Person & Identity | The merge cannot proceed without governed approval. | L2-A02 — Approval Policies; Merge Validation |
| A survivor must be identified and the merge policy satisfied. | Person & Identity | Survivor selection and policy validation precede merge execution. | L2-A02 — Merge Validation |
| Dependency validation must be complete and no conflicting legal restriction may exist. | Person & Identity | Failure of either validation blocks the merge. | L2-A02 — Merge Validation |
| Merge shall preserve business history. | Person & Identity | Historical information must not be destroyed by reconciliation. | L2-A02 — Person Merge; Identity Governance Business Rules |
| Merge and split operations shall not violate referential integrity. | Person & Identity | References must remain valid and traceable throughout reconciliation. | L2-A02 — Identity Governance Architectural Constraints |
| Historical audit references remain unchanged. | Person & Identity | Reconciliation must not rewrite historical audit identity references. | L2-A02 — Reference Migration |
| Business domains remain owners of their own business records. | Respective business domains | Reference reconciliation does not transfer ownership to Person & Identity. | L2-A02 — Canonical Ownership; Domain Events & Integration Contracts |
| All identity-governance operations are auditable. | Person & Identity | The merge decision and resulting Person changes must remain traceable. | L2-A02 — Identity Governance Business Rules |
| Audit evidence generated from canonical business events is immutable and does not transfer ownership of the underlying business activity. | Audit Infrastructure | Audit Infrastructure preserves evidence of the merge without owning the merge decision or Person lifecycle. | L2-A07 — Audit Principles; Domain Boundaries |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Surviving Person | Existing authoritative Person state | Selected as the canonical survivor; L2-A02 defines no merge-specific state transition for the survivor. | Existing authoritative state remains unchanged by the defined merge workflow. | Person & Identity | L2-A02 — Identity Merge; Merge Workflow |
| Non-surviving Person | `Active` | Archived as a historical merge source. | `Archived` | Person & Identity | L2-A02 — Allowed State Transitions; Merge Workflow |
| Non-surviving Person | `Inactive` | Archived as a historical merge source. | `Archived` | Person & Identity | L2-A02 — Allowed State Transitions; Merge Workflow |

L2-A02 does not define a valid merge-related archival transition for a non-surviving Person in `Draft`, `Created` or `Verified`. It also does not define merge-specific lifecycle effects for the associated Identity aggregates. These gaps are recorded in Section 17.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `PersonMerged` | Person aggregate | Admissions and Finance | Announces completion of the governed Person merge so approved consumers can synchronize their references without acquiring Person ownership. | L2-A02 — Person Events; Event Consumers; Merge Workflow |

`DuplicateCandidateDetected` precedes the frozen trigger when it represents only a potential duplicate and is therefore not treated as a completion event for this scenario.

## 14.2 Business Contracts and Cross-Domain Hand-offs

No authoritative named synchronous business contract or governed hand-off is defined for transferring downstream-owned business references before `PersonMerged` is published.

L2-A02 requires reference transfer but also prohibits Person & Identity from directly modifying downstream business-domain data. The resulting collaboration gap is recorded in Section 17 rather than being replaced with an invented contract.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Perform administrative duplicate review and survivor selection | Authorized administrators | Must act under applicable authorization and merge policies. | Person & Identity |
| Approve Person merge | Not defined in locked L2-A02 or L2-A04 | Approval is mandatory, and approval authority is determined by authorization policies; L3 does not assign an unsupported role. | Person & Identity |
| Execute the approved Person merge | Authorized administrators | Permitted only after all merge validations and required approval succeed. | Person & Identity |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty requirement is defined by the authoritative L2 architecture.

L3 does not infer that merge review, survivor selection and merge approval must be performed by different actors.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve complete merge history. | Traceable history of the duplicate determination, survivor selection, approved merge and resulting Person reconciliation. | Person & Identity |
| Preserve administrative approval. | The governed merge-approval outcome required by L2-A02. | Person & Identity |
| Preserve historical references. | Unchanged historical audit references and traceability from each historical merge source to the canonical survivor. | Person & Identity |
| Preserve immutable audit evidence. | Canonical audit evidence generated from the completed merge event without changing ownership of the underlying Person merge. | Audit Infrastructure |

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- One Person has been selected as the canonical survivor.
- All mandatory merge validations and administrative approval have succeeded.
- Duplicate Person records have become preserved historical merge sources through valid lifecycle handling.
- Applicable references have been reconciled to the canonical survivor without transferring ownership of downstream business records.
- Historical references and business history remain preserved.
- `PersonMerged` has been published by the Person aggregate.
- The reconciliation and its approval remain auditable.

Completion cannot be confirmed for associated Identity aggregates or for downstream-owned reference transfer until the issues in Section 17 are resolved.

## 16.2 Postconditions

- The surviving Person remains the canonical Person for the represented real individual within the applicable boundary.
- Non-surviving Person records remain historically traceable and do not replace the canonical survivor.
- Approved consumers may respond to `PersonMerged` without acquiring Person ownership.
- A later determination that the merge was erroneous must proceed through L3-A-006 rather than being reversed locally in this scenario.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Locked L1 §3.4 states that every real individual has one Person record inside an Institute, while locked L2-A01, locked L2-A02 and the frozen Phase 2/Stage 3 semantics establish Tenant-scoped Person and Identity ownership. | Duplicate uniqueness, same-boundary validation and merge eligibility cannot become architecture-approved or locked while the canonical scope differs between L1 and L2. | Lead Architect must align the Person uniqueness boundary across locked L1 and L2 through architecture change control. |
| L2-A02 requires administrative review and approval for Person merge but does not identify the canonical approving role; L2-A04 defines authorization policy ownership without mapping Person merge approval to a specific role. | The Draft can preserve mandatory approval but cannot assign complete business authority for L4 behavior. | Person & Identity and Authorization & Policy architecture must identify the canonical approving role or define an authoritative policy-based role-resolution rule. |
| L2-A02 describes an “Identity Merge” as consolidation of Person records but does not define what happens to the associated Identity aggregates, including their associations, states, lifecycle transitions or canonical references. | The frozen outcome requires duplicate identity reconciliation, but the Identity lifecycle portion of that outcome cannot be specified without inventing architecture. | L2-A02 must define merge behavior for every associated Identity aggregate and any exact resulting events. |
| The L2-A02 merge workflow archives duplicate Person records without constraining their entry state, while its allowed Person transitions permit `Active → Archived` and `Inactive → Archived` only. | A duplicate Person in `Draft`, `Created` or `Verified` has no authoritative transition to the merge workflow’s required archived historical-source outcome. | L2-A02 must define merge eligibility by Person state and add any approved lifecycle transitions through architecture change control. |
| L2-A02 requires Admissions, Student, HR, Examination, Finance, Library and Hostel references to be transferred before the duplicate Person is archived and `PersonMerged` is published. The same document prohibits Person & Identity from modifying business-domain data. Frozen Stage 3 lists only Domain A as participating, and no synchronous business contract or hand-off is defined for the transfer. | The mandatory reference-transfer responsibility, participating-domain boundary and exact scenario end boundary cannot be completed without either changing frozen participation or defining an approved collaboration contract. | Lead Architect must clarify whether downstream reference updates are inside A005 or post-event downstream outcomes. If inside, the Stage 3 participating-domain value and authoritative L2 contracts require controlled amendment; if outside, L2-A02 must move reference synchronization after `PersonMerged` and define the resulting boundary. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.