# Overdue Receivable to Governed Dunning Outcome

## 1. Specification Metadata

**Specification ID:** L4-F-040  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Accounts Receivable  
**Realizes Scenario:** L3-F-040  
**Primary Business Owner:** Accounts Receivable  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-F-040_Overdue_Receivable_to_Governed_Dunning_Outcome.md`; `L2-F07_Accounts_Receivable_Management.md`; `L2-F07_Accounts_Receivable_Management_Amendment_2026-08-21.md`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall govern ageing and collection classification when an outstanding Receivable becomes overdue, establishing an authoritative AR collection position while preserving source-obligation, penalty, payment, and communication ownership boundaries.

The behavior begins when an active outstanding Receivable crosses an applicable due/ageing boundary under effective AR policy. It ends when AR has established the current ageing classification, collection priority/escalation or permitted suspension position, and preserved supporting evidence.

This specification does not create penalties or charges, execute payment collection, modify the originating obligation, or design notification technology.

---

## 3. Required Business Context

The behavior requires an active outstanding Receivable, valid due information, current outstanding balance, valid Collection Profile, effective ageing and collection policies, Collection Review Authority, and Receivable Review Authority for exceptions.

Institution-specific ageing buckets, priorities, escalation thresholds, and suspension criteria remain configuration-governed and are not fixed here.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-040-BH-001 | When an outstanding Receivable crosses the applicable due/ageing boundary, the system shall evaluate it against effective AR ageing criteria. | L3-F-040 §§4.1, 7-9 |
| L4-F-040-BH-002 | The system shall update the AR-owned Aging Profile with the applicable governed ageing classification. | L3-F-040 §§3, 5, 9 step 2, 13 |
| L4-F-040-BH-003 | The system shall permit the Collection Review Authority to evaluate priority, escalation, and permitted suspension under effective AR policy. | L3-F-040 §§6, 9 step 3, 12 |
| L4-F-040-BH-004 | The system shall update the AR-owned Collection Profile/Status only using classifications and states supported by canonical F07 policy. | L3-F-040 §§5, 12-13 |
| L4-F-040-BH-005 | The system shall not invent a new Receivable lifecycle state merely to represent dunning or collection escalation. | L3-F-040 §13 |
| L4-F-040-BH-006 | Collection exceptions may be reviewed by Receivable Review Authority without granting ownership of corrective financial actions. | L3-F-040 §§6, 9 step 4 |
| L4-F-040-BH-007 | The originating obligation shall remain unchanged during ageing, escalation, suspension, or exception review. | L3-F-040 §§2, 4.4-5, 9 step 4, 12 |
| L4-F-040-BH-008 | The system shall not create late fees, penalties, or new charges as a consequence of AR collection classification alone. | L3-F-040 §§4.4, 6, 12, 16 |
| L4-F-040-BH-009 | The system shall not execute payment collection as part of this behavior. | L3-F-040 §§4.4, 16 |
| L4-F-040-BH-010 | On completion of review, the system shall preserve the authoritative ageing/collection position and its policy basis. | L3-F-040 §§3, 9 step 5, 15-16 |
| L4-F-040-BH-011 | Where communication is required, the system shall make the governed AR outcome available to the communication capability without transferring AR status ownership. | L3-F-040 §§5, 9 step 6 |
| L4-F-040-BH-012 | Communication activity by itself shall not alter AR ageing, collection status, or the originating obligation. | L3-F-040 §12 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-F-040-BH-013 | Where a material AR change is published under canonical F07 behavior, the system may publish `ReceivableUpdated` with Accounts Receivable as canonical publisher. | L3-F-040 §14 |
| L4-F-040-BH-014 | The system shall not invent a dunning-specific event name where canonical L2-F07 has not defined one. | L3-F-040 §§14, 17 |
| L4-F-040-BH-015 | Authoritative student/party context from Section C may inform review without transferring AR ownership. | L3-F-040 §5 |
| L4-F-040-BH-016 | Section H communication capability may consume the governed collection outcome without acquiring authority to change AR status. | L3-F-040 §§5, 9 step 6, 12 |

---

## 6. Failure / Rejection / Continuation

Missing or contradictory due/ageing evidence shall prevent authoritative escalation. The system shall preserve the last valid AR collection position and route the issue for business review. A disputed or policy-exception Receivable may remain under review or permitted collection suspension according to approved AR policy. Communication failure shall not alter AR status. No technical notification retry, rollback, or recovery behavior is defined.

---

## 7. Audit & Evidence Behavior

Evidence shall preserve Receivable identity, due basis, outstanding balance, effective ageing and collection policy, ageing classification, collection priority, escalation/suspension decision, reviewing authorities, party/student context references where applicable, downstream communication reference where applicable, `ReceivableUpdated` where published, and evidence that no penalty, payment execution, or source-obligation mutation occurred.

---

## 8. Observable Completion Conditions

Completion requires an authoritative ageing classification, authoritative collection position, governed and attributable escalation/suspension where applicable, unchanged source obligation ownership, no penalty creation, no payment execution, downstream-only communication reference, and reconstructable policy/decision evidence.

---

## 9. Traceability & Provenance

**Parent:** `L3-F-040` — Overdue Receivable to Governed Dunning Outcome  
**Primary verification source:** `L2-F07` — Accounts Receivable Management  
**Material amendment:** `L2-F07_Accounts_Receivable_Management_Amendment_2026-08-21.md`  

Canonical F07 verifies AR ownership of Aging Profile and Collection Profile and configuration-driven collection behavior. The locked amendment verifies Collection Review Authority and Receivable Review Authority boundaries and prohibits AR from creating penalties or modifying other Finance aggregates.

---

## 10. Open Issues / Deferred Design

Exact institution-specific ageing buckets, priorities, escalation thresholds, and suspension criteria remain configuration-governed. No dunning-specific event name is introduced because upstream sources do not define one. API/interface design, persistence, ageing calculation implementation, communication technology, notification delivery, event transport, infrastructure, and UI/UX remain deferred.

---

**Draft Validation Classification:** Prepared as an L4 Draft for architecture review.
