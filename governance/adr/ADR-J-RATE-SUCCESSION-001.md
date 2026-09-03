# ADR-J-RATE-SUCCESSION-001 — Rate Contract Successor Completion Boundary Correction

| Attribute | Value |
|---|---|
| ADR ID | ADR-J-RATE-SUCCESSION-001 |
| Date | 2026-09-02 |
| Status | Approved |
| Architecture Level | L3 / L4 Governance |
| Domain / Section | Section J — Procurement Planning & Sourcing |
| Branch | main |
| Change Category | Patch — architecture consistency correction |
| Approval Source | Explicit repository-change authorization recorded in the architecture working session on 2026-09-02: “Go ahead I authorize the changes do the changes and give the score again.” |

## 1. Context

`L3-J-013_Expiring_Rate_Contract_to_Approved_Successor_Arrangement.md` contains an internal completion-boundary contradiction.

- Section 4.2 permits the scenario to end when the successor reaches `Approved` or `Active`.
- Section 9.15 states that, where prerequisites are satisfied, the successor becomes `Approved` and then `Active` on its authorised effective date.
- Section 13 defines the successor Rate Contract result as `Active` when the approved successor becomes effective.
- Section 16.2 requires, upon successful scenario completion, that the successor Rate Contract is `Active` and eligible to receive demand.

The canonical J01 Rate Contract lifecycle already distinguishes `Approved` from `Active`, and adjacent scenario behavior treats an inactive Rate Contract as unavailable for governed sourcing. Therefore L4 cannot safely treat `Approved` alone as the canonical successful completion of L3-J-013.

## 2. Decision

For the successful successor path of `L3-J-013`, `Active` is the mandatory successor lifecycle position for successful scenario completion.

`Approved` remains a valid intermediate lifecycle position and retains its existing canonical event and governance meaning. This decision does not collapse or rename lifecycle states.

The scenario may still complete through the parent-supported non-successor dispositions already present in L3-J-013, including no successor, temporary gap, permitted extension, alternate sourcing, or discontinuation, where applicable.

## 3. Required Repository Synchronization

1. Amend `L3-J-013` so its Business Outcome, End Boundary, and Completion Criteria consistently require the successor to reach `Active` on the successful successor path.
2. Preserve the existing J01 lifecycle `Draft → Review → Approved → Active` and all existing event names and canonical publishers.
3. Remove the upstream-block condition from `L4-J-013` and make the existing `RateContractActivated` behavior part of the observable successful completion path.
4. Preserve all existing ownership and cross-domain boundaries: J01 owns Rate Contract lifecycle and events; J02 retains supplier truth; J03/requesting domains retain demand authority; Finance retains financial truth; J04 retains Purchase Order and commitment authority.

## 4. What This Decision Does Not Change

This decision does not change:

- L1 domain ownership;
- L2 J01 canonical ownership;
- the Rate Contract lifecycle states;
- any business event name;
- any canonical event publisher;
- any business rule outside the contradiction correction;
- scenario IDs or specification IDs;
- scenario titles or filenames;
- author allocation;
- J02, J03, Finance, or J04 authority boundaries.

No new architecture concept is introduced.

## 5. Rationale

Selecting `Active` as the successful completion point preserves the already-established architecture instead of introducing a new interpretation:

- L3-J-013 already states that the successor becomes `Approved` and then `Active` on its authorised effective date.
- Its lifecycle table already identifies `Active` as the resulting successor position.
- Its successful postcondition already requires an active successor eligible to receive demand.
- The canonical J01 lifecycle distinguishes approval from activation.
- Downstream sourcing requires a valid active arrangement rather than approval alone.

The correction therefore removes a contradictory alternative without redefining upstream ownership, lifecycle, rules, or events.

## 6. Validation Requirement

After repository synchronization:

- L3-J-013 shall have one unambiguous successful successor end position: `Active`.
- L4-J-013 shall no longer be `BLOCKED BY UPSTREAM` for HF-12.
- `RateContractApproved` shall remain observable at approval.
- `RateContractActivated` shall remain observable when the successor reaches `Active` and shall be required for the successful successor completion path.
- No ownership transfer, invented rule, lifecycle corruption, event corruption, unsupported authority, implementation leakage, or silent upstream repair may be introduced.

## 7. Architectural Effect

This is a controlled patch correction to reconcile an internally inconsistent Locked L3 scenario with its own lifecycle and postconditions. It does not reopen the wider 1,084-scenario corpus and does not authorize unrelated edits.
