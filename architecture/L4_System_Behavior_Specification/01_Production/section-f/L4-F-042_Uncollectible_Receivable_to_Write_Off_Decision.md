# Uncollectible Receivable to Write-Off Decision

## 1. Specification Metadata

**Specification ID:** L4-F-042  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Accounts Receivable  
**Realizes Scenario:** L3-F-042  
**Primary Business Owner:** Accounts Receivable  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-F-042_Uncollectible_Receivable_to_Write_Off_Decision.md`; `L2-F04_Financial_Adjustment_Management.md`; `L2-F07_Accounts_Receivable_Management.md`; `L2-F07_Accounts_Receivable_Management_Amendment_2026-08-21.md`; `L3-F-022`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall govern Accounts Receivable assessment and referral of a potentially uncollectible Receivable into the applicable Finance write-off process without allowing AR to approve, create, or apply the Write-Off Adjustment.

The behavior begins when an outstanding Receivable is identified as potentially uncollectible under applicable Finance policy. It ends when AR has either referred the Receivable for write-off governance with complete evidence or declined/deferred the referral with reasons preserved.

This specification does not approve or apply a Financial Adjustment, change the source obligation, post accounting entries, or execute Treasury activity.

---

## 3. Required Business Context

The behavior requires:

- a valid outstanding Receivable;
- collection/outstanding evidence;
- applicable write-off eligibility policy;
- traceable source obligation and Receivable lineage;
- Write-Off Referral Authority;
- Receivable Review Authority;
- applicable external write-off approval governance.

Financial Adjustment Management owns any resulting Write-Off Adjustment. For Student-Charge write-off, Board approval remains required under the locked Finance rule.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-042-BH-001 | When write-off assessment begins, the system shall validate the current outstanding amount and available collection history. | L3-F-042 §§7-9 step 1 |
| L4-F-042-BH-002 | The system shall permit the Write-Off Referral Authority to evaluate applicable eligibility policy and supporting evidence. | L3-F-042 §§6, 9 step 2 |
| L4-F-042-BH-003 | The system shall preserve the source obligation unchanged during AR referral assessment. | L3-F-042 §§4.4-5, 9 step 3 |
| L4-F-042-BH-004 | If referral criteria are satisfied, the system shall record a write-off referral containing Receivable identity, amount, reason, and supporting evidence. | L3-F-042 §§3, 9 step 4, 15 |
| L4-F-042-BH-005 | The system shall hand the referral to the canonical Financial Adjustment/write-off governance without transferring write-off ownership to AR. | L3-F-042 §§2-5, 9 step 5, 12 |
| L4-F-042-BH-006 | The system shall preserve the AR referral decision while awaiting an authoritative Finance write-off outcome. | L3-F-042 §§9 step 6, 16 |
| L4-F-042-BH-007 | The system shall not move the Receivable to `Written Off` merely because referral occurred. | L3-F-042 §§13, 16 |
| L4-F-042-BH-008 | The system shall permit `ReceivableWrittenOff` only after an authoritative approved/applied Finance write-off fact has been received and incorporated by AR. | L3-F-042 §§13-14 |
| L4-F-042-BH-009 | If referral criteria are not satisfied, the system shall retain the existing Receivable position and preserve the declined/deferred reason. | L3-F-042 §10 |
| L4-F-042-BH-010 | The system shall not permit AR to create, approve, or apply the Write-Off Adjustment. | L3-F-042 §§2, 4.4, 6, 12 |

---

## 5. Cross-Domain & Event Behavior

No new write-off-referral event shall be invented. After the authoritative Finance write-off is applied and AR incorporates that fact, the existing `ReceivableWrittenOff` event may represent the completed AR-owned outcome.

Financial Adjustment events and approval outcomes remain owned and published by Financial Adjustment Management and applicable approval governance.

---

## 6. Failure / Rejection / Continuation

Incomplete or contradictory evidence shall prevent a positive referral decision. The Receivable shall remain unchanged until the evidence is corrected or clarified. Collection governance may continue under the overdue/dunning scenario. No technical retry or rollback is specified.

---

## 7. Audit & Evidence Behavior

The system shall preserve Receivable identity, outstanding amount, collection history, applicable eligibility policy, Write-Off Referral Authority participation, referral decision/reason, Financial Adjustment hand-off reference, and evidence that AR did not approve or execute the write-off.

Actual write-off approval/application evidence remains owned by its canonical process.

---

## 8. Observable Completion Conditions

Completion requires a governed and traceable AR referral decision, preservation of the current Receivable position unless authoritative write-off outcome exists, no self-authorization by AR, and clear hand-off to Financial Adjustment governance where referral is approved.

---

## 9. Traceability & Provenance

**Parent:** `L3-F-042` — Uncollectible Receivable to Write-Off Decision  
**Primary verification source:** `L2-F07` — Accounts Receivable Management  
**Additional verification source:** `L2-F04` — Financial Adjustment Management and `L3-F-022`.

The parent explicitly preserves the distinction between referral and actual write-off. Student-Charge write-off Board approval is not generalized to unrelated classes.

---

## 10. Open Issues / Deferred Design

No behavior-specific L4 issue is identified from the parent semantics.

API/interface design, persistence, eligibility-calculation implementation, workflow transport, technical retries, approval-system mechanics, infrastructure, and UI/UX remain deferred.

---

**Draft Validation Classification:** Prepared as an L4 Draft for architecture review.
