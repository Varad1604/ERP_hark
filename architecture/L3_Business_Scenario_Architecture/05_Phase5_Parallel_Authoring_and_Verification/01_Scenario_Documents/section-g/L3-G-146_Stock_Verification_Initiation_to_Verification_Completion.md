# Stock Verification Initiation to Verification Completion

**Scenario ID:** L3-G-146  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Library Management  
**Participating Domains:** G, E, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-G-146 |
| Canonical Title | Stock Verification Initiation to Verification Completion |
| Primary Owner | Library Management |
| Participating Domains | G, E, H |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario defines the complete institutional outcome in which a Library Management stock verification is initiated, the applicable institutional collection is physically verified, differences between physical resources and authoritative catalogue records are analysed, verification results are established, and the verification is completed with its outcome preserved.

The locked L2 defines Stock Verification as operational verification of institutional library collections so that physical resources remain aligned with catalogue records. Its authoritative workflow is:

`Verification Scheduled → Physical Verification → Difference Analysis → Missing Resources → Verification Report → Administrative Closure`.

Supported verification types are Complete, Section, Branch, Surprise and Annual, with verification policies configurable. Verification may identify available, missing, damaged, misplaced and duplicate records.

The L2 also defines `LibraryVerificationStarted` and `LibraryVerificationCompleted` as canonical verification events.

## 3. Business Outcome

A defined institutional collection has completed its authorised stock verification cycle and the resulting verification outcome has been recorded.

Completion means:

1. The verification scope and applicable verification type are established.
2. Physical verification is conducted.
3. Differences are analysed.
4. Identified discrepancies are classified into the supported result categories.
5. A verification report/outcome is established.
6. Administrative closure is reached.
7. The verification completion event is published by Library Management.
8. Relevant operational history is preserved.

The scenario does not itself perform subsequent missing-resource investigation, write-off approval, procurement, or financial settlement.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
The scenario starts when an authorised stock verification is scheduled/ready to commence.

### 4.2 End Boundary
The scenario ends at administrative closure of the verification cycle and publication of the canonical `LibraryVerificationCompleted` event.

### 4.3 In Scope
- Verification scope.
- Verification type.
- Physical verification.
- Difference analysis.
- Identification/classification of discrepancies.
- Verification report.
- Administrative closure.
- Canonical verification events.
- Audit/history preservation.
- E personnel participation where applicable.
- H notification participation where applicable.

### 4.4 Out of Scope
- Detailed scanning/device implementation.
- Inventory database design.
- Technical queues/retries.
- Missing-resource investigation after the verification boundary.
- Write-off approval.
- Finance settlement.
- Procurement.
- UI.
- Infrastructure.

## 5. Ownership and Participating Domains

### Library Management — G
Owns the stock verification lifecycle, verification outcome, administrative closure and canonical verification events.

### People & Workforce — E
Provides authoritative personnel context where operational staff or responsibility information is required. Personnel ownership remains external.

### Communication — H
Consumes/handles notification outcomes where applicable. Notification delivery remains within Section H.

## 6. Business Actors, Roles and Responsibilities

- **Library Verification Authority:** establishes the verification scope/type and authorises the verification outcome.
- **Library Verification Personnel:** perform physical verification and difference analysis.
- **E Personnel Context:** provides authoritative personnel information where required.
- **H:** handles notification delivery where a governed notification is required.

No unsupported technical or security role is introduced.

## 7. Preconditions and Entry Criteria

1. A verification cycle is authorised/scheduled.
2. The verification scope is established.
3. A supported verification type is established.
4. The applicable collection/catalogue context is available.
5. Personnel required to perform verification are established.
6. The verification has not already been administratively closed.

## 8. Trigger

The scheduled/authorised stock verification reaches its commencement condition.

At business level, this corresponds to progression from **Verification Scheduled** to **Physical Verification**.

## 9. Main Business Flow

### Phase A — Establish Verification

1. Identify the verification cycle.
2. Establish its scope.
3. Establish its verification type.
4. Establish the applicable institutional collection.
5. Establish responsible verification personnel where required.
6. Publish `LibraryVerificationStarted`.

### Phase B — Physical Verification

7. Perform physical verification against the applicable catalogue/collection records.
8. Establish the verification status of resources encountered.
9. Record supported verification outcomes.
10. Continue until the authorised verification scope has been covered.

### Phase C — Difference Analysis

11. Analyse differences between physical resources and catalogue records.
12. Identify missing resources.
13. Identify damaged resources.
14. Identify misplaced resources.
15. Identify duplicate records where applicable.
16. Establish available resources consistent with verification findings.

The scenario does not prescribe technical means of detecting these conditions.

### Phase D — Verification Report

17. Consolidate the verification findings.
18. Establish the verification report/outcome.
19. Ensure the report represents the completed verification scope and identified differences.

### Phase E — Administrative Closure

20. Establish the administrative closure decision.
21. Complete the verification cycle.
22. Preserve the verification history.
23. Publish the canonical `LibraryVerificationCompleted` event.
24. End the scenario.

## 10. Alternate Business Flows

### 10.1 Complete Verification
The entire applicable collection is verified and the cycle proceeds through difference analysis, reporting and closure.

### 10.2 Section Verification
Only the authorised section is verified. The scenario completes when that defined scope is fully processed.

### 10.3 Branch Verification
The authorised branch scope is verified.

### 10.4 Surprise Verification
The configured surprise-verification process is followed without inventing additional notice requirements.

### 10.5 Annual Verification
The annual verification policy governs the verification scope and timing.

### 10.6 No Differences Found
The verification report records the outcome without inventing discrepancy resolution actions.

### 10.7 Differences Found
Differences are included in the verification outcome. Missing-resource investigation and other downstream resolution remain separate business outcomes.

### 10.8 Verification Scope Cannot Be Completed
Administrative closure is not falsely asserted for an incomplete scope unless an authoritative business decision defines how the verification is concluded.

## 11. Failure, Interruption and Business Recovery

### 11.1 Scope Cannot Be Established
Verification does not commence using an invented scope.

### 11.2 Verification Type Cannot Be Established
No unsupported verification type is substituted.

### 11.3 Physical Verification Is Incomplete
The cycle remains within its applicable verification process until the business completion condition is established.

### 11.4 Difference Analysis Is Incomplete
The verification is not represented as complete while required analysis remains unresolved.

### 11.5 Administrative Closure Cannot Be Established
The completion event is not asserted prematurely.

### 11.6 Recovery Boundary
Once the required business conditions are satisfied, the verification can continue toward completion.

No technical retries, queues, rollback, server failure, database recovery, API mechanics or infrastructure behaviour are specified.

## 12. Business Rules and Decision Constraints

The L2 defines:

- Complete Verification.
- Section Verification.
- Branch Verification.
- Surprise Verification.
- Annual Verification.

Verification policies remain configurable.

Verification may identify:

- Available Resources.
- Missing Resources.
- Damaged Resources.
- Misplaced Resources.
- Duplicate Records.

Operational handling remains within Library Management.

Write-off recommendations may arise from irrecoverable loss, severe damage, administrative decision or obsolete material, but final approval follows institutional policy. Those downstream decisions are not silently absorbed into this scenario.

## 13. State and Lifecycle Impact

The authoritative workflow is:

`Verification Scheduled → Physical Verification → Difference Analysis → Missing Resources → Verification Report → Administrative Closure`.

The scenario progresses the verification through these business stages and ends at **Administrative Closure**.

No new verification lifecycle states are invented.

## 14. Business Events and Cross-Domain Collaboration

The locked L2 explicitly defines:

- `LibraryVerificationStarted` — stock verification initiated.
- `LibraryVerificationCompleted` — verification completed.

Library Management is the canonical publisher because the L2 states that Library Management publishes immutable operational events and that significant business transitions publish domain events.

E provides personnel context where applicable.

H handles notification delivery where required; Library Management publishes business events and does not implement notification delivery.

## 15. Business Authorization, Approval and Audit Obligations

The verification must be conducted under the applicable Library Management authority and configured verification policy.

The following evidence should remain auditable:

- Verification identity.
- Scope.
- Verification type.
- Responsible personnel context.
- Physical verification outcome.
- Difference analysis.
- Verification findings.
- Verification report.
- Administrative closure.
- Canonical start/completion event history.

The L2 requires immutable operational history and auditability of lifecycle transitions.

No technical authorization/RBAC/IAM is specified.

## 16. Completion Criteria and Postconditions

### Completion Criteria

1. Verification scope established.
2. Supported verification type established.
3. Physical verification completed for the authorised scope.
4. Differences analysed.
5. Findings classified.
6. Verification report established.
7. Administrative closure established.
8. `LibraryVerificationStarted` and `LibraryVerificationCompleted` event semantics preserved.
9. Completion history retained.

### Postconditions

- The verification cycle is administratively closed.
- Verification findings are available for downstream operational handling.
- Missing/damaged/misplaced/duplicate findings are not silently treated as resolved.
- Any subsequent investigation or write-off follows its own authoritative workflow.
- Library Management remains owner of the verification outcome.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

The L2 does not define a detailed discrepancy-resolution decision matrix within the stock-verification scenario.

### Assumptions

- Verification policies remain configurable as stated by L2.
- Administrative Closure is the authoritative end boundary.
- Downstream missing-resource investigation is separate from verification completion.

### Deferred Questions

Confirm any upstream governance requirement for mandatory verification-report approval before Architecture Approval/Lock.

No unsupported approval rule is invented.
