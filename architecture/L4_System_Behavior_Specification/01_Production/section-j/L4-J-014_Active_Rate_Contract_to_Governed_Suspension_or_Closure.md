# Active Rate Contract to Governed Suspension or Closure

## 1. Specification Metadata

**Specification ID:** L4-J-014  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Procurement Planning & Sourcing  
**Realizes Scenario:** L3-J-014  
**Primary Business Owner:** Procurement Planning & Sourcing  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-014_Active_Rate_Contract_to_Governed_Suspension_or_Closure.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L2-J03_Purchase_Requisition_and_Approval_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall convert an authoritative trigger affecting an `Active` Rate Contract or governed part of it into an approved suspension or closure outcome that prevents unauthorised new sourcing from the effective time while preserving existing commitments, source-domain ownership, and immutable history.

The behavior begins when a governed trigger indicates that an `Active` Rate Contract, supplier position, catalogue line, location, or other parent-supported scope may no longer remain available for new sourcing. It ends when the authorised suspension or closure is effective, affected new call-offs are blocked, existing positions have a governed disposition under their canonical owners, and the remediation, reinstatement, successor, re-sourcing, expiry, or archival path is recorded where applicable.

This specification does not change J02 supplier lifecycle state, cancel or amend J04 Purchase Orders, create accounting/payable/receipt/payment outcomes, or delete contract history.

---

## 3. Required Business Context

- The authoritative Rate Contract identity/version and affected supplier, line, location, or scope.
- Current J01 Rate Contract lifecycle/availability facts.
- The trigger, evidence, reason, and proposed effective time.
- Current J02 supplier eligibility, restriction, suspension, blacklisting, performance, and remediation facts where material.
- Pending and completed sourcing outcomes plus J04-owned open Purchase Order/commitment references.
- Applicable notice, representation, urgent-action, suspension, closure, reinstatement, and authority rules.
- Residual demand and successor/alternate-sourcing context where applicable.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-014-BH-001 | When a governed availability trigger is recorded, the system shall preserve the affected Rate Contract identity/version, supplier/line/location/scope, trigger evidence, discovery time, and proposed effective time. | L3-J-014 §§4.1, 8, 9.1 |
| L4-J-014-BH-002 | Before a suspension or closure decision, the system shall make the current contract lifecycle, validity, ceiling, relevant J02 supplier facts, performance/incidents, disputes, and contractual conditions available to the governed assessment. | L3-J-014 §§7, 9.2 |
| L4-J-014-BH-003 | The system shall distinguish pending sourcing outcomes and J04-owned Purchase Orders, deliveries, warranties, claims, securities, and other existing commitments from new Rate Contract sourcing authority. | L3-J-014 §§9.3, 9.8, 12 |
| L4-J-014-BH-004 | When the recommendation is prepared, the system shall preserve the proposed action scope, reason, evidence, effective time, temporary duration where applicable, notice basis, affected new sourcing, residual demand, and recovery or successor path. | L3-J-014 §§9.5–9.7, 15.2 |
| L4-J-014-BH-005 | When an authorised partial suspension applies, the system shall prevent new sourcing only within the specified affected supplier, line, category, location, or call-off scope while keeping unaffected parent-supported scope distinguishable. | L3-J-014 §§10.1, 12 |
| L4-J-014-BH-006 | When valid urgent provisional authority applies, the system shall prevent new use within the authorised protective scope pending the final decision without inventing a provisional Rate Contract lifecycle state. | L3-J-014 §§9.6, 10.2, 11.1, 13 |
| L4-J-014-BH-007 | When the Procurement Approver authorises suspension, the system shall place the affected Rate Contract availability in `Suspended` from the authorised effective time and block new call-off sourcing within that scope. | L3-J-014 §§9.11–9.12, 12, 13, 16 |
| L4-J-014-BH-008 | When authorised closure or expiry becomes effective, the system shall place the affected Rate Contract availability in `Expired / Closed` as applicable and prevent it from authorising new call-offs. | L3-J-014 §§9.11, 9.13, 10.4–10.7, 12, 13, 16 |
| L4-J-014-BH-009 | Suspension or closure shall not by itself cancel, amend, hold, or otherwise change a J04-owned Purchase Order; the system shall make the J01 availability fact available to J04 for its own governed disposition. | L3-J-014 §§5, 9.8, 11.3, 12, 14.2 |
| L4-J-014-BH-010 | When suspension or closure affects pending uncommitted sourcing, the system shall permit only the parent-supported cancellation, return, redirection, successor, alternate-sourcing, or demand-change disposition. | L3-J-014 §§9.14–9.15 |
| L4-J-014-BH-011 | The system shall keep residual demand visible and shall not treat suspension, expiry, or closure as fulfilment of that demand. | L3-J-014 §§9.15, 12 |
| L4-J-014-BH-012 | A `Suspended` Rate Contract shall not return to available use merely because time elapsed or a restriction indicator changed; reinstatement requires the separate attributable, approved, effective-dated decision required by the parent. | L3-J-014 §§10.3, 10.8, 11.6, 12 |
| L4-J-014-BH-013 | A `Closed` controlled Rate Contract version shall not be reopened for new use; future need shall follow a source-supported successor or new arrangement. | L3-J-014 §§11.6, 12 |
| L4-J-014-BH-014 | The system shall preserve historical contract terms, earlier availability, call-offs, decisions, suspension intervals, reasons, actions, and versions without destructive overwrite. | L3-J-014 §§9.17, 11.7, 12, 15.3 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-014-BH-015 | When the Rate Contract enters the governed suspended lifecycle position, the system shall publish `RateContractSuspended` with Procurement Planning & Sourcing (J01) as the canonical publisher. | L3-J-014 §14.1; L2-J01 §§27.3, 35.10 |
| L4-J-014-BH-016 | When the Rate Contract reaches the governed expiry fact, the system shall publish `RateContractExpired` with Procurement Planning & Sourcing (J01) as the canonical publisher. | L3-J-014 §14.1; L2-J01 §35.10 |
| L4-J-014-BH-017 | When the Rate Contract reaches the governed closed fact, the system shall publish `RateContractClosed` with Procurement Planning & Sourcing (J01) as the canonical publisher. | L3-J-014 §14.1; L2-J01 §35.10 |

J02 remains the canonical owner of supplier restriction/suspension/blacklisting facts. J03/requesting units retain demand authority. J04 retains Purchase Order lifecycle. Finance retains financial truth.

---

## 6. Failure / Rejection / Continuation

- If the ordinary suspension/closure trigger is not sufficiently verified, permanent action shall remain unapproved; only a valid parent-supported urgent protective restriction may apply.
- If the affected scope cannot be established, the system shall withhold new use only within the authorised risk boundary until source-supported clarification is available.
- A suspension or closure shall not silently change existing J04 commitments even when downstream review is required.
- Any post-effective attempt to create new call-off sourcing against affected availability shall be prevented.
- A reinstatement attempt that lacks the required authoritative decision shall not restore availability.
- New evidence discovered after closure shall be preserved as supplemental governed evidence and shall not erase the historical closure.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- Rate Contract identity/version, lifecycle position, validity, suppliers, scope/catalogue, locations, rates, ceilings, and call-off rules;
- the trigger, evidence source, reason, discovery time, proposed and approved effective time;
- relevant supplier status/performance, incidents, legal/policy/commercial facts;
- pending sourcing outcomes, J04 commitment references, deliveries, disputes, warranties, securities, and residual demand;
- suspension/closure scope, urgency basis, notice/representation/remediation evidence, successor path, and downstream disposition;
- recommendation, approval/return/rejection/reinstatement/closure decision evidence supported by the parent;
- `RateContractSuspended`, `RateContractExpired`, and `RateContractClosed` facts where applicable.

---

## 8. Observable Completion Conditions

Successful completion is observable when:

1. an authoritative suspension or closure decision identifies the affected Rate Contract scope, reason, evidence, authority, and effective time;
2. the affected availability reaches `Suspended` or `Expired / Closed` as applicable;
3. new call-off sourcing within affected scope is blocked from the governed effective time;
4. existing J04 commitments remain under J04 governance and are not silently changed by J01;
5. pending uncommitted sourcing and residual demand have an explicit parent-supported disposition;
6. historical contract and decision evidence remains reconstructable;
7. applicable canonical J01 suspension/expiry/closure events are published.

---

## 9. Traceability & Provenance

**Parent:** L3-J-014 — Active Rate Contract to Governed Suspension or Closure  
**Primary verification source:** L2-J01 — Procurement Planning & Sourcing Workflows  
**Additional verification sources:** L2-J00 — Enterprise Resource Management Architecture & Governance; L2-J02 — Vendor & Supplier Management Workflows; L2-J03 — Purchase Requisition & Approval Workflows; locked L1 Business Domain Architecture.  

Every BH is derived from the Locked parent. L2 verification confirms J01 Rate Contract ownership/lifecycle, J02 supplier ownership, J03 demand ownership, and exact J01 event names.

---

## 10. Open Issues / Deferred Design

- Institution-specific notice, representation, urgent-action, remediation, and appeal requirements remain upstream-governed.
- The parent retains an unresolved boundary question between J01 Rate Contract suspension and J02 supplier restriction; this L4 consumes J02 facts without transferring supplier lifecycle ownership.
- Treatment of approved but uncommitted call-off outcomes at suspension remains operating-profile governance; this L4 does not invent a universal disposition.
- The parent lists `Section_J_Canonical_Register.md`; that exact dependency has not been retrieved in the current repository checks, so no behavior is inferred from an assumed substitute.
- API/interface design, persistence/data model, service/component design, event transport, technical access control, retry/transaction mechanics, infrastructure, and UI/UX remain deferred to L5/L6.

**Review Verdict:** PASS WITH NON-BLOCKING FINDINGS
