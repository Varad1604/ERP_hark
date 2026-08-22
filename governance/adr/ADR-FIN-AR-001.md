# ADR-FIN-AR-001 — Accounts Receivable Authority and Obligation Event Contract Clarification

| Field | Value |
|---|---|
| ADR ID | ADR-FIN-AR-001 |
| Change Type | L2 architecture clarification candidate |
| Target Locked Document | L2-F07 Accounts Receivable Management |
| Blocked Downstream Scenarios | L3-F-039 through L3-F-043 |
| Status | Proposed — Non-Authoritative |
| Decision Effect | None until Architecture Review approval and amendment/re-lock of L2-F07 |
| Domain | Section F — Finance / Accounts Receivable |
| Date | 2026-08-21 |

---

## 1. Need

L2-F07 defines Accounts Receivable behaviour but does not provide a sufficiently canonical actor/authority definition for receivable registration, validation, exception review, recognition, dunning, settlement, write-off referral, and reopening. It also identifies obligation sources more broadly than the exact consumed-event contract currently establishes.

This leaves L3-F-039 through L3-F-043 unable to state complete business authority and cross-domain trigger semantics without inventing upstream architecture.

## 2. Repository Constraints

Any approved amendment must preserve:

- Finance ownership of the Receivable and monetary receivable truth.
- Source-domain ownership of the originating obligation.
- No direct cross-domain mutation of source aggregates.
- Exactly one canonical publisher for each consumed obligation event.
- Business-level actor responsibilities rather than implementation roles.
- Existing receivable lifecycle semantics unless the review explicitly changes them.

## 3. Required Architecture Decisions

Architecture Review must determine and record:

1. the canonical business roles authorized to register, validate, review exceptions, recognize, pursue, settle, refer for write-off, and reopen receivables;
2. the separation-of-duties requirements between initiation, validation, approval, and write-off/referral where applicable;
3. the exact consumed business-event contract for each supported originating obligation class;
4. the canonical publisher of each such event;
5. the rule for adding future obligation sources without transferring ownership into Accounts Receivable.

## 4. Recommendation

**Recommendation only — not authoritative:** amend L2-F07 by adding a canonical Actors/Authorities section and a canonical Consumed Obligation Events table. Reuse only role names and event names already established by authoritative source domains; where no canonical event exists, escalate to that source domain rather than naming one in F07.

The AR boundary should remain: a source domain publishes an established financial obligation; Accounts Receivable recognizes and manages the Receivable representation while the originating obligation remains owned by its source domain.

## 5. Impact

A successful decision will unblock L3-F-039 through L3-F-043 without changing their Phase 4 canonical scenario identities.

No source entity ownership is transferred. No cross-domain direct write is introduced. No event name may be invented by this ADR.

## 6. Decision

**Blank pending Architecture Review.**

## 7. Acceptance Criteria

1. L2-F07 contains canonical business actors and decision authorities.
2. Each supported obligation source has an exact consumed event or is explicitly marked unresolved.
3. Each consumed event has exactly one source-domain publisher.
4. Recognition does not transfer ownership of the source obligation.
5. L3-F-039 through L3-F-043 can be authored without assumptions about actors or event publishers.
