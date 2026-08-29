# Institutional Campus Reference to Physical Campus Association

# 1. Scenario Metadata

**Scenario ID:** L3-G-279  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, B  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-B03 — Campus Workflows; `L2-G10_Infrastructure_Governance_Completion_Amendment_2026-08-26.md`  
**Referenced By:** Cross-domain consumers resolving institutional and physical campus context; L3 targeted revalidation and Architecture Approval review

---

# 2. Scenario Purpose

Preserve the canonical association boundary between B03 InstitutionalCampus identity and the G10 Physical Campus without duplicating either aggregate or transferring ownership.

---

# 3. Business Outcome

The institutional campus can reference the canonical Physical Campus without duplicating physical ownership.

The frozen outcome is satisfied through the locked G10 association rule: G10 persists the Physical Campus → B03 `InstitutionalCampus` reference, while B03 and other authorized consumers may reverse-resolve the corresponding Physical Campus through the G10 Physical Campus Summary Contract. B03 does not persist or own a duplicate physical association.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an InstitutionalCampus requires its corresponding physical infrastructure representation.

## 4.2 End Boundary

The scenario ends when the canonical G10 Physical Campus carries the valid B03 `InstitutionalCampus` reference, both identities remain independently owned, and the corresponding Physical Campus can be resolved by authorized consumers without a duplicate B03-owned association.

## 4.3 In Scope

- Resolution of the B03 `InstitutionalCampus` identity.
- Resolution of the G10 Physical Campus identity.
- Validation of Tenant and Institution consistency.
- Establishment or governed replacement of the G10-owned Physical Campus → InstitutionalCampus reference.
- Reverse resolution through the G10 Physical Campus Summary Contract.
- Audit evidence for association establishment or reassociation.

## 4.4 Out of Scope

- Creation of either aggregate.
- Copying Physical Campus facts into B03.
- Transferring InstitutionalCampus ownership into G10.
- Re-parenting, lifecycle change, operational activation, scheduling, or maintenance.

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / G10 | Owns Physical Campus identity, physical hierarchy, physical lifecycle and the persisted Physical Campus → InstitutionalCampus reference. |
| Institution & Structure / B03 | Owns `InstitutionalCampus` identity, institutional status and Institution-to-Campus governance. |

Reverse resolution does not create duplicate ownership.

---

# 6. Business Actors, Roles and Responsibilities

| Actor | Responsibility |
|---|---|
| Infrastructure Administrator | Establishes or replaces the Physical Campus → InstitutionalCampus reference where infrastructure administration is permitted. |
| Institution Administrator | Ensures that the referenced InstitutionalCampus is valid for the institution and institutional context. |

Neither actor may transfer aggregate ownership.

---

# 7. Preconditions and Entry Criteria

- A canonical `InstitutionalCampus` exists in B03.
- A canonical Physical Campus exists in G10.
- Both records resolve within the intended institutional context.
- The acting authorities are valid for their respective scopes.

---

# 8. Trigger

An InstitutionalCampus requires its corresponding physical infrastructure representation.

---

# 9. Main Business Flow

1. Resolve the authoritative B03 `InstitutionalCampus` identity.
2. Resolve the authoritative G10 Physical Campus identity.
3. Validate that the identities belong to the intended institutional context.
4. Infrastructure Administrator establishes the G10 Physical Campus reference to the B03 `InstitutionalCampus` identity, with Institution Administrator ensuring the referenced institutional context is valid.
5. Preserve the association without creating any duplicate B03-owned Physical Campus relationship.
6. Make the corresponding Physical Campus resolvable to authorized B03/consumer use through the G10 Physical Campus Summary Contract.
7. If an existing established reference is later corrected or reassociated, G10 publishes `PhysicalCampusInstitutionalReferenceChanged`.

---

# 10. Alternate Business Flows

- If no Physical Campus exists, complete the canonical G10 establishment scenario first.
- If the proposed references are institutionally inconsistent, reject the association.
- If a correction/reassociation is required, preserve prior association evidence and apply the governed G10 reference change.

---

# 11. Failure, Interruption and Business Recovery

Invalid or missing identities prevent association. A proposed cross-domain relationship that would duplicate ownership is rejected. Reassociation preserves history and is represented by the G10-owned change fact rather than by rewriting B03 ownership.

---

# 12. Business Rules and Decision Constraints

- B03 remains sole owner of `InstitutionalCampus` identity and institutional status.
- G10 remains sole owner of Physical Campus identity, physical hierarchy and physical lifecycle.
- The persisted cross-domain association is carried by G10 Physical Campus as a reference to B03 `InstitutionalCampus`.
- Reverse resolution through the G10 Physical Campus Summary Contract is read/reference capability only.
- Initial Physical Campus establishment does not require a duplicate association event merely because the institutional reference is established at creation.

---

# 13. State and Lifecycle Impact

No lifecycle transition is required solely because the cross-domain reference is established or corrected. Both aggregates remain within their independently governed lifecycles.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

- `PhysicalCampusInstitutionalReferenceChanged` — published only by G10 when an established Physical Campus changes its referenced B03 `InstitutionalCampus` identity through governed correction/reassociation.

No duplicate initial-association event is introduced.

## 14.2 Business Contracts and Cross-Domain Hand-offs

G10 Physical Campus Summary Contract supports authorized reverse resolution using the B03 `InstitutionalCampus` identity as governing reference. B03 does not persist a duplicate Physical Campus association.

---

# 15. Business Authorization, Approval and Audit Obligations

Infrastructure Administrator governs the physical reference action; Institution Administrator validates the institutional reference context. Evidence preserves both canonical identifiers, prior/new association where changed, actor/authority and institutional context.

---

# 16. Completion Criteria and Postconditions

Completion requires:

- G10 Physical Campus references the correct B03 `InstitutionalCampus` identity;
- B03 and G10 ownership remain unchanged;
- authorized consumers can resolve the corresponding Physical Campus through the G10 summary contract; and
- any later reassociation preserves history and publishes the exact G10 event.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Resolved Historical Blocker

The former ambiguity over reference direction, reverse resolution, association authority and change event was resolved by locked `L2-G10_Infrastructure_Governance_Completion_Amendment_2026-08-26.md` §§1 and 7.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
