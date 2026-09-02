# Published Financial Obligation to Accounts Receivable Recognition

## 1. Specification Metadata

**Specification ID:** L4-F-039  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Accounts Receivable  
**Realizes Scenario:** L3-F-039  
**Primary Business Owner:** Accounts Receivable  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-F-039_Published_Financial_Obligation_to_Accounts_Receivable_Recognition.md`; `L2-F07_Accounts_Receivable_Management.md`; `L2-F07_Accounts_Receivable_Management_Amendment_2026-08-21.md`; applicable source-obligation L2 architecture  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall establish an Accounts Receivable-owned Receivable representation only after an authoritative source domain has established a financial obligation, while preserving the source obligation under its canonical owner.

The behavior begins when AR receives an authoritative source-owned obligation-established fact. It ends when one Receivable has been registered and validated with an immutable reference to exactly one originating obligation, opening balance and classification are established, and `ReceivableCreated` has been published.

This specification does not create or alter the source obligation, collect payment, post accounting entries, approve write-off, or prescribe technical integration.

---

## 3. Required Business Context

The behavior requires an existing source-owned financial obligation, an authoritative and traceable obligation-established fact, source reference, amount, institution, classification, sufficient validation information, no duplicate Receivable for the same obligation, Receivable Registration Authority, and Receivable Validation Authority.

For Student Charge recognition, the exact trigger is `ChargeEstablished`, published by Student Charge Management. Other source classes may use only their exact upstream canonical event; if none is named, the event name remains unresolved.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-039-BH-001 | When AR receives an obligation-established fact, the system shall preserve the originating obligation owner and shall not treat AR as source owner. | L3-F-039 §§2-5, 9 step 1, 12 |
| L4-F-039-BH-002 | The system shall permit recognition only when the source fact is authoritative and traceable. | L3-F-039 §§4.1, 7-9 |
| L4-F-039-BH-003 | The system shall require validation of source reference, amount, institution, classification, and Receivable integrity before recognition completes. | L3-F-039 §§6-7, 9 steps 2 and 5 |
| L4-F-039-BH-004 | The system shall prevent a duplicate Receivable for the same originating obligation. | L3-F-039 §7 |
| L4-F-039-BH-005 | Every recognized Receivable shall preserve an immutable reference to exactly one originating obligation. | L3-F-039 §§3, 9 step 3, 12 |
| L4-F-039-BH-006 | When validation permits registration, the system shall create the Receivable under AR ownership without modifying the source obligation. | L3-F-039 §§3, 5, 9 step 3 |
| L4-F-039-BH-007 | The system shall establish opening balance and applicable AR classification from validated source context. | L3-F-039 §§3, 9 step 4, 16 |
| L4-F-039-BH-008 | Registration and validation shall remain independently governable responsibilities. | L3-F-039 §§6, 12, 15 |
| L4-F-039-BH-009 | AR actors shall not alter Student Charge, Payment, Refund, Financial Adjustment, Accounting, Treasury, or another source aggregate during recognition. | L3-F-039 §§4.4-5, 11-12 |
| L4-F-039-BH-010 | The system shall preserve source-to-Receivable lineage after successful recognition. | L3-F-039 §§3, 9 step 6, 15-16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-F-039-BH-011 | For Student Charge recognition, the system shall consume `ChargeEstablished` only from Student Charge Management as the canonical initial trigger. | L3-F-039 §§10.1, 14 |
| L4-F-039-BH-012 | The system shall not use `ChargeRevised` as a substitute for initial Student Charge Receivable recognition. | L2-F07 Amendment §2.1 |
| L4-F-039-BH-013 | For other obligation classes, the system shall consume only the exact canonical obligation-established event published by the source owner. | L3-F-039 §§10.2, 12, 14 |
| L4-F-039-BH-014 | If the source has not canonically named that event, the trigger name shall remain unresolved rather than invented. | L3-F-039 §§10.2, 17 |
| L4-F-039-BH-015 | When the Receivable has been successfully established, the system shall publish `ReceivableCreated` with Accounts Receivable as canonical publisher. | L3-F-039 §§9 step 6, 14, 16 |

---

## 6. Failure / Rejection / Continuation

Invalid, incomplete, duplicate, or untraceable source evidence shall prevent recognition. If source correction is required, AR shall return the matter to the source-owning workflow and shall not directly correct the source aggregate. Reassessment occurs only after authoritative corrected source evidence exists. No invented event names or technical retry/rollback behavior is defined.

---

## 7. Audit & Evidence Behavior

Evidence shall preserve source owner, source obligation reference, exact consumed event/fact and publisher, Receivable identity, registration and validation authority participation, validated amount/institution/classification/opening balance, duplicate-prevention result, immutable source relationship, and `ReceivableCreated`.

---

## 8. Observable Completion Conditions

Completion requires exactly one Receivable for the source obligation, one immutable originating-obligation reference, completed AR validation, established opening balance/classification, unchanged source ownership, published `ReceivableCreated`, and reconstructable lineage.

---

## 9. Traceability & Provenance

**Parent:** `L3-F-039` — Published Financial Obligation to Accounts Receivable Recognition  
**Primary verification source:** `L2-F07` — Accounts Receivable Management  
**Material amendment:** `L2-F07_Accounts_Receivable_Management_Amendment_2026-08-21.md`  
**Additional verification source:** applicable source-obligation L2 architecture  

The locked F07 amendment verifies AR ownership boundaries, Registration Authority, Validation Authority, exact Student Charge trigger `ChargeEstablished`, one-originating-obligation invariant, and prohibition on inventing unresolved source-event names.

---

## 10. Open Issues / Deferred Design

The exact obligation-established event remains unresolved for source classes whose canonical source L2 has not named it. API/interface design, persistence/data model, deduplication implementation, event transport, technical access control, infrastructure, and UI/UX remain deferred.

---

**Draft Validation Classification:** Prepared as an L4 Draft for architecture review.
