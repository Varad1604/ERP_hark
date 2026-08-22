# Institutional Campus Reference to Physical Campus Association

# 1. Scenario Metadata

**Scenario ID:** L3-G-279  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, B  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-B03 — Campus Workflows  
**Referenced By:** Cross-domain consumers resolving institutional and physical campus context

---

# 2. Scenario Purpose

Preserve the canonical association boundary between B03 InstitutionalCampus identity and the G10 Physical Campus without duplicating either aggregate or transferring ownership.

---

# 3. Business Outcome

The institutional campus can reference the canonical Physical Campus without duplicating physical ownership.

The frozen outcome conflicts with the locked B03/G10 relationship direction. B03 states that G10's PhysicalInfrastructureCampus references the InstitutionalCampus identity issued by B03, while the frozen scenario outcome is phrased in the opposite direction. This material ownership/contract ambiguity makes the scenario **BLOCKED**.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an InstitutionalCampus requires its corresponding physical infrastructure representation.

## 4.2 End Boundary

The scenario may end only when the authoritative B03/G10 reference direction and business contract are unambiguous, both canonical identities remain independently owned, and the permitted association is established without duplicating either aggregate.

## 4.3 In Scope

- Resolution of the B03 InstitutionalCampus identity.
- Resolution of the G10 Physical Campus identity.
- Validation of Tenant and Institution consistency.
- Preservation of independent B03 and G10 ownership.
- Establishment of only the source-authorized reference relationship.
- Audit evidence for the association decision.

## 4.4 Out of Scope

- Creation of either aggregate.
- Copying Physical Campus facts into B03 or institutional governance facts into G10.
- Re-parenting, lifecycle change, operational activation, scheduling, or maintenance.
- Inventing association states, events, commands, approval authority, or contract direction.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Prerequisite | L3-G-278 — Institutional Infrastructure Registration to Physical Campus Establishment | Establishes the G10 Physical Campus. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / G10 | Owns Physical Campus identity, physical hierarchy and physical lifecycle. |
| Institution & Structure / B03 | Owns InstitutionalCampus identity, institutional status and Institution-to-Campus governance association. |

The frozen participating-domain value remains exactly `G, B`.

---

# 6. Business Actors, Roles and Responsibilities

The locked sources identify Infrastructure Administrator, Institution Administrator and other scoped administrative roles, but do not assign a canonical actor to establish or replace this cross-domain association. No new association authority is inferred.

---

# 7. Preconditions and Entry Criteria

- A canonical InstitutionalCampus exists in B03.
- A canonical Physical Campus exists in G10.
- Both records resolve within a consistent Tenant and Institution context.
- Neither domain has transferred ownership of its aggregate to the other.

---

# 8. Trigger

An InstitutionalCampus requires its corresponding physical infrastructure representation.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Resolve the authoritative InstitutionalCampus identity. | Institution & Structure | The institutional identity is known. |
| 2 | Resolve the authoritative Physical Campus identity. | Infrastructure Management | The physical identity is known. |
| 3 | Validate that both identities belong to the intended institutional context. | B03; G10 | Cross-domain consistency is established. |
| 4 | Determine the authoritative reference direction and contract. | B03; G10 | **Blocked:** locked sources and frozen outcome conflict on this point. |
| 5 | Establish only the source-authorized reference without duplicating either aggregate. | B03; G10 | Cannot proceed canonically until Step 4 is resolved. |
| 6 | Preserve the association evidence and make the permitted reference available to authorized consumers. | Owning domain(s) defined upstream | Completion depends on upstream resolution. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome |
|---|---|---|---|
| Physical Campus absent | G10 representation does not yet exist. | Complete the Physical Campus establishment scenario first. | No shadow physical record is created in B03. |
| Cross-scope mismatch | Identities resolve to incompatible institutional contexts. | Reject the association. | Existing ownership remains unchanged. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Reference direction is ambiguous. | The canonical relationship cannot be established safely. | Stop and resolve the locked architecture conflict. | Architecture governance |
| One aggregate is missing. | The association cannot complete. | Establish the missing aggregate under its canonical owner. | B03 or G10 as applicable |
| Proposed association would duplicate ownership. | Canonical ownership would be violated. | Reject the proposal. | B03; G10 |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| InstitutionalCampus identity and institutional governance are owned by B03. | Institution & Structure | G10 may not own or redefine the institutional aggregate. | L2-B03 §§1, 14-15 |
| Physical Campus and physical infrastructure are owned by G10. | Infrastructure Management | B03 may not own or redefine physical aggregates. | L2-G10 §§1-3, 8 |
| B03 states that G10 PhysicalInfrastructureCampus references the InstitutionalCampus identity issued by B03. | B03 / G10 | This is the locked relationship direction currently documented. | L2-B03 §1 ownership split; §§30-31 |
| Cross-domain collaboration must not transfer ownership. | Enterprise architecture | The relationship must remain reference-only. | DEPENDENCY_MAP; validation standards |

---

# 13. State and Lifecycle Impact

No lifecycle transition is required by the frozen outcome. Both aggregates remain in their independently governed lifecycles. No association lifecycle is defined upstream.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

No canonical association event is defined in the locked B03 or G10 event catalogues. No event is invented.

## 14.2 Business Contracts and Cross-Domain Hand-offs

B03 exposes the InstitutionalCampus Summary Contract and G10 exposes the Physical Campus Summary Contract. The locked sources do not define a dedicated association contract that resolves the frozen outcome's opposite reference direction.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

Any association action must be authorized for both institutional and physical scope under existing upstream policy.

## 15.2 Approval and Separation of Duty

No canonical association approver or replacement authority is defined. None is inferred.

## 15.3 Business Audit and Evidence Obligations

Any future canonical association must preserve both identifiers, institutional context, actor, decision authority, effective date if applicable, and immutable history.

---

# 16. Completion Criteria and Postconditions

Completion requires an upstream decision that:

- resolves the reference direction between InstitutionalCampus and Physical Campus;
- defines the authoritative business contract or relationship owner;
- preserves B03 institutional ownership and G10 physical ownership; and
- defines any required event and authorization authority.

These conditions are not currently satisfied.

---

# 17. Open Issues, Assumptions and Deferred Questions

| ID | Severity | Issue / Deferred Question | Required Resolution |
|---|---|---|---|
| G-279-BLOCKER-01 | Blocking | Frozen outcome says the InstitutionalCampus can reference the Physical Campus, while locked B03 states the G10 PhysicalInfrastructureCampus references the B03 InstitutionalCampus identity. | Architecture change control must resolve the canonical relationship direction without transferring ownership. |
| G-279-BLOCKER-02 | Blocking | No dedicated cross-domain association workflow, contract, or canonical association event is defined. | B03/G10 must define the permissible hand-off/contract and any event if required. |
| G-279-BLOCKER-03 | Blocking | No business authority is assigned to create or replace the association. | Upstream architecture must define the actor/authority and constraints. |

**Source Assessment:** **BLOCKED**

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
