# Institutional Infrastructure Registration to Physical Campus Establishment

# 1. Scenario Metadata

**Scenario ID:** L3-G-278  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, B, A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-B03 — Campus Workflows; L2-A01 — Platform & Tenant Foundation  
**Referenced By:** Infrastructure hierarchy and downstream physical-context scenarios

---

# 2. Scenario Purpose

Establish one canonical G10-owned Physical Campus for an authorised institution while preserving B03 ownership of the InstitutionalCampus identity and institution-to-campus governance relationship.

---

# 3. Business Outcome

A canonical Physical Campus is registered with its identity, required foundational information and authorised physical attribute profile, while institutional ownership remains with B03.

The locked G10 source requires successful registration to publish a canonical registration event but gives contradictory canonical names for that event. The scenario is therefore **BLOCKED** until the conflict is resolved upstream.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorised institution requires a new physical infrastructure representation.

## 4.2 End Boundary

The scenario ends when the Physical Campus exists canonically under G10, references the authoritative institutional context from B03, preserves its immutable identity and registration evidence, and the required canonical registration fact has been published under an unambiguous upstream event definition.

## 4.3 In Scope

- Validation of the institution and InstitutionalCampus reference supplied by Section B.
- Validation of Physical Campus code, foundational physical information and applicable physical attributes.
- Creation of the G10 Physical Campus aggregate and immutable identity.
- Preservation of institutional ownership in B03 and physical ownership in G10.
- Registration audit evidence and publication of the required G10 registration fact.

## 4.4 Out of Scope

- Creation or modification of the B03 InstitutionalCampus.
- Physical hierarchy below the Physical Campus beyond references needed for registration.
- Operational activation, scheduling, allocation, maintenance or consumer-domain decisions.
- Choosing between conflicting locked event names or lifecycle states.
- Any implementation mechanism.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-G-279 — Institutional Campus Reference to Physical Campus Association | Governs the B03/G10 association boundary. |
| Related | L3-G-281 — Infrastructure Hierarchy Creation to Canonical Physical Structure | Establishes the physical hierarchy beneath the Physical Campus. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / G10 | Primary owner of the Physical Campus aggregate, physical identity, physical attributes and physical lifecycle. |
| Institution & Structure / B03 | Owns the InstitutionalCampus identity, institutional status and Institution-to-Campus governance association. |
| Platform Foundation / A | Provides the common configuration and authorization framework consumed by G10 without owning Physical Campus semantics. |

The frozen participating-domain value remains exactly `G, B, A`.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority Boundary |
|---|---|---|
| Infrastructure Administrator | Initiates and administers permitted Physical Campus registration. | Must remain within authorized infrastructure scope and G10 business rules. |
| Institution Administrator | Provides the institution-side governance context where required. | Does not create or own the G10 Physical Campus. |
| Auditor | Reviews immutable registration history. | Read-only. |

No universal approval actor is inferred beyond source-defined roles and configurable approval policy.

---

# 7. Preconditions and Entry Criteria

- The Institution and required InstitutionalCampus reference exist under their canonical owners.
- The requested Physical Campus code is unique under applicable G10 rules.
- Required foundational physical information and applicable configuration are available.
- The requester is authorised for Infrastructure Registration.
- No duplicate Physical Campus is being created for the same intended physical representation.

---

# 8. Trigger

An authorised institution requires a new physical infrastructure representation.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Resolve the authoritative Institution and InstitutionalCampus context. | B03; Infrastructure Management | Institutional context is confirmed without ownership transfer. |
| 2 | Validate the requested Physical Campus code, name and required foundational physical information. | Infrastructure Management | Registration inputs satisfy G10 rules or are rejected. |
| 3 | Validate the authorised physical attribute profile and applicable configuration. | Infrastructure Management | Only permitted physical facts proceed. |
| 4 | Establish the G10 Physical Campus aggregate and immutable identity. | Infrastructure Management | A canonical Physical Campus exists under G10 ownership. |
| 5 | Preserve the reference to the B03-owned institutional context without duplicating InstitutionalCampus ownership. | Infrastructure Management | Institutional and physical ownership remain separate. |
| 6 | Record the registration decision and resulting Physical Campus evidence. | Infrastructure Management | Registration is auditable. |
| 7 | Publish the required canonical Physical Campus registration event. | Infrastructure Management | **Blocked:** locked L2-G10 defines conflicting event names for this completed registration fact. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome |
|---|---|---|---|
| Existing physical representation | A matching canonical Physical Campus already exists. | Reuse the canonical G10 identity rather than creating a duplicate. | No duplicate Physical Campus is created. |
| Configured approval applies | Registration requires approval under institutional policy. | Complete only the source-supported approval path before treating registration as complete. | No unsupported approval authority is invented. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Institution or InstitutionalCampus reference is invalid. | Physical registration cannot be grounded. | Correct the authoritative B03 context before a new registration attempt. | B03; Infrastructure Management |
| G10 uniqueness or physical validation fails. | A canonical Physical Campus cannot be established. | Correct the proposed physical information; preserve the prior state. | Infrastructure Management |
| Required approval is incomplete. | Registration cannot be treated as complete where policy requires approval. | Retain the incomplete business position until source-defined approval completes. | Infrastructure Management |
| Registration event name remains contradictory upstream. | The mandatory completed business fact cannot be published canonically. | Do not invent or choose a local event name; resolve through architecture change control. | Infrastructure Management |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Physical Campus ownership belongs exclusively to G10. | Infrastructure Management | B03 and consumers may reference but never own the physical aggregate. | L2-G10 §§1-3, 8; L2-B03 §§1, 15 |
| InstitutionalCampus identity and institutional governance remain in B03. | Institution & Structure | G10 cannot redefine institutional campus ownership. | L2-B03 §§1, 14-15 |
| Physical Campus code must be unique and aggregate identity is immutable. | Infrastructure Management | Duplicate identity is rejected; identity is preserved after creation. | L2-G10 §§8.1, 13A.64 |
| Successful infrastructure workflows publish a canonical business event. | Infrastructure Management | Registration cannot be canonically complete while its exact event name is contradictory. | L2-G10 §22A.99; §§44, 44A |
| Business-domain configuration values remain owned by the respective business domain while Platform owns the configuration framework. | G10 / Platform Foundation | Section A does not take ownership of G10 physical semantics. | L2-A01 §§6.1-6.5 |

---

# 13. State and Lifecycle Impact

L2-G10 contains conflicting initial-state statements for newly registered infrastructure: the registration summary states `Draft`, while the detailed canonical lifecycle uses `Proposed → Planned → Approved → Under Construction → Operational...` and WF-INF-001 states `Proposed or Planned` initially.

This Draft therefore records only that a newly established Physical Campus is **not yet operational** and does not choose an initial lifecycle state.

| Business Concept / Aggregate | Entry Condition | Scenario Impact | Resulting Position | Canonical Owner |
|---|---|---|---|---|
| Physical Campus | No canonical physical representation exists for the requested site | Establishes the aggregate and immutable identity | Registered, non-operational; exact initial lifecycle state unresolved upstream | Infrastructure Management |
| InstitutionalCampus | Existing authoritative institutional record | Referenced only | Unchanged | Institution & Structure |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

The required registration event is unresolved. L2-G10 §44 names `CampusRegistered`, while WF-INF-001, the workflow event summary and §44A use `CampusCreated`. Both describe the same G10 registration completion and the same canonical publisher, so L3 cannot select one without changing locked architecture.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider | Consumer | Business Contract / Hand-off | Purpose |
|---|---|---|---|
| B03 | G10 | InstitutionalCampus identity / institutional reference | Grounds the Physical Campus in the authoritative institutional context. |
| G10 | Authorized consumers | Physical Campus Summary | Provides canonical Physical Campus identity and physical facts without transferring ownership. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

Infrastructure registration requires source-supported authorization for the applicable institutional and infrastructure scope.

## 15.2 Approval and Separation of Duty

Approval is used only where configured by authoritative policy. No universal approver or separation-of-duty rule is created locally.

## 15.3 Business Audit and Evidence Obligations

Registration evidence must preserve the actor, Institution/InstitutionalCampus reference, Physical Campus identity, submitted physical facts, validation result, applicable approval reference, and completed business outcome. Audit history remains immutable.

---

# 16. Completion Criteria and Postconditions

The scenario is complete only when:

- the B03 institutional reference is valid;
- the G10 Physical Campus is uniquely established with immutable identity and required physical information;
- institutional and physical ownership remain separate;
- registration evidence is preserved; and
- the exact canonical registration event has been resolved upstream and published by G10.

Until the event conflict is resolved, the frozen outcome cannot be treated as canonically complete.

---

# 17. Open Issues, Assumptions and Deferred Questions

| ID | Severity | Issue / Deferred Question | Required Resolution |
|---|---|---|---|
| G-278-BLOCKER-01 | Blocking | L2-G10 defines the mandatory Physical Campus registration event as `CampusRegistered` in §44 but as `CampusCreated` in WF-INF-001, the workflow summary and §44A. | Architecture governance must resolve one canonical event name while retaining G10 as publisher. |
| G-278-ISSUE-02 | Non-blocking | L2-G10 conflicts on the initial registration lifecycle position (`Draft` versus `Proposed or Planned`). | Align the locked lifecycle and registration workflow before downstream specifications depend on the exact initial state. |

**Source Assessment:** **BLOCKED**

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
