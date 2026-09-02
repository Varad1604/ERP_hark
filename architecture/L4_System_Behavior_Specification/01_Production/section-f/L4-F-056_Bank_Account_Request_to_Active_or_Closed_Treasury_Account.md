# Bank Account Request to Active or Closed Treasury Account

## 1. Specification Metadata

**Specification ID:** L4-F-056  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Treasury & Banking  
**Realizes Scenario:** L3-F-056  
**Primary Business Owner:** Treasury & Banking  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-F-056_Bank_Account_Request_to_Active_or_Closed_Treasury_Account.md`; `L2-F10_Treasury_and_Banking.md`; `L2-F10_Treasury_and_Banking_Amendment_2026-08-21.md`; authoritative institution/organisation architecture  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall govern establishment, activation, and eventual closure of the internal Treasury Account under the canonical F10 lifecycle while preserving external bank infrastructure and bank-originated records outside SutraERP ownership.

The behavior begins when an authorized institutional requirement exists to establish a Treasury Account. It ends when the Treasury Account reaches `Active`, or later reaches terminal `Closed` after governed closure checks.

This specification does not own external bank accounts, banking APIs, payment execution, settlement, ledger posting, or technical account provisioning.

---

## 3. Required Business Context

The behavior requires valid institution/Treasury context, traceable banking references, required currency/account context, independently governable request/verification/approval/closure responsibilities, and the canonical Treasury Account lifecycle.

F10 solely owns the Treasury Account business concept and lifecycle. Section B supplies institution/organisation references only. External banks remain authoritative for external bank infrastructure.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-056-BH-001 | The system shall permit the Treasury Account Requestor to submit a governed establishment request with supporting context. | L3-F-056 §§6, 8-9 step 1 |
| L4-F-056-BH-002 | The system shall require Treasury Account Verifier confirmation of institution, Treasury, currency, and banking-reference completeness before approval progression. | L3-F-056 §§6-7, 9 step 2 |
| L4-F-056-BH-003 | The system shall require Treasury Account Approval Authority approval before registration/activation progression. | L3-F-056 §§6, 9 step 3, 12 |
| L4-F-056-BH-004 | Initiation or verification alone shall not constitute approval. | L3-F-056 §§6, 12 |
| L4-F-056-BH-005 | On approved establishment, the system shall create the Treasury Account in `Registered`. | L3-F-056 §§9 step 4, 12-13 |
| L4-F-056-BH-006 | When registration completes, the system shall publish `TreasuryAccountRegistered` with Treasury & Banking as canonical publisher. | L3-F-056 §§9 step 4, 14 |
| L4-F-056-BH-007 | The system shall prevent ordinary Treasury use while the account remains `Registered`. | L2-F10 Amendment §2 |
| L4-F-056-BH-008 | After required verification/approval gates, the system shall permit the Treasury Account Administrator to move `Registered → Active`. | L3-F-056 §§9 step 5, 12-13 |
| L4-F-056-BH-009 | When activation completes, the system shall publish `TreasuryAccountActivated` with Treasury & Banking as canonical publisher. | L3-F-056 §§9 step 5, 14 |
| L4-F-056-BH-010 | The system shall permit an `Active` Treasury Account to participate only in Treasury operations allowed by governing restrictions. | L3-F-056 §§9 step 6, 10, 12 |
| L4-F-056-BH-011 | Before `Active → Closed`, the system shall require closure checks confirming no prohibited pending Treasury activity, addressed settlement/reconciliation responsibilities, traceable references, and recorded closure authorization. | L3-F-056 §§9 step 7, 12, 15 |
| L4-F-056-BH-012 | The system shall require Treasury Account Closure Authority before `Active → Closed`. | L3-F-056 §§6, 9 step 7 |
| L4-F-056-BH-013 | When closure completes, the system shall move the Treasury Account to terminal `Closed` and publish `TreasuryAccountClosed`. | L3-F-056 §§3, 9 step 8, 13-14 |
| L4-F-056-BH-014 | The system shall preserve historical identity, references, activity, and audit evidence after closure. | L3-F-056 §§9 step 9, 12, 15-16 |
| L4-F-056-BH-015 | Treasury-wide `Restricted` or `Suspended` conditions shall not be duplicated as Treasury Account lifecycle states. | L3-F-056 §§10, 12 |

---

## 5. Cross-Domain & Event Behavior

`TreasuryAccountRegistered`, `TreasuryAccountActivated`, and `TreasuryAccountClosed` are published solely by Treasury & Banking. Section B references remain externally owned. External bank records shall not be modified or re-owned by Treasury Account lifecycle behavior.

---

## 6. Failure / Rejection / Continuation

A rejected establishment request shall not reach `Registered`. Missing verification, approval, or closure evidence shall block the relevant transition. Recovery requires completing the missing governed business prerequisite. No technical retry, rollback, concurrency, persistence, or publication-recovery mechanics are specified.

---

## 7. Audit & Evidence Behavior

The system shall preserve request identity, requester, verification evidence, approval decision, Treasury Account identity, activation evidence, applicable restrictions, closure checks, Closure Authority decision, lifecycle transitions, canonical events, and external banking references.

---

## 8. Observable Completion Conditions

The behavior is complete when the Treasury Account has reached the applicable canonical outcome (`Active` or terminal `Closed`) with required authorities/evidence, external bank ownership unchanged, and complete lifecycle evidence reconstructable.

---

## 9. Traceability & Provenance

**Parent:** `L3-F-056` — Bank Account Request to Active or Closed Treasury Account  
**Primary verification source:** `L2-F10` — Treasury & Banking  
**Locked amendment:** `L2-F10_Treasury_and_Banking_Amendment_2026-08-21.md`

The locked amendment verifies the canonical lifecycle `Registered → Active → Closed`, actors/authorities, events, closure checks, and the distinction between the internal Treasury Account and external bank infrastructure.

---

## 10. Open Issues / Deferred Design

Exact institution-specific closure thresholds/checklists remain policy-governed.

API/interface design, bank provisioning, persistence, technical access control, banking integration, infrastructure, and UI/UX remain deferred.

---

**Draft Validation Classification:** Prepared as an L4 Draft for architecture review.
