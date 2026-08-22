# ADR-FIN-ADJ-002 — Student-Charge Write-Off Approval Authority

| Field | Value |
|---|---|
| ADR ID | ADR-FIN-ADJ-002 |
| Change Type | Governance clarification / L2 amendment candidate |
| Target Locked Documents | L2-F00 Finance Domain Model and Foundation; L2-F04 Financial Adjustment Management |
| Blocked Downstream Scenario | L3-F-022 |
| Status | Proposed — Non-Authoritative |
| Decision Effect | None until Architecture Review approval and amendment/re-lock of affected Locked source documents |
| Domain | Section F — Finance |
| Date | 2026-08-21 |
| Prepared For | Finance Architecture Review and Lead Architect |

---

## 1. Need

The Finance architecture contains an unresolved authority conflict for Student-Charge write-offs.

The existing adjustment amendment establishes a governed enterprise approval authority model in L2-F04, while the Finance foundation retains a more specific Board-approval requirement for Student-Charge write-off. L3-F-022 cannot state a single canonical approving authority without either overriding one Locked source or inventing a compromise.

This ADR isolates that remaining conflict so it can be decided without reopening the already-approved portions of ADR-FIN-ADJ-001.

## 2. Architecture Constraint

The resolution must preserve all of the following:

- Finance remains owner of monetary truth and financial adjustment outcomes.
- Student-Charge ownership outside Finance is not transferred by the approval decision.
- A write-off remains a governed financial adjustment, not an operational deletion of the originating obligation.
- Exactly one authoritative approval rule must apply to the Student-Charge write-off case.
- L3-F-022 must consume the approved rule; L3 must not choose the rule itself.

## 3. Decision Options

### Option A — Preserve the Specific Board Requirement

Retain the Student-Charge-specific Board approval rule as the governing exception within the broader enterprise approval model.

Under this option, L2-F04 would clarify that its generic approval authority is subject to a more specific approved authority rule where such a rule exists in the Finance foundation.

**Advantages**

- Preserves the more specific existing governance rule.
- Minimizes change to higher-level Finance policy.
- Avoids silently weakening a previously explicit approval control.

**Trade-off**

- Requires L2-F04 to state the exception/precedence rule explicitly.

### Option B — Supersede the Board-Specific Requirement

Amend the Finance foundation so Student-Charge write-offs use the same governed Enterprise Approval Authority model as other adjustments.

**Advantages**

- Produces a uniform approval model.
- Removes the special-case authority rule.

**Trade-off**

- Explicitly changes an existing specific governance requirement and therefore requires clear approval authority and traceability.

## 4. Recommendation

**Recommendation only — not authoritative:** Option A is the lower-risk repository-consistency resolution because it preserves the more specific existing governance constraint unless the Architecture Review explicitly decides to supersede it.

No source document is amended by this recommendation.

## 5. Impact Analysis

If Option A is approved:

- L2-F00 remains authoritative for the Student-Charge-specific Board requirement.
- L2-F04 requires a narrow clarification that the generic approval authority does not supersede a more specific approved authority rule.
- L3-F-022 can state the approved authority without ambiguity.

If Option B is approved:

- L2-F00 requires an explicit amendment to remove or supersede the Board-specific requirement.
- L2-F04 remains the canonical approval-authority model for the case.
- L3-F-022 must be synchronized to the approved rule.

No entity ownership, aggregate ownership, lifecycle ownership, or event publisher is changed by either option.

## 6. Decision

**Blank pending Architecture Review.**

This ADR authorizes no Locked-source change until the required review roles record approval and the approved source amendment is applied and re-locked.

## 7. Acceptance Criteria

The issue is resolved only when:

1. one approval rule is explicitly selected;
2. L2-F00 and L2-F04 no longer conflict;
3. the approved rule is traceable to this ADR or its superseding decision;
4. L3-F-022 can reference the rule without adding architecture;
5. no ownership or event-publishing responsibility is changed implicitly.
