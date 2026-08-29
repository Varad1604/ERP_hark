# Missing Library Resource to Investigation Outcome

**Scenario ID:** L3-G-147  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Library Management  
**Participating Domains:** G, F, E, H  
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
| Final L3 ID | L3-G-147 |
| Canonical Title | Missing Library Resource to Investigation Outcome |
| Primary Owner | Library Management |
| Participating Domains | G, F, E, H |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario defines the institutional business outcome in which a library resource identified as missing during stock verification is investigated, an administrative review is completed, and the resource reaches the appropriate investigation outcome.

The locked L2 defines the missing-resource workflow:

`Resource Missing → Investigation → Administrative Review → Recovered? → Yes: Inventory Updated / No: Write-Off Review`.

It also states that write-off recommendations may arise from irrecoverable loss, severe damage, administrative decision or obsolete material, with final approval following institutional policy.

The canonical event `MissingResourceIdentified` is defined by the L2 for the detection of a missing resource.

## 3. Business Outcome

A resource identified as missing has completed its business investigation and administrative review, with the outcome established as either:

1. recovered, resulting in the applicable inventory update; or
2. not recovered, resulting in progression to write-off review.

The scenario does not invent final write-off approval, accounting settlement, replacement procurement, or disciplinary outcomes.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
A library resource has been identified as missing through the authoritative stock-verification process.

### 4.2 End Boundary
Administrative review establishes the recovered/not-recovered outcome and the applicable next business action.

### 4.3 In Scope
- Missing-resource identification context.
- Investigation.
- Recovery determination.
- Administrative review.
- Inventory update following recovery.
- Write-off review following non-recovery.
- Canonical missing-resource event.
- Cross-domain collaboration.
- Audit/history preservation.

### 4.4 Out of Scope
- Technical asset tracking.
- Physical security investigation mechanics.
- Finance accounting/payment.
- Procurement.
- Final write-off approval unless explicitly owned by an upstream rule.
- Disciplinary action.
- Replacement-resource acquisition.
- UI/infrastructure.

## 5. Ownership and Participating Domains

### Library Management — G
Owns missing-resource operational handling and administrative review.

### Finance — F
Participates where replacement cost, outstanding dues or financial consequences are relevant. Finance owns accounting/payment processing.

### People & Workforce — E
Provides personnel context where required. Personnel ownership remains external.

### Communication — H
Handles governed notification delivery. Library Management publishes business events rather than implementing notification delivery.

Cross-domain participation does not transfer missing-resource ownership from G.

## 6. Business Actors, Roles and Responsibilities

- **Library Operations/Verification Personnel:** identify and investigate the missing resource.
- **Library Administrative Authority:** conducts/establishes the administrative review outcome.
- **E:** supplies authoritative personnel context where required.
- **F:** supplies/consumes financial information where applicable.
- **H:** handles governed notification delivery where required.

No unsupported investigator, security officer, or disciplinary authority is invented.

## 7. Preconditions and Entry Criteria

1. The resource has been identified as missing.
2. The missing condition originates from the authorised verification process.
3. The resource can be uniquely identified within the authoritative library context.
4. Investigation authority is available.
5. Required personnel context is available where applicable.
6. Any required financial context can be obtained from F without transferring ownership.

## 8. Trigger

The canonical business event `MissingResourceIdentified` establishes that a library resource has been detected as missing.

## 9. Main Business Flow

### Phase A — Establish Investigation

1. Identify the missing library resource.
2. Establish the verification context in which it was identified.
3. Preserve the `MissingResourceIdentified` business event.
4. Establish the investigation scope.
5. Establish the responsible library operational context.

### Phase B — Investigate

6. Review the authoritative library records relevant to the missing resource.
7. Review the physical-verification findings.
8. Determine whether the resource can be recovered.
9. Record the investigation outcome.
10. Where personnel context is relevant, obtain authoritative context from E.
11. Where financial implications are relevant, obtain applicable financial information from F.

### Phase C — Administrative Review

12. Present the investigation outcome for administrative review.
13. Establish whether the resource has been recovered.
14. Establish the applicable administrative outcome.

### Phase D — Recovered Branch

15. If recovered, establish the recovery outcome.
16. Update the applicable inventory condition.
17. Preserve the recovery/update history.
18. End the missing-resource investigation outcome.

### Phase E — Not Recovered Branch

19. If not recovered, establish the non-recovery outcome.
20. Progress the resource to **Write-Off Review**.
21. Preserve the administrative review history.
22. End this scenario at the write-off-review boundary.

Final write-off approval is not absorbed into this scenario because the L2 states that final approval follows institutional policy.

## 10. Alternate Business Flows

### 10.1 Resource Recovered

The resource is recovered and the inventory is updated as specified by the L2.

### 10.2 Resource Not Recovered

The resource progresses to Write-Off Review.

### 10.3 Investigation Finds Resource Was Misplaced Rather Than Lost

The L2 separately identifies misplaced resources as a verification result. The scenario does not invent a separate misplaced-resource lifecycle; the administrative investigation outcome governs the resolution.

### 10.4 Financial Context Required

F supplies financial/replacement-cost information as applicable. Accounting and payment remain F-owned.

### 10.5 Personnel Context Required

E supplies authoritative personnel information where required.

### 10.6 Notification Required

H handles notification delivery where a governed notification is required.

### 10.7 Evidence Is Insufficient

Administrative review does not falsely establish recovery or non-recovery without an authoritative business decision.

## 11. Failure, Interruption and Business Recovery

### 11.1 Missing Resource Cannot Be Identified

Investigation does not proceed using an invented resource identity.

### 11.2 Investigation Incomplete

The scenario remains within investigation until the business investigation condition is satisfied.

### 11.3 Recovery Status Unclear

Administrative review remains unresolved until the recovery outcome becomes authoritative.

### 11.4 Financial Information Pending

F remains the owner of financial information; G does not invent amounts or payment status.

### 11.5 Personnel Context Pending

E remains authoritative for personnel information.

### 11.6 Recovery

Once sufficient business evidence exists, administrative review establishes the appropriate recovered/non-recovered branch.

No technical retries, queues, rollback, infrastructure, database, API or deployment mechanics are described.

## 12. Business Rules and Decision Constraints

The L2 defines:

`Resource Missing → Investigation → Administrative Review → Recovered?`

with:

- **Yes:** Inventory Updated.
- **No:** Write-Off Review.

Write-off recommendations may arise from irrecoverable loss, severe damage, administrative decision or obsolete material. Final approval follows institutional policy.

Finance owns accounting/payment processing; Library Management may consume financial information such as replacement cost references and outstanding dues.

## 13. State and Lifecycle Impact

The authoritative missing-resource workflow is:

`Resource Missing → Investigation → Administrative Review → Recovered? → Inventory Updated / Write-Off Review`.

No new states such as `Under Investigation`, `Recovery Pending`, or `Lost Confirmed` are introduced as formal lifecycle states.

The scenario ends after the recovered/non-recovered administrative outcome and does not silently consume the downstream write-off approval process.

## 14. Business Events and Cross-Domain Collaboration

The L2 explicitly defines `MissingResourceIdentified` as the event for a missing resource detected during verification. Library Management is the canonical publisher because it owns the verification and missing-resource workflow and publishes immutable operational events.

### G — Library Management
Owns investigation, administrative review and outcome.

### F — Finance
Provides financial information where applicable; owns accounting/payment processing.

### E — People & Workforce
Provides authoritative personnel context where applicable.

### H — Communication
Handles notification delivery where applicable.

No participating domain assumes G's missing-resource lifecycle ownership.

## 15. Business Authorization, Approval and Audit Obligations

The investigation and administrative review must follow applicable Library Management authority and institutional policy.

Audit evidence should include:

- Resource identity.
- Verification context.
- Missing-resource identification.
- Investigation findings.
- Recovery determination.
- Administrative review outcome.
- Inventory update where recovered.
- Write-off-review progression where not recovered.
- Relevant financial/personnel context where applicable.

Final write-off approval is governed by institutional policy and must not be invented locally.

## 16. Completion Criteria and Postconditions

### Completion Criteria

1. Missing resource identified.
2. Investigation completed.
3. Recovery status established.
4. Administrative review completed.
5. Recovered branch updates inventory, or non-recovered branch progresses to Write-Off Review.
6. Relevant history preserved.
7. Required cross-domain information consumed without ownership transfer.

### Postconditions

**Recovered branch**
- Resource is recovered.
- Applicable inventory is updated.
- Investigation outcome is preserved.

**Not recovered branch**
- Resource remains unresolved as recovered.
- Write-Off Review is established as the next business boundary.
- Final write-off approval is not implied.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. The L2 does not define the detailed investigation evidence checklist.
2. The L2 does not define the exact administrative review decision matrix.
3. Final write-off approval authority is governed by institutional policy but is not specified here.

### Assumptions

- `MissingResourceIdentified` is the authoritative trigger.
- Inventory update follows recovery as stated in L2.
- Write-Off Review is the authoritative non-recovery boundary.

### Deferred Questions

Confirm the upstream institutional policy for final write-off approval before Architecture Approval/Lock.

No invented financial amount, disciplinary outcome, replacement obligation, or approval authority is introduced.
