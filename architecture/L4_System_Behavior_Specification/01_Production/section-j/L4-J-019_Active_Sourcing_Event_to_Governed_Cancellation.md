# Active Sourcing Event to Governed Cancellation

## 1. Specification Metadata

**Specification ID:** L4-J-019  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Procurement Planning & Sourcing  
**Realizes Scenario:** L3-J-019  
**Primary Business Owner:** Procurement Planning & Sourcing  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-019_Active_Sourcing_Event_to_Governed_Cancellation.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L2-J03_Purchase_Requisition_and_Approval_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`  
**Referenced By:** Downstream demand release, re-sourcing, bidder communication, security release, supplier-governance, commitment-prevention, reporting, and audit specifications where applicable.

---

## 2. Behavioral Objective & Boundary

SutraERP shall govern cancellation of an active J01 sourcing event, or an independently separable part of it, before a valid downstream commitment exists.

The behavior begins when an authorised cancellation proposal identifies a material permitted ground affecting active sourcing scope. It ends when cancellation is approved or rejected, the J01 lifecycle position and downstream controls are explicit, bidder and stakeholder treatment is attributable, demand and monetary/custodial hand-offs are preserved, and historical sourcing evidence remains immutable.

This specification does not terminate a valid J04 commitment, change J03-owned demand, change J02 Supplier lifecycle, alter Finance-owned monetary facts, or delete or rewrite bids, evaluations, approvals, communications, or sourcing history.

---

## 3. Required Business Context

- Identifiable J01 Sourcing Event and affected event, lot, line, category, location, or demand scope.
- Current canonical J01 lifecycle position and event-stage evidence.
- Recorded cancellation ground, discovery chronology, materiality, and initiating authority.
- J03-owned demand status and requesting/technical owner position.
- Bidder submissions, evaluations, recommendations, notices, complaints, and security/fee/custody references where applicable.
- J04-owned commitment status for the affected scope.
- J02-owned supplier status and integrity facts where material.
- Finance/funding/custodial facts for securities, guarantees, fees, deposits, release, refund, and budget consequences.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-019-BH-001 | When cancellation is proposed, the system shall preserve the event identity, affected scope, ground, discovery time, urgency, evidence, and initiating authority. | L3-J-019 §§8, 9.1 |
| L4-J-019-BH-002 | Before a cancellation decision, the system shall make the current event lifecycle, publication/response history, lots, bidders, submissions, clarifications, opening, evaluation, recommendation, award notices, approvals, and any commitment attempt available for review. | L3-J-019 §§7, 9.2 |
| L4-J-019-BH-003 | When continued sourcing activity could prejudice bidders, create avoidable cost, or produce an unauthorised award, the system shall prevent new affected-scope progression pending the authorised cancellation decision without inventing a new lifecycle state. | L3-J-019 §§9.3, 13 |
| L4-J-019-BH-004 | The system shall require the proposed cancellation ground to be attributable to a permitted material basis and shall preserve its factual cause, chronology, responsibility, materiality, and affected parties. | L3-J-019 §§4.1, 9.4–9.5, 12 |
| L4-J-019-BH-005 | Before cancellation is approved, the system shall preserve the source-supported alternatives considered, including correction, clarification, extension, separable partial cancellation, suspension, reissue, revised demand, alternate method, or continuation. | L3-J-019 §§9.6, 10.5–10.6 |
| L4-J-019-BH-006 | The system shall preserve the authoritative demand-owner position on whether the need ended, changed, was deferred, remains urgent, or requires a successor without changing J03-owned demand itself. | L3-J-019 §§9.7, 12, 14.2 |
| L4-J-019-BH-007 | The system shall preserve bidder-impact treatment for equal treatment, confidentiality, challenge rights, securities, deposits, fees, samples, intellectual property, and submitted documents where applicable. | L3-J-019 §§9.8, 12, 15.2 |
| L4-J-019-BH-008 | The system shall preserve evaluation and recommendation evidence exactly as it stood before cancellation and shall not permit cancellation to erase an unfavourable result or expose protected competitive information. | L3-J-019 §§2, 9.9, 12 |
| L4-J-019-BH-009 | Where integrity, conflict, fraud, collusion, complaint, litigation, regulatory, or investigative concerns exist, the system shall preserve the referral and shall not treat cancellation as resolution of the separate authoritative matter. | L3-J-019 §§9.10, 10.7, 12 |
| L4-J-019-BH-010 | The system shall preserve Finance/custodial determinations for security, guarantee, deposit, fee, encashment, release, refund, budget, and monetary consequences without changing Finance-owned facts. | L3-J-019 §§9.11, 14.2 |
| L4-J-019-BH-011 | The cancellation recommendation shall identify exact affected scope, reason, authority, effective time, bidder treatment, demand disposition, successor/re-sourcing path, publication requirements, controls, and corrective actions. | L3-J-019 §§9.12, 15.2 |
| L4-J-019-BH-012 | When the Procurement Approver authorises cancellation, the system shall place only the approved affected J01 scope in `Cancelled` and preserve all prior lifecycle positions and versions. | L3-J-019 §§9.14–9.15, 13 |
| L4-J-019-BH-013 | After cancellation becomes effective, the system shall prevent new award or commitment progression for cancelled scope. | L3-J-019 §§4.3, 9.15, 11.6, 12 |
| L4-J-019-BH-014 | For partial or lot cancellation, the system shall limit the cancelled scope to objectively separable portions and preserve unaffected competition without changing its evaluation basis. | L3-J-019 §§10.5, 12 |
| L4-J-019-BH-015 | If cancellation is rejected or returned, the system shall preserve the existing canonical event position and permit only the attributable continuation, correction, suspension, or review path authorised by the parent. | L3-J-019 §§3, 11.1, 13 |
| L4-J-019-BH-016 | When re-sourcing follows cancellation, the system shall preserve predecessor lineage and require the successor sourcing event to have a distinct identity rather than reusing the cancelled event as a new event. | L3-J-019 §§10.8, 12 |
| L4-J-019-BH-017 | If a valid J04 commitment may already exist, the system shall prevent this scenario from treating sourcing cancellation as commitment termination and shall preserve the hand-off to the authoritative commitment owner. | L3-J-019 §§4.4, 11.2, 12 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-019-BH-018 | When an authorised sourcing cancellation becomes effective, the system shall publish `SourcingEventCancelled` with Procurement Planning & Sourcing (J01) as canonical publisher. | L3-J-019 §14.1; L2-J01 §35.13 |
| L4-J-019-BH-019 | When the sourcing event reaches its source-supported closure fact, the system shall publish `SourcingEventClosed` with Procurement Planning & Sourcing (J01) as canonical publisher. | L3-J-019 §14.1; L2-J01 §35.13 |

J03 retains demand lifecycle. J02 retains Supplier lifecycle. J04 retains Purchase Order/commitment lifecycle. Finance and custodial owners retain monetary and security truth. Legal, compliance, oversight, and investigative authorities retain their specialist decisions.

---

## 6. Failure / Rejection / Continuation

- If the cancellation ground is not sustained, the proposal shall not change the sourcing lifecycle; continuation requires an attributable decision addressing any temporary protective restriction and elapsed time.
- If a valid downstream commitment may exist, sourcing cancellation shall be held from being treated as commitment termination.
- Failed bidder notification shall remain an evidenced unresolved business obligation without selectively changing cancellation effect.
- Unresolved security, fee, refund, or custody disposition shall remain with its authoritative owner after sourcing cancellation is recorded.
- Activity attempted after effective cancellation shall not alter the cancelled sourcing outcome and shall remain attributable for investigation.
- Complaint, stay, review, remand, or later evidence shall create additional attributable facts and shall not erase cancellation history.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- sourcing-event identity, method, scope, demand references, publication/invitation, response, opening, evaluation, recommendation, and current lifecycle evidence;
- cancellation ground, discovery chronology, materiality, urgency, affected parties, and alternatives considered;
- demand-owner position, bidder impact, integrity/legal review, complaints, and commitment check;
- security, guarantee, fee, deposit, sample, submission, confidentiality, release/refund, and custody hand-offs where applicable;
- recommendation, approval/conditional approval/return/rejection, effective time, communications, publication, successor lineage, and corrective actions;
- exact canonical J01 events `SourcingEventCancelled` and `SourcingEventClosed` where their authoritative conditions occur.

---

## 8. Observable Completion Conditions

The L4 behavior is complete when:

1. the cancellation proposal and material ground are attributable;
2. the affected sourcing scope and current lifecycle position are established;
3. an authorised approval, conditional approval, return, or rejection exists;
4. approved cancelled scope is in canonical `Cancelled`, with prior history preserved;
5. award/commitment progression is prevented for cancelled scope without terminating any valid J04 commitment;
6. bidder, demand, security/fee, communication, and successor hand-offs are attributable;
7. any non-cancelled scope retains its source-supported lifecycle position;
8. no bid, evaluation, recommendation, approval, or sourcing-history evidence has been destructively rewritten.

---

## 9. Traceability & Provenance

**Parent:** L3-J-019 — Active Sourcing Event to Governed Cancellation  
**Primary verification source:** L2-J01 — Procurement Planning & Sourcing Workflows  
**Additional verification sources:** L2-J00 — Enterprise Resource Management Architecture & Governance; L2-J02 — Vendor & Supplier Management Workflows; L2-J03 — Purchase Requisition & Approval Workflows.

The Locked parent and L2-J01 define `Cancelled` as the governed sourcing outcome and define `SourcingEventCancelled` and `SourcingEventClosed` as canonical J01 events. J02, J03, J04, and Finance ownership boundaries remain unchanged.

---

## 10. Open Issues / Deferred Design

- Institution-specific cancellation grounds, approval authorities, notice/publication requirements, bidder compensation, security/fee treatment, complaint rules, and retention periods remain upstream-governed.
- The parent expressly distinguishes protective suspension from permanent cancellation but does not define a new L4 hold state; none is invented here.
- `Section_J_Canonical_Register.md`, although listed by the Locked parent, could not be located by exact filename during current repository verification. Material behavior remains independently supported by the Locked L3 parent and Locked J00/J01/J02/J03 sources.
- API/interface design, persistence, services/components, workflow mechanics, event transport, technical access control, retries/rollback, infrastructure, and UI/UX are deferred to L5/L6.

**Review Verdict:** PASS WITH NON-BLOCKING FINDINGS
