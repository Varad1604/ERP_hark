# ADR-FIN-AP-001 — Accounts Payable and Procurement-Finance Event Ownership Clarification

| Field | Value |
|---|---|
| ADR ID | ADR-FIN-AP-001 |
| Change Type | Cross-domain event-contract clarification candidate |
| Target Locked Documents | L2-F08 Accounts Payable Management; L2-F14 Procurement Finance Consumption Workflows; relevant Section J procurement workflow source(s) |
| Blocked Downstream Scenarios | L3-F-044 through L3-F-050 |
| Status | Proposed — Non-Authoritative |
| Decision Effect | None until Architecture Review approval and amendment/re-lock of affected Locked source documents |
| Domains | Section F — Finance; Section J — Enterprise Resource Management |
| Date | 2026-08-21 |

---

## 1. Need

The current Finance workflow architecture contains conflicting or incomplete semantics for the procurement-to-payable hand-off.

L2-F08 treats `VendorInvoiceApproved` as a consumed Procurement event, while L2-F14 also lists `VendorInvoiceApproved` within Finance-published events. L2-F14 publishes `VendorLiabilityRecognized`, while L2-F08 publishes `PayableRegistered` and `PayableValidated`; the authoritative sequence and semantic relationship between those events is not sufficiently defined.

L3-F-044 through L3-F-050 cannot safely select publishers or infer equivalence between those events.

## 2. Architecture Constraints

Any approved resolution must preserve:

- Section J ownership of procurement, supplier/vendor, purchasing and procurement-side approval behaviour assigned to it by the locked architecture.
- Section F ownership of payable liability and monetary truth.
- Exactly one canonical publisher for each business event.
- Cross-domain interaction through explicit business events/contracts only; no direct writes.
- No renaming or merging of existing events without an explicit approved migration decision.

## 3. Required Decisions

Architecture Review must record:

1. the canonical publisher of `VendorInvoiceApproved`;
2. whether `VendorInvoiceApproved` is a procurement-side approval outcome, a Finance-side approval outcome, or whether one of the current references is erroneous;
3. the canonical business meaning and publisher of `VendorLiabilityRecognized`;
4. the canonical business meaning and ordering of `PayableRegistered` and `PayableValidated`;
5. which event constitutes the authoritative cross-domain hand-off into Accounts Payable;
6. whether any currently duplicated event reference must be removed from F08, F14 or the relevant Section J source.

## 4. Recommendation

**Recommendation only — not authoritative:** preserve domain ownership boundaries by assigning procurement-side approval outcomes to the authoritative procurement owner and payable-liability recognition outcomes to Finance. Treat registration, validation and liability recognition as distinct only where the locked business workflows establish distinct business outcomes; do not declare them synonyms merely to resolve the conflict.

The exact event publisher and sequence must be confirmed from the authoritative Section J and Finance workflow sources during review.

## 5. Impact Analysis

An approved resolution is expected to require coordinated amendments to L2-F08 and L2-F14 and, only if contradicted there, the relevant Section J L2 source. The change must be applied as one repository-consistent decision so no event has dual publishers.

No supplier/vendor ownership or procurement aggregate ownership transfers to Finance. No payable ownership transfers to Section J.

## 6. Decision

**Blank pending Architecture Review.**

## 7. Acceptance Criteria

1. `VendorInvoiceApproved` has exactly one canonical publisher.
2. `VendorLiabilityRecognized`, `PayableRegistered` and `PayableValidated` each have one defined business meaning and publisher.
3. Their ordering or independence is explicit.
4. The procurement-to-Finance hand-off has one authoritative contract.
5. L2-F08, L2-F14 and affected Section J sources no longer contradict one another.
6. L3-F-044 through L3-F-050 can consume the contract without inventing architecture.
