# SUTRAERP INTEGRITY FINDINGS — SECTION A L3 PHASE 1 CONFLICT REGISTER

**Date:** 2026-08-08
**Scope:** Repository conflicts surfaced by the independent Section A Phase 1 L3 verification
**Source review:** `generated/reviews/l3-phase1-section-a-verification-2026-08-08.md`
**Finding ID block:** INT-301 … INT-304 (INT-3xx allocated to this cycle; INT-0xx and INT-2xx are in use by prior cycles)
**Applied by:** Section A Phase 1 audit application, 2026-08-08

---

## METHOD AND SCOPE DISCLOSURE

Every statement quoted below was re-verified against the working tree on 2026-08-08 with
file and line citations, independently of the source review. Two findings are conflicts
between locked architecture documents and are **recorded, not resolved** — resolving them
requires an Enterprise Architecture Board decision under
`L2-A00_Platform_Foundation_Architecture.md:3031`. Two are unambiguous defects and were
corrected the same day; their remediation is recorded at the end of this file.

This register did not review the free prose of the Section A corpus in full. A01, A02,
A05 and A06 total 30,477 lines and were read at their ownership, scope, lifecycle,
aggregate, workflow and event sections.

---

# FINDINGS

---

Issue ID          : **INT-301**
Conflict Type     : Type 2 — Ownership Conflict (duplicate canonical ownership)
File A            : `architecture/L2_Domain_Workflow_Architecture/section-a/L2-A02_Person_and_Identity_Workflows.md`
File B            : `architecture/L2_Domain_Workflow_Architecture/section-a/L2-A03_Authentication_Workflows.md`, `architecture/L2_Domain_Workflow_Architecture/section-a/L2-A04_Authorization_and_Policy_Workflows.md`
Concept           : Canonical ownership of Credential, Authentication Session, MFA / Authentication Factors, Role, Role Assignment, Delegation
Severity          : **Critical** — BLOCKER for L3 Phase 2 consolidation
Confidence        : Certain

**Canonical Source:** `L2-A00_Platform_Foundation_Architecture.md:414` — "Every architectural
capability SHALL have exactly one canonical owner." `:427` — "Ownership SHALL never be
shared." `:3031` — "The Enterprise Architecture Board SHALL determine the canonical owner."
All three documents carry `Status: Locked` and are Level 2 peers, so document rank does not
adjudicate between them.

**Conflict Description:**

`L2-A02` claims exclusive ownership of authentication and authorization concepts:

- `L2-A02:130-136` — "The Person & Identity domain is the exclusive owner of:" … "**Login
  Credentials**", "**Authentication Factors**". `:138` — "No other domain may create,
  directly modify, or delete these aggregates."
- `L2-A02:443-446` — Canonical Aggregates table, "owned exclusively by this domain":
  `User Account | Canonical`, **`Session | Canonical`**, **`Authentication Credential |
  Canonical | Login secrets and authentication factors`**.
- `L2-A02:183` — Domain Objectives: "Centralize authentication and authorization."
- `L2-A02:201-209` — Core Responsibilities: "Login credential management",
  "**Multi-factor authentication**", "**Session ownership**".
- `L2-A02:2036-2037`, `:2602-2603`, `:3095-3097` — Published Events include `RoleAssigned`
  and `RoleRevoked`. `:1613-1615`, `:3109-3111` — `SessionStarted`, `SessionEnded`.

`L2-A03` claims the same aggregates exclusively:

- `L2-A03:188` — "The Authentication domain is the exclusive owner of the following platform
  capabilities." `:192-198` — `Credential`, `Authentication Session`, `MFA Enrollment`.
- `L2-A03:209-211` — Lifecycle Ownership: "Credential lifecycle", "Session lifecycle".
- `L2-A03:2097` — "The Credential aggregate is the **sole owner** of authentication
  credentials."

`L2-A04` claims Role and Delegation exclusively:

- `L2-A04:331` — "The following aggregates are owned exclusively by the Authorization
  domain." `:335` `Role`, `:339` `Delegation`.

**`L2-A02` also contradicts itself in three places.** Its own matrices cede every one of
these capabilities:

- `L2-A02:7880-7881` — Canonical Ownership Matrix: "Authentication | **L2-A03**
  Authentication", "Authorization | **L2-A04** Authorization & Policy".
- `L2-A02:8058-8060` — Integration Responsibility Matrix: "Authentication | **L2-A03**",
  "Authorization | **L2-A04**", "**Session Management | L2-A03 Authentication**".
- `L2-A02:7897` — "Ownership is singular and unambiguous."

`L2-A03:109-127` (Out of Scope) reciprocally cedes Person Lifecycle, Identity Lifecycle and
User Account Lifecycle back to `L2-A02`. `L2-A04:136-160` (Out of Scope) cedes Login,
Authentication, MFA, Sessions, Credentials and Password Policies to `L2-A03`, and Person,
Identity and User Account to `L2-A02`. `L2-A04:557-576` (Ownership Boundaries) repeats it.

The reciprocity is one-directional: A03 and A04 cede to A02, and A02's *matrices* cede back,
but A02's *ownership, aggregate, responsibility and event sections* do not.

**Prior partial remediation.** Integrity finding INT-016
(`generated/reviews/event-naming-registry-2026-08-04.md`, applied 2026-08-04) renamed the
colliding event **names** — `SessionCreated` → `PersonSessionCreated`, `SessionExpired` →
`PersonSessionExpired`, `AuthenticationFailed` → `PersonAuthenticationFailed` — giving A02 its
own namespace. That resolved the naming collision. It did **not** address aggregate
ownership, and row 295 of that register records the underlying dispute as still open:
"publisher attribution disputed: scan says A02, review says A03."

**Repository Impact:**

Six of the ten Section A platform capabilities cannot be assigned an unambiguous owner for
the concepts above. Ten Phase 1 L3 candidates sit directly on the fault line:

| Candidate | Contested concept |
|---|---|
| CAN-L3-A-005 | User Account and its credentials, at the boundary of Person establishment |
| CAN-L3-A-006, CAN-L3-A-007 | Person aggregate is uncontested; listed for completeness only |
| CAN-L3-A-008 | Authentication Session |
| CAN-L3-A-009 | Credential |
| CAN-L3-A-010 | MFA Enrollment / Authentication Factors |
| CAN-L3-A-011 | Role, Role Assignment, `RoleAssigned` / `RoleRevoked` publisher |
| CAN-L3-A-012 | Delegation (contested indirectly via `L2-A02` Delegated Administration) |
| CAN-L3-A-056, CAN-L3-A-057, CAN-L3-A-058 | Workspace, Separation of Duties Policy, Authorization Policy — A04 aggregates in the same contested block |
| CAN-L3-A-059, CAN-L3-A-060, CAN-L3-A-061 | Person lifecycle closure, identity onboarding, provider onboarding — adjacent to the User Account boundary |

Role assignment is the mechanism every business domain in Sections B through J depends on
for authorization. An L3 scenario authored before resolution would carry duplicate ownership
into L4 System Behavior Specification and L5 Solution Architecture, where it becomes
expensive to unwind. This is the same defect class as INT-202 (dependency cycle), which was
treated as Critical.

**Authority Analysis:**

Applying the repository precedence order: (1) no locked repository decision resolves this;
(2) `L2-A00:414` and `:427` are governance standards and forbid shared ownership but do not
name the owner; (4) locked L1 `:2.2` assigns "Person → Identity" and is silent on Credential,
Session, MFA and Role Assignment, so L1 does not adjudicate; (8) the three L2 sources are
peers.

The evidence within the sources points one way — three of A02's own matrices, plus A03's and
A04's reciprocal Out-of-Scope tables, place authentication with A03 and authorization with
A04, against A02's four claiming sections. **This is a logical inference and is recorded as
such. It is not a repository decision and must not be treated as one.**

**Resolution:** **NOT RESOLVED. Requires Enterprise Architecture Board decision under
`L2-A00:3031`.**

Two coherent options; the Board must select one explicitly and record it as a locked
decision:

1. **Align `L2-A02` with its own matrices.** Remove Login Credentials, Authentication
   Factors, Session, Authentication Credential, MFA, Role, Role Assignment and Delegation
   from A02's exclusive-ownership statements at `:130-136`, `:443-446`, `:183`, `:201-209`,
   and remove `RoleAssigned`, `RoleRevoked`, `SessionStarted`, `SessionEnded` from A02's
   Published Events. This is the minimal-change option and makes A02 internally consistent.
2. **Issue a locked decision recording a different split**, and amend `L2-A03`, `L2-A04` and
   A02's three matrices to match.

No L2 ownership statement was amended by this register. Affected Phase 1 candidates retain
their audit-recommended Primary Owner with an explicit `owner contested — INT-301` marker so
the conflict remains visible until the Board rules.

---

Issue ID          : **INT-302**
Conflict Type     : Type 3 — Lifecycle / Locked-Decision Conflict (L2 violates locked L1)
File A            : `architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md`
File B            : `architecture/L2_Domain_Workflow_Architecture/section-a/L2-A10_Operational_Readiness_Workflows.md`
Concept           : Whether Operational Readiness is a calculated, non-persisted evaluation or a persisted, certified governance artifact
Severity          : **Critical** — BLOCKER for L3 Phase 2 consolidation
Confidence        : Certain

**Canonical Source:** `SutraERP_L1_Business_Domain_Architecture_Locked.md` — `Status: LOCKED`.
`:3128` — "**No Level 2 workflow may violate a locked decision defined in this document.**"
The repository precedence order places locked L1 above original L2 source documents.

**Conflict Description:**

Locked L1, section 15 (L1-06 — Operational Readiness):

- `L1:1620-1621` — Business Rules: Operational Readiness "**is calculated**", "**is not
  stored**".
- `L1:1539` — "**No global 'Institute Ready' flag exists.**"
- `L1:1629-1630` — Canonical Entities: "**Readiness Rule**", "**Readiness Result**".
- `L1:1645` — Publishes: `OperationReady`, `OperationBlocked`. Outputs: `READY` / `NOT READY`.
- `L1:1652` — Locked Decisions: "Operational readiness is evaluated dynamically."
- `L1:3095` — Final Lock Decisions table: "Operational Readiness evaluated dynamically |
  **LOCKED**".

`L2-A10` defines a persisted governance model:

- `L2-A10:40-45` — front matter `canonical_entities`: `Operational Readiness Assessment`,
  `Readiness Requirement`, `Readiness Finding`, `Readiness Improvement Plan`, **`Operational
  Readiness Certificate`**.
- `L2-A10:229` — section 5.5 "Immutable Governance History".
- `L2-A10:326` — section 9.1 Aggregate Root: "Operational Readiness Assessment".
- `L2-A10:506` — section 10.4 Operational Readiness Certificate, with a seven-state
  lifecycle at section 20.3 (Draft, Issued, Active, Suspended, Superseded, Revoked, Archived).
- `L2-A10:2920` — "`OperationalReadinessCertificateIssued`: Notifies B01 to update the
  institution's **readiness flag**, enabling academic and admission operations."

`L2-A10:2920` and `L1:1539` are directly contradictory statements about the same artifact.
One says an institution readiness flag is set by a certificate event; the other says no such
flag exists.

`L2-A10` contains **no reference to L1-06** and no reconciliation or amendment note.

**Partial alignment, recorded for accuracy.** `L2-A10` does retain calculation:
`:1441-1442` and `:1694` publish `DomainReadinessCalculated` and
`OperationalScopeReadinessCalculated`. The divergence is the **persistence** of the Assessment
and Certificate aggregates and the derived institution readiness flag, not the loss of dynamic
evaluation.

**Repository Impact:**

Nine Phase 1 L3 candidates — CAN-L3-A-047 through CAN-L3-A-055, sixteen percent of the
Section A inventory — describe outcomes whose central artifacts are contested.
CAN-L3-A-051 (certificate issuance) and CAN-L3-A-052 (certificate lifecycle) depend entirely
on the Certificate aggregate existing.

Beyond L3: `L1:5` places L1-06 Operational Readiness in the canonical institutional lifecycle
between Academic Calendar Setup and Admission Management, and `L2-A10:20-25` declares
`consumed_by: L2-B01 … L2-B05`. Section B and Section C go-live sequencing depend on which
model is authoritative. `L2-A10:2920` would have B01 hold institutional readiness state that
locked L1 says does not exist.

**Authority Analysis:**

Locked L1 governs. `L1:3128` is unambiguous and `L1:3095` records the decision as LOCKED in
the Final Lock Decisions table.

This is the same divergence class as integrity findings INT-006 (Tenant lifecycle), INT-007
(StudentContext gate) and INT-008 (Institute lifecycle). In all three cases the repository
resolved the divergence by **amending L1** so the L2 document became a strict extension rather
than a divergent state machine — see the L1 amendment history at `L1:5-6`. That precedent
favours amendment over constraint, but the choice is a Board decision, not an audit finding,
and the direct `:2920` / `:1539` contradiction must be resolved explicitly rather than absorbed.

**Resolution:** **NOT RESOLVED. Requires Enterprise Architecture Board decision.**

Two coherent options:

1. **Amend locked L1 sections 15 and 29** to recognise persisted readiness governance
   artifacts, following the INT-006 / INT-007 / INT-008 precedent, and rule explicitly on
   whether an institution readiness flag exists. This preserves `L2-A10` as written.
2. **Constrain `L2-A10`** so readiness remains calculated and the Assessment and Certificate
   aggregates are recorded as governance *evidence of a calculation* rather than as the
   authoritative readiness state, and remove or reword `L2-A10:2920`.

Neither locked L1 nor `L2-A10` was amended by this register. CAN-L3-A-047 through
CAN-L3-A-055 are retained in the Phase 1 inventory — Phase 1 preserves discovery breadth —
and are marked `provisional — INT-302` so no L3 authoring proceeds under either assumption
until the decision is recorded.

---

Issue ID          : **INT-303**
Conflict Type     : Type 1 — Terminology / Title-Scope Mismatch
File A            : `architecture/L2_Domain_Workflow_Architecture/section-a/L2-A08_Module_Registry_Configuration_Workflows.md`
File B            : `architecture/L2_Domain_Workflow_Architecture/section-a/L2-A01_Platform_Tenant_Foundation_Architecture.md`, `architecture/ARCHITECTURE_INDEX.md`
Concept           : Whether L2-A08 owns configuration
Severity          : Medium
Confidence        : Certain

**Canonical Source:** `L2-A08` section 5 (Canonical Ownership) — the document's own
enumeration of what it owns.

**Conflict Description:**

The filename, the H1 at `L2-A08:1`, the Document Metadata "Document Name" field at
`L2-A08:33`, `architecture/ARCHITECTURE_INDEX.md:83` and
`L2_DOMAIN_WORKFLOW_MATRIX.md:63` all read "Module Registry **& Configuration** Workflows".

The document disclaims it:

- `L2-A08` section 2.2 Outside the Scope — "The Module Registry does **not** own: …
  **Runtime configuration**".
- `L2-A08` section 5.1 Explicit Non-Ownership — "Runtime Configuration | Platform Foundation".
- `L2-A08` section 5 Canonical Ownership lists seventeen owned concepts. **None is
  configuration.**
- `L2-A08` section 1.1 — "The registry maintains architectural metadata only."

The actual owner is `L2-A01` section 6 (Platform Configuration Framework), with ownership
distributed by layer at section 6.5, corroborated by `L2-A01` section 3.9 ("The Platform
owns: … Platform Configuration") and `L2-A00:2953` section 7.11 ("Configuration SHALL have
one authoritative owner").

There is no ownership conflict — A08 and A01 agree. The defect is that a document's title
misstates its scope and the misstatement is propagated into the Architecture Index and the
Workflow Matrix.

**Repository Impact:**

No Phase 1 candidate is affected in substance: CAN-L3-A-031 through CAN-L3-A-036 all stay
within module metadata. The risk is prospective and is compounded by INT-304 — a reader
triangulating configuration ownership from A08's title and A09's pointer reaches two wrong
answers and never reaches `L2-A01` section 6.5. Newly added candidate CAN-L3-A-062 (Platform
Configuration) must be owned by A01, and the A08 title is the only artifact suggesting
otherwise.

**Resolution:** **FIXED 2026-08-08 by clarification note, not by rename.**

A rename was assessed and rejected as disproportionate. `L2-A08` carries `Status: Locked`
(CHANGELOG `[1.6.0]`, 2026-08-07 — "final lock decision for the business architecture
levels"), and its title string is referenced by `ARCHITECTURE_INDEX.md:83`,
`L2_DOMAIN_WORKFLOW_MATRIX.md:63`, `L2-A09:4084`, `L2-A10:19`,
`SECURITY_REVIEW_CHANGELOG.md:51` and three generated `AI_CONTEXT` artifacts, one of which
(`SECURITY_REVIEW_CHANGELOG.md`) is a historical record that must not be rewritten. Changing
a locked document's identity across eight files is a change-management decision under
`governance/constitution/Volume_5_Change_Management.md` section 7, where "Corrections,
clarifications, documentation improvements" are Patch class and repository reorganization is
Major class.

Applied: a scope-clarification blockquote after `L2-A08:1` stating that the title does not
establish configuration ownership, naming `L2-A01` section 6 as the actual owner, and
recording why the title was retained. `L2-A08` section 5.1's "Platform Foundation" pointer —
ambiguous, since Platform Foundation is the whole of Section A — was made precise to
"`L2-A01` section 6 Platform Configuration Framework (ownership distributed by layer in
section 6.5)". A rename remains available to the Architecture Review Board.

---

Issue ID          : **INT-304**
Conflict Type     : Type 5 — Broken Reference (dangling ownership pointer)
File A            : `architecture/L2_Domain_Workflow_Architecture/section-a/L2-A09_Master_Data_Workflows.md`
File B            : `architecture/L2_Domain_Workflow_Architecture/section-a/L2-A10_Operational_Readiness_Workflows.md`, `architecture/L2_Domain_Workflow_Architecture/section-a/L2-A01_Platform_Tenant_Foundation_Architecture.md`
Concept           : Which document owns Configuration Data
Severity          : Medium
Confidence        : Certain

**Canonical Source:** `L2-A01` section 6.5 Configuration Ownership — the repository's only
configuration ownership table.

**Conflict Description:**

`L2-A09` section 3.4 (pre-correction, line 339) — "Configuration Data … is owned by the
configuration management domain that will be defined separately in **A10**."

`L2-A09` section 59 Terminology (pre-correction, line 4060) — "Configuration Data | Runtime
or tenant-specific settings owned by the configuration management domain, not by the Master
Data Platform."

`L2-A10` is **Operational Readiness Workflows**. Its front matter `business_capabilities`
(`:32-39`) lists eight readiness capabilities; its `canonical_entities` (`:40-45`) lists five
readiness entities. Sections 1.2 and 4.1 define no configuration responsibility.
**`L2-A10` does not define a configuration management domain and never did.**

The phrasing "will be defined separately in A10" indicates the reference predates A10 being
authored as Operational Readiness. It is a dangling pointer to a domain that was never
created at that address.

`L2-A09`'s own RACI at section 58 is internally consistent with `L2-A01` — "Manage Platform
Configuration | Platform Administrator | R/A" — which confirms this is an isolated stale
reference rather than a systemic divergence.

**Repository Impact:**

No current Phase 1 candidate is affected: CAN-L3-A-037 through CAN-L3-A-046 stay within
Master Data and none claims configuration. Combined with INT-303, the effect is that
configuration ownership had one correct home (`L2-A01` section 6.5) and two misleading
signposts. Newly added candidate CAN-L3-A-062 would have been mis-assigned to A10 by anyone
following A09 section 3.4.

**Resolution:** **FIXED 2026-08-08.**

Both occurrences repointed to `L2-A01` section 6, naming the section 6.5 layer distribution
explicitly (Platform Defaults and Tenant Configuration → L2-A01; Institution Configuration →
L2-B01; Organisation Configuration → L2-B02; Business Domain Configuration → respective
Level 2 domain; Operational Runtime Configuration → Platform Services). A correction note
records the change in place.

A09's substantive boundary statement — "The Master Data Platform shall not absorb
configuration ownership, even when configuration values are reused across multiple domains" —
was **not** modified. It is correct and consistent with `L2-A01`. **No configuration domain
was created and no configuration ownership was invented, moved or inferred.** This is the
same remediation pattern as INT-206, where consumed events were repointed to their real
publishers in locked documents without altering ownership.

---

# REMEDIATION SUMMARY — 2026-08-08

| ID | Severity | Outcome |
|---|---|---|
| INT-301 | Critical | **RESOLVED 2026-08-13 — APPROVED ARCHITECTURE DECISION.** A02 owns Person, Identity, User Account and identity/context references. A03 owns Credential, Authentication Session, authentication factors/MFA Enrollment, Authentication Provider and their lifecycles. A04 owns Role, Role Assignment, Delegation and authorization-policy lifecycles. Locked L2 sources and affected L3 owner markers were amended. |
| INT-302 | Critical | **RESOLVED 2026-08-13 — APPROVED ARCHITECTURE DECISION.** L1-06 is dynamic, operation-specific Capability Readiness Evaluation and returns non-persisted `READY`/`NOT READY`. L2-A10 is Operational Readiness Governance & Certification and persists assessments, evidence references, findings, improvement plans and certificates. Certificates are evidence and never a global readiness flag. Locked L1/L2 sources, B01 handoff and affected L3 wording were amended. |
| INT-303 | Medium | **Fixed by clarification.** Scope note added after `L2-A08:1`; section 5.1 pointer made precise. Title deliberately retained — rationale recorded. No ownership changed. |
| INT-304 | Medium | **Fixed.** Two dangling `A10` configuration references in `L2-A09` repointed to `L2-A01` section 6.5. A09's substantive boundary statement unchanged. No configuration domain created. |

## Architecture decisions applied 2026-08-13

The user approved the audit recommendations as the required architecture authority for this repository change. The two conflicts are no longer open:

1. **INT-301** now has a single-owner boundary across A02, A03 and A04. The A02 source carries an explicit authoritative amendment over remaining legacy implementation prose; the A03 and A04 canonical ownership sections record their exclusive concepts. Phase 2 may author the affected authentication and authorization candidates under those owners.
2. **INT-302** now separates the runtime decision from governance evidence. Dynamic `L1-06` evaluation remains authoritative for one requested operation. A10 assessments and certificates remain durable governance artifacts, but no certificate event sets an institution-wide readiness flag. Phase 2 may author the A10 block within that boundary.

The approval and exact recommended rulings are preserved in `generated/reviews/L3_Phase1_Audit_Dhruv_Correction_Application_2026-08-13.md`; the completed application is recorded in `generated/reviews/L3_Phase1_Audit_Dhruv_Approved_Decisions_Application_2026-08-13.md`.

## Verification note

The two corrections applied under INT-303 and INT-304 touch documents carrying
`Status: Locked`. Both are Patch-class under
`governance/constitution/Volume_5_Change_Management.md` section 7 — "Corrections,
clarifications, documentation improvements". Neither changes canonical ownership, lifecycle
ownership, event ownership, dependency direction, terminology of a canonical entity, or any
locked decision. Both were applied in place with an inline record of what changed and why,
following the correction convention already used for INT-004, INT-008 and INT-018.
