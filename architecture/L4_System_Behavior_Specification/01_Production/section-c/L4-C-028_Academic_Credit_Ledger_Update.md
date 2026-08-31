# L4-C-028 — Academic Credit Ledger Update

**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** `L3-C-028` — Academic Credit Ledger Update  
**Primary Owner:** Academic Progression  
**Participating Domains:** C, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Author:** Varad

## 1. Purpose
Define behavior for maintaining the authoritative Academic Credit Ledger from verified academic outcomes.

## 2. Preconditions and Trigger
An official academic outcome or progression evaluation requires credit recalculation/update. Applicable curriculum credit structure, credit policy and existing ledger history are available.

## 3. Main Behavior
1. Establish Student and academic-period context.
2. Evaluate official academic outcomes.
3. Read existing ledger history for duplicate recognition.
4. Identify earned credits.
5. Include transferred/recognised credits only where required institutional approval exists.
6. Apply curriculum credit rules.
7. Prevent duplicate recognition and invalid credit balances.
8. Calculate earned, recognised, remaining and qualification credit position as applicable.
9. Update the Academic Credit Ledger owned by Academic Progression.
10. Validate the resulting ledger and produce the credit summary.
11. Preserve audit evidence and historical credit history.
12. Where applicable, make the outcome available to the ABC synchronization workflow without transferring ledger ownership.

## 4. Alternate Behavior
- Unofficial outcome: do not recognise credit.
- Missing curriculum mapping: defer calculation.
- Duplicate credit: reject duplicate recognition.
- Transfer approval absent: do not recognise the transferred credit.
- Revised official result: perform governed fresh evaluation while preserving history.

## 5. Ownership
Academic Progression is canonical owner of the Academic Credit Ledger. Examination remains owner of examination outcomes; Curriculum remains authoritative for curriculum credit structure.

## 6. Lifecycle
`Authoritative Academic Outcome → Credit Evaluation → Credit Validation → Ledger Update → Credit Summary Available → Progression/ABC Consumption`. This is behavioral flow, not a newly invented ledger state machine.

## 7. Events
The source architecture explicitly identifies an ABC Synchronisation Event but does not safely define a canonical ledger-update event name in the available C04 material. Therefore no `CreditLedgerUpdated` event is invented.

## 8. Rules and Audit
Only successfully earned credits contribute to progression; transferred credits require approval; duplicate academic credits are never recognised; ledger balances remain valid; historical credit records remain auditable/immutable.

## 9. Business Failure Handling
Unverified evidence, missing curriculum mapping, duplicate credit, missing transfer approval or policy violation prevents completion until resolved by the relevant owning process.

## 10. Traceability
L3-C-028; L2-C04 APW-03 and APW-12; Academic Progression credit ownership.

## 11. Open Issues
Exact ledger-update event name, ABC synchronization contract semantics, revised-result reconciliation and unresolved policy ownership remain upstream questions.

## 12. Implementation Boundary
No data schema, API payload, database, service, connector, queue, retry or infrastructure design is defined.

**Classification: PASS WITH ISSUES — Draft**
