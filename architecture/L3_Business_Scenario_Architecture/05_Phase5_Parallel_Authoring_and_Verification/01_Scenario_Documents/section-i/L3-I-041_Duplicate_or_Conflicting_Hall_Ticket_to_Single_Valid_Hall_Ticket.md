# Duplicate or Conflicting Hall Ticket to Single Valid Hall Ticket

**Scenario ID:** L3-I-041  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Hall Ticket Management  
**Participating Domains:** C — Student Lifecycle; I — Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2-I04 — Examination Registration Workflows; L2-I05 — Hall Ticket Management Workflows  
**Referenced By:** L2-I06 — Practical & Viva Examination Conduct; L2-I07 — Theory Examination Conduct and other downstream examination workflows consuming valid Hall Tickets  

## 1. Scenario Metadata

The frozen Stage 3 canonical row defines this scenario as a Critical P1 Hall Ticket Management scenario involving Student Lifecycle and Examination. The exact Phase 5 filename is `L3-I-041_Duplicate_or_Conflicting_Hall_Ticket_to_Single_Valid_Hall_Ticket.md`.

## 2. Scenario Purpose

Resolve duplicate or conflicting Hall Tickets for the same candidate and Examination Session so that only one Hall Ticket remains the authoritative valid examination-entry authorization for the applicable participation context.

The scenario preserves the Hall Ticket lifecycle and ownership defined by L2-I05 and does not alter the underlying Examination Registration.

## 3. Business Outcome

A single Hall Ticket is established as the authoritative valid examination-entry authorization for the candidate and applicable Examination Session; competing duplicate or conflicting Hall Ticket positions no longer authorize examination participation, while the resolution remains historically traceable.

## 4. Scenario Boundary and Scope

**Start Boundary:** Two or more Hall Ticket positions are identified as potentially representing the same candidate's examination-entry authorization for the same Examination Session.

**In Scope:**
- identification of the candidate and Examination Session;
- identification of the Hall Tickets and their current business positions;
- determination of whether the apparent duplication is genuine or represents a permitted reissue/replacement history;
- establishment of the authoritative operational Hall Ticket;
- withdrawal, replacement or cancellation of the non-authoritative operational position where supported by L2-I05;
- preservation of history and audit evidence;
- communication of the authoritative Hall Ticket outcome to downstream examination workflows.

**Out of Scope:**
- changing Examination Registration;
- redefining candidate identity ownership;
- creating new Hall Ticket policy;
- examination scheduling;
- examination conduct;
- technical duplicate detection;
- database or document implementation.

**End Boundary:** One Hall Ticket is authoritative for operational examination entry and no competing Hall Ticket independently authorizes the same examination participation.

## 5. Ownership and Participating Domains

**Hall Ticket Management:** owns Hall Ticket lifecycle, validity and examination-entry authorization.

**Student Lifecycle:** supplies authoritative candidate context where required but does not own Hall Ticket lifecycle.

**Examination Registration:** remains the authoritative source for the underlying registration; it is not transferred into Hall Ticket ownership.

A duplicate or conflict does not transfer ownership to the participating domain.

## 6. Business Actors, Roles and Responsibilities

| Actor / Domain | Responsibility |
|---|---|
| Examination Administrator | Reviews duplicate/conflicting Hall Tickets and establishes the operationally authoritative position |
| Controller of Examination | Provides examination governance for exceptional or material resolution |
| Candidate (Student) | Is the affected examination participant and may provide clarification where the governing process requires it |
| Examination Registration Domain | Provides the authoritative registration context against which Hall Ticket validity is understood |
| Student Lifecycle | Provides authoritative candidate context where relevant |
| Examination Conduct Workflows | Consume the single valid Hall Ticket outcome |
| Audit Infrastructure | Preserves the Hall Ticket resolution history |

No new actor, approval authority or technical role is introduced.

## 7. Preconditions and Entry Criteria

- An Examination Session exists and is applicable.
- A Hall Ticket has been generated or published for the candidate.
- A second Hall Ticket, conflicting Hall Ticket position, or duplicate operational authorization has been identified.
- The candidate and Examination Session can be unambiguously established.
- The underlying Examination Registration remains governed by L2-I04.
- Hall Ticket lifecycle governance remains governed by L2-I05.

## 8. Trigger

Duplicate or conflicting Hall Tickets are detected or reported for the same candidate and Examination Session.

## 9. Main Business Flow

1. Identify the candidate, Examination Session and all Hall Tickets relevant to the apparent duplication.
2. Establish the current business state and validity position of each Hall Ticket.
3. Determine whether multiple Hall Tickets represent a legitimate reissue or replacement history or an actual competing operational authorization.
4. If the positions are legitimate historical versions, identify the currently authoritative Hall Ticket under the L2-I05 lifecycle.
5. If there is a genuine competing operational authorization, determine which Hall Ticket is the authoritative valid Hall Ticket using the applicable registration, schedule and Hall Ticket business context.
6. Retain the authoritative Hall Ticket in its valid lifecycle position.
7. Apply the existing L2-I05 withdrawal, replacement or cancellation lifecycle to the non-authoritative operational Hall Ticket where applicable.
8. Preserve the resolution rationale and historical relationship between the Hall Tickets.
9. Publish the applicable canonical Hall Ticket lifecycle event(s).
10. Ensure downstream examination conduct workflows rely only on the authoritative valid Hall Ticket.

The flow ends once the operational ambiguity is removed and one valid Hall Ticket remains authoritative.

## 10. Alternate Business Flows

### 10.1 Legitimate Reissue

If the second Hall Ticket is a permitted reissue, it is not treated as an independent competing authorization. The current valid Hall Ticket is retained as authoritative and the superseded version remains historical according to the L2-I05 lifecycle.

### 10.2 Legitimate Replacement

If the second Hall Ticket is a permitted replacement, the replacement becomes the operational Hall Ticket and the replaced Hall Ticket no longer authorizes examination entry.

### 10.3 Duplicate Hall Ticket Before Publication

If duplicate Hall Tickets are identified before publication, the non-authoritative draft/review position does not proceed to valid publication. The authoritative Hall Ticket continues through the existing lifecycle.

## 11. Failure, Interruption and Business Recovery

### 11.1 Authoritative Hall Ticket Cannot Be Determined

If the available business evidence does not establish which Hall Ticket is authoritative, examination entry authorization remains unresolved rather than selecting an arbitrary Hall Ticket.

### 11.2 Candidate Context Is Conflicting

If candidate information is conflicting, Student Lifecycle remains authoritative for its own candidate context. Hall Ticket Management does not redefine identity ownership.

### 11.3 Underlying Registration Is Withdrawn

If the underlying Examination Registration is withdrawn, the registration-withdrawal consequence is applied through the L2-I04/L2-I05 relationship. This scenario does not override the registration lifecycle.

### 11.4 Duplicate Detected After Examination Conduct

If the duplicate is discovered after the examination session has concluded or conduct has already occurred, the historical and audit consequence is preserved and the Hall Ticket lifecycle is applied only to the extent permitted by L2-I05. This scenario does not rewrite completed examination conduct.

## 12. Business Rules and Decision Constraints

- L2-I05 permits institutional review when duplicate Hall Tickets are detected.
- Hall Tickets represent examination-entry authorization and must not independently contradict the authoritative Examination Registration.
- A Hall Ticket belongs to one Candidate and one Examination Session.
- Reissue and replacement preserve linkage to the underlying Examination Registration.
- A withdrawn or cancelled Hall Ticket must not authorize examination participation.
- Hall Ticket Management owns the Hall Ticket resolution; Student Lifecycle does not become Hall Ticket owner.
- No new duplicate-specific Hall Ticket state or event is introduced.

## 13. State and Lifecycle Impact

| Business Concept | Relevant States | Scenario Impact | Result |
|---|---|---|---|
| Hall Ticket | Draft, Under Review, Approved, Published, Active, Reissued, Replaced, Withdrawn, Cancelled, Completed, Archived | Establishes which Hall Ticket is operationally authoritative and removes competing authorization using existing lifecycle transitions. | One current valid Hall Ticket; competing operational position becomes Replaced, Withdrawn or Cancelled as applicable. |
| Examination Registration | Confirmed / Locked and other existing L2-I04 states | Provides authoritative registration context but is not changed merely because Hall Tickets conflict. | Existing registration lifecycle position remains authoritative. |

The scenario does not introduce a `Duplicate` or `Conflicting` Hall Ticket state.

## 14. Business Events and Cross-Domain Collaboration

### Events Published by Hall Ticket Management

Only existing L2-I05 event names are used:

- **Hall Ticket Generated**
- **Hall Ticket Reviewed**
- **Hall Ticket Published**
- **Hall Ticket Validated**
- **Hall Ticket Reissued**
- **Hall Ticket Replaced**
- **Hall Ticket Withdrawn**
- **Hall Ticket Cancelled**
- **Hall Ticket Archived**

The event published depends on the actual existing lifecycle transition applied to the non-authoritative Hall Ticket.

No `DuplicateHallTicketResolved` event is invented.

### Cross-Domain Collaboration

The Hall Ticket decision consumes the authoritative Examination Registration context and candidate information while preserving ownership. Downstream examination conduct workflows consume the resulting valid Hall Ticket position.

## 15. Business Authorization, Approval and Audit Obligations

- Duplicate/conflict resolution must be performed under the business authority defined for Hall Ticket Management.
- Exceptional decisions require the existing Controller of Examination or institutional authority where applicable.
- The authoritative Hall Ticket decision and the reason for disposition must be auditable.
- Historical Hall Ticket versions must remain traceable.
- The resolution must preserve the distinction between operational validity and historical records.
- No technical authorization or implementation mechanism is defined.

## 16. Completion Criteria and Postconditions

**Completion Criteria:**
- All relevant Hall Tickets are identified.
- Legitimate reissue/replacement history is distinguished from actual conflicting authorization.
- One Hall Ticket is established as operationally authoritative.
- Any competing operational Hall Ticket is withdrawn, replaced or cancelled through the existing lifecycle where applicable.
- The resolution is auditable.
- Applicable Hall Ticket lifecycle events are published.
- Downstream examination workflows have one valid Hall Ticket authority.

**Postconditions:**
- Exactly one Hall Ticket is the current valid examination-entry authorization for the affected candidate and Examination Session.
- Historical Hall Ticket positions remain traceable.
- The underlying Examination Registration remains unchanged unless its own lifecycle independently changes.

## 17. Open Issues, Assumptions and Deferred Questions

1. L2-I05 explicitly states that duplicate Hall Tickets require institutional review to determine the authoritative Hall Ticket. This scenario applies that rule without inventing a duplicate-specific state or event.
2. L2-I05 distinguishes Reissue, Replacement, Withdrawal and Cancellation. The scenario therefore does not collapse all duplicate outcomes into a new generic `Invalidated` state.
3. The Stage 3 frozen participating domains are C and I. Student Lifecycle participation is limited to authoritative candidate context; Hall Ticket ownership remains with I.
4. If a later locked upstream decision defines a specific duplicate-resolution transition or event, that definition takes precedence through controlled architecture change.
