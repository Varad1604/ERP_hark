# L2-F09 — Financial Period & Closing — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-F09 — Financial Period & Closing v1.0 |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-08-21 |
| Approved Through | ADR-FIN-PERIOD-001 |
| Authority | Lead Architect approval recorded 2026-08-21 |

This amendment is part of the canonical L2-F09 architecture. Where it conflicts with L2-F09 v1.0 or L2-F05 v1.0, this amendment is authoritative. Unaffected source content remains unchanged.

## 1. Canonical Ownership

L2-F09 — Financial Period & Closing is the canonical Finance bounded context for **Financial Year lifecycle governance** and **Financial Period lifecycle governance**.

Two distinct aggregate roots exist for those distinct lifecycles:

- **Financial Year Aggregate** — owns Financial Year identity, annual lifecycle, year-level opening/closing governance and association to its Financial Periods.
- **Financial Period Aggregate** — owns one accounting-window lifecycle, posting-window status, close/reopen/lock governance and its immutable reference to one Financial Year.

A Financial Year is **not** an internal entity owned separately by each Financial Period. Any L2-F09 wording that states otherwise is superseded.

One Financial Year may be associated with many Financial Period aggregates. Every Financial Period references exactly one Financial Year. Neither aggregate is contained inside the transactional boundary of the other.

L2-F05 consumes these authoritative year/period governance facts for accounting posting; it does not own their lifecycle.

## 2. Financial Year Lifecycle

The canonical Financial Year lifecycle reuses the already-established Finance lifecycle semantics:

`Planned → Open → Closing → Closed → Archived`

Rules:

1. `Planned → Open` establishes the Financial Year as the governing annual accounting context.
2. `Open → Closing` begins governed year-end completion.
3. `Closing → Closed` requires all governed Financial Periods to satisfy closure requirements.
4. `Closed → Archived` preserves historical annual evidence; it does not delete or alter the year.
5. A Closed or Archived Financial Year does not accept ordinary new accounting-period activity except through an explicitly governed corrective process.

Canonical existing Finance events are retained:

- `FinancialYearOpened` — published by L2-F09 when `Planned → Open` completes.
- `FinancialYearClosed` — published by L2-F09 when `Closing → Closed` completes.

No duplicate event name is introduced for “establishment”; for downstream business scenarios, successful establishment/availability culminates in `FinancialYearOpened`.

## 3. Financial Period Lifecycle

The Financial Period Aggregate remains the operational accounting-window lifecycle. Its canonical progression is:

`Planned/Registered → Open → Closing → Closed → Locked → Archived`

Governed reopening is permitted only through the L2-F09 reopening authority and produces an `Open` period again. A reopened period must pass the same close controls before returning to Closed/Locked. Locking represents a stronger historical protection than ordinary closure.

Where older L2-F09 text uses multiple labels for the pre-open preparation stage, `Planned/Registered` denotes the same preparation boundary and shall be normalized in the next full source consolidation; downstream L3 shall use the exact state name already frozen for its scenario where applicable and shall not invent a third state.

## 4. Canonical Business Authorities

| Authority | Responsibility | Boundary |
|---|---|---|
| Financial Calendar Administrator | Establishes Financial Year and Financial Period definitions and submits them for governed opening. | Cannot independently approve exceptional reopening. |
| Period Opening Authority | Confirms readiness and opens a Financial Period. | Cannot bypass year-level governance. |
| Period Closing Authority | Confirms close prerequisites and closes the Financial Period. | Requires accounting readiness evidence from L2-F05 where applicable. |
| Period Reopening Authority | Authorizes exceptional reopening of a Closed period. | Reopening is exceptional, auditable and does not erase prior closure evidence. |
| Period Lock Authority | Confirms a Closed period is ready for protected historical status. | Locking may not bypass unresolved close exceptions. |
| Financial Year Closing Authority | Confirms all year-level close prerequisites and closes the Financial Year. | Does not own Journal/Ledger records. |

Institutional authorization may map these responsibilities to local roles while preserving these decision boundaries.

## 5. Cross-Domain Contract with Accounting

L2-F05 provides accounting readiness facts such as pending-journal resolution, reconciliation status and verified Trial Balance evidence. L2-F09 uses those facts in year/period close governance.

L2-F09 publishes the authoritative Financial Year/Period lifecycle facts. L2-F05 consumes them to determine posting eligibility. Neither domain directly modifies the other domain's aggregates.

## 6. Locked Decisions

The aggregate ownership, year/period relationship, Financial Year lifecycle, existing event ownership and authority rules in this amendment are Locked under ADR-FIN-PERIOD-001. L3-F-051 through L3-F-055 shall consume these decisions without introducing aggregate-version, persistence, rollback or other implementation mechanics.
