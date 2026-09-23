# SutraERP L4 R3A — IAM Safe Authority — Architecture Approval Decision — 2026-09-23

**Version:** 1.0.0  
**Status:** Architecture Approved  
**Owner:** Lead Architect  
**Architecture Level:** L4 — System Behavior Specification Governance  
**Decision Type:** Post-L5-validation upstream blocker remediation / controlled L4 parent-set approval  
**Scope:** `L4-A-053`, `L4-A-054`, `L4-A-055`  
**Downstream Impact:** partial D01 blocker remediation; no automatic L5 promotion  

---

## 1. Decision

The following three canonical L4 System Behavior Specifications have completed scoped validation and are approved to advance from `Draft` to `Architecture Approved`:

- `L4-A-053` — Person Institutional Exit to Governed Identity Closure;
- `L4-A-054` — Existing Institutional Population to Governed Canonical Identity Onboarding;
- `L4-A-055` — Institutional Authentication Provider Onboarding to Governed Provider Availability.

**Verdict:** `PASS / Architecture Approved` for R3A.

This decision intentionally excludes `L4-A-004` through `L4-A-009`, which retain genuine upstream semantic/authority blockers requiring controlled L2/L3 remediation before L4 approval.

---

## 2. Validation Basis

Validation used canonical `main` at:

`78b26bc0316d60be07e92508b423575b3be56f15`

and current Locked L1/L2/L3 and L4 governance/validation standards.

No L5 specification was used as authority to back-fill business semantics.

---

## 3. Hard-Fail Result

The three scoped specifications pass the L4 hard-fail review.

Verified:

- A053 preserves Person/Identity ownership and only coordinates source-governed closure and authentication-session consequences;
- A054 preserves per-record onboarding outcomes without inventing a Migration Job aggregate/lifecycle;
- A055 preserves Authentication Provider ownership, eligibility and canonical provider events without selecting vendor/protocol mechanisms;
- unresolved actor/order/recovery details remain bounded and do not become invented L4 authority;
- no API/database/service/UI/infrastructure details leak into L4.

---

## 4. Controlled Non-Blocking Conditions

### A053

The exact relationship-owning-domain exit handoff, privacy/deletion approval actors and exact Person/Identity archival ordering remain upstream details.

L4 preserves them as bounded issues while still enforcing that closure cannot be claimed complete while required authentication/session consequences remain incomplete.

### A054

No canonical Migration Job aggregate, job lifecycle/events or formal migration-approver role exists.

L4 operates only on source-defined per-record onboarding outcomes and preserves source-approved corrective recovery.

### A055

Provider lifecycle contains Configured/Validated concepts without separate configure/validate commands/events, and no universal human onboarding authority is defined.

L4 preserves only source-supported provider registration/enablement and canonical provider events.

---

## 5. Explicitly Unresolved R3 Set

The following remain unapproved and require upstream architecture remediation:

```text
L4-A-004
L4-A-005
L4-A-006
L4-A-007
L4-A-008
L4-A-009
```

Major unresolved subjects include:

- Person establishment/verification authority;
- merge/split approval authority and lifecycle consequences;
- Identity/User Account effects of merge/split;
- downstream reference reconciliation contract;
- User Account authentication-eligible state terminology;
- Authentication event publishers;
- Authentication Session lifecycle wording;
- Credential-generic recovery semantics;
- replacement Credential activation/invalidation/event semantics;
- administrative recovery authority;
- MFA enrollment lifecycle/command wording.

These shall not be repaired at L4 or L5.

---

## 6. Downstream Effect

R3A removes Draft-L4 authority for these three parents only.

No D01 L5 blocker is fully cleared solely by this approval because the D01 family still materially depends on unresolved A004–A009 semantics.

---

## 7. Final Decision

```text
R3A candidates reviewed                3
Architecture Approved                  3 / 3
L4-A-004…009                           PRESERVED BLOCKED
L5 specifications changed              0
L5 blocker dispositions changed        0
New L5 ADRs                            0
Final L5 Corpus Lock                   NOT AUTHORIZED
```

**Final Decision:** `R3A PASS — SAFE IAM PARENTS APPROVED; CORE IAM SEMANTIC REMEDIATION REMAINS REQUIRED UPSTREAM`.
