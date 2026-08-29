# Institutional Infrastructure Registration to Physical Campus Establishment

# 1. Scenario Metadata

**Scenario ID:** L3-G-278  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, B, A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-26  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-G10 Locked Amendment 2026-08-26; ADR-G-INF-REG-001; L2-B03 — Campus Workflows; L2-A01 — Platform & Tenant Foundation  
**Referenced By:** Infrastructure hierarchy and downstream physical-context scenarios

# 2. Scenario Purpose

Establish one canonical G10-owned Physical Campus for an authorised institution while preserving B03 ownership of InstitutionalCampus identity and institution-to-campus governance.

# 3. Business Outcome

A canonical Physical Campus is registered under G10 with immutable identity, required physical registration information, initial lifecycle state `Proposed`, and the canonical `CampusRegistered` business event, while institutional governance remains with B03.

# 4. Scenario Boundary and Scope

**Start:** An authorised institution requires a new physical infrastructure representation.  
**End:** The G10 Physical Campus exists in `Proposed`, references the authoritative B03 context, preserves registration evidence, and `CampusRegistered` has been published by G10.

In scope: institutional reference validation, physical-campus registration, uniqueness, immutable identity, initial lifecycle position, registration audit and event publication. Out of scope: B03 InstitutionalCampus creation, operational activation, lower physical hierarchy, maintenance, allocation, scheduling, UI/API/database or other implementation design.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| G10 Infrastructure Management | Owns Physical Campus identity, physical attributes, lifecycle and `CampusRegistered`. |
| B03 Institution & Structure | Owns InstitutionalCampus identity and Institution-to-Campus governance. |
| A Platform Foundation | Supplies governed authorization/configuration/audit capability without owning Physical Campus semantics. |

# 6. Business Actors, Roles and Responsibilities

- **Infrastructure Administrator:** initiates/administers permitted Physical Campus registration.
- **Institution Administrator:** supplies/validates institution-side governance context where required.
- **Auditor:** reviews immutable registration history in a read-only capacity.

# 7. Preconditions and Entry Criteria

- Authoritative Institution and InstitutionalCampus context exists.
- Physical Campus code satisfies G10 uniqueness rules.
- Required physical registration information is available.
- The requester is authorized for Infrastructure registration.
- No duplicate canonical Physical Campus represents the same intended site.

# 8. Trigger

An authorised institution requires a new physical infrastructure representation.

# 9. Main Business Flow

| Step | Behaviour | Responsible Domain / Role | Result |
|---|---|---|---|
| 1 | Resolve the authoritative Institution and InstitutionalCampus context. | B03; Infrastructure Administrator | Institutional context is confirmed without ownership transfer. |
| 2 | Validate requested Physical Campus code, name and required physical information. | G10 | Registration inputs satisfy G10 rules. |
| 3 | Establish the G10 Physical Campus aggregate with immutable identity. | G10 | Canonical Physical Campus exists. |
| 4 | Associate the G10 Physical Campus to the B03-owned institutional context by reference. | G10/B03 boundary | Institutional and physical ownership remain separate. |
| 5 | Set the canonical initial lifecycle state to `Proposed`. | G10 | Physical Campus is registered but non-operational. |
| 6 | Preserve registration evidence. | G10 | Registration is auditable. |
| 7 | Publish `CampusRegistered`. | G10 | The registration completion fact is available to authorized consumers. |

# 10. Alternate Business Flows

- If the intended physical representation already has a canonical G10 identity, reuse/reference it rather than creating a duplicate.
- Where institutional policy requires additional approval before registration, complete only that source-supported approval path before the G10 registration fact is established.

# 11. Failure, Interruption and Business Recovery

- Invalid B03 context prevents registration.
- Failed G10 uniqueness/validation leaves no new canonical Physical Campus.
- Incomplete required approval leaves registration incomplete.
- A failed attempt does not publish `CampusRegistered`.

# 12. Business Rules and Decision Constraints

- Physical Campus ownership belongs exclusively to G10.
- InstitutionalCampus identity and institutional governance remain in B03.
- Physical Campus code must be unique; aggregate identity is immutable.
- A newly registered Physical Campus starts in `Proposed`.
- Registration does not imply operational availability.
- Only `Operational` infrastructure may be consumed as operationally available under G10 rules.
- `CampusRegistered` is the sole canonical event for successful Physical Campus registration; `CampusCreated` is not a duplicate alias for this fact.

# 13. State and Lifecycle Impact

| Concept | Scenario Impact | Result | Owner |
|---|---|---|---|
| Physical Campus | Created/registered canonically | `Proposed` | G10 |
| InstitutionalCampus | Referenced only | Unchanged | B03 |

The downstream canonical Physical Campus progression begins `Proposed → Planned → Approved → Under Construction → Operational`.

# 14. Business Events and Cross-Domain Collaboration

| Event / Hand-off | Publisher / Provider | Consumer / Purpose |
|---|---|---|
| `CampusRegistered` | G10 | Signals successful Physical Campus registration. |
| InstitutionalCampus reference | B03 | Grounds G10 physical registration in authoritative institutional context. |
| Physical Campus summary/reference | G10 | Supplies physical identity/context without ownership transfer. |

# 15. Business Authorization, Approval and Audit Obligations

Infrastructure registration requires authorization for the applicable institutional and infrastructure scope. Audit evidence preserves initiating actor, B03 reference, Physical Campus identity, validated physical facts, approval reference where applicable, resulting `Proposed` state and `CampusRegistered` publication.

# 16. Completion Criteria and Postconditions

- B03 institutional context is valid.
- G10 Physical Campus is uniquely registered with immutable identity.
- Initial state is `Proposed` and therefore non-operational.
- Ownership remains separated between B03 institutional governance and G10 physical representation.
- Registration evidence is preserved.
- `CampusRegistered` is published by G10.

# 17. Open Issues, Assumptions and Deferred Questions

**Open Issues:** None.  
**Assumptions:** None.  
**Deferred Questions:** None.  
**Architecture assessment:** `PASS`
