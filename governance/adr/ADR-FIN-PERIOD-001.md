# ADR-FIN-PERIOD-001 — Financial Year and Financial Period Canonical Ownership and Lifecycle

| Field | Value |
|---|---|
| ADR ID | ADR-FIN-PERIOD-001 |
| Change Type | Aggregate ownership and lifecycle clarification candidate |
| Target Locked Document | L2-F09 Financial Period and Closing |
| Blocked Downstream Scenarios | L3-F-051 through L3-F-055 |
| Status | Proposed — Non-Authoritative |
| Decision Effect | None until Architecture Review approval and amendment/re-lock of L2-F09 |
| Domain | Section F — Finance |
| Date | 2026-08-21 |

---

## 1. Need

L2-F09 describes a Financial Year as containing multiple Financial Periods while also describing Financial Year as an internal entity owned by each Financial Period aggregate. It does not define a sufficiently independent establishment lifecycle or authoritative establishment event for the Financial Year required by downstream Phase 4 scenario semantics.

This creates a canonical-ownership ambiguity and prevents L3-F-051 through L3-F-055 from describing one coherent institutional lifecycle without inventing aggregate ownership or states.

## 2. Architecture Constraints

The approved resolution must preserve:

- exactly one canonical owner for Financial Year;
- exactly one canonical owner for Financial Period;
- one coherent containment/association direction between Financial Year and Financial Period;
- Finance ownership of accounting-period governance;
- existing closing, reopening and locking controls unless explicitly changed by the approved decision;
- implementation independence: no database, persistence or transaction semantics.

## 3. Required Decisions

Architecture Review must explicitly decide:

1. whether Financial Year is an independent aggregate, a lifecycle-governing concept within another Finance aggregate, or a subordinate entity;
2. how multiple Financial Periods relate to one Financial Year without duplicate ownership;
3. the business lifecycle for establishing and governing a Financial Year;
4. the authoritative business event(s), if any, signaling Financial Year establishment/availability;
5. the relationship between year-level governance and period-level open, close, reopen and lock behaviour;
6. which existing contradictory lifecycle statement in L2-F09 is superseded.

## 4. Recommendation

**Recommendation only — not authoritative:** choose one year-level canonical owner and model Financial Periods as governed members associated with that year-level context, rather than describing the Financial Year as independently owned by every Financial Period. Preserve Financial Period as the operational accounting-window lifecycle where that is already established by L2-F09.

The exact aggregate designation and event names must be approved from the repository architecture; this ADR does not invent them.

## 5. Impact Analysis

An approved resolution will require a controlled amendment of L2-F09 and synchronization of L3-F-051 through L3-F-055. It may also require terminology normalization wherever the current contradictory ownership statement is referenced.

No ownership moves outside Section F.

## 6. Decision

**Blank pending Architecture Review.**

## 7. Acceptance Criteria

1. Financial Year has exactly one canonical owner.
2. Financial Period has exactly one canonical owner.
3. Their relationship is non-contradictory.
4. Year establishment and period governance lifecycles are explicitly distinguishable.
5. Required business states/events are authoritative rather than inferred by L3.
6. L3-F-051 through L3-F-055 can be synchronized without implementation mechanics or invented ownership.
