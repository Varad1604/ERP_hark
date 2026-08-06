# SUTRAERP REPOSITORY-WIDE INTEGRITY & SEMANTIC CONSISTENCY REVIEW

**Review date:** 2026-08-06
**Working tree:** branch `dhruv`, HEAD `0613700`
**Governing constraint:** the verification addendum in
`generated/reviews/review-repository-integrity-2026-08-05.md:457-462` requires that every
finding cite a file path and line number read at review time. Every citation below was
read from the working tree during this review. No finding was carried forward from a
prior review.

---

## METHOD AND SCOPE DISCLOSURE

Read in full (46 files): `README.md`, `CONTRIBUTING.md`, `DEPENDENCY_MAP.md`,
`CHANGELOG.md`, `architecture_audit_report.md`, `architecture/ARCHITECTURE_INDEX.md`,
`L2_DOMAIN_WORKFLOW_MATRIX.md`, `TEAM_ASSIGNMENTS.md`,
`governance/constitution/Volume_2_Repository_Standards.md`,
`governance/policies/Architecture_Level_Validation_Standards.md`,
`AI_CONTEXT/INDEX.md`, `AI_CONTEXT/REPO_UNDERSTANDING.md`,
`AI_CONTEXT/SECTION_G/Dependency_Contract.md`, all four L3-L6 READMEs, both L5 source
documents, all six templates, and the two most recent prior reviews.

Analysed by targeted extraction plus verification reads (120 files): the 107 L2
documents, the L0 and L1 baselines, and the AI context corpus. The L2 corpus alone is
11.6 MB. It was not read line-by-line. It was processed by mechanical extraction of
document-control metadata, dependency declarations, event identifiers, lifecycle state
tables and cross-document references, after which **every candidate finding was
confirmed by opening the cited file at the cited line.** Findings below are what
survived that confirmation step; the registers are complete for the extracted
dimensions, not for free prose.

**Consequence for the reader:** absence of a finding in a given dimension of a given file
is not proof of consistency in that file's prose. It is proof that the mechanical
register and the verification reads found nothing.

---

## REPOSITORY INTEGRITY SCORECARD

| Integrity Dimension | Score /10 | One-Line Verdict |
|---|---:|---|
| Terminology Consistency | 8 | Result-publication and Graduation migrations are complete; a residual completion-event divergence survives in C08. |
| Workflow Consistency | 7 | Workflow inventory, IDs and titles reconcile cleanly across Index, Matrix and files. |
| Lifecycle & State Consistency | 8 | Student and Tenant lifecycles were reconciled to L1; no new divergence found. |
| Business Rule Consistency | 7 | No contradictory rule pairs surfaced; scholarship and infrastructure boundaries now hold. |
| Ownership Consistency | 4 | 44 L2 documents carry a second, non-canonical `Owner` field that the 2026-08-05 ownership rule forbids. |
| Dependency Consistency | 3 | Section G declares C/D/F upstream against a map granting only A/B/E, producing a D↔G cycle. |
| Cross-Reference Accuracy | 4 | Two L5 documents exist that every control artifact denies; README cites a deleted `tools/`. |
| Domain Event Consistency | 6 | `ExaminationResultPublished` is now singular, but C08 subscribes to four events nobody publishes. |
| Aggregate & Entity Consistency | 8 | Alumni, Infrastructure and Scholarship aggregate boundaries are cleanly separated. |
| Structural & Naming Consistency | 6 | All 107 files carry the Volume 3 §18 crosswalk; residual YAML corruption and phantom folder trees remain. |
| **OVERALL INTEGRITY** | **6** | **Materially improved and internally coherent on terminology; still unsafe as a build-order and discovery source of truth.** |

---

## FILE-PAIR CONFLICT MAP

| File A | File B | Concept in Conflict | Conflict Type | Severity |
|---|---|---|---|---|
| `architecture/L5_Solution_Architecture/L5-00_Technology_Stack_Baseline.md`, `L5-01_Infrastructure_Persistence_Baseline.md` | `architecture/L5_Solution_Architecture/README.md:3-5` | Existence of L5 source documents | Type 5/8 | Critical |
| Same two L5 files | `architecture/ARCHITECTURE_INDEX.md:290,327,330` | L5 document count and indexing | Type 8 | Critical |
| Same two L5 files | `README.md:57,82` | L5 declared a placeholder folder | Type 8 | Critical |
| Same two L5 files | `AI_CONTEXT/REPO_UNDERSTANDING.md:118-119` | L5 declared to hold no source documents | Type 8 | High |
| `L2-G02:17`, `G03:17`, `G04:17-18`, `G05:17-18`, `G06:17-18`, `G07:17`, `G09:15-18` | `DEPENDENCY_MAP.md:114` | Section G upstream set | Type 4 | Critical |
| `L2-G02:17`, `G04:18`, `G05:18`, `G06:18`, `G09:16` | `DEPENDENCY_MAP.md:111` | D depends on G while G depends on D | Type 4 | Critical |
| Section G upstream blocks | `DEPENDENCY_MAP.md:326-336` | Runtime register omits every Section G entry | Type 4 | High |
| `AI_CONTEXT/SECTION_G/Dependency_Contract.md:121` | `L2-G00:13` … `L2-G09:11` | Existence of upstream/downstream dependency tables | Type 5 | High |
| `L2-I13:39`, `I14:39`, `I15:39`, `I16:39` | `DEPENDENCY_MAP.md:116` | Section I foundational set omits A and B | Type 4 | High |
| `DEPENDENCY_MAP.md:116` | `DEPENDENCY_MAP.md:130` | A "used by every business domain" vs I's row excluding A | Type 4 | High |
| 44 L2 documents (see INT-205) | `TEAM_ASSIGNMENTS.md:274-278` | Second, non-canonical `Owner` field | Type 2/6 | High |
| `L2-C08:7` | `L2-C08:6858` | Two different `Document Owner` values in one file | Type 2 | Medium |
| `L2-C08:2496-2500` | Sections I and C source files | Four consumed events with no publisher | Type 1/5 | High |
| `L2-C08:2497` | `L2-C05` event set | `QualificationCompleted` vs `QualificationAwarded` | Type 1 | High |
| `governance/constitution/Volume_2_Repository_Standards.md:147` | `architecture/ARCHITECTURE_INDEX.md:337-351` | "mirrors this table exactly" vs a deliberate 9-of-13 subset | Type 8 | Medium |
| `architecture/ARCHITECTURE_INDEX.md:365` | `governance/constitution/Volume_2_Repository_Standards.md:129-145` | Volume 2 v1.1 described as a 9-value vocabulary; it defines 13 | Type 8 | Medium |
| `README.md:50-51` | Filesystem | `tools/` listed as current structure; directory deleted | Type 5 | Medium |
| `README.md:26,46-47` | `generated/` | Generated HTML presentation artifacts claimed; none exist | Type 5 | Medium |
| `CHANGELOG.md:5` | `DEPENDENCY_MAP.md:388`, `TEAM_ASSIGNMENTS.md:149`, `ARCHITECTURE_INDEX.md:359` | Changelog ends 2026-08-04; two remediation rounds unrecorded | Type 8 | Medium |
| `CHANGELOG.md:5` | `CONTRIBUTING.md:100` | Mandatory post-merge changelog update not performed | Type 8 | Medium |
| `L2-C04:21` | NONE | Corrupted YAML dependency entry | Type 6 | Medium |
| `AI_CONTEXT/INDEX.md:23,29` | `AI_CONTEXT/SECTION_H/` | `README.md` listed as present; deleted from working tree | Type 5 | Low |
| `L2-C08:6954-6970` | `governance/constitution/Volume_2_Repository_Standards.md:83` | Non-existent folder tree presented as repository structure | Type 5/6 | Low |
| `L2-B05:14` | `L2-B05:15` | Duplicate frontmatter keys `document_owner` / `document-owner` | Type 6 | Low |
| `generated/reviews/review-repository-integrity-2026-08-05.md:440` | `architecture_audit_report.md:1-5` | Addendum states the file does not exist; it does | Type 8 | Observation |

---

# FINDINGS

---

Issue ID          : **INT-201**
Conflict Type     : Type 5 + Type 8 — Broken Reference / Index Mismatch
File A            : `architecture/L5_Solution_Architecture/L5-00_Technology_Stack_Baseline.md`, `architecture/L5_Solution_Architecture/L5-01_Infrastructure_Persistence_Baseline.md`
File B            : `architecture/L5_Solution_Architecture/README.md`, `architecture/ARCHITECTURE_INDEX.md`, `README.md`, `AI_CONTEXT/REPO_UNDERSTANDING.md`
Concept           : Existence and indexing of L5 Solution Architecture source documents
Severity          : **Critical**
Confidence        : Certain

**Canonical Source:** The filesystem. Both documents are committed and tracked
(`git ls-files` lists them). `README.md:221` requires that new architecture documents be
"added to `architecture/ARCHITECTURE_INDEX.md`".

**Conflict Description:**

Two L5 source documents exist:

- `L5-00_Technology_Stack_Baseline.md:1-6` — "L5-00 — Technology Stack Baseline",
  Status Draft, Document Owner Dhruv, Created 2026-08-04, created under integrity
  finding INT-002 to remove a technology stack from L0.
- `L5-01_Infrastructure_Persistence_Baseline.md:1-9` — "L5-01 — Infrastructure
  Persistence & Repository Baseline", Status Draft, Document Owner Devang, Created
  2026-08-05, created under integrity finding INT-008 to remove persistence design from
  `L2-B03`.

Four control artifacts deny them:

- `architecture/L5_Solution_Architecture/README.md:3-5` — "This folder is reserved for
  future L5 Solution Architecture artifacts. **No L5 source documents are present yet.**"
  This README sits in the same directory as the two files.
- `architecture/ARCHITECTURE_INDEX.md:290` — "No L3, L4, L5, or L6 source architecture
  documents are present in this workspace yet."
- `architecture/ARCHITECTURE_INDEX.md:327` — "L5 Solution Architecture Documents Present
  | 0". Line 330 puts total indexed documents at 113, which excludes both.
- `README.md:57` lists `architecture/L5_Solution_Architecture/` under "Architecture
  placeholder folders currently present"; `README.md:82` — "L3 through L6 have
  placeholder folders only and are not yet present as source architecture documents."
- `AI_CONTEXT/REPO_UNDERSTANDING.md:118-119` — "Placeholder folder only — NO SOURCE
  DOCUMENTS PRESENT".

**Repository Impact:** Two real architecture documents are invisible to every discovery
path in the repository. An architect or AI following the index will conclude the
technology stack and the infrastructure persistence model are undocumented, and will
either re-derive them or, worse, re-introduce them into L0 and L2 — undoing the exact
level violations these files were created to fix. The L5 README actively instructs the
reader not to look. Ownership is also unscoped: `TEAM_ASSIGNMENTS.md` defines no L5
ownership scheme, and both files name individuals in a repository whose
`TEAM_ASSIGNMENTS.md:274-277` has moved to capability-area ownership.

Note that both documents are candid about their own irregular status
(`L5-00:22-30` states the content "has not been re-reviewed as L5 architecture" and that
no approved L4 exists). The defect is not that they were created; it is that no control
artifact records them.

**Resolution:** Add an `L5 Solution Architecture` section to
`architecture/ARCHITECTURE_INDEX.md` listing both documents with owner and `Draft`
status; correct the statistics at lines 327 and 330 to 2 and 115; replace lines 3-5 of
`architecture/L5_Solution_Architecture/README.md` with a statement that two extraction
baselines are present and are not L4-traced; correct `README.md:57` and `:82` to exclude
L5 from the placeholder list; add an L5 row to the capability-owner table in
`TEAM_ASSIGNMENTS.md`.

---

Issue ID          : **INT-202**
Conflict Type     : Type 4 — Dependency Conflict (with cycle)
File A            : `architecture/L2_Domain_Workflow_Architecture/section-g/` — G02, G03, G04, G05, G06, G07, G09
File B            : `DEPENDENCY_MAP.md`
Concept           : Section G foundational upstream dependency set
Severity          : **Critical**
Confidence        : Certain

**Canonical Source:** `DEPENDENCY_MAP.md` — `REPO_UNDERSTANDING.md:616` names it
"authoritative for the L1 business domain dependency graph".

**Conflict Description:**

`DEPENDENCY_MAP.md:114` — "Section G - Campus Operations | Sections A, B, E".
`DEPENDENCY_MAP.md:231-233` restates it: Section G "Consumes institutional and workforce
information."

Seven Section G documents declare more, under a heading literally named **Upstream
Dependencies**:

| File | Line | Declared upstream not permitted by the map |
|---|---|---|
| `L2-G02_Laboratory_Management_Workflows.md` | 17 | Section D — Academic Delivery |
| `L2-G03_Hostel_Management_Workflows.md` | 17 | Section C — Student Lifecycle |
| `L2-G04_Transport_Management_Workflows.md` | 17, 18 | Section C, Section D |
| `L2-G05_Library_Management_Workflows.md` | 17, 18 | Section C, Section D |
| `L2-G06_Institutional_Health_Services_and_Medical_Workflows.md` | 17, 18 | Section C, Section D |
| `L2-G07_Security_Operations_Workflows.md` | 17 | Section C |
| `L2-G09_Teacher_Guardian_Scheme_TGS_Workflows.md` | 15, 16, 18 | Section C, Section D, Section F |

`DEPENDENCY_MAP.md:111` declares "Section D - Academic Delivery | Sections A, B, C, E,
**G**". Combined with G02:17, G04:18, G05:18, G06:18 and G09:16 declaring **D** upstream
of G, this is a direct **D ↔ G foundational cycle**, violating `DEPENDENCY_MAP.md:12`
("dependency direction remains acyclic at the architectural level") and rule 1 at
`DEPENDENCY_MAP.md:342`.

The map anticipated this class of error and rules on it directly.
`DEPENDENCY_MAP.md:334-336`: "A section listing another section under 'Depends On'
without a matching entry in Section 6 is **a defect in that section**, not a new
foundational dependency." The Runtime Contract and Event Consumption Register at
`DEPENDENCY_MAP.md:326-332` — the mechanism that exists precisely to legitimise
non-foundational dependencies — contains three rows, **none for Section G**.

**Repository Impact:** Build order is unresolvable. A team sequencing from the map builds
G after A/B/E; a team sequencing from the G files waits on C, D and F. The D↔G cycle
makes both wrong. Because the map's own rule already classifies these as defects, the
repository currently contains a written rule that seven files violate without anything
flagging it.

**Resolution:** Either (a) reclassify these as runtime/event-consumption dependencies —
rename the G heading from "Upstream Dependencies" to "Consumed Contracts" and add the
corresponding rows to `DEPENDENCY_MAP.md:328-332`; or (b) if any is genuinely
foundational, amend `DEPENDENCY_MAP.md:114`. Option (a) fits the evidence: G05's library
dependency on Section C is student-identity lookup, and G09's on Section F is billing
consequence — both consumption, not foundation. The D↔G cycle must be broken under
option (a) regardless.

---

Issue ID          : **INT-203**
Conflict Type     : Type 5 — Broken Reference in a generated authority
File A            : `AI_CONTEXT/SECTION_G/Dependency_Contract.md:121`
File B            : `architecture/L2_Domain_Workflow_Architecture/section-g/` — G00 through G09
Concept           : Whether Section G declares dependency tables
Severity          : High
Confidence        : Certain

**Canonical Source:** The Section G source files.

**Conflict Description:**

`AI_CONTEXT/SECTION_G/Dependency_Contract.md:121`, under "Missing Information", states:

> "No Section G document declares upstream or downstream dependency tables under those
> headings. Dependency direction is expressed through §4 Out of Scope and the Appendix B
> ownership matrices: Not specified in Source as a dependency table."

Ten of the eleven Section G files carry a literal `**Upstream Dependencies**` heading
followed by a bulleted section list, and most also carry `**Downstream Consumers**`:
G00:13, G01:13, G02:13, G03:13, G04:13, G05:13, G06:13, G07:13, G08:13, G09:11.

The file asserts it is authoritative and current: line 5 — "Files processed: 11 of 11
expected"; line 8 — "Fully regenerated from current source documents. No content was
carried forward from the previous version of this file."; line 13 — "No inferred
dependencies."

The same file then contradicts its own line 121 at line 112, which correctly summarises
that "Section G routes to Section A …, Section C (student identity), Section D (academic
operations, practical timetable), Section E …, Section F (finance and billing), Section H
… and Section J".

**Repository Impact:** This is the artifact an AI agent reads *instead of* the eleven
source files. It tells that agent the repository's largest dependency conflict does not
exist. An agent asked to validate Section G dependencies against `DEPENDENCY_MAP.md`
would return a false PASS.

**Resolution:** Regenerate `AI_CONTEXT/SECTION_G/Dependency_Contract.md` against the
current source. Replace line 121 with the actual per-file upstream table, and cross-
reference INT-202. Because line 8 claims full regeneration, the generator itself should
be checked — it evidently did not parse the `**Upstream Dependencies**` bold-paragraph
form.

---

Issue ID          : **INT-204**
Conflict Type     : Type 4 — Dependency Conflict
File A            : `DEPENDENCY_MAP.md:116`
File B            : `L2-I13:39`, `L2-I14:39`, `L2-I15:39`, `L2-I16:39`
Concept           : Section I foundational dependency set
Severity          : High
Confidence        : Certain

**Canonical Source:** `DEPENDENCY_MAP.md`, but the map's own text at line 130 shows the
row is the error.

**Conflict Description:**

`DEPENDENCY_MAP.md:116` — "Section I - Examination | Sections C, D, F". Sections A and B
are absent.

Four Section I documents declare A and B as dependencies:

- `L2-I13_Academic_Completion_CGPA_and_Completion_Evidence_Workflows.md:39` — "Depends On
  | L2-A01 Platform & Tenant Foundation, L2-A02 Person & Identity, L2-A04 Authorization &
  Policy, L2-A06 Event Infrastructure, L2-A07 Audit Infrastructure, **L2-B04 Curriculum
  Framework, L2-B05 Academic Calendar**, L2-I01 through L2-I12"
- `L2-I14:39`, `L2-I15:39`, `L2-I16:39` — "Depends On | L1 Business Domain Architecture,
  **L2-A01–A08, L2-B01–B05**, L2-I01–…"

`DEPENDENCY_MAP.md:130` states Section A "Provides shared infrastructure used by **every**
business domain", and every other section's row in the matrix at lines 108-117 includes
A. Section I is the only row that omits it. Line 146 likewise makes Section B the owner
of "Curriculum framework" and "Academic Calendar", which I13:39 explicitly depends on.

**Repository Impact:** Section I is the only domain whose foundational row understates its
real dependencies. Any tooling built on the matrix would compute an Examination build
that omits Platform Foundation and Institution & Structure. It also means the matrix
cannot be used to validate the I files — the direction of the error is opposite to
INT-202, so a single validation rule cannot catch both.

**Resolution:** Amend `DEPENDENCY_MAP.md:116` to "Section I - Examination | Sections A,
B, C, D, F". This is a correction to the map, not to the four I files, which are right.

---

Issue ID          : **INT-205**
Conflict Type     : Type 2 + Type 6 — Ownership Conflict / Structural Drift
File A            : 44 L2 documents (enumerated below)
File B            : `architecture/L2_Domain_Workflow_Architecture/TEAM_ASSIGNMENTS.md:274-278`
Concept           : Single canonical owner per L2 document
Severity          : High
Confidence        : Certain

**Canonical Source:** `TEAM_ASSIGNMENTS.md:147-268` (Canonical Capability Owners table,
established 2026-08-05 under INT-003) and the rules at lines 274-278.

**Conflict Description:**

`TEAM_ASSIGNMENTS.md:274-277` states:

> "Every Level 2 document has exactly one primary owner, and that owner is a **capability
> area** from the Canonical Capability Owners table above. Individual names shall not
> appear in an L2 document's owner fields. Review boards and approval authorities are
> recorded in separate approval fields, never in the owner field."

The INT-003 remediation added a correct `Document Owner` field to all 107 files but **did
not remove the pre-existing `Owner` field**. 44 documents therefore declare two owners,
the second of which is a review board, a team, an individual's role, or a domain label
that is not in the canonical table:

| Document | Line(s) | Non-canonical `Owner` value | Canonical capability owner |
|---|---|---|---|
| L2-A00 | 7 | Enterprise Architecture Board | Platform Foundation |
| L2-A09 | 7 | Platform Architecture Team | Platform Foundation — Master Data |
| L2-B01 | 6 | Section B Architect | Institution & Structure — Institution |
| L2-B02 | 49 | Architecture Review Board | Institution & Structure — Organisation |
| L2-B05 | 13, 62 | Enterprise Architecture Team | Institution & Structure — Academic Calendar |
| L2-C00 | 9, 45 | Student Lifecycle Domain | Student Lifecycle |
| L2-C02 | 6, 12752, 14010 | Section C Architect / Architecture Review Board / Student Domain Architecture Team | Student Lifecycle — Activation |
| L2-C04 | 8 | Student Lifecycle Architecture | Student Lifecycle — Academic Progression |
| L2-C05 | 13 | Student Lifecycle Domain Team | Student Lifecycle — Qualification Completion & Exit |
| L2-C08 | 6052 | Enterprise Architecture Board | Student Lifecycle — Alumni Transition |
| L2-D01 | 11 | Academic Operations Domain | Academic Delivery — Course & Offering |
| L2-E00 | 9 | Enterprise Architecture Team | People & Workforce |
| L2-F00 | 7 | Section F Architect | Finance |
| L2-F02 / F03 / F04 | 37 each | Finance Domain | Finance — Payments / Refunds / Financial Adjustments |
| L2-F05 | 36 | Finance Architecture Team | Finance — Accounting & General Ledger |
| L2-F07 | 13 | Finance Architecture Board | Finance — Accounts Receivable |
| L2-F12 / F13 / F14 | 35 / 39 / 38 | Finance Architecture Team | Finance — Asset Accounting / Reporting & Analytics / Procurement Consumption |
| L2-G00 … L2-G08, L2-G10 | 10 each | Campus Operations Domain | Campus Operations — <capability> |
| L2-H02 | 35 | Notification Domain | Institutional Services — Notifications |
| L2-H04 / H05 | 10 each | Enterprise Architecture Team | Institutional Services — Placement / Research |
| L2-H06 | 9 | Alumni Domain Team | Institutional Services — Alumni Engagement |
| L2-I00 … L2-I05 | 9 and 45 each | Examination Domain | Examinations — <capability> |
| L2-I14 / I15 / I16 | 35 each | Examination Domain | Examinations — <capability> |

54 occurrences across 44 documents. Three cases are worse than a stale duplicate:

- **L2-C08** declares two different `Document Owner` values: line 7 "Student Lifecycle —
  Alumni Transition" and line 6858 "Enterprise Architecture Team".
- **L2-B01:6** and **L2-C02:6** and **L2-F00:7** name a person's role ("Section B
  Architect", "Section C Architect", "Section F Architect"), which line 276 forbids
  outright.
- **L2-B02:49**, **L2-C02:12752**, **L2-A00:7** and **L2-C08:6052** put review boards in
  the owner field, which line 277 forbids outright.

**Repository Impact:** Modification authority is ambiguous in 41% of the L2 corpus. Under
`TEAM_ASSIGNMENTS.md:279` only the assigned owner may modify a document; with two owner
fields there is no determinate answer to who that is. This is the same
fix-the-header-leave-the-rest pattern the 2026-08-05 addendum identified for `L2-B03`.

**Resolution:** Delete the legacy `Owner` field from all 44 documents. Where the removed
value names a genuine approval authority (Enterprise Architecture Board, Architecture
Review Board, Finance Architecture Board), re-add it as `Review Authority:` per
`TEAM_ASSIGNMENTS.md:277`. Reconcile `L2-C08:6858` to line 7. Then add a repository check
asserting exactly one `Owner`-class field per L2 file, matching
`TEAM_ASSIGNMENTS.md:160-268`.

---

Issue ID          : **INT-206**
Conflict Type     : Type 1 + Type 5 — Terminology / Broken Event Reference
File A            : `L2-C08_Alumni_Transition_Workflows.md:2494-2501`
File B            : `L2-C05_Qualification_Completion_and_Student_Exit_Workflows.md`, Section I files
Concept           : Consumed domain event identifiers
Severity          : High
Confidence        : Certain

**Canonical Source:** The publishing domain's own document. `DEPENDENCY_MAP.md:346`
— "Every business concept has exactly one canonical owner."

**Conflict Description:**

`L2-C08:2490` opens "# 10.5 Consumed Domain Events" — "The Alumni Transition bounded
context consumes business events published by upstream bounded contexts." The table at
lines 2496-2500 lists five events. Four cannot be resolved:

| C08 line | Consumed event | Declared publisher | Verified state |
|---|---|---|---|
| 2496 | `QualificationCompletionVerified` | Examination | Zero occurrences anywhere outside C08 |
| 2497 | `QualificationCompleted` | Qualification Completion | L2-C05 publishes `QualificationAwarded` (13 occurrences); never `QualificationCompleted` |
| 2499 | `AlumniTransitionApprovalCompleted` | Approval Management | Zero occurrences outside C08 |
| 2500 | `AlumniTransitionPolicyUpdated` | Policy Management | Zero occurrences outside C08 |

For line 2497 the correct name is established repeatedly and normatively elsewhere.
`L2-C02:1604` — "This workflow consumes `QualificationAwarded`, published by the
Qualification Completion domain (L2-C05)". `L2-C02:3659` (BR-C02-032) and `L2-C02:4404`
say the same. `QualificationAwarded` is also the canonical example in
`governance/constitution/Volume_2_Repository_Standards.md:177`. C08 repeats the wrong
name in its sequence diagram at line 5783.

Three of the four declared publishers — "Clearance Management", "Approval Management",
"Policy Management" (C08:2498-2500) — are not repository sections and appear in no
ownership table, contrary to `Volume_2:226` ("Business entities shall always reference
their canonical owner").

This is a recognised defect class in this repository. `L2-I13:1376-1378` records the
identical problem being fixed: an event "named `FinalResultPublished` here and nowhere
else in the repository … this document was subscribing to an event no [publisher]
declared."

**Repository Impact:** Alumni Transition cannot be implemented from C08. Four of its five
inbound integration points name events that do not exist, and the fifth
(`ClearanceCompleted`, C08:2498) resolves only to `L2-G05:1840`, not to the "Clearance
Management" domain C08 names. A developer would build subscribers that never fire.

**Resolution:** In `L2-C08:2497` and `:5783` replace `QualificationCompleted` with
`QualificationAwarded` and the publisher with `L2-C05`. For lines 2496, 2499 and 2500,
either identify the real publisher and event name in Section I / Section A workflow
engine and correct the row, or delete the row and record the gap. Replace the three
non-section publisher labels with canonical section or L2 identifiers. Then run the
`FinalResultPublished` check repo-wide: every event listed as *consumed* must be
*published* by a named L2 document.

---

Issue ID          : **INT-207**
Conflict Type     : Type 8 — Governance Document Mismatch
File A            : `governance/constitution/Volume_2_Repository_Standards.md:147`
File B            : `architecture/ARCHITECTURE_INDEX.md:337-351`
Concept           : Canonical document status vocabulary
Severity          : Medium
Confidence        : Certain

**Canonical Source:** `Volume_2_Repository_Standards.md` §7, which is LOCKED (line 8) and
self-declares primacy at line 147.

**Conflict Description:**

`Volume_2:147` — "`ARCHITECTURE_INDEX.md`'s Document Status Legend **mirrors this table
exactly** and shall be kept in sync with it; if the two ever diverge, this section
governs."

Volume 2's table at lines 131-145 defines **13** values (Draft, Proposed, Review, Trust
Certification Ready, Architecture Approved, Approved, Approved Baseline, Architecture
Baseline, Architecture Locked, Certified & Locked, Locked, Deprecated, Archived).

`ARCHITECTURE_INDEX.md:337-347` lists **9**, omitting Proposed, Approved, Deprecated and
Archived. Line 351 does not present this as drift — it declares it deliberate: "Volume 2
defines **13** status values, all of which remain legal; the table above lists those the
repository has used."

So Volume 2 mandates an exact mirror and the Index states, correctly and knowingly, that
it is not one. Both cannot be right.

Separately, `ARCHITECTURE_INDEX.md:365` records that Volume 2 was amended "v1.0 → v1.1 to
formally adopt the **9-value** status vocabulary already in use here". Volume 2 is at
v1.1 (line 6) and defines 13. The historical record misdescribes the amendment it
performed.

**Repository Impact:** A contributor validating a document's status against the Index will
reject `Proposed`, `Approved`, `Deprecated` and `Archived` as illegal, though Volume 2 —
which governs — makes them legal. The instruction at `Volume_2:147` also implies a sync
obligation that, if a maintainer ever honours it literally, will delete the Index's
deliberate and useful "in use" distinction.

**Resolution:** Amend `Volume_2:147` to: "`ARCHITECTURE_INDEX.md`'s Document Status Legend
lists the subset of these values currently in use and shall not introduce a value absent
from this table; this section governs." Correct `ARCHITECTURE_INDEX.md:365` to say
Volume 2 v1.1 adopted a 13-value vocabulary of which 9 are in use.

---

Issue ID          : **INT-208**
Conflict Type     : Type 5 — Broken Reference
File A            : `README.md:26, 46-47, 50-51`
File B            : Filesystem
Concept           : Current repository structure
Severity          : Medium
Confidence        : Certain

**Canonical Source:** The filesystem. `Volume_2:83` — "Planned folders shall not be
referenced as existing paths until they are created." `README.md:222` states the same
rule for the repository.

**Conflict Description:**

`README.md:30` heads the section "Current Repository Structure". Two entries are wrong:

- `README.md:50-51` — "[tools/](/F:/Pravaha/ERP_Planning/tools/) — Local helper scripts
  used for presentation and conversion support." The directory does not exist; `ls tools`
  returns "No such file or directory". `git status` shows `tools/convert_all.js`,
  `tools/paths.js` and `tools/wrap.js` deleted in the working tree.
  `README.md:26` repeats the claim in the philosophy statement: this repository "contains
  architecture, governance, templates, generated presentation artifacts, and helper
  scripts."
- `README.md:46-47` — "[generated/](…) Generated HTML presentation artifacts derived from
  architecture documents." `generated/` contains six markdown review files and no HTML;
  a repo-wide search for `*.html` returns zero files.

**Repository Impact:** `README.md` is the repository's entry point and the first document
an AI agent reads. It directs readers to a deleted directory and describes a class of
artifact the repository does not contain, in the one section headed "Current".

**Resolution:** Delete the `tools/` bullet at `README.md:50-51` and drop "and helper
scripts" from line 26; change lines 46-47 to "Generated review and analysis artifacts
derived from architecture documents." If the deletion of `tools/` is unintentional,
restore the three files instead — but the README must match whichever is chosen.

---

Issue ID          : **INT-209**
Conflict Type     : Type 8 — Control Artifact Not Synchronized
File A            : `CHANGELOG.md:5`
File B            : `CONTRIBUTING.md:100`, `DEPENDENCY_MAP.md:388`, `TEAM_ASSIGNMENTS.md:149`, `ARCHITECTURE_INDEX.md:359`
Concept           : Post-merge changelog obligation
Severity          : Medium
Confidence        : Certain

**Canonical Source:** `CONTRIBUTING.md:97-100` — on merge, "the author of the changes must
immediately update the repository indexes… **CHANGELOG.md**: Record exact changes… under
the appropriate version release block. This guarantees that `main` is always
self-consistent."

**Conflict Description:**

`CHANGELOG.md:5` shows the most recent block as "[1.4.0] - 2026-08-04". At least two
subsequent remediation rounds are recorded elsewhere but not there:

- `TEAM_ASSIGNMENTS.md:149` — "Established 2026-08-05 under integrity finding INT-003"
  (the entire capability-owner model, affecting all 107 L2 documents).
- `DEPENDENCY_MAP.md:311` — "Added 2026-08-05 under integrity finding INT-035" (the
  Dependency Classes section); `DEPENDENCY_MAP.md:388` — "Last Updated: 2026-08-05".
- `ARCHITECTURE_INDEX.md:359-361` — "2026-08-05 — Core context alignment".
- Commits `0613700`, `631690b`, `6914ec8`, `780eb65`, `d10593f` post-date the 1.4.0 block.

**Repository Impact:** The changelog is the repository's history of record for
architectural decisions. It omits the single largest ownership-model change the
repository has made. Anyone reconstructing why 107 documents changed owner format will
find the reason only in a footnote inside `TEAM_ASSIGNMENTS.md`.

**Resolution:** Add a `[1.5.0] - 2026-08-05` block covering INT-003 capability ownership,
INT-035 dependency classes, the INT-004/005/017 boundary separations, the INT-008
event-variant consolidation, and the creation of `L5-01`. Add the 2026-08-06 index/owner
alignment as `[1.5.1]`.

---

Issue ID          : **INT-210**
Conflict Type     : Type 6 — Structural Drift
File A            : `L2-C04_Student_Academic_Progression_Workflows.md:21`
File B            : NONE
Concept           : YAML frontmatter dependency list
Severity          : Medium
Confidence        : Certain

**Conflict Description:** Line 21 of the `depends_on` block reads:

```
  - L2-A07 Audit Infrastructure WorkflowsUpstream Domain Dependencies
```

Two strings are concatenated without a separator: the dependency entry "L2-A07 Audit
Infrastructure Workflows" and a heading fragment "Upstream Domain Dependencies". The
adjacent entries (lines 15-20, 22-25) are clean.

**Repository Impact:** Any parser reading `depends_on` from C04 gets a corrupt final A-
section entry. It also indicates that a heading was absorbed into the frontmatter during
an edit, so the block boundary itself is untrustworthy.

**Resolution:** Split into `  - L2-A07 Audit Infrastructure Workflows` and restore the
"Upstream Domain Dependencies" heading below the frontmatter, or delete the fragment if
the heading exists elsewhere in the document. While there, note that `L2-C04:24`
references "L2-B05 Flexible Academic Calendar Workflows"; the document's actual title is
"Academic Calendar Workflows" (`L2-B05:2`).

---

Issue ID          : **INT-211**
Conflict Type     : Type 5 — Broken Reference
File A            : `AI_CONTEXT/INDEX.md:23, 29`
File B            : `AI_CONTEXT/SECTION_H/`
Concept           : AI context file inventory
Severity          : Low
Confidence        : Certain

**Conflict Description:** `AI_CONTEXT/INDEX.md:23` lists Section H's files present as
"`Dependency_Contract.md`, `H_Canonical_AI_Context.md`, `README.md`". Line 29 adds
"`SECTION_H/README.md` and `SECTION_J/README.md` are outside the prompt's output list and
were left untouched."

`AI_CONTEXT/SECTION_H/` contains only `Dependency_Contract.md` and
`H_Canonical_AI_Context.md`. `git status` shows `AI_CONTEXT/SECTION_H/README.md` deleted.
`SECTION_J/README.md` does still exist.

`AI_CONTEXT/INDEX.md:80` states this index "must be regenerated directly from the
filesystem, never hand-edited from memory" — the rule this violates.

**Resolution:** Remove `README.md` from the Section H row at line 23 and drop the Section
H clause from line 29. Regenerate rather than hand-edit.

---

Issue ID          : **INT-212**
Conflict Type     : Type 5 + Type 6 — Phantom Path
File A            : `L2-C08_Alumni_Transition_Workflows.md:6954-6970`
File B            : `governance/constitution/Volume_2_Repository_Standards.md:83`
Concept           : Declared repository structure
Severity          : Low
Confidence        : Certain

**Conflict Description:** Section "## 26.8 Repository Structure" (C08:6952) presents a
directory tree as fact:

```
L2-C08_Alumni_Transition_Workflows/
├── L2-C08_Alumni_Transition_Workflows.md
├── diagrams/ (D01_Domain_Context.mmd … D20_Complete_Business_Journey.mmd)
├── adr/ (ADR-001.md … ADR-008.md)
└── assets/supporting-images/
```

None of it exists. C08 is a flat file at
`architecture/L2_Domain_Workflow_Architecture/section-c/L2-C08_Alumni_Transition_Workflows.md`;
there is no per-document folder, no `diagrams/`, and the eight ADR files are absent
(`governance/adr/` exists but is empty). `Volume_2:83` — "Planned folders shall not be
referenced as existing paths until they are created." The tree also conflicts with the
flat `section-<x>/` layout used by all 107 documents.

**Resolution:** Either delete section 26.8, or retitle it "Proposed Repository Structure
(not yet created)" and state that the ADRs referenced at lines 6963-6966 are unwritten.

---

Issue ID          : **INT-213**
Conflict Type     : Type 6 — Structural Drift
File A            : `L2-B05_Academic_Calendar_Workflows.md:14-15`
File B            : NONE
Concept           : Duplicate frontmatter keys
Severity          : Low
Confidence        : Certain

**Conflict Description:** The YAML frontmatter declares the same fact under two key
spellings:

```
14: document_owner: Institution & Structure — Academic Calendar
15: document-owner: Institution & Structure — Academic Calendar
```

The values agree, so there is no semantic conflict, but strict YAML parsers treat these
as two distinct keys and the repository has no rule electing one. Section C uses
underscore (`L2-C04:9`); most sections use neither, preferring a markdown metadata table.

**Resolution:** Delete line 15. Separately, elect one metadata form for the repository in
`Volume_2:107-121`, which currently lists field names but not their encoding.

---

## OBSERVATION — accuracy of the governing addendum

`generated/reviews/review-repository-integrity-2026-08-05.md:440` dismisses INT-018 with
"File does not exist in the repository." `architecture_audit_report.md` **does** exist
(10,420 bytes, tracked, listed in `git ls-files`). The underlying finding is nevertheless
resolved — lines 3-5 now read "**Status:** Superseded — 2026-07-31 … must not be treated
as a current certification" — so the substance is correct and the stated reason is not.

This matters only because that addendum is now the repository's record of what is fixed.
Its "Not reproducible" table is the document a future reviewer will trust when deciding
not to re-verify. One wrong row in it is worth correcting.

**Resolution:** Amend line 440 to: "File exists; relabelled as Superseded — 2026-07-31 at
`architecture_audit_report.md:3-5`, which resolves the finding."

---

## CHECKLIST RESULTS

### Terminology checks

| Check | Result |
|---|---|
| Every L1 business entity used with the same name in all L2 files | PASS (extraction scope) |
| No L2 file introduces a new canonical term without L1 authority | FAIL — INT-206, four unpublished event names in C08 |
| Role names consistent across all files | FAIL — INT-205, "Section B Architect", "Finance Architecture Board" etc. persist as owners |
| Section names match README, Index, Team Assignments and L2 headers | PASS |
| Workflow names in L2 files match the L2 Matrix | PASS — all 107 IDs and titles reconcile |
| Status/state names for the same entity identical across files | PASS |

### Ownership checks

| Check | Result |
|---|---|
| Every workflow has exactly one declared owner | FAIL — INT-205, 44 documents declare two |
| Owner in each L2 file matches TEAM_ASSIGNMENTS | PASS for `Document Owner` in 107/107; FAIL for the legacy `Owner` field in 44 |
| Owner in each L2 file matches ARCHITECTURE_INDEX | PASS — Index owner column matches the capability table |
| No workflow appears as primary in more than one L2 file | PASS |
| Finance review team responsibilities consistent | PASS — `TEAM_ASSIGNMENTS.md:134-143`, `CONTRIBUTING.md:63-64`, `README.md:150-153` agree |

### Dependency checks

| Check | Result |
|---|---|
| Every cross-section dependency declared in DEPENDENCY_MAP | FAIL — INT-202, seven Section G files |
| Every DEPENDENCY_MAP dependency reflected in L2 files | FAIL — INT-204, Section I row understates |
| No L2 file declares a dependency on L3–L6 | PASS |
| Dependency direction never goes L2 → L1 or L1 → L0 | PASS |
| No circular dependencies | FAIL — INT-202, D ↔ G |

### Lifecycle and state checks

| Check | Result |
|---|---|
| Shared entities have identical lifecycle states across files | PASS |
| Terminal states consistent | PASS — C02 and L1 reconciled under INT-007 |
| Approval states consistent | PASS |
| Suspension and reactivation states consistent | PASS — `L2-C02:5375-5405` and L1 agree |

### Matrix and index checks

| Check | Result |
|---|---|
| Every file listed in ARCHITECTURE_INDEX exists | PASS |
| Every file that exists is listed in ARCHITECTURE_INDEX | **FAIL — INT-201, both L5 documents unlisted** |
| Every Matrix section matches the actual L2 files | PASS |
| Matrix status matches each section file header | PASS — all 107 `Draft` in Matrix, Index and headers |
| Section ownership in Matrix matches TEAM_ASSIGNMENTS | PASS — Matrix names the Accountable Architect, which matches column 3 of `TEAM_ASSIGNMENTS.md:160-268` |

### Structural checks

| Check | Result |
|---|---|
| All L2 files follow the L2 template structure | PASS — 107/107 carry the Volume 3 §18 crosswalk |
| All Architecture IDs follow `L2-[section][number]` | PASS |
| No file references a folder or path that does not exist | FAIL — INT-208 (`tools/`), INT-212 (C08 tree) |
| README paths match actual repository structure | FAIL — INT-208 |

---

## CONSISTENCY MATRIX

| Concept | Canonical File | Canonical Definition | Conflicting File(s) | Status | Severity |
|---|---|---|---|---|---|
| L5 document inventory | Filesystem | Two L5 baselines exist | L5 README, ARCHITECTURE_INDEX, README, REPO_UNDERSTANDING | Broken Reference | Critical |
| Section G upstream set | `DEPENDENCY_MAP.md:114` | Sections A, B, E | G02, G03, G04, G05, G06, G07, G09 | Contradictory | Critical |
| D ↔ G direction | `DEPENDENCY_MAP.md:12,111` | Acyclic; D depends on G | G02, G04, G05, G06, G09 | Contradictory | Critical |
| Section G dependency declarations | Section G source files | Ten files declare them | `AI_CONTEXT/SECTION_G/Dependency_Contract.md:121` | Broken Reference | High |
| Section I foundational set | `DEPENDENCY_MAP.md:130` | A is used by every domain | `DEPENDENCY_MAP.md:116` | Contradictory | High |
| L2 document owner | `TEAM_ASSIGNMENTS.md:160-278` | One capability-area owner | 44 L2 documents | Duplicate | High |
| Completion event | `L2-C05` + `Volume_2:177` | `QualificationAwarded` | `L2-C08:2497,5783` | Contradictory | High |
| Alumni inbound events | Publishing domains | Must have a declared publisher | `L2-C08:2496,2499,2500` | Broken Reference | High |
| Result publication event | `Volume_2:177` | `ExaminationResultPublished` | none — 32/32 consistent | Consistent | — |
| Qualification vs Graduation | L1 locked architecture | Qualification Completion | none — migration complete | Consistent | — |
| Status vocabulary | `Volume_2:129-147` | 13 values, exact mirror required | `ARCHITECTURE_INDEX.md:337-351,365` | Ambiguous | Medium |
| Repository structure | Filesystem | No `tools/`, no HTML | `README.md:26,46-47,50-51` | Broken Reference | Medium |
| Change history | `CONTRIBUTING.md:100` | Every merge recorded | `CHANGELOG.md:5` | Contradictory | Medium |
| C04 dependency list | YAML well-formedness | One entry per line | `L2-C04:21` | Broken Reference | Medium |
| AI context inventory | Filesystem | SECTION_H has two files | `AI_CONTEXT/INDEX.md:23,29` | Broken Reference | Low |
| C08 folder layout | Filesystem + `Volume_2:83` | Flat `section-c/` file | `L2-C08:6954-6970` | Broken Reference | Low |

---

## FINAL INTEGRITY REPORT

### Executive Summary

The repository has improved substantially since 2026-08-05 and the improvement is real,
not asserted. The terminology migrations that dominated the previous three reviews are
**closed**: `ExaminationResultPublished` is now the single result-publication event
across 32 occurrences with no competing spelling; `Graduation` survives only in the five
L1 passages that normatively declare it replaced; the Alumni, Infrastructure and
Scholarship ownership boundaries hold; all 107 L2 documents carry the Volume 3 §18
section crosswalk; and Matrix, Index and all 107 headers agree on `Draft`. Several
findings this review expected to reproduce were verified as genuinely fixed and are
reported nowhere below.

What remains is a different and narrower class of defect: **the control plane no longer
describes the repository.** The two highest-severity findings are both of this kind. Two
L5 architecture documents exist that the index, the README, the folder's own README and
the AI context all state do not exist. Seven Section G documents declare a dependency
shape that the dependency map forbids — and the AI context generated to summarise
Section G's dependencies states, falsely, that those declarations are absent.

- Files read in full: **46**
- Files analysed by extraction with verification reads: **120**
- Concepts tracked across the three registers: 107 document identities, 107 owner
  declarations, 118 dependency declarations, 6 event-name families, 4 lifecycle state sets
- Confirmed findings: **13** plus 1 observation
- Critical: **2** · High: **4** · Medium: **4** · Low: **3**

**The three most dangerous inconsistencies:**

1. **INT-201 — two L5 documents that no control artifact records.** This is the most
   dangerous because it is silent. The other findings produce a contradiction a reader can
   notice; this one produces confident absence. The technology stack and the
   infrastructure persistence model were deliberately moved out of L0 and L2 to fix level
   violations, and then became undiscoverable. The likeliest failure is that someone
   re-adds them upstream, recreating the exact violation.
2. **INT-202 — the D ↔ G foundational cycle.** Build order is genuinely unresolvable for
   Campus Operations and Academic Delivery, and the dependency map already contains the
   rule (line 334) that classifies these seven declarations as defects, so the repository
   is self-contradicting rather than merely incomplete.
3. **INT-203 — the Section G AI context denies the conflict in INT-202.** This is what
   makes INT-202 durable. Any agent that validates dependencies from the AI context layer,
   as that layer exists to permit, returns a false PASS on the repository's worst
   dependency problem.

### Consistency Breakdown by Type

| Type | Count | Most Severe Example | Pattern |
|---|---:|---|---|
| Type 1 — Terminology | 1 | `QualificationCompleted` in C08 | Near-eliminated. The single survivor is in a *consumed-event* table, not a definition — consumer-side references are where migrations leak. |
| Type 2 — Ownership | 1 | 44 documents with two owner fields | The INT-003 migration added the correct field and did not remove the wrong one. |
| Type 3 — Lifecycle | 0 | — | Fully remediated; no divergence found. |
| Type 4 — Dependency | 2 | D ↔ G cycle | Errors run in both directions: G over-declares, the map under-declares for I. A one-way validator catches neither reliably. |
| Type 5 — Broken Reference | 5 | L5 documents unindexed | Every instance is a control artifact describing a filesystem that has since changed. |
| Type 6 — Structural | 3 | C04 corrupted YAML | Residual damage from bulk edits, not from authoring. |
| Type 7 — Business Rule | 0 | — | No contradictory rule pairs surfaced. |
| Type 8 — Matrix/Index | 4 | Volume 2 vs Index status legend | Control artifacts drift from the documents they govern within a day of those documents changing. |

**The pattern across types.** Nine of thirteen findings are a control artifact
contradicting the working tree, and every one of them was introduced by a *remediation*,
not by original authoring: the L5 files were created by INT-002/INT-008 and never
indexed; the duplicate owners were created by INT-003's additive fix; the Section G AI
context was regenerated on 2026-08-05 and got the answer wrong; the changelog stopped at
the round before last. The repository's authoring discipline is now better than its
remediation discipline.

### Files With Most Conflicts

1. `architecture/ARCHITECTURE_INDEX.md` — INT-201 (three lines), INT-207 (two lines)
2. `L2-C08_Alumni_Transition_Workflows.md` — INT-205, INT-206, INT-212
3. `DEPENDENCY_MAP.md` — INT-202 (as canonical source), INT-204 (as the file in error)
4. `README.md` — INT-201, INT-208 (three lines)
5. `architecture/L5_Solution_Architecture/README.md` — INT-201, and it is the artifact
   most likely to be believed, being co-located with the files it denies

### Merge Decision

# PASS WITH MAJOR CHANGES

This is an upgrade from the standing `REJECT`. The justification for lifting REJECT: the
three conditions that produced it — dependency reversals in A10/D07/F14, contested
ownership of Infrastructure/Scholarship/Alumni, and uncanonical lifecycle and result-event
names — were each re-verified against the working tree during this review and each is
resolved. `L2-A10:13-22` now confines `depends_on` to A0x with B under `consumed_by`;
`DEPENDENCY_MAP.md:328-332` legitimises F14→J as event consumption; the result event is
singular across 32 occurrences.

The justification for withholding a clean PASS: INT-201 and INT-202 are both Critical and
neither is cosmetic. A repository cannot serve as an implementation source of truth while
it contains a foundational dependency cycle and two architecture documents that its own
index says do not exist.

Conditions for PASS:

- INT-201 — index both L5 documents and correct the four denials
- INT-202 — break the D ↔ G cycle and reconcile the seven Section G declarations
- INT-203 — regenerate the Section G dependency contract
- INT-204 — correct the Section I row in `DEPENDENCY_MAP.md`
- INT-205 — remove the legacy `Owner` field from all 44 documents
- INT-206 — resolve or delete the four unpublished event subscriptions in C08

### Priority Remediation List

**1 — Fix before anything else (Critical)**

1. INT-201 — index the two L5 documents; correct `L5_Solution_Architecture/README.md:3-5`,
   `ARCHITECTURE_INDEX.md:290,327,330`, `README.md:57,82`, `REPO_UNDERSTANDING.md:118-119`.
2. INT-202 — reclassify or ratify the seven Section G upstream declarations; the D ↔ G
   cycle must not survive the change.

**2 — Fix before merge (High)**

3. INT-203 — regenerate `AI_CONTEXT/SECTION_G/Dependency_Contract.md`; fix the generator,
   since line 8 claims a full regeneration that missed ten files.
4. INT-204 — amend `DEPENDENCY_MAP.md:116` to include Sections A and B.
5. INT-205 — delete the legacy `Owner` field from 44 documents; re-add genuine approval
   authorities as `Review Authority`; reconcile `L2-C08:6858` with `L2-C08:7`.
6. INT-206 — correct `QualificationCompleted` to `QualificationAwarded` in `L2-C08:2497`
   and `:5783`; resolve or delete the three orphaned subscriptions.

**3 — Fix in the next review cycle (Medium / Low)**

7. INT-207 — reconcile `Volume_2:147` with `ARCHITECTURE_INDEX.md:351`; correct the
   9-vs-13 statement at `ARCHITECTURE_INDEX.md:365`.
8. INT-208 — remove the `tools/` entry and the HTML claim from `README.md`.
9. INT-209 — add the 2026-08-05 and 2026-08-06 changelog blocks.
10. INT-210 — repair the corrupted YAML at `L2-C04:21`.
11. INT-211 — regenerate `AI_CONTEXT/INDEX.md` from the filesystem.
12. INT-212 — retitle or delete `L2-C08` section 26.8.
13. INT-213 — delete the duplicate frontmatter key at `L2-B05:15`.
14. Observation — correct line 440 of the 2026-08-05 addendum.

### Requirement for the next review

The 2026-08-05 line-number requirement is retained and has been met here. Add one
requirement to it:

**A finding shall not be reported as remediated on the basis of the fix that was applied.
It shall be re-verified against the working tree at review time.** Nine of the thirteen
findings above exist because a remediation was performed correctly and its consequences
were never checked: files were created and not indexed, a field was added and its
predecessor not removed, a context file was regenerated and not validated against its
sources. The failure mode has moved from *carrying findings forward* to *closing findings
early*. Both produce a review that describes a repository that does not exist.

Two mechanical checks would have caught six of these findings and should be added:

- every file under `architecture/` appears in `ARCHITECTURE_INDEX.md` (catches INT-201)
- every L2 file declares exactly one `Owner`-class field, matching
  `TEAM_ASSIGNMENTS.md:160-268` (catches INT-205)

---

# REMEDIATION ADDENDUM — 2026-08-06

**All 13 findings and the observation were remediated on the day of the review.**
Authorised by Dhruv (Lead Architect) as a repository-wide governance synchronisation under
`TEAM_ASSIGNMENTS.md:279`. Every item below was re-verified against the working tree after
the fix.

| ID | Severity | Outcome |
|---|---|---|
| INT-201 | Critical | **Fixed.** Both L5 documents indexed; `ARCHITECTURE_INDEX.md` gains an L5 section, statistics → 2 / 115; four denials corrected across the L5 README, `README.md` and `REPO_UNDERSTANDING.md`. |
| INT-202 | Critical | **Fixed by reclassification.** C/D/F moved from `**Upstream Dependencies**` to a new `**Consumed From**` block in G02–G09; 15 rows added to the `DEPENDENCY_MAP.md` §7A register. Section G's foundational set unchanged. **D ↔ G cycle resolved.** |
| INT-203 | High | **Fixed.** False "no dependency tables" claim replaced with the real per-file table and the current-state summary. |
| INT-204 | High | **Fixed.** `DEPENDENCY_MAP.md:116` → "Sections A, B, C, D, F". |
| INT-205 | High | **Fixed.** 54 occurrences across 44 documents: 41 deleted, 13 converted to `Review Authority`. 107/107 retain their canonical capability owner. |
| INT-206 | High | **Fixed, one gap retained.** Three consumed events repointed to real publishers, one deleted as C08's own event. `AlumniTransitionPolicyUpdated` retained and marked — see Open below. |
| INT-207 | Medium | **Fixed without amending Volume 2.** Index legend completed to all 13 values verbatim. Volume 2 remains LOCKED at v1.1. Verified exact row-for-row match. |
| INT-208 | Medium | **Fixed.** `tools/` entry and the generated-HTML claim removed from `README.md`. |
| INT-209 | Medium | **Fixed.** `[1.5.0]`, `[1.5.1]`, `[1.5.2]` blocks reconstructed. |
| INT-210 | Medium | **Fixed.** `L2-C04` YAML repaired; stale B05 title reference corrected. |
| INT-211 | Low | **Fixed.** Deleted `SECTION_H/README.md` delisted. |
| INT-212 | Low | **Fixed.** `L2-C08` §26.8 retitled "Proposed Repository Structure — Not Yet Created". |
| INT-213 | Low | **Fixed.** Duplicate `document-owner` key removed from `L2-B05`. |
| Observation | — | **Fixed.** 2026-08-05 addendum's incorrect dismissal of INT-018 corrected in place. |

## Corrections to this review, found during remediation

Three of this review's own statements were incomplete. Recording them here rather than
editing the findings above, so the review remains an honest record of what it caught.

1. **INT-202 understated the Section F conflict.** The review named only `L2-G09`. In
   fact **six** documents declare Section F upstream — G04, G05, G06, G07 and G08 use the
   label "Finance Domain", which the review's section-name scan did not match, plus G09's
   "Section F — Finance".
2. **INT-202 understated the dead declarations.** Investigation found **four** Section D
   declarations with no supporting content, not two. G04, G05 and G06 were deleted; G09's
   proved substantiated on closer reading, because G09 says "Academic" rather than
   "Academic Delivery" and has an explicit `Consumes:` block.
3. **INT-206 misdiagnosed one row.** `AlumniTransitionApprovalCompleted` was reported as an
   event with no publisher. It is not a consumed event at all — `L2-C08` §10.4 publishes
   `AlumniTransitionApproved` itself, and the row duplicated its own output under an
   invented "Approval Management" domain. A subscriber built from the original table would
   have listened for its own event.

## Open after remediation

Neither item is a contradiction. Both are gaps where the repository is silent.

1. **Alumni Transition policy has no owner.** `L2-C08` consumes
   `AlumniTransitionPolicyUpdated`; nothing defines or updates that policy. C08 publishes
   `TransitionPolicyValidated` — it evaluates policy — but owns no policy aggregate, and
   A04 and A08 declare no policy-change events. Decision 2026-08-06: retain the
   subscription marked "Unresolved — no publisher" so the missing contract stays visible.
   No subscriber shall be implemented until an owner is assigned.
2. **Section G routes to H and J without declaring them.** Found while fixing INT-202.
   Several Section G ownership matrices route Notifications to Section H and
   supplier/procurement operations to Section J, but neither section appears in any G
   dependency block. Same defect class as INT-202, deferred rather than expanding scope
   mid-fix.

## Revised merge decision

# PASS WITH MINOR CHANGES

Both Critical findings are resolved and verified. The conditions this review set for a
PASS — index the L5 documents, break the D ↔ G cycle, regenerate the Section G contract,
correct the Section I row, remove the duplicate owner fields, resolve the C08 event
subscriptions — have all been met.

The decision is not a clean PASS because of the two open gaps above. Neither blocks
implementation: the first is confined to one subscription that carries an explicit
do-not-implement instruction, and the second is undeclared-but-real routing that
misleads no one about ownership. Both should be closed in the next cycle.

**Not re-verified by this addendum:** the free prose of the 11.6 MB L2 corpus, which was
outside this review's extraction scope from the start. See the Method and Scope
Disclosure at the top.
