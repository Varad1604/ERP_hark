# Cost Center Request to Active or Closed Cost Center

## 1. Specification Metadata

**Specification ID:** L4-F-037
**Architecture Level:** L4 — System Behavior Specification
**Domain:** Budget & Cost Center Management
**Realizes Scenario:** L3-F-037
**Primary Business Owner:** Budget & Cost Center Management
**Version:** 1.0.0
**Status:** Review
**Owner:** Pallav
**Last Updated:** 2026-09-02
**Depends On:** `L3-F-037_Cost_Center_Request_to_Active_or_Closed_Cost_Center.md`; `L2-F06_Budget_and_Cost_Center_Management.md`; `L2-B02_Organisation_Workflows.md`
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

SutraERP shall support the source-defined Cost Center establishment and activation behavior while preserving Finance ownership and authoritative Section B organisational references. The parent also names a terminal `Closed` outcome, but the parent explicitly states that this terminal lifecycle term is not established by authoritative upstream architecture; L4 therefore shall not manufacture that transition.

The source-supported L4 boundary covers Cost Center establishment and activation. Terminal processing beyond the last authoritative lifecycle position is blocked by upstream lifecycle ambiguity and is recorded rather than silently repaired.

---

## 3. Required Business Context

The behavior requires:

- an authoritative Section B organisational-unit reference;
- the requested Cost Center classification and institutional scope;
- Finance ownership of the Cost Center aggregate;
- the source-supported `Active` lifecycle position;
- the explicit parent prohibition on inventing the terminal `Closed` state;

No source-owned fact or business authority is transferred to L4 by its use in this specification.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-037-BH-001 | The system shall require a valid authoritative Section B organisational-unit reference and required Cost Center classification/scope before establishing the Finance Cost Center. | L3-F-037 §§7, 9 step 1, 11 |
| L4-F-037-BH-002 | The system shall establish Cost Center identity under Budget & Cost Center Management ownership without creating a Finance-owned duplicate of Section B organisational structure. | L3-F-037 §§2, 5, 9 step 2, 12 |
| L4-F-037-BH-003 | When source-supported establishment criteria are satisfied, the system shall transition the Cost Center to the canonical `Active` lifecycle position. | L3-F-037 §§9 step 3, 13, 16 |
| L4-F-037-BH-004 | On activation, the system shall publish `CostCenterActivated` with Budget & Cost Center Management retained as canonical publisher. | L3-F-037 §§14, 16 |
| L4-F-037-BH-005 | When a requested terminal action depends on a lifecycle position not established by authoritative upstream architecture, the system shall preserve the last valid Cost Center position and shall not record an invented `Closed` transition. | L3-F-037 §§4.2, 9 step 4, 11, 13, 17 |
| L4-F-037-BH-006 | The system shall preserve the Cost Center request, authoritative organisational reference, classification/scope, and source-supported activation or terminal-processing decision as attributable evidence. | L3-F-037 §15.3 |

---

## 5. Cross-Domain & Event Behavior

Section B owns the referenced organisational structure; Finance owns the Cost Center. `CostCenterActivated` is the only source-supported canonical event for this scenario. No terminal event or terminal state is introduced at L4.

No event transport, payload, queue, delivery, retry, or technical integration behavior is defined at L4.

---

## 6. Failure / Rejection / Continuation

An invalid organisational reference prevents Cost Center establishment. A terminal request relying on the unresolved `Closed` lifecycle term cannot be completed; the system shall preserve the last valid business position pending a governed upstream lifecycle decision.

Technical retry, rollback, transaction compensation, exception-handler, and infrastructure-recovery mechanics are outside L4.

---

## 7. Audit & Evidence Behavior

Preserve the Cost Center request, authoritative Section B reference, classification/scope, activation decision and resulting Active position, `CostCenterActivated` evidence, and any unresolved terminal request without fabricating terminal-state evidence.

Section A retains ownership of Authorization and Audit infrastructure. The business-domain owners identified by the parent retain ownership of the underlying business decisions and evidence.

---

## 8. Observable Completion Conditions

The establishment path completes when the Cost Center is `Active` and `CostCenterActivated` is published. The terminal path cannot have an L4 completion condition until the parent/upstream architecture authoritatively establishes its terminal lifecycle term and transition.

---

## 9. Traceability & Provenance

**Parent:** `L3-F-037` — Cost Center Request to Active or Closed Cost Center
**Primary verification source:** L2-F06 — Budget & Cost Center Management
**Additional verification:** L2-B02 — Organisation Workflows; parent L3-F-037 explicit lifecycle-gap statement.

Every BH above is anchored to the approved Locked L3 parent. Higher-level/L2 sources are used only to verify exact ownership, event, rule, lifecycle, or dependency semantics; no material L3 omission is silently repaired.

---

## 10. Open Issues / Deferred Design

The final L3 corpus lock dated 2026-08-29 establishes `L3-F-037` as a Locked parent eligible for L4 derivation. Earlier parent wording describing the Cost Center terminal-lifecycle gap remains traceability evidence and does not, by itself, reopen that Locked parent. The lock decision does not create a new Cost Center terminal state or terminal event.

Accordingly, this specification realizes the parent exactly as locked: the establishment path may complete at `Active` with `CostCenterActivated`; a terminal request that depends on an unestablished `Closed` transition remains at the last valid lifecycle position and shall not manufacture a terminal state or event. Any future controlled upstream/L3 amendment that establishes additional Cost Center terminal semantics requires synchronized revalidation of this L4 specification.

Named Cost Center actors remain no more specific than the Locked parent permits. Implementation details remain deferred.

---

**Review Classification:** READY FOR ARCHITECTURE REVIEW — locked-parent activation behavior is fully specified; terminal processing preserves the Locked parent's governed deferral and introduces no unsupported state, event, or authority.
