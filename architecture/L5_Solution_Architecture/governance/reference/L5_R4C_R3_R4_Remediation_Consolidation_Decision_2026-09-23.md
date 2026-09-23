# SutraERP L5 R4C — R3/R4 Remediation Consolidation Decision — 2026-09-23

**Version:** 1.0.0  
**Status:** Architecture Approved  
**Owner:** Lead Architect  
**Architecture Level:** L5 — Solution Architecture Governance  
**Programme:** Post-Validation Upstream Blocker Resolution & Final-Lock Readiness  
**Scope:** Canonical consolidation of validated R3D + R4A + R4B remediation  
**Final L5 Corpus Lock:** NOT AUTHORIZED

---

## 1. Decision

R4C consolidates the validated R3D IAM remediation and R4A/R4B operational-evidence / technical-service-acceptance remediation on one fresh branch created from the verified live `main` baseline.

R4C does not redesign R3 or R4. It reconciles their already validated artifacts, shared traceability store, ADR state and living 105-row register without rewriting historical validation decisions.

---

## 2. Verified Live Baseline

At R4C start:

```text
main HEAD
200417d5a5c4373cce0de6ba2ffdf1ef25a2c161

PR #70  OPEN / mergeable / not merged
PR #71  OPEN / mergeable / not merged
PR #73  OPEN / mergeable / not merged
```

PR #73 remained stacked on the R4A branch. R3D and R4 shared `traceability/L4_to_L5/section-a.md` with different validated relation populations, so branch-local files could not be merged by replacement.

---

## 3. Integrated R3D Content

R4C carries forward the validated R3D artifacts, including:

- R3A approval of `L4-A-053`, `L4-A-054`, `L4-A-055`;
- Locked A02/A03 IAM amendments;
- reconciled `L3-A-004…009`;
- approved `L4-A-004…009`;
- `ADR-L5-0048` — Non-Human Technical Principal Lifecycle Profile;
- R3C and R3D governance records;
- approved `L5-D-01-001…004`;
- the 136 validated D01 Section-A relations;
- ADR register synchronization.

The consolidated branch also removes three residual current-tense Draft-L4 statements in cleared D01 specifications that contradicted their R3D-approved parent state. Historical statements explicitly describing the former blocker remain historical evidence.

---

## 4. Integrated R4 Content

R4C carries forward:

- `L4-A-042` v1.0.1 — Architecture Approved;
- `L4-A-045` v1.0.1 — Architecture Approved;
- R4A operational-readiness evidence-authority decision;
- Technical Service Acceptance Authority decision;
- `L5-E-05-001` v1.3.0 — Architecture Approved;
- `L5-E-05-002` v1.2.0 — Architecture Approved;
- `L5-E-05-003` v1.1.0 — Architecture Approved;
- `L5-E-05-004` v1.1.0 — Architecture Approved;
- R4B E05 targeted revalidation decision;
- the 3 validated E05 Section-A relations.

Technical Service Acceptance Authority remains distinct from L2-A10 Governance Authority and does not issue Operational Readiness Certificates or runtime `READY / NOT READY` outcomes.

---

## 5. Canonical Section-A Traceability Union

Exact relation identity is:

```text
(L4 BH ID, L5 Specification ID)
```

Mechanical reconciliation result:

```text
R3D D01 relations                         136
R4B E05 relations                           3
Canonical Section-A validated total       139
Duplicate relation identities               0
Missing BH identities                       0
Stale L4 version bindings                   0
Stale L5 version bindings                   0
Artificial E05-002/003/004 relations        0
```

The canonical consolidated partition is:

`architecture/L5_Solution_Architecture/traceability/L4_to_L5/section-a.md`

---

## 6. ADR Governance

R4C preserves the R3C ADR decision:

```text
Allocated ADRs         48
Highest                ADR-L5-0048
ADR-L5-0048 status     Locked
Next allocation        ADR-L5-0049
```

No additional ADR is allocated by R4C.

R4B's Technical Service Acceptance Authority remains an explicit technical-governance responsibility decision and does not require a separate technology/product ADR.

---

## 7. Living 105-Row Register

Historical Batch-9 state remains historical:

```text
Review                    51
Architecture Approved     21
Locked                    33
Blocked                   30
Total                    105
Final L5 Corpus Lock      NOT AUTHORIZED
```

R4C current consolidated state:

```text
Draft                       0
Review                     43
Architecture Approved      29
Locked                     33
Total                     105

Blocked upstream           22
Pass/non-blocking Review   16
Approval-eligible Review    5
Architecture Approved      29
Locked                     33
Total                     105
```

Arithmetic:

```text
30 historical blockers
-4 D01 blockers cleared by R3D
-4 E05 blockers cleared by R4
=22 current blockers

51 historical Review
-8 promoted cleared rows
=43 current Review

21 historical Architecture Approved
+8 promoted cleared rows
=29 current Architecture Approved
```

No historical Batch-9 decision is rewritten.

---

## 8. Integrity Gate

R4C mechanical checks:

| Check | Result |
|---|---|
| Source changed-path union from R3D + R4 present | PASS — 42 / 42 |
| Canonical register identities | PASS — 105 / 105 |
| Draft L5 | PASS — 0 |
| Review | PASS — 43 |
| Architecture Approved | PASS — 29 |
| Locked | PASS — 33 |
| Current blockers | PASS — 22 |
| D01/E05 cleared rows | PASS — 8 / 8 Architecture Approved |
| Section-A validated relations | PASS — 139 |
| Duplicate relation identities | PASS — 0 |
| Missing BH identities | PASS — 0 |
| Stale traceability versions | PASS — 0 |
| ADR count | PASS — 48 |
| Next ADR | PASS — ADR-L5-0049 |
| New ownership transfer | PASS — none detected |
| L5 invention of missing business semantics | PASS — none introduced |
| L6/UI leakage | PASS — none introduced by R4C |
| Final L5 Corpus Lock | NOT AUTHORIZED |

---

## 9. Governance Boundaries Preserved

R4C preserves:

- canonical Person / Identity / User Account ownership;
- D02 authorization ownership;
- D03 credential/secret ownership;
- L2-A10 operational-readiness governance/certification ownership;
- Technical Service Acceptance Authority as L5 technical governance only;
- source-owner evidence truth;
- historical Batch-9 NO-LOCK decision;
- separate explicit final-lock governance.

R4C does not fabricate new L4 relations merely to increase coverage.

---

## 10. Supersession / Integration Effect

This R4C branch is the intended canonical integration line for the validated content previously carried by:

- PR #70 — R3D;
- PR #71 — R4A;
- PR #73 — R4B.

Once the R4C pull request is open and verified to contain the complete validated source-path union, those parallel PRs may be marked superseded and closed to prevent accidental double integration.

Closing a superseded PR does not delete its branch or historical decision evidence.

---

## 11. Final R4C Decision

```text
R3D validated remediation integrated          PASS
R4A validated remediation integrated          PASS
R4B validated remediation integrated          PASS
Section-A reconciliation                      PASS — 139 / 0 duplicates
ADR reconciliation                            PASS — 48 / next 0049
105-row current-state reconciliation          PASS — 43 / 29 / 33
Current upstream blockers                     22
Historical Batch-9 decision                   PRESERVED
Final L5 Corpus Lock                          NOT AUTHORIZED
R5 entry                                      READY AFTER R4C PR INTEGRATION REVIEW
```

**Decision:** `R4C PASS — CONSOLIDATED REMEDIATION LINE READY FOR PULL-REQUEST REVIEW; DO NOT MERGE WITHOUT EXPLICIT LEAD ARCHITECT AUTHORIZATION.`
