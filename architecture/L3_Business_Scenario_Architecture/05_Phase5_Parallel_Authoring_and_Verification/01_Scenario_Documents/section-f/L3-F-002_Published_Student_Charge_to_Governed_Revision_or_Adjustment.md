# Published Student Charge to Governed Revision or Adjustment

**Scenario ID:** L3-F-002  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Student Charge & Fee Management  
**Participating Domains:** F — Finance  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-F00 — Finance Domain Model & Foundation; L2-F01 — Student Charge & Fee Management; applicable downstream Financial Adjustment architecture  
**Referenced By:** Finance settlement and accounting consumers as applicable  

## 2. Scenario Purpose

Govern a required change to an already published Student Charge without rewriting the historical financial obligation.

The scenario preserves the immutability of published charges and ensures that any permitted financial correction is represented through the governed financial-adjustment path rather than direct mutation of the historical published obligation.

## 3. Business Outcome

A published Student Charge is either preserved unchanged because no valid revision is required, or its required financial correction is represented by the governed successor/adjustment outcome while the original published charge remains historically immutable and traceable.

The final financial position is therefore explainable through the original published obligation and its governed corrective outcome.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a Published Student Charge is identified as requiring a governed revision or financial correction.

### 4.2 End Boundary

The scenario ends when the correction has either been rejected/declined or has produced the authoritative governed adjustment outcome, without directly rewriting the published Student Charge.

### 4.3 In Scope

- Identification of a correction need affecting a published Student Charge.
- Validation that the correction is within Finance authority.
- Preservation of the published charge.
- Creation/application of a governed adjustment or successor financial outcome where the applicable adjustment architecture permits.
- Preservation of historical lineage.
- Publication of the applicable adjustment/correction financial events.

### 4.4 Out of Scope

- Direct modification or deletion of published Student Charge data.
- Payment collection.
- Refund processing unless separately initiated through Refund Management.
- Accounting journal correction outside the canonical Financial Adjustment/Accounting domains.
- Technical rollback or database update mechanics.

## 5. Ownership and Participating Domains

Student Charge & Fee Management remains the canonical owner of Student Charge.

Where a financial adjustment is required, the canonical Financial Adjustment capability remains the owner of the adjustment process. The original Student Charge does not transfer ownership to an adjustment record.

L2-F01 explicitly establishes that published charges remain historically immutable and that financial corrections are represented through downstream financial adjustments rather than modifying historical obligations. fileciteturn114file0

## 6. Business Actors, Roles and Responsibilities

The authoritative L2-F01 document does not define an actor catalogue. No new scenario-specific actor is therefore invented.

The Finance domain's applicable authorized business participants act under the governing Financial Adjustment architecture when a correction is required.

## 7. Preconditions and Entry Criteria

- A canonical Student Charge exists.
- The Student Charge is Published or otherwise in a post-publication state.
- A valid business reason for correction exists.
- The requested outcome can be represented through the governed financial adjustment model.
- Historical traceability can be preserved.
- The correction does not require direct mutation of the published Student Charge.

## 8. Trigger

A published Student Charge is determined to require a governed financial revision or adjustment.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Identify the published Student Charge requiring correction. | F — Student Charge | A correction need exists against a canonical published obligation. |
| 2 | Determine the business reason and nature of the required correction. | F — Finance | A governed correction path is selected. |
| 3 | Validate that the published Student Charge itself must remain immutable. | F — Student Charge | Historical obligation is protected from direct change. |
| 4 | Where a financial adjustment is appropriate, establish the adjustment through the canonical Financial Adjustment process. | F — Financial Adjustment | A separate governed correction outcome exists. |
| 5 | Associate the adjustment with the originating Student Charge for traceability. | F | Financial lineage between original obligation and correction is preserved. |
| 6 | Publish the applicable adjustment/correction business event. | F | Downstream Finance consumers can consume the correction. |
| 7 | Re-evaluate the resulting financial position through the canonical downstream Finance processes. | F | The current financial position reflects the original obligation plus governed correction. |

## 10. Alternate Business Flows

### A1 — No Correction Required

Review determines that the published Student Charge is already correct. The charge remains unchanged and no adjustment is created.

### A2 — Correction Rejected

The correction request does not satisfy governing Finance requirements. The published charge remains unchanged and the correction is closed as rejected.

### A3 — Adjustment Instead of Revision

The required financial change is represented through an adjustment rather than altering the charge itself. The original Student Charge remains immutable.

### A4 — Refund Required

If the correction creates a refund consequence rather than a pure charge adjustment, the case exits into the canonical Refund Management scenario rather than creating a local refund process.

## 11. Failure, Interruption and Business Recovery

| Business Situation | Resolution |
|---|---|
| Correction reason is insufficient | Do not alter the published charge; obtain sufficient business evidence through the governed correction process. |
| Requested correction would mutate published history | Reject that path and require a governed adjustment/successor outcome. |
| Adjustment is not valid | Published charge remains unchanged. |
| Refund consequence is identified | Route to Refund Management ownership. |
| Accounting consequence requires additional action | Route through the canonical downstream Accounting/Financial Adjustment process. |

No technical rollback or retry mechanics are defined.

## 12. Business Rules and Decision Constraints

Authoritative L2-F01 constraints include:

- Published Student Charges remain historically immutable.
- Financial corrections are represented through downstream financial adjustments rather than direct modification of historical obligations.
- Aggregate ownership remains with Student Charge & Fee Management.
- External domains do not modify Student Charges directly.
- Complete financial lineage must be preserved.
- Historical financial information remains immutable.
- Downstream financial correction does not redefine the original obligation.

fileciteturn114file0turn127file0

No new local correction rule is introduced.

## 13. State and Lifecycle Impact

The original Student Charge remains in its existing governed lifecycle and historical state.

Where a correction is required, a separate financial adjustment/successor outcome is established under the applicable Finance domain. The original Student Charge is not rewritten to appear as if the correction had always existed.

This scenario does not create a new Student Charge state.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

L2-F01 defines immutable Student Charge lifecycle/financial event categories. The adjustment-specific event vocabulary belongs to the Financial Adjustment architecture.

This scenario therefore publishes the canonical event of the owning adjustment process rather than inventing a new Student Charge correction event.

### 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose |
|---|---|---|---|
| Student Charge & Fee Management | Financial Adjustment Management | Published Student Charge and correction context | Establishes the original obligation against which the adjustment is evaluated. |
| Financial Adjustment Management | Student Charge / downstream Finance | Governed financial adjustment outcome | Provides the authoritative correction result. |
| Financial Adjustment / Student Charge | Accounting | Financial correction context | Enables canonical accounting treatment without direct mutation of the Student Charge. |
| Financial Adjustment / Student Charge | Refund Management | Refund consequence where applicable | Routes refund ownership to Refund Management. |

## 15. Business Authorization, Approval and Audit Obligations

The applicable Finance adjustment authority and approval requirements remain owned by the Financial Adjustment architecture.

The original Student Charge, correction reason, adjustment decision and financial lineage must remain historically traceable.

L2-F01 states that Student Charge audit participation covers Charge Generation, Validation, Publication, Lifecycle Progression and State Changes; the adjustment domain retains its own adjustment evidence. fileciteturn126file0turn127file0

No Finance-specific actor or approval role is invented here because L2-F01 does not provide one.

## 16. Completion Criteria and Postconditions

- The Published Student Charge remains historically immutable.
- The correction need has reached a governed outcome.
- Any required adjustment exists under the canonical Financial Adjustment ownership.
- The original charge-to-adjustment lineage is preserved.
- Downstream financial consumers can consume the resulting financial position.
- No external domain has directly modified the published Student Charge.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

The exact adjustment command, state and event vocabulary belongs to the applicable locked Financial Adjustment L2 document rather than L2-F01. The scenario therefore intentionally does not invent that vocabulary.

### 17.2 Assumptions

No assumptions override locked architecture.

### 17.3 Deferred Questions

Confirm the exact Financial Adjustment business contract and event mapping during scenario review against L2-F04.
