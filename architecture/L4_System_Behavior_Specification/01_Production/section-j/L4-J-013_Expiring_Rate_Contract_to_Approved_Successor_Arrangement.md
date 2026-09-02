# Expiring Rate Contract to Approved Successor Arrangement

## 1. Specification Metadata

**Specification ID:** L4-J-013  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Procurement Planning & Sourcing  
**Realizes Scenario:** L3-J-013  
**Primary Business Owner:** Procurement Planning & Sourcing  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-013_Expiring_Rate_Contract_to_Approved_Successor_Arrangement.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L2-J03_Purchase_Requisition_and_Approval_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall govern the transition from an expiring Rate Contract toward a separately identified successor arrangement while preserving the predecessor's immutable history, preventing unauthorised post-cut-off use, and keeping predecessor and successor authority distinct.

The behavior begins when an `Active` predecessor Rate Contract enters its governed expiry-review window or continuing need is authoritatively identified before expiry.

**Upstream blocking condition:** the parent L3 defines two materially different successful end positions. Section 4.2 permits completion when the successor reaches `Approved` **or** `Active`, while Section 16.2 states that the successor Rate Contract **is Active** as a postcondition. L4 cannot choose which lifecycle position constitutes the canonical successful end boundary without silently repairing the Locked parent.

All behavior below is limited to unambiguous parent-supported obligations and does not resolve that completion conflict.

This specification does not govern ordinary demand call-off, final predecessor closure, supplier lifecycle, Purchase Order lifecycle, receipt, payable, accounting, payment, or technical migration mechanics.

---

## 3. Required Business Context

- An identifiable predecessor Rate Contract that is `Active` or within the parent-supported expiry-review context and has a known end date.
- Continuing institutional need, forecast, scope, locations, and transition risk facts.
- Predecessor utilisation, supplier performance, open call-offs, ceilings, disputes, commercial history, and existing commitment references.
- Current J02 supplier eligibility and performance facts.
- Applicable renewal, extension, re-sourcing, competition, notice, transition, and approval rules.
- Any J04-owned open Purchase Order or commitment facts relevant to transition, without transfer of J04 ownership.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-013-BH-001 | When the predecessor enters its governed expiry-review window or continuing need is identified, the system shall preserve the predecessor identity, version, validity, suppliers, catalogue/scope, rates, ceilings, locations, call-off rules, and open-position references used for successor review. | L3-J-013 §§4.1, 7, 8, 9.1 |
| L4-J-013-BH-002 | The system shall preserve the continuing-need, forecast, utilisation, supplier-performance, incident, dispute, capacity, eligibility, market, rate, and scope facts considered in the successor decision. | L3-J-013 §§9.2–9.3, 15.2 |
| L4-J-013-BH-003 | When a successor strategy is selected, the system shall record the governed method, aggregate value basis, competition path, transition basis, predecessor cut-off, contingency position, and applicable approval context without altering the predecessor identity. | L3-J-013 §§9.4–9.5, 12 |
| L4-J-013-BH-004 | The system shall not carry forward predecessor suppliers, ranks, rates, terms, or ceilings into the successor merely because they existed in the predecessor arrangement. | L3-J-013 §§9.7–9.8, 12 |
| L4-J-013-BH-005 | When an extension or interim arrangement is proposed, the system shall require the parent-supported policy or legal basis, bounded duration, scope, rates, ceiling, supplier, reason, value, and approval authority before recognising that arrangement. | L3-J-013 §§9.9, 10.2–10.3, 11.1–11.2, 12 |
| L4-J-013-BH-006 | The system shall keep predecessor and successor Rate Contract identities, versions, suppliers, rates, ceilings, validity periods, and authority distinct throughout transition. | L3-J-013 §§3, 9.10, 9.14, 12 |
| L4-J-013-BH-007 | The system shall distinguish existing J04-owned predecessor commitments from authority for new call-off sourcing and shall not treat contract expiry as cancellation of those commitments. | L3-J-013 §§9.11, 12; §14.2 |
| L4-J-013-BH-008 | When controlled overlap is authorised, the system shall preserve the approved overlap scope, precedence, ceilings, call-off eligibility, and end date; otherwise ambiguous or conflicting overlap shall prevent successor activation. | L3-J-013 §§10.4, 11.5, 12 |
| L4-J-013-BH-009 | At the predecessor's expiry or authorised earlier cut-off, the system shall prevent new predecessor call-off sourcing except where an expressly authorised transition condition remains valid. | L3-J-013 §§9.16, 10.5, 12 |
| L4-J-013-BH-010 | The system shall preserve predecessor history and shall not destructively rewrite it when a successor, extension, interim arrangement, or post-approval successor change is governed. | L3-J-013 §§3, 9.14, 11.6, 12 |
| L4-J-013-BH-011 | When no successor is authorised, the system shall preserve the no-successor, gap, alternate-sourcing, or discontinuation disposition supported by the parent instead of silently extending the predecessor. | L3-J-013 §§4.2, 10.5–10.6, 12 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-013-BH-012 | When the governed renewal/successor business fact is created, the system shall publish `RateContractRenewalCreated` with Procurement Planning & Sourcing (J01) as the canonical publisher. | L3-J-013 §14.1; L2-J01 §35.10 |
| L4-J-013-BH-013 | When a distinct successor Rate Contract is created, the system shall publish `RateContractCreated` with Procurement Planning & Sourcing (J01) as the canonical publisher. | L3-J-013 §14.1; L2-J01 §35.10 |
| L4-J-013-BH-014 | When the successor Rate Contract is authoritatively approved, the system shall publish `RateContractApproved` with Procurement Planning & Sourcing (J01) as the canonical publisher. | L3-J-013 §14.1; L2-J01 §35.10 |
| L4-J-013-BH-015 | If the successor subsequently reaches the parent-supported `Active` lifecycle position, the system shall publish `RateContractActivated` with Procurement Planning & Sourcing (J01) as the canonical publisher. | L3-J-013 §§9.15, 14.1; L2-J01 §§27.3, 35.10 |
| L4-J-013-BH-016 | When the predecessor reaches the applicable expiry business fact, the system shall publish `RateContractExpired` with Procurement Planning & Sourcing (J01) as the canonical publisher. | L3-J-013 §§13, 14.1; L2-J01 §35.10 |
| L4-J-013-BH-017 | When the predecessor reaches the applicable closed business fact, the system shall publish `RateContractClosed` with Procurement Planning & Sourcing (J01) as the canonical publisher. | L3-J-013 §§13, 14.1; L2-J01 §35.10 |

J02 retains supplier truth, J03/requesting domains retain demand authority, Finance retains financial truth, and J04 retains Purchase Order and commitment authority.

---

## 6. Failure / Rejection / Continuation

- A late review or sourcing delay shall not automatically authorise extension or incumbent continuation; only a parent-supported alternate, interim, extension, or gap disposition may proceed.
- An extension lacking authoritative basis, justification, bounded duration, or approval shall be rejected and shall not alter the predecessor's originally governed expiry.
- A supplier performance, suspension, blacklisting, qualification, or risk fact shall affect successor eligibility only through the authoritative J02 fact and parent-supported sourcing decision.
- Failed successor sourcing may continue only through the parent-supported re-sourcing, interim, alternate-method, redesign, or gap-management outcome.
- Unclear overlap or combined ceiling authority shall prevent the affected successor activation until the source-supported authority is unambiguous.
- A material successor change after approval shall not destructively alter the approved successor or predecessor; a controlled source-supported successor decision is required.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- predecessor identity, version, validity, suppliers, catalogue/scope, rates, ceilings, utilisation, performance, and open commitment references;
- continuing need, forecast, revised scope, market assessment, and successor strategy;
- successor sourcing, responses, evaluation, and current supplier eligibility evidence;
- extension or interim basis, duration, value, rate, scope, supplier, reason, and authority where applicable;
- transition cut-off, overlap, gap, residual predecessor obligations, and intended successor effective date;
- recommendation, approval/return/rejection decisions, actors, reasons, timestamps, and predecessor-successor lineage;
- applicable J01 Rate Contract events.

---

## 8. Observable Completion Conditions

The unambiguous L4 behavior is observable when:

1. the predecessor and successor remain separately identified and historically traceable;
2. successor sourcing/extension/interim decisions are governed by current authoritative evidence rather than silent carry-forward;
3. predecessor cut-off and any controlled overlap or gap are explicit;
4. new predecessor call-off sourcing is prevented after the governed cut-off or expiry except under an expressly authorised transition condition;
5. existing J04-owned commitments remain separately governed;
6. the applicable successor approval and predecessor expiry/closure facts are preserved.

**Canonical successful completion cannot be finalised at L4 until the Locked parent resolves whether the required successor end position is `Approved` or `Active`.**

---

## 9. Traceability & Provenance

**Parent:** L3-J-013 — Expiring Rate Contract to Approved Successor Arrangement  
**Primary verification source:** L2-J01 — Procurement Planning & Sourcing Workflows  
**Additional verification sources:** L2-J00 — Enterprise Resource Management Architecture & Governance; L2-J02 — Vendor & Supplier Management Workflows; L2-J03 — Purchase Requisition & Approval Workflows; locked L1 Business Domain Architecture.  

The BHs above cover only parent-supported behavior unaffected by the unresolved completion-boundary conflict. No BH redefines the successful end state.

---

## 10. Open Issues / Deferred Design

### Blocking upstream issue

**BLOCKED BY UPSTREAM:** L3-J-013 §4.2 allows the scenario to end when the successor reaches `Approved` **or** `Active`, while §16.2 requires the successor Rate Contract to be `Active` as a successful postcondition. This materially affects the final outcome and end boundary. Under the locked L4 Validation Standard, choosing one interpretation locally would constitute an invalid parent baseline / silent upstream repair risk (HF-12 / HF-07).

Architecture change control must establish whether:
- `Approved` is a valid successful terminal position for this scenario with later activation outside its completion boundary; or
- `Active` is mandatory for successful completion.

Additional parent-level open issues remain institution-specific extension limits, treatment of pre-expiry call-offs delivered after expiry, and the boundary among extension, amendment, successor, interim arrangement, and emergency procurement.

The parent also lists `Section_J_Canonical_Register.md`; that exact dependency has not been retrieved in the current repository checks, so no behavior is inferred from an assumed substitute.

API/interface design, persistence/data model, service/component design, event transport, technical access control, retry/transaction mechanics, infrastructure, and UI/UX remain deferred to L5/L6.

**Review Verdict:** BLOCKED BY UPSTREAM
