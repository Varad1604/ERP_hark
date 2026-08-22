# Finalize Examination Registration Population for Downstream Examination Processing

**Scenario ID:** L3-I-036  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Examination  
**Participating Domains:** C — Student Lifecycle, I — Examination  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-I02 Examination Session Management Workflows; L2-I03 Examination Planning & Scheduling Workflows; L2-I04 Examination Registration Workflows  
**Referenced By:** L2-I05 Hall Ticket Management; L2-I06 Practical & Viva Examination Conduct; L2-I07 Theory Examination Conduct; L2-I08 Marks Collection & Evaluation; L2-I09 Result Processing; L2-I10 Result Publication and other downstream Examination workflows

## 2. Scenario Purpose

Establish the authoritative Examination Registration population that downstream examination processing may consume after registration governance for the applicable examination cycle has reached its final permitted business position.

The scenario closes the registration population boundary without redesigning the registration lifecycle or transferring ownership to downstream examination workflows.

## 3. Business Outcome

A final, authoritative population of valid Examination Registrations is established for downstream examination processing, with registrations that are withdrawn, rejected, invalid or otherwise not eligible for downstream consumption excluded according to the governing registration rules.

## 4. Scenario Boundary and Scope

**Start Boundary:** The applicable examination registration window and permitted registration activities have reached the point at which the candidate population must be finalised for downstream examination processing.

**In Scope:** identification of the applicable Examination Session and Schedule; evaluation of current registration positions; application of confirmed/locked registration status; treatment of withdrawals and other invalid positions; establishment of the final population; audit; and publication of applicable registration lifecycle events.

**Out of Scope:** Hall Ticket generation; examination conduct; marks collection; result processing; changes to Examination Policy; changes to Student Lifecycle status; and technical batch, queue, database or infrastructure mechanisms.

**End Boundary:** The authoritative registration population is finalised and is available for downstream examination workflows.

## 5. Ownership and Participating Domains

**Examination:** owns Examination Registration and the final registration population.

**Student Lifecycle:** provides authoritative candidate/student context relevant to registration validity.

No downstream workflow becomes owner of Examination Registration merely because it consumes the final population.

## 6. Business Actors, Roles and Responsibilities

- **Examination Administrator:** reviews and finalises the registration population under the applicable examination process.
- **Controller of Examination:** provides institutional examination governance and required approval.
- **Candidate (Student):** is represented in the population only through the candidate's valid Examination Registration.
- **Student Lifecycle:** supplies authoritative candidate status where required.
- **Downstream Examination Workflows:** consume the final registration population without owning its lifecycle.

## 7. Preconditions and Entry Criteria

- An Examination Session is Active and applicable to the registration population.
- An approved Examination Schedule has been published.
- The applicable registration window has reached its governed finalisation point.
- Registration eligibility and required approvals have been completed for registrations intended for downstream consumption.
- Registration amendments and withdrawals are subject to the applicable L2-I04 cut-off and exception rules.
- The Examination Registration lifecycle remains the authoritative source of registration validity.

## 8. Trigger

The examination registration population reaches the governed point at which it must be finalised for downstream examination processing.

## 9. Main Business Flow

1. Identify the Examination Session and approved Examination Schedule for which the registration population is being finalised.
2. Identify all current Examination Registrations associated with the applicable examination population.
3. Evaluate each registration against its current L2-I04 business state and applicable eligibility/approval position.
4. Exclude registrations that are rejected, withdrawn or otherwise not valid for downstream examination participation.
5. Ensure registrations requiring final permitted amendment or approval are resolved before the population is finalised.
6. Establish the resulting population of valid confirmed/locked Examination Registrations.
7. Apply the L2-I04 registration-locking lifecycle where the registration population must be frozen.
8. Record the finalisation decision and preserve the population's audit evidence.
9. Publish the applicable canonical registration lifecycle events, including **Registration Locked** where locking occurs.
10. Make the final population available as the authoritative business input to downstream examination workflows.

## 10. Alternate Business Flows

### 10.1 No Eligible Registrations

If no registrations remain valid for the applicable examination population, the final population is established as empty and the downstream business consequence follows institutional examination governance. No artificial registration is created.

### 10.2 Registration Remains Pending

A registration that has not reached the required valid position is not included in the final population unless the applicable policy explicitly permits it.

### 10.3 Late or Exceptional Registration

A late or exceptional registration may enter the final population only if the applicable L2-I04 policy and authorised approval permit it before the relevant downstream population is finalised.

### 10.4 Withdrawal Before Finalisation

A registration withdrawn before finalisation is excluded from the final population.

### 10.5 Legitimate Registration Amendment Before Finalisation

An authorised amendment is reflected in the final population if the registration reaches the required valid state before finalisation.

## 11. Failure, Interruption and Business Recovery

### 11.1 Registration Population Cannot Be Determined

If the current validity of a registration cannot be established, that registration is not silently included as valid. Examination governance determines the required business resolution before finalisation.

### 11.2 Conflicting Registration Positions

If duplicate or conflicting registrations exist, the applicable duplicate/conflicting registration scenario is resolved before finalisation. The final population contains only the authoritative valid registration position.

### 11.3 Upstream Student Status Conflict

If Student Lifecycle status conflicts with the registration eligibility position, Student Lifecycle remains authoritative for its own fact and the affected registration is not treated as final until the business consequence is established.

### 11.4 Finalisation Requires Exceptional Change

If a required change arises after registration locking, it follows the existing exceptional institutional process. This scenario does not invent a post-lock mechanism.

### 11.5 Downstream Consumer Is Not Ready

The registration population remains an Examination business outcome. Downstream readiness does not change the registration lifecycle or ownership.

## 12. Business Rules and Decision Constraints

- Only valid Examination Registrations may enter the final downstream population.
- Confirmed/locked registration status is governed by L2-I04.
- Withdrawn or rejected registrations do not authorise downstream examination participation.
- Registration locking freezes the registration population for routine modification.
- Examination owns the final registration population; downstream workflows consume it.
- Student Lifecycle remains authoritative for candidate/student facts.
- No new “Finalized Population” registration state is introduced unless upstream architecture defines it.
- No downstream workflow may redefine the eligibility or validity of an Examination Registration.

## 13. State and Lifecycle Impact

The finalisation scenario applies the existing L2-I04 lifecycle, particularly the transition to **Locked** where registration locking is required.

Relevant states include:

`Draft → Pending Validation → Pending Review → Approved → Confirmed → Locked`

with `Amended`, `Withdrawn`, `Completed` and `Archived` used only where supported by the applicable registration lifecycle.

The phrase “final registration population” describes the scenario outcome; it is not introduced as a new registration state.

## 14. Business Events and Cross-Domain Collaboration

L2-I04 is the canonical publisher for registration lifecycle events.

The materially relevant event is:

- **Registration Locked** — published by Examination Registration when the registration is frozen for examination operations.

Other registration events may already have occurred as part of the individual registration lifecycle, including **Registration Confirmed**, **Registration Withdrawn**, and **Registration Amended**.

Student Lifecycle provides authoritative candidate context where required.

The final population is a business hand-off to downstream Examination workflows; it does not transfer registration ownership.

## 15. Business Authorization, Approval and Audit Obligations

- Population finalisation must be performed by an authorised Examination business actor.
- Required examination governance approval must be completed.
- The final population decision and the basis for excluding registrations must remain auditable.
- Registration locking must comply with the existing institutional registration rules.
- Any exceptional post-lock change requires the applicable institutional authority.
- No technical authorization or implementation mechanism is specified.

## 16. Completion Criteria and Postconditions

**Completion Criteria:**
- The applicable Examination Session and Schedule are established.
- All candidate registrations relevant to the population have been evaluated.
- Invalid, withdrawn or rejected registrations are excluded.
- Required approvals and permitted amendments are complete.
- The authoritative final population is established.
- Required registrations are locked according to L2-I04.
- Audit evidence is complete.
- Applicable canonical registration events have been published.
- Downstream examination workflows can consume the final population.

**Postconditions:**
- One authoritative registration population exists for the applicable examination processing boundary.
- Downstream examination workflows consume only valid registration authority.
- Routine changes to locked registrations are no longer permitted.
- The finalisation decision remains historically traceable.

## 17. Open Issues, Assumptions and Deferred Questions

1. L2-I04 explicitly establishes Registration Locking as the mechanism that freezes registrations for downstream examination operations. This scenario therefore treats population finalisation as an outcome of the existing registration lifecycle rather than inventing a separate registration state.
2. The Stage 3 row fixes Type = **Critical**, Priority = **P0**, Primary Owner = **Examination Registration**, and Participating Domains = **C, I**. These frozen semantics are preserved.
3. The exact institutional point at which the population must be locked is policy-governed; no new deadline rule is introduced here.
4. If an authoritative upstream source later defines a dedicated final-population event or state, that source takes precedence and must be incorporated through controlled architecture change.
