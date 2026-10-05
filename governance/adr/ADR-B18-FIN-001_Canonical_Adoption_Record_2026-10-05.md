# ADR-B18-FIN-001 — Canonical Adoption Record — 2026-10-05

| Field | Value |
|---|---|
| Scope | B18-J02 Admissions → Finance → Student Stage-5 closure |
| Decision | Option A prospective Admission-scoped F01 charge + B2 C01-owned clearance evaluation |
| Status | **CANONICALLY-ADOPTED for Stage-5 deterministic mock-operational architecture scope** |
| Architecture PR | #135 |
| Issue | #132 |
| Decision basis | Existing ADR-B18-FIN-001 plus project-owner attestation on 2026-10-05 that required Finance/source review approved proceeding |
| Production backend/API/event transport | **Not approved by this record** |

## Adopted amendments

- `L2-F01_PreStudent_Admission_Charge_Context_Amendment_2026-10-05.md`
- `L2-C01_Admission_Finance_Clearance_Evidence_Amendment_2026-10-05.md`

## Binding Stage-5 interpretation

1. C01 `AdmissionApproved` may form the governed source fact for a prospective Admission-scoped F01 financial obligation when applicable F01 policy and all required canonical scope/identity validations pass.
2. F01 exclusively owns applicability, Fee Structure, monetary calculation, validation, Student Charge creation/publication and outstanding financial truth.
3. F02 owns receipt, allocation and settlement truth. An Unallocated Advance remains a Liability and is never settlement or C01 financial clearance.
4. C01 owns its own policy-driven `FinancialCleared` decision from current authoritative Finance evidence. No new universal `FinancialClearanceCompleted` publisher is created.
5. C01 may record `AdmissionConfirmed` only after its own valid clearance and other admission gates.
6. C02 alone creates/reuses Student and StudentContext and performs activation after `AdmissionConfirmed`.
7. Same-subject lineage must preserve Person, Admission, Tenant, Institution/Institute and applicable Programme/Session correlation without retroactive rewriting.
8. This record authorizes deterministic Stage-5 mock realization only. It does not define production APIs, event schemas, transport, permissions or backend release status.

## Required implementation evidence

J02 must prove positive execution, wrong-subject/scope negatives, missing/stale/insufficient Finance evidence, unallocated-only rejection, replay/idempotency, premature-clearance/confirmation/activation rejection, duplicate-Student protection, immutable lineage and deterministic reset through ordinary product surfaces.
