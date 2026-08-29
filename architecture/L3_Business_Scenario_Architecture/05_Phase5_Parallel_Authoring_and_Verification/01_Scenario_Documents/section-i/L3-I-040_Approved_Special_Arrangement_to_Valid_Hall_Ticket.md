# Approved Special Arrangement to Valid Hall Ticket

**Scenario ID:** L3-I-040  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Hall Ticket Management  
**Participating Domains:** C — Student Lifecycle; I — Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Author:** Varad  
**Last Updated:** 2026-08-21  

## 1. Scenario Metadata

| Item | Value |
|---|---|
| Final L3 ID | L3-I-040 |
| Canonical Title | Approved Special Arrangement to Valid Hall Ticket |
| Primary Owner | Hall Ticket Management |
| Participating Domains | C, I |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Locked |
| Governing L2 | L2-I05 — Hall Ticket Management Workflows |
| Related L2 | L2-I04 — Examination Registration Workflows; applicable Student Lifecycle workflow defining the approved candidate arrangement |

The frozen Stage 3 title, owner, participating domains, Type and Priority are preserved.

## 2. Scenario Purpose

Reflect an already approved candidate-specific special examination arrangement in a valid Hall Ticket without transferring ownership of the underlying student arrangement or Examination Registration.

The Hall Ticket becomes an accurate expression of the authorised examination-entry context.

## 3. Business Outcome

A candidate with an authoritative approved special examination arrangement receives a valid Hall Ticket that reflects the approved arrangement while preserving the candidate's existing Examination Registration and Hall Ticket ownership boundaries.

## 4. Scenario Boundary and Scope

**Start Boundary:** An approved special examination arrangement exists for a candidate who has a valid Examination Registration and is eligible for Hall Ticket processing.

**In Scope:**
- identify the candidate and Examination Registration;
- establish the authoritative approved arrangement;
- validate that the arrangement is applicable to the examination context;
- incorporate the approved arrangement into Hall Ticket processing;
- review, publish or replace the Hall Ticket as required by the existing L2-I05 lifecycle;
- preserve audit evidence.

**Out of Scope:**
- deciding or approving the special arrangement;
- redefining Student Lifecycle rules;
- modifying Examination Registration;
- changing Examination Policy;
- examination scheduling;
- examination conduct;
- candidate identity ownership;
- technical document formatting or delivery.

**End Boundary:** A valid Hall Ticket exists and accurately reflects the approved special arrangement for the candidate's examination participation.

## 5. Ownership and Participating Domains

**Hall Ticket Management (I):** owns Hall Ticket generation, publication and examination entry authorization.

**Student Lifecycle (C):** owns the authoritative candidate/student context and any approved upstream arrangement that belongs to that domain.

The special arrangement is consumed as an authoritative business input. Hall Ticket Management does not become owner of the underlying arrangement.

## 6. Business Actors, Roles and Responsibilities

| Actor / Domain | Responsibility |
|---|---|
| Examination Administrator | Ensures the Hall Ticket reflects the approved arrangement |
| Controller of Examination | Governs examination entry authorization and applicable Hall Ticket decisions |
| Candidate (Student) | Receives and uses the Hall Ticket reflecting the approved arrangement |
| Student Lifecycle / Academic Administration | Provides the authoritative approved candidate arrangement where owned there |
| Examination Registration Domain | Provides the valid Examination Registration |
| Audit Infrastructure | Preserves the arrangement-to-Hall-Ticket decision history |

The scenario does not create a new actor or technical role for special arrangements.

## 7. Preconditions and Entry Criteria

- A valid Examination Registration exists.
- The Examination Session is applicable and Active where required by L2-I05.
- The Examination Schedule is Published where required for Hall Ticket issuance.
- An approved special examination arrangement exists and is authoritative.
- The arrangement can be associated unambiguously with the candidate and examination context.
- The arrangement is within the scope permitted by the applicable examination policy.

## 8. Trigger

An approved special examination arrangement becomes available for a candidate who requires a Hall Ticket reflecting that arrangement.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the candidate's valid Examination Registration and examination context. | Hall Ticket Management | Affected Hall Ticket context is established. |
| 2 | Establish the authoritative approved special arrangement and its applicability to the candidate/examination context. | Student Lifecycle / owning source domain | Approved arrangement is confirmed as a business input. |
| 3 | Validate Hall Ticket prerequisites under L2-I05, including the applicable registration, session and schedule conditions. | Hall Ticket Management | Candidate remains eligible for Hall Ticket processing. |
| 4 | Incorporate the approved arrangement into the Hall Ticket's authorised examination-entry information. | Hall Ticket Management | Hall Ticket accurately reflects the approved arrangement. |
| 5 | Complete administrative review where required by the Hall Ticket policy. | Examination Administration | Hall Ticket is approved for publication where review is required. |
| 6 | Publish or replace the Hall Ticket through the existing L2-I05 lifecycle. | Hall Ticket Management | Candidate receives a valid Hall Ticket reflecting the approved arrangement. |
| 7 | Preserve the arrangement-to-Hall-Ticket relationship and applicable audit evidence. | Hall Ticket Management / Audit Infrastructure | The business outcome is traceable. |

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Arrangement approved before Hall Ticket generation | Arrangement is available before Hall Ticket creation. | Include the approved arrangement during initial Hall Ticket generation. | Newly issued Hall Ticket reflects the arrangement. | Hall Ticket Management |
| Arrangement approved after Hall Ticket publication | A valid Hall Ticket already exists. | Review the existing Hall Ticket and use the permitted replacement/reissue lifecycle where the arrangement requires a changed Hall Ticket. | A valid successor Hall Ticket reflects the arrangement. | Hall Ticket Management |
| Arrangement does not affect Hall Ticket content | Approved arrangement does not require a change to entry-authorisation information. | Preserve the existing valid Hall Ticket while retaining the approved arrangement in its owning process. | Existing Hall Ticket remains valid. | Hall Ticket Management + owning domain |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Special arrangement is not authoritative or approved | Hall Ticket cannot rely on the arrangement. | Do not apply the arrangement until the owning process establishes an approved business outcome. | C / owning domain |
| Arrangement cannot be associated with the candidate/examination context | Hall Ticket cannot safely reflect the arrangement. | Resolve the business association through the owning process before Hall Ticket update. | Student Lifecycle / Examination |
| Examination Registration is not confirmed | Hall Ticket issuance cannot proceed. | Resolve registration through L2-I04; do not bypass the registration prerequisite. | Examination Registration |
| Examination Schedule is not Published | Valid Hall Ticket cannot be issued for the scheduled examination context. | Continue only when the existing L2-I05 schedule prerequisite is satisfied. | Examination Planning / Hall Ticket Management |
| Existing Hall Ticket must change after publication | Current Hall Ticket no longer fully represents the authorised examination context. | Apply the existing Hall Ticket reissue or replacement lifecycle. | Hall Ticket Management |

No technical recovery mechanism is specified.

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Hall Tickets are derived from approved Examination Registration and applicable examination schedule information. | Hall Ticket Management | Arrangement is applied without changing registration authority. | L2-I05 §4.4, §7.3 |
| Approved special arrangements may affect Hall Ticket validity/content where applicable. | Hall Ticket Management / owning upstream arrangement process | Hall Ticket must reflect the authorised business outcome. | L2-I05 §7.5 and special-arrangement scenario boundary |
| Hall Ticket generation requires confirmed Examination Registration. | Examination Registration / Hall Ticket Management | Unconfirmed registration prevents issuance. | L2-I05 §4.1, §9.1 |
| Hall Ticket generation requires an Active Examination Session and Published Examination Schedule. | Examination Session / Planning | Required examination context must exist before issuance. | L2-I05 §4.1, §9.1 |
| Hall Ticket Management does not modify Examination Registration. | Hall Ticket Management | Arrangement application cannot change registration ownership. | L2-I05 §4.4 |

The underlying approval criteria for the special arrangement remain upstream-owned. No new arrangement eligibility rule is introduced.

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Hall Ticket | Draft / Under Review, or Published / Active when a permitted change occurs | Approved arrangement is incorporated through normal generation, reissue or replacement. | Approved → Published → Active, or existing Published/Active Hall Ticket replaced/reissued as applicable | Hall Ticket Management | L2-I05 §20 |
| Examination Registration | Confirmed / Locked as applicable | Arrangement is reflected without changing registration. | Existing registration state remains unchanged. | Examination Registration | L2-I04 |

No new “Special Arrangement” Hall Ticket state is created.

## 14. Business Events and Cross-Domain Collaboration

### Events

L2-I05 is the canonical publisher of Hall Ticket lifecycle events relevant to this scenario:

- **Hall Ticket Generated**
- **Hall Ticket Reviewed**
- **Hall Ticket Published**
- **Hall Ticket Reissued**
- **Hall Ticket Replaced**

Where the existing Hall Ticket must be changed after publication, the appropriate event is determined by the L2-I05 lifecycle.

The available L2-I05 event catalogue does not define a dedicated special-arrangement event; none is invented here.

### Cross-Domain Collaboration

The approved arrangement is consumed from its canonical owning process/domain. Student Lifecycle participation does not transfer Hall Ticket ownership.

Examination Registration remains the authoritative source for the candidate's registration position.

Where an immediate authoritative answer is required, the repository's existing synchronous business-contract pattern applies without defining a new contract name in L3.

## 15. Business Authorization, Approval and Audit Obligations

- The special arrangement must already be approved by its authoritative business process before Hall Ticket Management relies upon it.
- Examination Administration and Controller of Examination authority apply according to L2-I05.
- If a published Hall Ticket must be replaced or reissued, the applicable institutional authority must be followed.
- The arrangement reflected on the Hall Ticket must be traceable to the approved business decision.
- Hall Ticket generation, review, publication, reissue or replacement must remain auditable.
- No technical access control/IAM design is introduced.

## 16. Completion Criteria and Postconditions

**Completion Criteria:**
- Candidate and valid Examination Registration are identified.
- Approved special arrangement is authoritative and applicable.
- Hall Ticket prerequisites are satisfied.
- Hall Ticket accurately reflects the approved arrangement where the arrangement affects examination-entry information.
- Required review is completed.
- Hall Ticket is Published and valid, or an existing Hall Ticket is appropriately reissued/replaced.
- Relevant Hall Ticket lifecycle event(s) are published.
- Audit evidence is complete.

**Postconditions:**
- Candidate has a valid Hall Ticket reflecting the approved arrangement where applicable.
- Examination Registration remains unchanged and owned by L2-I04.
- The underlying special arrangement remains owned by its canonical source.
- Examination conduct workflows can consume the resulting valid Hall Ticket.

## 17. Open Issues, Assumptions and Deferred Questions

1. The Stage 3 participating-domain set is frozen as **C, I**. This scenario treats Student Lifecycle as the authoritative source where the approved special arrangement belongs there, without assuming that all special arrangements are owned by C.
2. L2-I05 supports special arrangements as part of Hall Ticket eligibility/processing, but the available locked L2 material does not define a separate canonical special-arrangement lifecycle or event publisher. This scenario therefore consumes the approved arrangement without inventing a new state, event or ownership model.
3. If a specific upstream L2 document defines a different canonical owner for a particular class of special arrangement, that owner remains authoritative for that class.
4. The precise institutional content that must appear on a Hall Ticket is configurable; this scenario defines only the business requirement that an applicable approved arrangement be accurately reflected.
