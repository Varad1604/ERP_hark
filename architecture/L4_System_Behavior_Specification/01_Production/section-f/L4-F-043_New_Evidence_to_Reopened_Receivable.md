# New Evidence to Reopened Receivable

## 1. Specification Metadata

**Specification ID:** L4-F-043  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Accounts Receivable  
**Realizes Scenario:** L3-F-043  
**Primary Business Owner:** Accounts Receivable  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-F-043_New_Evidence_to_Reopened_Receivable.md`; `L2-F07_Accounts_Receivable_Management.md`; `L2-F07_Accounts_Receivable_Management_Amendment_2026-08-21.md`; applicable authoritative Finance source architecture  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall reopen the same previously resolved Receivable only when new authoritative Finance evidence establishes that an outstanding receivable position exists again, while preserving all prior settlement, adjustment, write-off, and closure history.

The behavior begins when AR receives new authoritative Finance evidence concerning a previously resolved Receivable. It ends when the same Receivable has been validly reopened, its current outstanding position is established, and `ReceivableReopened` has been published.

This specification does not create or correct the source Finance fact, rewrite prior resolution history, create an unrelated replacement Receivable, or prescribe technical reprocessing.

---

## 3. Required Business Context

The behavior requires:

- a previously resolved/closed Receivable;
- new authoritative and traceable Finance evidence;
- a determinable effect on the Receivable position;
- available prior resolution history;
- Reopening Review Authority;
- Receivable Validation Authority.

The triggering source Finance capability retains ownership of the authoritative fact causing reopening assessment.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-043-BH-001 | When reopening assessment begins, the system shall identify the previously resolved Receivable and the new authoritative source fact. | L3-F-043 §§4.1, 7-9 step 1 |
| L4-F-043-BH-002 | The system shall require Receivable Validation Authority validation of source reference, resulting outstanding position, and AR integrity. | L3-F-043 §§6, 9 step 2 |
| L4-F-043-BH-003 | The system shall require Reopening Review Authority confirmation that the new authoritative evidence justifies reopening. | L3-F-043 §§6, 9 step 3, 12 |
| L4-F-043-BH-004 | When reopening is justified, the system shall reopen the same Receivable identity rather than create an unrelated replacement. | L3-F-043 §§3, 9 step 4, 12 |
| L4-F-043-BH-005 | The system shall establish the current outstanding balance and collection context only from authoritative Finance evidence. | L3-F-043 §§3, 9 step 5 |
| L4-F-043-BH-006 | The system shall preserve prior settlement, adjustment, write-off, and closure history unchanged. | L3-F-043 §§2-4, 9 step 6, 12 |
| L4-F-043-BH-007 | The system shall preserve the same Receivable identity and originating-obligation lineage across reopening. | L3-F-043 §§3, 12, 16 |
| L4-F-043-BH-008 | The system shall not invent an intermediate reopening state beyond existing F07 reopening semantics. | L3-F-043 §13 |
| L4-F-043-BH-009 | If the evidence does not justify an outstanding position, the system shall retain the Receivable in its prior resolved position and preserve the review decision. | L3-F-043 §10 |
| L4-F-043-BH-010 | AR shall not create or alter the source Finance fact used to justify reopening. | L3-F-043 §§4.4-5, 6, 12 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-F-043-BH-011 | When the Receivable has been validly reopened, the system shall publish `ReceivableReopened` with Accounts Receivable as canonical publisher. | L3-F-043 §§4.2, 9 step 7, 14 |
| L4-F-043-BH-012 | The triggering source fact shall remain owned and published by its canonical Finance source. | L3-F-043 §§5, 14 |

---

## 6. Failure / Rejection / Continuation

Uncertain, incomplete, or contradictory source evidence shall prevent reopening. AR shall retain the prior valid resolved position until the source-owning Finance workflow establishes an authoritative fact. No technical retry, reload, or reprocessing mechanism is specified.

---

## 7. Audit & Evidence Behavior

The system shall preserve prior resolution state/evidence, new source fact and publisher, validation outcome, Reopening Review Authority participation, resulting outstanding position, unchanged prior history, same Receivable identity/source lineage, and `ReceivableReopened`.

---

## 8. Observable Completion Conditions

Completion requires the same Receivable to be reopened on authoritative evidence, current outstanding position to be valid, prior history to remain immutable, source ownership unchanged, and `ReceivableReopened` plus reopening evidence to be reconstructable.

---

## 9. Traceability & Provenance

**Parent:** `L3-F-043` — New Evidence to Reopened Receivable  
**Primary verification source:** `L2-F07` — Accounts Receivable Management  
**Additional verification source:** applicable authoritative Finance source architecture.

No behavior creates a new reopening authority, source fact, intermediate state, event, publisher, or Receivable replacement identity.

---

## 10. Open Issues / Deferred Design

No behavior-specific L4 issue is identified from the parent semantics.

API/interface design, persistence, source-event transport, technical reprocessing, access-control implementation, infrastructure, and UI/UX remain deferred.

---

**Draft Validation Classification:** Prepared as an L4 Draft for architecture review.
