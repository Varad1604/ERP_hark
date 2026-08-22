# Re-Conduct Supplementary, Backlog, or Improvement Examination Following Authorized Decision

**Scenario ID:** L3-I-137  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Supplementary, Backlog & Improvement Examination  
**Participating Domains:** I — Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-I02 Examination Session Management Workflows; L2-I03 Examination Planning & Scheduling Workflows; L2-I04 Examination Registration Workflows; L2-I05 Hall Ticket Management Workflows; L2-I06 Practical & Viva Examination Conduct Workflows; L2-I07 Theory Examination Conduct Workflows; L2-I08 Marks Collection & Evaluation Workflows; L2-I12 Supplementary / Backlog / Improvement Examination Execution Workflows; L2-I15 Examination Exception & Contingency Workflows  
**Referenced By:** L2-I08 Marks Collection & Evaluation; L2-I09 Result Processing & Computation; L2-I10 Result Publication

## 1. Scenario Metadata

This scenario is the final L3-I-137 canonical output. The frozen Stage 3 semantics define it as a Critical P1 scenario owned by Supplementary, Backlog & Improvement Examination within the Examination domain.

The scenario governs the business path from an authorized decision that an affected subsequent examination must be conducted again to completion of the re-conducted examination occurrence and initiation of the normal downstream evaluation path.

## 2. Scenario Purpose

Restore a valid examination opportunity by conducting a Supplementary, Backlog, or Improvement examination again when an authorized institutional decision requires re-conduct.

The scenario preserves the boundary between subsequent-examination administration, examination planning, registration, examination conduct, evaluation and result processing.

## 3. Business Outcome

The authorized re-conducted examination is completed as a valid subsequent-examination occurrence, with the completion outcome made available to the existing evaluation and downstream result-processing workflows.

The scenario does not itself determine marks, grades, computed results or official publication.

## 4. Scenario Boundary and Scope

**Start Boundary:** An authorized institutional decision requires a previously scheduled or conducted Supplementary, Backlog, or Improvement examination to be re-conducted.

**In Scope:**
- establish the authorized re-conduct decision;
- identify the affected examination opportunity and scope;
- establish the applicable Examination Session and schedule;
- coordinate the replacement/re-conduct examination schedule;
- ensure the applicable registration and participation context is valid;
- initiate required Hall Ticket consequences through the owning workflow where applicable;
- coordinate practical or theory re-conduct;
- complete the re-conducted examination occurrence;
- initiate downstream evaluation.

**Out of Scope:**
- deciding whether the original examination should be invalidated or re-conducted;
- creating or changing examination policy;
- owning Examination Registration;
- owning Examination Schedule;
- issuing Hall Tickets;
- conducting practical/theory examination under the detailed conduct lifecycle;
- evaluating marks;
- computing or publishing results.

**End Boundary:** The authorized re-conducted examination is completed and the downstream evaluation process has been initiated.

## 5. Ownership and Participating Domains

**Supplementary, Backlog & Improvement Examination:** owns administration and coordination of the subsequent-examination opportunity and its attempt lifecycle.

**Examination Exception & Contingency:** owns the exception/contingency decision when that decision is the source of the re-conduct requirement. Where another authoritative examination decision governs re-conduct, this scenario consumes that decision without re-owning it.

**Examination Registration (L2-I04):** owns the Examination Registration lifecycle for Supplementary, Backlog and Improvement categories.

**Examination Planning & Scheduling (L2-I03):** owns the Examination Schedule required for the re-conduct.

**Hall Ticket (L2-I05):** owns Hall Ticket lifecycle consequences.

**Practical/Viva and Theory Conduct (L2-I06/L2-I07):** own actual examination conduct.

**Marks Collection & Evaluation (L2-I08):** owns evaluation after re-conduct completion.

## 6. Business Actors, Roles and Responsibilities

| Actor / Domain | Responsibility |
|---|---|
| Controller of Examination | Governs the authorized re-conduct decision and examination administration |
| Examination Administrator | Coordinates the re-conducted examination opportunity |
| Supplementary Examination Officer | Coordinates subsequent-examination context and attempt administration |
| Examination Planning & Scheduling | Produces the authoritative schedule for the re-conduct |
| Hall Ticket Management | Governs required Hall Ticket consequences |
| Practical & Viva Examination Conduct | Conducts an applicable practical/viva re-examination |
| Theory Examination Conduct | Conducts an applicable theory re-examination |
| Marks Collection & Evaluation | Receives the completed re-conduct outcome for evaluation |

No new re-conduct authority is introduced.

## 7. Preconditions and Entry Criteria

- A Supplementary, Backlog, or Improvement examination opportunity exists.
- An authoritative institutional decision requiring re-conduct exists.
- The affected examination scope is identified.
- The applicable Examination Session remains valid or a governed replacement session is established.
- Any required registration context is established through L2-I04.
- A valid schedule can be established through L2-I03.
- Applicable downstream conduct arrangements can be established.

## 8. Trigger

An authorized institutional decision requires the affected subsequent examination to be conducted again.

## 9. Main Business Flow

1. Identify the affected subsequent-examination opportunity and the authoritative re-conduct decision.
2. Establish the examination scope to be re-conducted.
3. Confirm the applicable Examination Session and examination category.
4. Determine the required registration/participation context through L2-I04 without creating a parallel registration lifecycle.
5. Prepare the re-conduct schedule through L2-I03.
6. Complete the required schedule validation and approval.
7. Make required Hall Ticket consequences available through L2-I05.
8. Conduct the applicable practical/viva or theory examination through L2-I06 or L2-I07.
9. Confirm that the re-conducted examination occurrence has completed.
10. Record the subsequent-examination attempt completion within the L2-I12 lifecycle.
11. Initiate evaluation through L2-I08.
12. Publish the applicable subsequent-examination lifecycle event(s).
13. End the scenario at initiation of downstream evaluation.

## 10. Alternate Business Flows

### 10.1 Practical/Viva Re-Conduct

Where the affected examination is practical or viva, L2-I06 conducts the re-conduct using the authorized schedule and participation context. The subsequent-examination administration remains responsible for coordination, not conduct ownership.

### 10.2 Theory Re-Conduct

Where the affected examination is theory, L2-I07 conducts the re-conduct. The scenario retains the same administrative and scheduling boundaries.

### 10.3 Re-Conduct for a Defined Examination Scope

Where only a defined subject or examination occurrence is affected, the re-conduct is limited to that authorized scope. Unaffected examination outcomes are not implicitly re-opened by this scenario.

### 10.4 Existing Registration Remains Valid

Where the existing Examination Registration remains valid for the re-conduct, L2-I04 remains the authoritative registration record and the scenario does not create a duplicate registration.

## 11. Failure, Interruption and Business Recovery

### 11.1 Re-Conduct Decision Is Not Authoritative

If the re-conduct decision is incomplete, disputed or not institutionally authorized, the re-conduct does not begin. The affected examination remains governed by its existing authoritative business position.

### 11.2 No Valid Replacement Schedule

If a valid schedule cannot be approved, the re-conduct cannot reach the conduct boundary. L2-I03 and, where relevant, L2-I15 govern the next permitted scheduling/contingency decision.

### 11.3 Registration Context Is Not Valid

If the required registration context is not valid, participation does not proceed by bypassing L2-I04. The registration position must be resolved through its canonical lifecycle.

### 11.4 Hall Ticket Consequence Is Unresolved

If a required Hall Ticket consequence cannot be established, the examination does not proceed on assumption. L2-I05 remains authoritative.

### 11.5 Re-Conduct Is Interrupted

If the re-conduct itself is interrupted by a new authorized exception, the new exception follows L2-I15. The existing re-conduct is not declared complete unless the conduct-owning workflow establishes completion.

### 11.6 Evaluation Cannot Be Initiated

Completion of the re-conduct remains distinct from evaluation. If evaluation cannot yet begin, the completed examination outcome remains with the owning conduct/marks workflows until L2-I08 can accept the evaluation initiation.

## 12. Business Rules and Decision Constraints

- Re-conduct occurs only on the basis of an authoritative institutional decision.
- Supplementary, Backlog and Improvement examinations remain within the L2-I12 framework.
- Examination Registration remains owned by L2-I04.
- Examination Scheduling remains owned by L2-I03.
- Hall Ticket remains owned by L2-I05.
- Actual practical/viva and theory conduct remain owned by L2-I06 and L2-I07.
- Evaluation remains owned by L2-I08.
- The re-conduct does not itself determine the validity of the original result or the final academic consequence.
- Every subsequent examination attempt must remain uniquely identifiable and traceable.
- No new re-conduct state or rule is invented.

## 13. State and Lifecycle Impact

L2-I12 defines the subsequent-examination governance as:

`Eligible → Application Submitted → Fee Verified → Registered → Session Allocated → Scheduled → Hall Ticket Initiated → Examination Conducted → Evaluation Initiated → Result Processing Initiated → Attempt Closed → Archived`

The re-conduct uses the existing lifecycle rather than introducing a `Re-Conduct` state.

The Examination Registration lifecycle remains governed by L2-I04, and the schedule and conduct lifecycles remain independently owned by L2-I03, L2-I06 and L2-I07.

## 14. Business Events and Cross-Domain Collaboration

L2-I12 publishes canonical subsequent-examination milestones including:
- **Examination Scheduled**
- **Hall Ticket Initiation Requested**
- **Supplementary Examination Started**
- **Supplementary Examination Completed**
- **Evaluation Initiated**
- **Result Processing Initiated**
- **Examination Attempt Closed**

Where the applicable category is Backlog or Improvement, L2-I12 also defines category-specific eligibility events and the common examination lifecycle events.

L2-I04 publishes registration milestones such as **Registration Confirmed** and **Registration Locked**.

L2-I03 publishes schedule milestones such as **Schedule Approved** and **Schedule Published**.

L2-I06/L2-I07 remain publishers of their own conduct outcomes where defined.

No `ReConductStarted` or `ReConductCompleted` event is invented because the locked L2-I12 event catalogue does not define those names.

## 15. Business Authorization, Approval and Audit Obligations

- The re-conduct decision must be institutionally authorized before operational preparation begins.
- Schedule approval follows L2-I03.
- Registration changes or exceptional administrative actions follow L2-I04 authority.
- Conduct decisions follow the relevant L2-I06/L2-I07 governance.
- The re-conduct decision, affected scope, schedule, participation context, conduct completion and evaluation initiation must be auditable.
- The original examination history must remain distinguishable from the re-conducted occurrence.
- No technical access-control or infrastructure design is specified.

## 16. Completion Criteria and Postconditions

**Completion Criteria:**
- Authoritative re-conduct decision is established.
- Affected examination scope is identified.
- Required session and registration context is valid.
- Replacement schedule is approved and available.
- Required Hall Ticket consequence is handled by L2-I05.
- Applicable examination conduct is completed by L2-I06 or L2-I07.
- L2-I12 records the attempt completion.
- Evaluation initiation is established through L2-I08.
- Applicable lifecycle events are published.

**Postconditions:**
- The authorized re-conducted examination occurrence is complete.
- The re-conduct outcome is available for downstream evaluation.
- The subsequent-examination administration remains traceable.
- Evaluation, computation and publication remain downstream responsibilities.

## 17. Open Issues, Assumptions and Deferred Questions

1. L2-I12 explicitly defines Attempt Lifecycle Management and requires every attempt to remain uniquely identifiable and traceable.
2. L2-I15 contains a specific `Authorize Re-Conduct Following Invalidated Examination` contingency workflow, but the canonical Stage 3 scenario is broader: it is driven by an authorized decision, not exclusively by one exception type.
3. The available L2 sources do not define a dedicated `Re-Conduct` state or event. This scenario therefore uses the existing L2-I12 lifecycle and conduct events without inventing new lifecycle terminology.
4. The exact business basis for the authorized re-conduct decision remains upstream-owned by the workflow that makes that decision. This scenario does not decide whether re-conduct is warranted.
