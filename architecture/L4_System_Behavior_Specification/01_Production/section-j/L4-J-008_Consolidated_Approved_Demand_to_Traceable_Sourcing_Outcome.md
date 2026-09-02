# Consolidated Approved Demand to Traceable Sourcing Outcome

## 1. Specification Metadata

**Specification ID:** L4-J-008  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Procurement Planning & Sourcing  
**Realizes Scenario:** L3-J-008  
**Primary Business Owner:** Procurement Planning & Sourcing  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-008_Consolidated_Approved_Demand_to_Traceable_Sourcing_Outcome.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `L2-J03_Purchase_Requisition_and_Approval_Workflows.md`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall consolidate multiple compatible approved procurement demands into one traceable sourcing exercise without replacing their J03-owned authority, obscuring source ownership, or losing quantity and outcome lineage.

The behavior begins when two or more valid approved demands or demand lines are identified as consolidation candidates before supplier commitment. It ends when the consolidated sourcing exercise has an approved sourcing outcome with complete allocation/residual treatment, or when consolidation is rejected, dissolved, cancelled, or redirected.

This behavior shall not create a new approved procurement demand, alter J03 demand truth, or create supplier commitment.

---

## 3. Required Business Context

Required context includes:

- at least two valid approved J03 demands or lines;
- positive remaining unsourced quantity;
- source-demand ownership and approval references;
- specification/category/timing/location/funding context;
- applicable consolidation and aggregation policies;
- compatibility and institutional-benefit evidence;
- method-specific sourcing governance for execution.

J03 owns every contributing demand. J01 owns consolidation membership, sourcing traceability, allocation, residual treatment, and final sourcing outcome.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-008-BH-001 | The system shall admit only valid approved demand lines with remaining unsourced quantity into consolidation. | L3-J-008 §§7-9, 12 |
| L4-J-008-BH-002 | For each candidate line, the system shall preserve source demand, line, quantity, unit, specification, date, location, owner, value, funding, and existing sourcing/commitment references. | L3-J-008 §§7, 9, 15 |
| L4-J-008-BH-003 | Duplicate, withdrawn, superseded, fully sourced, already committed, or otherwise ineligible lines shall be excluded with a traceable reason. | L3-J-008 §§9, 11 |
| L4-J-008-BH-004 | The system shall assess compatibility and institutional consolidation benefit before creating the controlled consolidated package. | L3-J-008 §§8-9, 12 |
| L4-J-008-BH-005 | Aggregate anticipated value shall govern sourcing method and authority where applicable. | L3-J-008 §§9, 12 |
| L4-J-008-BH-006 | The system shall prevent consolidation from concealing artificial fragmentation, incompatible funding, conflicting specifications, or unacceptable schedule impacts. | L3-J-008 §§9, 11-12 |
| L4-J-008-BH-007 | The system shall preserve controlled consolidation membership and membership versions without overwriting prior membership history. | L3-J-008 §§9-10, 12 |
| L4-J-008-BH-008 | The consolidated sourcing package shall preserve line, lot, quantity, location, delivery, specification, and allocation identifiers sufficient to trace back to source demands. | L3-J-008 §§9, 12, 15 |
| L4-J-008-BH-009 | Method-specific sourcing shall remain governed by its applicable canonical sourcing scenario rather than being redefined here. | L3-J-008 §§4.4, 9, 12 |
| L4-J-008-BH-010 | Supplier responses and evaluations shall remain attributable to the relevant consolidated line or lot. | L3-J-008 §§9, 12 |
| L4-J-008-BH-011 | A post-issue source-demand change shall trigger governed assessment of fairness, quantity, aggregate value, method, competition, and response impact before membership is changed. | L3-J-008 §§9-11 |
| L4-J-008-BH-012 | For every source-demand line, the system shall preserve included quantity, awarded quantity, remaining quantity, allocation, and resulting sourcing-outcome reference. | L3-J-008 §§9, 12, 16 |
| L4-J-008-BH-013 | The system shall prevent allocated quantity from exceeding valid remaining source demand. | L3-J-008 §§11-12, 16 |
| L4-J-008-BH-014 | Residual demand shall remain visible and shall not be marked fulfilled merely because the consolidated Sourcing Event completed. | L3-J-008 §§10, 12, 16 |
| L4-J-008-BH-015 | The system shall require approval of the exact consolidated recommendation, allocations, residuals, and exceptions before the sourcing outcome becomes approved. | L3-J-008 §§9, 15-16 |
| L4-J-008-BH-016 | The system shall correlate each approved sourcing outcome back to every contributing source demand without altering J03 lifecycle truth. | L3-J-008 §§3, 9, 16 |

---

## 5. Cross-Domain & Event Behavior

When their source-supported conditions occur, J01 may publish exact canonical events including:

- `ProcurementDemandConsolidated`
- `SourcingEventCreated`
- `ApprovedSourcingOutcomeFinalised`

J03 remains the publisher and owner of demand facts. The consolidation event records a sourcing relationship among existing approved demands; it does not create new demand authority.

---

## 6. Failure / Rejection / Continuation

Invalid source demand, duplicate membership, incompatible requirements, aggregate-method error, material post-issue membership change, allocation imbalance, funding/authority conflict, or interrupted approval shall prevent a false consolidated success.

The affected source demands retain their authoritative J03 positions. No technical matching, retry, allocation-engine, or rollback behavior is specified.

---

## 7. Audit & Evidence Behavior

The system shall preserve:

- every contributing demand and line;
- consolidation eligibility and benefit;
- aggregate-value/method determination;
- membership versions;
- exclusions and changes;
- source-demand-to-package mapping;
- supplier response/evaluation mapping;
- recommendation and allocation;
- awarded and residual quantities;
- exceptions;
- final approval and authority;
- forward and reverse traceability.

---

## 8. Observable Completion Conditions

The behavior is complete when the consolidated exercise has an approved traceable sourcing outcome and every included source-demand line has an awarded, partially awarded, residual, removed, cancelled, or redirected position, or when consolidation has an explicit governed unsuccessful disposition.

No quantity may be lost, duplicated, or over-awarded.

---

## 9. Traceability & Provenance

**Parent:** L3-J-008 — Consolidated Approved Demand to Traceable Sourcing Outcome  
**Primary verification source:** L2-J01 — Procurement Planning & Sourcing Workflows  
**Demand source:** L2-J03 — Purchase Requisition & Approval  
**Governance source:** L2-J00.

---

## 10. Open Issues / Deferred Design

The parent records unresolved institution-specific aggregation windows/category grouping and exact J03 remaining-quantity integration semantics. L4 does not invent them.

No separate canonical consolidation-record lifecycle is defined in J01; none is created here.

Matching, unit-conversion, allocation, residual, rounding, concurrency, API, persistence, event-payload, and UI implementation remain deferred.

---

## Canonical Validation Note

This file is a Draft and requires self-review and independent review before Architecture Approval.
