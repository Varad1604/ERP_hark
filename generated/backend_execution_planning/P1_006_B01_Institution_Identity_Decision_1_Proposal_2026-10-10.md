# P1-006 — Decision 1: Institution identity model (PROPOSED)

**Date:** 2026-10-10  
**Status:** **USER-PREFERRED DESIGN PROPOSAL — NOT ARCHITECTURE APPROVED; NOT DDL OR AN API CONTRACT**  
**Location:** Architecture PR #161, planning branch only. Does not change canonical L1–L6 sources.

## 1. User selection and recommendation

The user selected **Option A** on 2026-10-10 for planning: **one permanent canonical Institution business UUID per Institution, associated with the UUID of exactly one parent Tenant.** The proposed external Institution identity is stable across Web and later Mobile clients and across supported storage-placement changes.

**Important distinction:** This approves neither a database primary-key column nor the claim that a business UUID and a technical UUID can safely be conflated. Locked ADR-L5-0036 distinguishes implementation technical identifiers from source-owned business identifiers. **DB-01 remains OPEN** until B01/B02 and the Lead Architect decide the physical key representation and document ADR conformance.

Proposed rules, for review:
1. B01 owns and issues the permanent canonical Institution business UUID at source-governed establishment. Do not create a second competing Institution business identity.
2. Every Institution references one A01-owned Tenant identity; one Tenant may have multiple Institutions. Do not presume a database per Tenant.
3. The canonical Institution UUID is intended to be globally unique across Institution identities. UUIDv7 is the locked *default for newly introduced technical persistent IDs*, not a source-approved format for the B01 business UUID by itself. Confirm how B01 generates and validates its business UUID. UUID randomness alone is not a database uniqueness constraint or an authorization mechanism.
4. Institution Code remains unique **within its owning Tenant**, not globally. Archived Institution identities and codes must remain historically identifiable and non-reusable as source-governed.
5. Any Institution read requires validated A04 Tenant/Institution association and D02 resource-specific authorization; knowing a UUID provides no access rights.
6. Retain the current tenant association unless a future explicitly approved ownership-transfer workflow is defined; do not invent a transfer operation.

## 2. Candidate physical interpretation — NOT DECIDED

- **Logical contract:** one stable B01 business UUID, one immutable business identity for the Institution, and a mandatory A01 Tenant reference.
- **Open DB-01 implementation choice:** (a) same physical `uuid` field could serve B01 business identity and SQL primary key **only if** ADR-L5-0036 and canonical B01 requirements permit; **or** (b) a distinct B02 technical primary key may be needed internally, with an explicit unique B01 business UUID and lossless mapping. The user chose **one business identity**, not a blanket ban on internal technical identifiers.
- **Recommended constraints subject to sign-off:** unique canonical Institution business UUID within the authorized B01 owner-store/placement; `UNIQUE(tenant_id, institution_code)`; non-null Tenant association; stable historical references. Multi-placement/global collision safeguards and FK/reference integrity are to be designed with B02/A01/A04.
- **No physical column, PostgreSQL schema, FK, indexing strategy, migration or OpenAPI field name is approved by this memo.** P1-005 remains the existing column-level draft.

## 3. Source support and boundaries

- [Locked L2-B01 Institution workflow](../../architecture/L2_Domain_Workflow_Architecture/section-b/L2-B01_Institution_Workflow.md) §§4,7,8,12–13: one Tenant per Institution, immutable Institution business identity (UUID and code), Tenant-scoped Institution Code.
- [Locked L2-B01 activation amendment](../../architecture/L2_Domain_Workflow_Architecture/section-b/L2-B01_Institution_Activation_Authority_Amendment_2026-08-24.md) §2: B01 canonical aggregate and institutional identity ownership.
- [ADR-L5-0036](../../architecture/L5_Solution_Architecture/adr/ADR-L5-0036_Persistence_Implementation_Conventions_and_Technical_Identity_Standard.md) §3: technical UUIDv7 default; source-owned business identifiers remain separate; owner-local PostgreSQL boundaries and selective physical placement.
- [L5-B-02-002](../../architecture/L5_Solution_Architecture/specifications/L5-B/L5-B-02-002_Domain_Persistence_Ownership_and_Isolation_Architecture.md): cross-owner SQL restrictions and hybrid Tenant placement.
- [Existing P1-005 table/column draft](P1_005_A01_B01_Column_Level_Persistence_Blueprint_2026-10-10.md) DB-01/DB-08: still proposed, to be reconciled following physical key approval.

## 4. Required approval and proof before column freeze

- **B01 business owner:** confirm issuance, uniqueness, permanence, non-reuse and meaning of canonical Institution UUID and Code, including Archived history.
- **A01/A04:** confirm authoritative Tenant association, validated resource scoping, prohibition of implicit Tenant switching and any transfer-policy gap.
- **B02 + Lead Architect:** resolve whether one physical UUID column can fulfill business and technical identities under ADR-L5-0036; decide PK, uniqueness, local FK/reference enforcement and cross-placement collision/migration checks.
- **D02/C01:** approve external UUID disclosure and exact scoped read contract; do not release `institutionId` or a public API solely from this planning preference.
- **Lead Architect:** approve any final design promotion under constitution. No PR merge, schema migration, backend code, test or CI is authorized by this note.

## 5. Next discussion — Decision 2 (OPEN)

Where should B01 store/select the **current authoritative Institution Name** while preserving approved historical Institution Profile versions? Compare (a) current fields plus immutable history, (b) authoritative profile versions plus a current-version reference, and (c) effective-date-selected versions. No option is adopted yet. The Locked L3-B-004 mandates historically traceable effective profile revision; L4-B-004 file metadata remains `Review` and needs governance interpretation before implementation.
