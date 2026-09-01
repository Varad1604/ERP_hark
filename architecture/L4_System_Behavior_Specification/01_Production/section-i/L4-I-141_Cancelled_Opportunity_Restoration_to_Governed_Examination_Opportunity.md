# L4-I-141 — Cancelled Opportunity Restoration to Governed Examination Opportunity

## 1. Specification Metadata
- Specification ID: L4-I-141
- Architecture Level: L4 — System Behavior Specification
- Realizes: L3-I-141 — Cancelled Opportunity Restoration to Governed Examination Opportunity
- Primary Owner: Supplementary, Backlog & Improvement Examination
- Participating Domains: I — Examination
- Scenario Type: Critical; Priority: P1
- Status: Draft; Version: 1.0

## 2. Behavioral Objective & Boundary
Govern restoration of a previously cancelled subsequent-examination opportunity when an authorized institutional decision permits restoration. End when the opportunity has a governed active/usable position under the existing lifecycle.

## 3. Required Business Context
- A cancelled subsequent-examination opportunity exists.
- Restoration authority is established.
- Original opportunity history is available.
- Applicable examination governance is available.

## 4. Required System Behaviors
1. Identify the cancelled opportunity and its historical context.
2. Verify the authority and scope of the restoration decision.
3. Determine whether restoration conditions are satisfied.
4. Restore the opportunity using the existing governed lifecycle position; do not invent a new state.
5. Reconcile schedule, participation and downstream consequences through canonical owners where required.
6. Preserve cancellation and restoration history.
7. Prevent restoration outside the authorized scope.

## 5. Cross-Domain Behavior
Subsequent-examination administration owns opportunity restoration. Examination scheduling, registration, conduct and result capabilities retain their respective lifecycle ownership.

## 6. Failure / Recovery
Invalid or withdrawn restoration authority prevents restoration. Conflicting downstream states require governed reconciliation before the opportunity is treated as usable. Historical cancellation evidence remains intact.

## 7. Audit & Evidence
Preserve original cancellation, restoration authority, eligibility/context evaluation, resulting opportunity position and reconciliation evidence.

## 8. Observable Completion
A previously cancelled opportunity is restored to an existing governed examination opportunity position with complete traceability.

## 9. Traceability
Parent: L3-I-141 — Cancelled Opportunity Restoration to Governed Examination Opportunity.

## 10. Deferred Design
Technical state persistence, integration and notifications remain outside L4.