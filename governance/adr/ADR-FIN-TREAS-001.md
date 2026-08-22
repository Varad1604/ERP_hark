# ADR-FIN-TREAS-001 — Treasury Account Lifecycle, Authority and Event Clarification

| Field | Value |
|---|---|
| ADR ID | ADR-FIN-TREAS-001 |
| Change Type | L2 lifecycle and authority clarification candidate |
| Target Locked Document | L2-F10 Treasury and Banking |
| Blocked Downstream Scenarios | L3-F-056 through L3-F-061 |
| Status | Proposed — Non-Authoritative |
| Decision Effect | None until Architecture Review approval and amendment/re-lock of L2-F10 |
| Domain | Section F — Finance / Treasury |
| Date | 2026-08-21 |

---

## 1. Need

L2-F10 does not define a sufficiently canonical Treasury Account lifecycle, transition authority, or complete event contract to support the frozen downstream scenario semantics that distinguish an active treasury account from a closed treasury account.

It also lacks a canonical business-actor definition sufficient for requesting, verifying, approving, activating and closing Treasury Accounts.

L3-F-056 through L3-F-061 therefore cannot safely state exact actors, states, transition authority or events without creating architecture at L3.

## 2. Architecture Constraints

Any approved amendment must preserve:

- Finance/Treasury ownership of Treasury Account business truth;
- business-level lifecycle semantics rather than implementation status flags;
- explicit authority for controlled activation and closure;
- separation of duties where required by financial governance;
- exactly one canonical publisher for each Treasury event;
- historical continuity after closure;
- no technical persistence, rollback, retry or transaction semantics in L2/L3 business architecture.

## 3. Required Decisions

Architecture Review must determine and record:

1. the canonical Treasury Account business states needed to govern establishment, activation, restriction/suspension if applicable, and closure;
2. allowed business transitions between those states;
3. the authorized business roles for request, verification, approval, activation, amendment and closure;
4. required separation-of-duties constraints;
5. canonical business events for the approved lifecycle transitions;
6. closure preconditions and the business meaning of historical preservation after closure.

## 4. Recommendation

**Recommendation only — not authoritative:** define the minimum lifecycle necessary to express the already-approved institutional outcomes, including an authoritative Active state and Closed state if confirmed by Architecture Review, and add only intermediate states that are evidenced by the locked Treasury workflow. Do not add states solely to mirror software processing.

Use existing Treasury role/event terminology where authoritative; unresolved names should remain open for review rather than being invented here.

## 5. Impact Analysis

An approved decision will require controlled amendment and re-lock of L2-F10 followed by synchronization of L3-F-056 through L3-F-061.

No Treasury ownership or banking responsibility moves to another domain.

## 6. Decision

**Blank pending Architecture Review.**

## 7. Acceptance Criteria

1. Treasury Account has one authoritative business lifecycle.
2. Active and Closed semantics are explicitly governed if retained as canonical states.
3. Every lifecycle transition has an authorized business authority.
4. Required separation-of-duties rules are explicit.
5. Each published lifecycle event has exactly one canonical publisher.
6. L3-F-056 through L3-F-061 can be synchronized without invented states, roles, events or implementation recovery mechanics.
