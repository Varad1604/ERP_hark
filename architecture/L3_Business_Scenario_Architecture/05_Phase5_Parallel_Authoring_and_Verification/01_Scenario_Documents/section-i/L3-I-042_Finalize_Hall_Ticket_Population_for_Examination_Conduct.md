# Finalize Hall Ticket Population for Examination Conduct

**Scenario ID:** L3-I-042  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Hall Ticket Management  
**Participating Domains:** C — Student Lifecycle; I — Examination  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2-I02 — Examination Session Management Workflows; L2-I03 — Examination Planning & Scheduling Workflows; L2-I04 — Examination Registration Workflows; L2-I05 — Hall Ticket Management Workflows  
**Referenced By:** L2-I06 — Practical & Viva Examination Conduct; L2-I07 — Theory Examination Conduct and subsequent examination conduct workflows  

## 1. Scenario Metadata

The frozen Stage 3 canonical row defines this scenario as a Critical P0 Hall Ticket Management scenario involving Student Lifecycle and Examination. The exact Phase 5 filename is `L3-I-042_Finalize_Hall_Ticket_Population_for_Examination_Conduct.md`.

## 2. Scenario Purpose

Establish the authoritative population of valid Hall Tickets that may be consumed by examination conduct workflows for the applicable Examination Session.

The scenario finalizes the examination-entry authorization boundary without redefining Hall Ticket lifecycle ownership or creating a new Hall Ticket state.

## 3. Business Outcome

A final authoritative Hall Ticket population is established for examination conduct, containing only candidates with valid examination-entry authorization for the applicable Examination Session.

## 4. Scenario Boundary and Scope

**Start Boundary:** Hall Ticket issuance and required review have reached the institutional point at which the population of candidates authorised for examination conduct must be finalized.

**In Scope:**
- identify the Examination Session and relevant examination population;
- evaluate Hall Ticket lifecycle and validity;
- exclude withdrawn, cancelled, invalid or otherwise non-authorising Hall Tickets;
- resolve outstanding duplicate/conflicting Hall Ticket positions;
- establish the final valid Hall Ticket population;
- preserve audit evidence;
- make the population available to examination conduct workflows.

**Out of Scope:**
- creating or modifying Examination Registrations;
- creating or changing Examination Schedules;
- conducting examinations;
- attendance capture;
- marks or results;
- technical batch processing, queues or database operations;
- redefining Hall Ticket policy.

**End Boundary:** Examination conduct workflows have one authoritative business population of valid Hall Tickets for the applicable Examination Session.

## 5. Ownership and Participating Domains

**Hall Ticket Management:** owns Hall Ticket validity and the final Hall Ticket population used for examination-entry authorization.

**Student Lifecycle:** provides authoritative candidate context where relevant but does not own Hall Ticket lifecycle.

**Examination Registration:** remains the upstream owner of registration authority. Hall Ticket population finalization consumes registration consequences rather than redefining them.

Downstream examination conduct workflows consume the final population without becoming Hall Ticket owners.

## 6. Business Actors, Roles and Responsibilities

| Actor / Domain | Responsibility |
|---|---|
| Examination Administrator | Reviews and finalizes the Hall Ticket population |
| Controller of Examination | Provides examination governance and approval where required |
| Candidate (Student) | Is represented in the final population only when holding valid examination-entry authorization |
| Examination Registration Domain | Provides the authoritative registration context |
| Examination Planning & Scheduling | Provides the governing Examination Session and published schedule |
| Examination Conduct Workflows | Consume the final valid Hall Ticket population |
| Audit Infrastructure | Preserves finalization and population evidence |

## 7. Preconditions and Entry Criteria

- The applicable Examination Session is established.
- The Examination Schedule has been Published.
- Hall Ticket eligibility processing has been completed to the applicable business point.
- Valid Hall Tickets have been generated and reviewed according to L2-I05.
- Withdrawn, cancelled or otherwise invalid Hall Tickets can be identified.
- Duplicate/conflicting Hall Ticket positions have been resolved or are explicitly excluded from the final valid population.
- The examination conduct boundary requires a definitive valid Hall Ticket population.

## 8. Trigger

The institution reaches the governed point at which Hall Ticket eligibility and issuance must be finalized for examination conduct.

## 9. Main Business Flow

1. Identify the Examination Session and the examination population for which Hall Tickets are being finalized.
2. Identify all Hall Tickets associated with the applicable candidate population.
3. Evaluate each Hall Ticket against its current L2-I05 lifecycle state and validity.
4. Include Hall Tickets that are valid for examination entry.
5. Exclude Hall Tickets that are Withdrawn, Cancelled, superseded by a valid Replacement, or otherwise not valid for examination entry.
6. Resolve any remaining duplicate or conflicting Hall Ticket positions before final population establishment.
7. Confirm that each included candidate has a single valid Hall Ticket for the applicable Examination Session.
8. Establish the final authoritative Hall Ticket population for examination conduct.
9. Preserve the population decision and relevant audit evidence.
10. Publish the applicable Hall Ticket lifecycle events already defined by L2-I05 for any final business transitions occurring as part of the finalization.
11. Make the final population available to the applicable examination conduct workflows.

The Main Business Flow ends when the authoritative valid Hall Ticket population is established and handed to examination conduct.

## 10. Alternate Business Flows

### 10.1 No Eligible Hall Tickets

If no Hall Tickets are valid for the applicable examination population, the final population is empty. No candidate is represented as examination-entry authorized without a valid Hall Ticket.

### 10.2 Late or Exceptional Hall Ticket

If an institutionally authorised late or exceptional Hall Ticket reaches a valid lifecycle position before the population is finalized, it is included subject to the applicable L2-I05 policy.

### 10.3 Hall Ticket Reissued Before Finalization

Where a Hall Ticket has been reissued, the currently valid Hall Ticket is included and the superseded version is not treated as a second operational authorization.

### 10.4 Hall Ticket Replaced Before Finalization

Where a Hall Ticket has been replaced, the replacement is included when valid; the replaced Hall Ticket is excluded from the operational population.

## 11. Failure, Interruption and Business Recovery

### 11.1 Hall Ticket Validity Cannot Be Determined

A Hall Ticket with unresolved validity is not silently included in the final authorized population. The business ambiguity must be resolved under the applicable Hall Ticket governance before inclusion.

### 11.2 Duplicate or Conflicting Hall Tickets Remain

The I041 duplicate/conflict resolution applies before the final population is established.

### 11.3 Registration Withdrawal Occurs

If an underlying Examination Registration is withdrawn before finalization, the associated Hall Ticket consequence is applied through L2-I04/L2-I05 and the affected Hall Ticket is excluded if it no longer authorizes entry.

### 11.4 Examination Schedule Changes

If the approved schedule changes before conduct and the change affects Hall Ticket validity or information, L2-I05 review/reissue/replacement rules apply. The final population is established only after the applicable Hall Ticket positions are valid.

### 11.5 Candidate Context Change

If authoritative Student Lifecycle information changes the candidate context relevant to Hall Ticket validity, the affected Hall Ticket is re-evaluated using the canonical upstream business outcome.

## 12. Business Rules and Decision Constraints

- A Hall Ticket represents institutional authorization for examination entry.
- Hall Tickets are generated only from approved Examination Registration and applicable published Examination Schedule information.
- A valid Hall Ticket belongs to one Candidate and one Examination Session.
- Only valid Hall Tickets may be consumed by examination conduct workflows.
- Withdrawn or cancelled Hall Tickets do not authorize examination participation.
- Reissued/replaced Hall Tickets preserve the underlying Examination Registration and must not create competing operational authorization.
- Hall Ticket Management owns the final Hall Ticket population.
- The final population does not create a new Hall Ticket lifecycle state.
- Downstream examination conduct workflows consume the final population and do not redefine Hall Ticket validity.

## 13. State and Lifecycle Impact

| Business Concept | Relevant States | Scenario Impact | Result |
|---|---|---|---|
| Hall Ticket | Approved, Published, Active, Reissued, Replaced, Withdrawn, Cancelled, Completed, Archived | Determines which current Hall Tickets are eligible for the final operational population. | Valid current Hall Tickets are included; non-authorising positions are excluded. |
| Examination Registration | Confirmed / Locked and other existing L2-I04 states | Provides the upstream participation authority reflected by Hall Ticket validity. | Registration lifecycle remains unchanged by population finalization. |

The phrase **final Hall Ticket population** is a scenario outcome, not a new Hall Ticket lifecycle state.

## 14. Business Events and Cross-Domain Collaboration

### Events

L2-I05 is the canonical publisher of Hall Ticket lifecycle events. Relevant existing events include:

- **Hall Ticket Generated**
- **Hall Ticket Reviewed**
- **Hall Ticket Published**
- **Hall Ticket Validated**
- **Hall Ticket Reissued**
- **Hall Ticket Replaced**
- **Hall Ticket Withdrawn**
- **Hall Ticket Cancelled**
- **Hall Ticket Archived**

Population finalization itself does not invent a `HallTicketPopulationFinalized` event because the locked L2-I05 event catalogue does not define one.

### Cross-Domain Collaboration

The final population consumes:

- Examination Registration authority from L2-I04;
- Examination Session and Schedule context from L2-I02/L2-I03;
- authoritative candidate context from Student Lifecycle where required.

Ownership remains with the source workflows.

The final population is a business hand-off to L2-I06/L2-I07 and does not transfer Hall Ticket ownership.

## 15. Business Authorization, Approval and Audit Obligations

- Finalization must be performed under the examination authority defined by L2-I05.
- Controller of Examination governance applies where required.
- The final population decision must be auditable.
- Exclusions from the population must remain traceable to the Hall Ticket business state or other authoritative business reason.
- Duplicate/conflicting Hall Ticket resolutions must remain traceable.
- Any exceptional late or post-finalization action must follow existing institutional authority; no new exception authority is introduced.

## 16. Completion Criteria and Postconditions

**Completion Criteria:**
- Examination Session and applicable schedule are established.
- All relevant Hall Tickets have been evaluated.
- Non-authorising Hall Tickets are excluded.
- Duplicate/conflicting operational authorizations have been resolved.
- Each included candidate has one valid Hall Ticket for the applicable Examination Session.
- The final valid Hall Ticket population is established.
- Required audit evidence is preserved.
- Applicable existing Hall Ticket lifecycle events have been published.
- Examination conduct workflows can consume the final valid population.

**Postconditions:**
- The institution has one authoritative population of candidates with valid Hall Ticket examination-entry authorization.
- No withdrawn, cancelled or superseded Hall Ticket independently authorizes examination conduct.
- Hall Ticket ownership remains with Hall Ticket Management.
- Downstream examination conduct begins from the finalized valid population.

## 17. Open Issues, Assumptions and Deferred Questions

1. L2-I05 defines the Hall Ticket completion criteria and states that valid Hall Ticket information must be available for downstream examination conduct workflows. This scenario uses that existing boundary without inventing a separate finalization state.
2. L2-I05's state model includes `Active`, `Reissued`, `Replaced`, `Withdrawn`, `Cancelled`, `Completed` and `Archived`; the scenario uses those existing states to determine inclusion/exclusion.
3. L2-I05 does not define a dedicated `HallTicketPopulationFinalized` event. No such event is introduced here.
4. The exact institutional cut-off for finalizing the population is policy/configuration governed. This scenario intentionally does not invent a date or deadline.
5. The Stage 3 frozen participating domains are C and I. Student Lifecycle is retained as a participating source of candidate context; Hall Ticket Management remains the canonical owner of the outcome.
