# SutraERP L3 Phase 1 — Chetan Section C Verification


> **Numbering note (2026-08-17).** Candidate IDs in this historical report reflect the numbering in force when it was written. Repository-wide candidate numbering was normalized on 2026-08-17; for current IDs see `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Phase1_Candidate_ID_Migration_2026-08-17.md`.
**Audit type:** Independent, adversarial, evidence-based, repository-first verification
**Subject artifact:** `architecture/L3_Business_Scenario_Architecture/01_Phase1_Discovery/L3_Phase1_Inventory_Chetan.md`
**Discovery area audited:** Section C — Student Lifecycle (L2-C00 … L2-C08)
**Candidates reviewed:** 45 rows / 36 distinct Candidate IDs (9 rows carry duplicated IDs — verified by extraction)
**Date:** 2026-08-08
**Status:** Phase 1 discovery-quality audit. Not an L3 authoring artifact. Section C is **not** recommended for Phase 2 consolidation in its current state.

**Scope guard:** This audit concerns Phase 1 candidate discovery only. It creates no L3 documents, assigns no final L3 numbering, locks no ownership, and modifies no L1 or L2 content. No API, database, UI, schema, or deployment content appears in any proposed candidate.

---

## 1. Verdict

```text
FAIL — BOTH COVERAGE AND ARCHITECTURE PROBLEMS
```

```text
Existing candidates: 45 rows / 36 distinct Candidate IDs
Credible missing candidates: 27
Candidates needing merge: 2 (2 merge pairs)
Candidates needing split: 1
Candidates needing rewording: 17
Ownership/boundary defects: 13
Potential duplicates: 10 (9 duplicated Candidate IDs + 1 semantic overlap)
Blocked by repository conflict: 0 blocked, 5 conflict-dependent
```

---

## 2. Executive Assessment

The inventory is **not exhaustive**. It is a competent conversion of *some* L2 workflow headings into outcome-shaped rows, but it stops at roughly 60% of what the Section C sources actually support, and the shortfall is not evenly distributed — it is concentrated exactly where discovery required reading past the top-level workflow diagrams.

Evidence-based judgment:

- **C06 and C07 are the worst.** `L2-C06` §7.2 defines a twelve-workflow canonical catalogue (GW-01…GW-12) and §5 enumerates fifteen exclusively-owned business capabilities. Chetan produced **four** rows and omitted Legal Custody, Consent, Communication Authorization, Financial Responsibility, Pickup Authorization, and Guardian Suspension — every one of which `L2-C06` §5 names as a capability **"exclusively owned"** by the domain and which "shall not be duplicated by other domains." `L2-C07` §7 defines ten workflows (DW-01…DW-10 / WS-01…WS-10) and §5.1 gives the domain ownership of *Scholarship Definition, Catalogue, Availability, Eligibility Policy, Recommendation, Renewal, Expiration*. Chetan produced **three** rows covering only assessment, reassessment, and the Finance hand-off. The entire scholarship-definition and eligibility-policy half of the bounded context has zero coverage.
- **Terminal states without candidates.** `L2-C01` §6.8/§6.13/§6.15 define `Expired` as a canonical terminal state with an `AdmissionExpired` event — no candidate. `L2-C05` §9.3/§11.11 define `Administrative Exit` and `Deceased` as canonical alternative exits — no candidates. `L2-C05` §12.4 defines a `QualificationAwardRevoked` event — no candidate. `L2-C02` WF-C02-10 defines `Archived` with a `StudentArchived` event — no candidate. A published canonical event with no discovered scenario is a discovery gap, not a judgment call.
- **C04 was read as one decision.** `L2-C04` §5.2 lists APW-04 Conditional Promotion, APW-05 Academic Probation, APW-06 Academic Detention as three separate workflows, each with its own monitoring cycle and its own terminal outcomes (APW-04 → Progressed *or* Probation; APW-05 → Good Standing *or* Detention; APW-06 → Repeat Semester / Repeat Year / Appeal / Academic Exit). All three collapsed into `CAN-L3-C-017`. `L2-C04` §5.2 state matrix also carries `Detained → Under Evaluation : Re-evaluation Approved` and the rule that "policy-approved revisions shall generate new progression records rather than modifying existing decisions" — no revision/appeal candidate exists.
- **C03 was read as the student's registration only.** `L2-C03` §7.3 defines WF-C03-01 Registration Window Initialization, WF-C03-09 Registration Freeze and WF-C03-10 Registration Publication as owned workflows, and §1.2/§4.4 place *Backlog Registration, Repeat Course Registration, Add/Drop, Late Registration* explicitly in scope. Only the student-facing registration path was inventoried.
- **Structural integrity is broken independently of coverage.** Nine rows carry duplicated Candidate IDs (`C-017`–`C-020` reused between C03 and C04; `C-028`–`C-031` reused between C05 and C06; `C-031` used a third time in C07). The C07 and C08 tables have no section headings. The header omits the mandated `**Prepared By:**` line and the status is `In Progress`, not `Submitted for Review`.
- **What is genuinely good:** the C04↔I boundary is respected (progression *consumes* published results, `C-017`/`C-018` domains `C, I`), Course Offering ownership was not pulled from Section D (`C-018` lists D as participant only), the Academic Credit Ledger is correctly retained by Academic Progression across the ABC hand-off (`C-024`), and eligibility (C04) is correctly separated from completion (C05). These boundaries did not need correcting.

---

## 3. Coverage by L2 Source

### C01 — Admissions
```text
Coverage: PARTIAL
```
**Covered:** the approved-candidate→active-student spine (WF-C01-01…05 correctly composed into one outcome in `C-001`), cancellation (WF-C01-06), withdrawal (WF-C01-07), revocation (WF-C01-08), readmission (WF-C01-09).
**Missing:** the `Expired` terminal path (§6.7, §6.8, §6.13, §6.15 `AdmissionExpired`) — an offer lapse is policy-driven and institutionally distinct from cancellation (a deliberate act) and withdrawal (an applicant act).
**Correctly not separated:** WF-C01-02 Document Verification, WF-C01-03 Approval, WF-C01-04 Financial Clearance (states inside `C-001`); WF-C01-10 Admission Audit (a cross-cutting obligation, §6.16, not an outcome).

### C02 — Student Lifecycle & Activation
```text
Coverage: PARTIAL
```
**Covered:** enrollment update (WF-C02-03), academic assignment (WF-C02-04), suspension/restoration (WF-C02-06/07), transfer (WF-C02-08), profile (WF-C02-12), preferences (WF-C02-14), identity correction (WF-C02-15), duplicate resolution (WF-C02-16), document request (WF-C02-11).
**Missing:** WF-C02-10 **Student Exit / Archive** — the `Archived` terminal state of the Student Context and the `StudentArchived` event have no candidate. The C05 exit candidates end at `Withdrawn`/`Transferred`/`Discontinued`/`Terminated`; archival is the separately governed permanent-closure step ("physical deletion prohibited… archive permanent").
**Correctly not separated:** WF-C02-01/02 (inside `C-001`), WF-C02-05 Status Change (a generic mechanism, not an outcome), WF-C02-09 Qualification Handoff (inside the C05 candidates).
**Conflict:** WF-C02-13 Guardian & Emergency Contact Management overlaps C06 ownership — see §14.

### C03 — Academic Registration
```text
Coverage: INCOMPLETE
```
**Covered:** registration (WF-C03-03/04/05), modification (WF-C03-07), approval/rejection (WF-C03-06), cancellation (WF-C03-08), electives.
**Missing:** WF-C03-01 **Registration Window Initialization** (§21.6: "The Registration Domain shall reject all registration requests outside an active Registration Window" — a hard gate on the P0 outcome); WF-C03-09 **Freeze** + WF-C03-10 **Publication** (§11.3 `Frozen`, the finalization and downstream hand-off); **Backlog / Repeat Course Registration** (§1.2 lines listing Backlog Registration and Repeat Course Registration; §4.3; §8.4 "Register repeat subjects"; configuration for "backlog limits", "repeat semester policies").
**Correctly not separated:** Late Registration (§7.14, BR-C03-009 — an alternate flow with a penalty policy), Administrative Override (BR-C03-010), Registration Hold, Credit Overload Approval (all §7.14 exceptional paths).

### C04 — Student Academic Progression
```text
Coverage: INCOMPLETE
```
**Covered:** progression evaluation + standing (APW-01/02), credit ledger (APW-03), credit transfer (APW-07), programme migration (APW-08), academic break (APW-09), rejoining (APW-10), qualification eligibility (APW-11), ABC sync (APW-12).
**Missing:** APW-04 **Conditional Promotion** condition-monitoring outcome; APW-05 **Academic Probation** review-and-resolution outcome; APW-06 **Academic Detention** and repeat determination (L1 publishes `StudentRepeated`); **progression decision revision after re-evaluation / appeal** (§5.2 APW-01 triggers "Approved Re-evaluation, Appeal Decision, Supplementary Result Publication"; state matrix `Detained → Under Evaluation : Re-evaluation Approved`; §14.9 Appeals & Review Support covering promotion, detention, credit-transfer and migration appeals; "superseded decisions shall remain historically available").

### C05 — Qualification Completion & Student Exit
```text
Coverage: PARTIAL
```
**Covered:** qualification completion (§11.1–11.9), MEME, withdrawal, transfer exit, discontinuation, termination, alumni hand-off.
**Missing:** **Institutional Exit Clearance** (§3 Institutional Clearance, §10.5, §11.5; published as `InstitutionalClearanceCompleted` and consumed by C08 §3); **Administrative Exit** (§9.3, §11.11); **Deceased student closure** (§9.3, §11.11); **post-conferment qualification correction/revocation** (§10.7 "Revocation or correction of a conferred degree shall require institutionally governed exceptional procedures"; §12.4 `QualificationAwardRevoked`); **qualification deferral after failed degree audit/clearance/approval rejection** (§9.4 `Audit Failed`, `Clearance Failed`, `Approval Rejected`; §11.12).

### C06 — Guardian Workflows
```text
Coverage: INCOMPLETE
```
**Covered:** GW-01/03 establishment, GW-04 modification, GW-12 termination, GW-08 emergency contact.
**Missing:** GW-05 Legal Custody, GW-06 Consent, GW-07 Communication Authorization, GW-09 Financial Responsibility, GW-10 Pickup Authorization, GW-11 Guardian Suspension — six of twelve catalogued workflows, each mapping to a capability §5 names as exclusively owned.
**Correctly not separated:** GW-02 Verification (a gated state inside establishment — but `C-028`'s outcome wording must be corrected to reach `Active`, see §8).

### C07 — Scholarship Eligibility
```text
Coverage: INCOMPLETE
```
**Covered:** DW-03 assessment, DW-06 reassessment, Finance hand-off.
**Missing:** DW-01 Scholarship Definition & Publication; DW-02 Eligibility Policy Management; DW-05 Conditional Eligibility Processing; DW-07 Renewal Evaluation; DW-08 Expiration & Archival; DW-09 External Synchronization; DW-10 Bulk Recalculation. DW-04 Recommendation Generation is absent but is best fixed by rewording `C-031` (see §8).
**Boundary check passed:** no Finance behaviour was duplicated — `C-033` correctly hands off without transferring eligibility ownership.

### C08 — Alumni Transition
```text
Coverage: COMPLETE (with one overlap defect)
```
AT-WF-01…06 are correctly composed into one outcome (`C-034`); AT-WF-07 correction and AT-WF-08 archival are separately and correctly captured. The defect is not coverage but duplication with `C-031` — see §9.

### C00
Correctly excluded. `L2-C00` §1 states it "does not define student lifecycle business workflows"; it is a control/review document and generates no candidates.

---

## 4. Missing L3 Candidates — MUST ADD

Use `NEXT-CAN-L3-C-nn` as placeholders; final numbering is to be assigned after the ID collisions in §8 are resolved.

| Proposed Temporary ID | Scenario Title | Trigger | Final Outcome | Likely Primary Owner | Domains | Type | Priority | Source Evidence | Why Independent |
|---|---|---|---|---|---|---|---|---|---|
| NEXT-CAN-L3-C-01 | Admission Offer Expiry | Configured admission validity period elapses without the required applicant or institutional actions | Admission record reaches the Expired terminal state without student creation, with history preserved | Admissions | C | Simple | P1 | `L2-C01` §6.7 Alternate Lifecycle Paths; §6.8 "Expired"; §6.13 Terminal States; §6.15 `AdmissionExpired` | Canonical terminal state reached by policy elapse, not by an act of cancellation or withdrawal; different actor (none), different evidence, different reopen rules |
| NEXT-CAN-L3-C-02 | Student Context Archival and Permanent Record Closure | Student exit is complete and all institutional obligations are resolved | Student Context reaches Archived state; record retained permanently and immutably for historical reference | Student Lifecycle | C | Critical | P1 | `L2-C02` WF-C02-10 Student Exit / Archive; §3 SP-010; §6.6 invariants; `StudentArchived` event | Archival is a separately governed terminal transition after any exit reason; the C05 exit candidates terminate at exit states, not at archival |
| NEXT-CAN-L3-C-03 | Academic Registration Window Opening and Closure | Institution plans a registration cycle for an academic period | Authorized registration window is configured, approved, published and activated, enabling registration; closure concludes the cycle | Academic Registration | B, C, D | Cross-domain | P0 | `L2-C03` §7.3 WF-C03-01; §8.2 WS-C03-01; §21.6; BR-C03-009/010; §15.4 Registration Window Configuration | Hard precondition for every registration: "The Registration Domain shall reject all registration requests outside an active Registration Window." Owned by C03, distinct actors (registrar/academic administration), distinct approval chain |
| NEXT-CAN-L3-C-04 | Academic Registration Finalization and Publication | Approved registrations reach the configured finalization point for the academic period | Registration is frozen and published as the authoritative registration record for downstream academic, attendance, examination and finance processing | Academic Registration | C, D, F, I | Cross-domain | P0 | `L2-C03` §7.3 WF-C03-09, WF-C03-10; §8.10, §8.11; §11.3 `Frozen`; §21.14, §21.15; §14.4 Downstream Consumers | The point at which registration becomes an immutable institutional fact consumed by four other domains; distinct from an individual student's approval |
| NEXT-CAN-L3-C-05 | Backlog and Repeat Course Registration | Official results establish that a student must re-register for backlog or failed courses in a permitted period | Backlog/repeat registration is completed within institutional limits and reflected in the registration record | Academic Registration | C, D, I | Cross-domain | P1 | `L2-C03` §1.2 (Backlog Registration, Repeat Course Registration); §4.3; §4.4; §8.4 "Register repeat subjects"; §15 "backlog limits", "repeat registration policies" | Different trigger (published results, not a window opening), different eligibility ruleset, different credit treatment from initial registration |
| NEXT-CAN-L3-C-06 | Conditional Promotion Condition Fulfilment | Student is conditionally promoted with defined academic conditions attached | Conditions are satisfied and progression is confirmed, or conditions fail and the student moves to academic probation | Academic Progression | C, I | Critical | P1 | `L2-C04` §5.2 APW-04 (Conditions Identified → Approval → Condition Monitoring → Satisfied? Yes/No); §6.6 Conditional Promotion Rules | The conditional promotion *decision* is inside `C-017`; the monitored fulfilment outcome resolves months later with two distinct institutional terminals |
| NEXT-CAN-L3-C-07 | Academic Probation Management and Resolution | Student is placed on academic probation following conditional-promotion failure or progression evaluation | Student returns to good standing, continues probation, or escalates to academic detention, with standing history preserved | Academic Progression | C | Critical | P1 | `L2-C04` §5.2 APW-05 (Create Probation Record, Assign Conditions, Periodic Review, Exit or Escalation; Possible Outcomes); §6.7 Academic Probation Rules; APW-02 standing type "Probation" | Distinct supervised regime with its own review cycle, own record, and three defined terminal outcomes |
| NEXT-CAN-L3-C-08 | Academic Detention and Repeat Determination | Institutional progression requirements remain unsatisfied after progression evaluation | Student is detained, progression is blocked, and the repeat semester or repeat academic year outcome is authoritatively determined | Academic Progression | C, I | Critical | P1 | `L2-C04` §5.2 APW-06 (Reasons; Outcomes: Repeat Semester, Repeat Academic Year, Appeal, Re-evaluation, Academic Exit); §6.8 Academic Detention Rules; L1 §20 publishes `StudentRepeated` | Detention blocks the lifecycle and produces a repeat plan — an institutional outcome with downstream registration consequences, not a value of the progression decision |
| NEXT-CAN-L3-C-09 | Progression Decision Revision After Re-evaluation or Appeal | Approved re-evaluation, appeal decision, supplementary result, or governed marks correction becomes available | A new authoritative progression record supersedes the earlier decision; the superseded decision remains historically available | Academic Progression | C, I | Critical | P1 | `L2-C04` §5.2 APW-01 triggers; §6 "Re-evaluation or supplementary examination outcomes shall trigger a fresh progression evaluation"; "Policy-approved revisions shall generate new progression records rather than modifying existing decisions"; state matrix `Detained → Under Evaluation : Re-evaluation Approved`; §14.9 Appeals & Review Support; §9 exception matrix | Historically sensitive correction of a published academic decision, with distinct authorization and distinct audit obligations |
| NEXT-CAN-L3-C-10 | Institutional Exit Clearance Completion | Student enters qualification completion or an alternative exit pathway | All mandatory institutional clearances are completed and the authoritative clearance outcome is published, unblocking exit approval | Qualification Completion & Student Exit | C, F, G, H | Cross-domain | P1 | `L2-C05` §3 Institutional Clearance; §9.2 "Institutional Clearance" state; §10.5 Institutional Clearance Rules; §11.5 Institutional Clearance Workflow; `InstitutionalClearanceCompleted` consumed by `L2-C08` §3 | Genuine multi-domain outcome (finance, library, hostel, laboratory, department, scholarship, asset return) that gates *both* qualification and every alternative exit; publishes its own canonical event |
| NEXT-CAN-L3-C-11 | Administrative Exit Closure | Institution administratively closes a student's academic relationship under governed authority | Student Context reaches the administrative exit terminal state without degree conferment, with academic history preserved | Qualification Completion & Student Exit | C | Critical | P1 | `L2-C05` §3 "Administrative exit"; §9.3 Administrative Exit; §9.6 lifecycle diagram; §11.11 Alternative Exit Workflow; "Administrative exits permanently conclude the student's institutional relationship without degree conferment" | A canonical alternative exit pathway distinct from withdrawal (student act), discontinuation (cessation) and termination (disciplinary/regulatory) |
| NEXT-CAN-L3-C-12 | Deceased Student Academic Record Closure | Institution receives and verifies notification of a student's death | Academic record is closed and exit completed under the governed compassionate-closure pathway, with full history preserved | Qualification Completion & Student Exit | C | Critical | P2 | `L2-C05` §3 "Deceased student closure"; §9.3 Deceased; §9.6 lifecycle diagram `ProgrammeCompleted → Deceased → StudentExitCompleted`; §11.11 | Named canonical exit pathway with distinct evidence requirements and distinct downstream suppression obligations; rare but institutionally mandated |
| NEXT-CAN-L3-C-13 | Conferred Qualification Correction or Revocation | Authorized institutional determination that a conferred qualification must be corrected or revoked | Qualification award is corrected or revoked through the governed exceptional procedure; prior award history remains immutable | Qualification Completion & Student Exit | C, H, I | Critical | P2 | `L2-C05` §10.7 "Revocation or correction of a conferred degree shall require institutionally governed exceptional procedures"; §9.5 post-completion amendment constraint; §12.4 Exception Events `QualificationAwardRevoked` | A published canonical event with no covering scenario; highest-risk historical correction in Section C |
| NEXT-CAN-L3-C-14 | Qualification Completion Deferral After Failed Audit or Clearance | Degree audit fails, mandatory clearance remains outstanding, or qualification approval is rejected | Qualification completion is deferred for the cycle, deficiencies are authoritatively recorded, and the student is returned for remediation | Qualification Completion & Student Exit | C, F, G | Critical | P1 | `L2-C05` §9.4 transitions (`Audit Failed`, `Clearance Failed`, `Qualification Completion Rejected → Degree Audit`); §11.12 Exception Handling (failed degree audit, outstanding clearances, approval rejection, deferrals); §10.3 Degree Audit Rules | Produces a real institutional result — a deficiency determination the student and departments must act on. *If preferred, may instead be absorbed as the documented alternate flow of `CAN-L3-C-025`; both options flagged.* |
| NEXT-CAN-L3-C-15 | Guardian Legal Custody Designation and Change | Legal custody arrangement for a student must be established or changed on verified legal evidence | Authoritative custody designation is recorded and governs guardian authority, with prior custody history preserved | Guardian | A, C | Critical | P1 | `L2-C06` §7.2 GW-05 Legal Custody Management; §8.6; §5 owned capability "Legal Guardian Designation"; §6.3 "Support for legal custody changes" | Legally binding determination that changes who may act for the student; distinct evidence and authorization from a routine relationship modification |
| NEXT-CAN-L3-C-16 | Guardian Consent Authorization and Withdrawal | Institution requires guardian consent for a governed institutional action, or consent is withdrawn | Authoritative consent record is established or withdrawn, governing which institutional actions are permitted | Guardian | A, C | Critical | P1 | `L2-C06` §7.2 GW-06 Consent Management; §8.7; §9.4 Consent Rules; §5 owned capability "Consent Management" | Compliance-bearing authorization outcome consumed by other domains; independent of relationship establishment |
| NEXT-CAN-L3-C-17 | Guardian Communication Authorization Governance | Institution or guardian requests a change to which guardians may receive or act on institutional communication | Authoritative communication authority for the guardian relationship is established or changed | Guardian | A, C | Simple | P1 | `L2-C06` §7.2 GW-07; §8.8; §9.5 Communication Rules; §5 owned capability "Communication Authorization" | Distinct from `C-012` (student-owned channel preferences); this governs a guardian's *authority*, not a student's preference |
| NEXT-CAN-L3-C-18 | Guardian Financial Responsibility Assignment | A guardian must be made institutionally accountable for a student's financial obligations | Authoritative financial responsibility assignment is established and made available to Finance without transferring Finance ownership | Guardian | A, C, F | Cross-domain | P1 | `L2-C06` §7.2 GW-09; §8.10; §9.7 Financial Responsibility Rules; §5 owned capability "Financial Responsibility Assignment"; §5 places Fee Collection/Payment with Finance | C06 owns the *assignment*, Finance owns the *transactions* — a clean cross-domain outcome absent from the inventory |
| NEXT-CAN-L3-C-19 | Student Pickup Authorization Management | Institution must establish or change who is authorized to collect a student | Authoritative pickup authority is recorded and available to campus operations | Guardian | C, G | Simple | P2 | `L2-C06` §7.2 GW-10; §8.11; §5 owned capability "Pickup Authorization" | Safety/custody control with its own authorization list and its own consumer; not a step of relationship establishment |
| NEXT-CAN-L3-C-20 | Guardian Relationship Suspension and Reinstatement | Guardian relationship must be temporarily deactivated, or a suspended relationship is authorized for reinstatement | Relationship reaches Suspended state and later returns to Active, with all authority suspended and restored accordingly, history intact | Guardian | A, C | Critical | P1 | `L2-C06` §6.3 lifecycle states `Suspended`, transitions `Active → Suspended`, `Suspended → Active`, `Suspended → Terminated`; §7.2 GW-11; §8.12 | A canonical reversible lifecycle state absent from the inventory, which jumps from modification straight to closure |
| NEXT-CAN-L3-C-21 | Scholarship Definition Publication and Availability | Institution establishes a scholarship to be made available for eligibility evaluation | Scholarship definition is published and active, becoming available for student eligibility assessment within the institution | Scholarship Eligibility | B, C | Simple | P1 | `L2-C07` §7 DW-01; §8 WS-01; §5.1 "Scholarship Definition & Availability" (registration, catalogue, publication, retirement, provider association); §6.6 Scholarship Definition Lifecycle Draft→Published→Active→Suspended→Expired→Archived | The domain owns an entire aggregate lifecycle with zero candidate coverage; no student eligibility can exist without it |
| NEXT-CAN-L3-C-22 | Scholarship Eligibility Policy Lifecycle Governance | Institution creates, versions, activates, suspends or retires a scholarship eligibility policy | Approved eligibility policy version becomes effective and available for assessment, with historical versions permanently replayable | Scholarship Eligibility | C | Critical | P1 | `L2-C07` §7 DW-02; §8 WS-02; §6.6 Eligibility Policy Lifecycle Draft→Active→Retired→Archived; §6.5 invariant requiring permanent immutable reference to the exact policy version used | Governs the reproducibility and auditability of every eligibility decision; a distinct actor and approval path from assessment |
| NEXT-CAN-L3-C-23 | Conditional Scholarship Eligibility Resolution or Expiry | Eligibility assessment cannot reach a final decision due to missing documents or unverified conditions | Conditions are satisfied yielding an Eligible outcome, or the deadline elapses yielding a final Not Eligible or Expired outcome | Scholarship Eligibility | C | Critical | P1 | `L2-C07` §7 DW-05; §8 WS-05; §6.7 lifecycle `Conditionally Eligible → Re-evaluate → Eligible`; DW-05 deadline transition rule | A held decision with its own deadline clock and its own two terminals; not an alternate flow of a completed assessment |
| NEXT-CAN-L3-C-24 | Scholarship Renewal Evaluation | Renewable scholarship reaches its renewal cycle or academic period boundary | Renewal is approved, conditionally approved or rejected, preserving prior renewal records as immutable versions | Scholarship Eligibility | C | Critical | P1 | `L2-C07` §7 DW-07; §8 WS-07; §5.1 "Eligibility renewal"; §6.5 "Renewals shall create new historical versions" | Different trigger (cycle boundary), different rule set and different history semantics from `C-032` reassessment (which is change-driven) |
| NEXT-CAN-L3-C-25 | Scholarship Eligibility Expiration and Archival | Scholarship validity period elapses or institutional retention policy is reached | Eligibility record is expired and archived without loss of historical information and remains available for audit | Scholarship Eligibility | C | Simple | P2 | `L2-C07` §7 DW-08; §8 WS-08; §6.7 lifecycle `Expired`, `Archived`; §5.1 "Eligibility validity management", "Eligibility expiration" | Named terminal lifecycle outcome with retention obligations; no candidate reaches it |
| NEXT-CAN-L3-C-26 | Bulk Scholarship Eligibility Recalculation | Eligibility policy revision, scholarship rule change or authorized administrative request requires cohort-wide re-evaluation | Updated eligibility assessments and recommendations are published for the affected cohort while all historical versions are preserved | Scholarship Eligibility | C | Critical | P1 | `L2-C07` §7 DW-10; §8 WS-10; §13.5 Bulk Recalculation Decision Matrix; §10.6 Bulk Processing Events | Cohort-scale institutional outcome with its own authorization, its own decision matrix and its own event family; not a repetition of `C-032` |
| NEXT-CAN-L3-C-27 | External Scholarship Ecosystem Synchronization | Institutional eligibility information must be synchronized with a supported external scholarship provider | Institutional eligibility references and externally published outcomes remain synchronized, with external processing ownership unchanged | Scholarship Eligibility | C | Cross-domain | P2 | `L2-C07` §7 DW-09 *(Optional)*; §8 WS-09; §10.7 External Synchronization Events; §13.6 External Synchronization Decision Matrix; §5.2 external providers own applications | Marked optional in source but has its own events and decision matrix; parallels the accepted ABC-sync candidate `C-024` |

**Labelling per No-Hallucination Rule:** all twenty-seven are *Repository Fact* — each maps to a named L2 workflow ID, canonical lifecycle state, business-rule section, or canonical event. Only `NEXT-CAN-L3-C-14` involves *Logical L3 Composition* (composing §9.4 failure transitions and §11.12 exception handling into one deferral outcome) and is flagged as such above.

---

## 5. Candidates That Should NOT Be Added Separately

Deliberately rejected, with reason:

| Rejected possibility | Source | Reason |
|---|---|---|
| Document Verification, Admission Approval, Financial Clearance | `L2-C01` WF-C01-02/03/04 | Sequential states inside the admission→student outcome (`C-001`); no independent institutional result |
| Admission Audit | `L2-C01` WF-C01-10 | Cross-cutting audit obligation (§6.16), not a business outcome |
| Student Creation, Student Activation | `L2-C02` WF-C02-01/02 | Contained in `C-001`; splitting would fragment the canonical entry point |
| Student Status Change | `L2-C02` WF-C02-05 | Generic mechanism; the concrete outcomes (suspension, restoration, transfer, exit) are separately inventoried |
| Qualification Completion Handoff | `L2-C02` WF-C02-09 | Consumes `QualificationAwarded`; publishes no competing event (stated in the source); contained in `C-025` |
| Late Registration, Administrative Override Registration, Registration Hold, Credit Overload Approval | `L2-C03` §7.14, BR-C03-009/010 | Alternate flows and policy exceptions of the registration outcome |
| Registration Archival | `L2-C03` §11.3 `Archived` | Retention step following finalization; absorbed by `NEXT-CAN-L3-C-04` |
| Student Eligibility Validation, Registration Validation | `L2-C03` WF-C03-02/05 | Validation steps inside registration |
| Academic Standing Management | `L2-C04` APW-02 | Already stated as part of the outcome of `C-017` |
| Manual Credit Correction Request | `L2-C04` §9 exception list | The source routes marks/result corrections through re-evaluation — absorbed by `NEXT-CAN-L3-C-09` |
| Programme Completion Verification, Degree Audit, Qualification Approval, Degree Conferment, Academic Record Finalization | `L2-C05` §11.2/11.3/11.4/11.6/11.7/11.9 | States within qualification completion; see §7 for the one defensible split |
| Alumni Transition Ineligibility / Rejection / Awaiting Prerequisites | `L2-C08` AT-WF-02, AT-WF-04 | Alternative flows of `C-034`; the aggregate has one lifecycle and a rejection does not produce a separate institutional record |
| AT-WF-01…06 individually | `L2-C08` §7.3 | Sequential stages of one transition; correctly composed already |
| Guardian Verification | `L2-C06` GW-02 | Gated state inside establishment — fix by rewording `C-028`'s outcome, not by adding a row |
| Scholarship Recommendation Generation | `L2-C07` DW-04 | Invariant: "Recommendations shall never exist without a completed eligibility decision" — fix by extending `C-031`'s outcome |
| Every `Send Notification` / `Publish Event` / `Generate Audit Record` step across C01–C08 | all sources | Workflow steps; explicitly disallowed by Discovery Rules §8 |

---

## 6. Existing Candidates Requiring Merge

| Existing Candidate(s) | Recommended Consolidated Scenario | Reason |
|---|---|---|
| `CAN-L3-C-031` (C05 — Qualification Completion to Alumni Transition) + `CAN-L3-C-034` (C08 — Completed Student to Alumni Transition) | **Completed or Eligible Exit Student to Alumni Transition** — Primary Owner: Alumni Transition; Domains: C, H; Cross-domain; P1 | Same final outcome. `L2-C05` §11.10 and `L2-C08` AT-WF-01…06 describe one transition; C05's role is publishing `QualificationAwarded`, which is a hand-off step, not a second outcome. Keeping both creates two owners for one alumni transition record |
| `CAN-L3-C-012` (Communication Preference Update) into `CAN-L3-C-011` (Student Profile Update) | **Student-Owned Profile and Preference Update** — Student Lifecycle; C; Simple; P1 | `C-012` alone is a thin CRUD outcome (Discovery Rules §8 pattern "Update Student Status"). *Caveat:* `L2-C02` keeps WF-C02-12 and WF-C02-14 as separate workflows, so retaining both is a legitimate reviewer decision — flagged, not forced |

---

## 7. Existing Candidates Requiring Split

| Existing Candidate | Recommended Split | Reason |
|---|---|---|
| `CAN-L3-C-025` — Qualification Completion and Student Exit | (a) **Qualification Completion and Degree Conferment** — outcome: institution formally approves and confers the academic qualification; publishes `QualificationAwarded`. (b) **Academic Record Finalization and Student Exit Completion** — outcome: academic record permanently finalized and the institutional relationship concluded | `L2-C05` §9.4 defines these as separate governed transitions with separate terminal semantics (`Qualification Completion Approved → Degree Conferred` vs `Academic Record Finalized → Student Exit Completed`). The conferment outcome is independently consumed by C08 and the certificate/transcript domains; record finalization carries its own immutability rule. Both survive the Independence Test on their own |

---

## 8. Existing Candidates Requiring Rewording / Boundary Correction

| Candidate | Problem | Recommended Correction |
|---|---|---|
| `CAN-L3-C-001` | Domains `A, C, F` omit Section B, though admission requires programme reference validation (`L2-C01` §6.11) | Domains → `A, B, C, F` |
| `CAN-L3-C-006` | Domains `C` only; `L2-C02` §7.7 lists Academic Delivery participation for Enrollment Update | Domains → `C, D`; Type → `Cross-domain` |
| `CAN-L3-C-007` | Domains `C` only; §7.7 lists Academic Delivery for Academic Assignment | Domains → `C, D`; Type → `Cross-domain` |
| `CAN-L3-C-008` | Domains `C` only; §7.7 lists Finance/Discipline participation in Suspension | Domains → `C, F` |
| `CAN-L3-C-009` | Domains `C` only; §7.7 lists Finance/Administration in Restoration | Domains → `C, F` |
| `CAN-L3-C-010` | Title "Student Transfer" is ambiguous against `C-028` "Student Transfer Exit"; domains omit Section B (§7.7 Institution & Structure) | Retitle → **Intra-Institution Student Transfer**; Domains → `B, C` |
| `CAN-L3-C-015` | Domains `C` only; `L2-C02` WF-C02-11 states the Document Services domain exclusively owns generation, rendering, issuance and verification | Add the Document Services domain to Domains; keep Primary Owner as Student Lifecycle (request initiation only) |
| `CAN-L3-C-016` | Type `Cross-domain`; registration determines attendance, examination and fee eligibility — Type rule §7 says high institutional risk takes precedence | Type → `Critical` |
| `CAN-L3-C-018` | Title omits Choice-Based Credit Registration, which L1 §4 locks to Student Lifecycle ("CBCR belongs to Student Lifecycle") and L1 §17 lists under the NEP extension | Retitle → **Elective and Choice-Based Credit Registration** |
| `CAN-L3-C-020` (C03) | Title asserts a registration "Withdrawal" outcome; `L2-C03` §11.3 defines no Withdrawn registration state — only `Cancelled` | Retitle → **Academic Registration Cancellation**; Type → `Critical` (post-freeze cancellation is historically sensitive) |
| `CAN-L3-C-024` (C04) | Domains `C` only; APW-12 begins with "Validate Student Identity" and L1 §4 locks APAAR to Identity (Section A); ABC is an external integration | Domains → `A, C` and note the external integration |
| `CAN-L3-C-025` (C05) | Domains `C, I` omit the clearance participants (Finance, campus operations, institutional services) | Domains → `C, F, G, H, I` (and apply the split in §7) |
| `CAN-L3-C-027` / `C-028` / `C-029` / `C-030` (C05) | All show Domains `C` only, yet `L2-C05` §3 makes institutional clearance mandatory for every exit pathway | Domains → `C, F, G` for each |
| `CAN-L3-C-028` (C06 — Guardian Relationship Establishment) | Final outcome stops at "relationship is established", leaving GW-02 verification unaccounted for | Restate outcome → "Guardian relationship is verified and reaches Active state under institutional policy" |
| `CAN-L3-C-031` (C07 — Scholarship Eligibility Determination) | Outcome stops at the eligibility decision; DW-04 recommendation generation is then orphaned | Restate outcome → "Authoritative eligibility decision is established and eligible scholarship recommendations are generated for downstream consumption" |
| `CAN-L3-C-026` | Priority `P0` for MEME intermediate exit — a NEP policy variant, not first-release baseline | Priority → `P1` (low severity) |
| `CAN-L3-C-034` | Priority `P0` for alumni transition — not required for the first-release institutional baseline | Priority → `P1` (low severity; superseded if the §6 merge is applied) |

---

## 9. Duplicate / Overlap Findings

| Candidate A | Candidate B | Relationship | Recommendation |
|---|---|---|---|
| `CAN-L3-C-031` (C05 Alumni hand-off) | `CAN-L3-C-034` (C08 Alumni transition) | OVERLAP | Merge per §6 |
| `CAN-L3-C-017`–`C-020` (C03) | `CAN-L3-C-017`–`C-020` (C04) | DUPLICATE — identifier collision, distinct scenarios | Renumber the C04 block to a free range; content is fine |
| `CAN-L3-C-028`–`C-031` (C05) | `CAN-L3-C-028`–`C-031` (C06) | DUPLICATE — identifier collision, distinct scenarios | Renumber the C06 block; content is fine |
| `CAN-L3-C-031` (C06 Emergency Contact) | `CAN-L3-C-031` (C07 Scholarship Eligibility) | DUPLICATE — third reuse of the same ID | Renumber; content is fine |
| `CAN-L3-C-010` (Student Transfer, C02) | `CAN-L3-C-028` (Student Transfer Exit, C05) | RELATED-BUT-INDEPENDENT | Keep both; retitle `C-010` per §8 to remove ambiguity |
| `CAN-L3-C-008` (Student Suspension, C02) | `CAN-L3-C-021` (Academic Break, C04) | RELATED-BUT-INDEPENDENT | Keep both — different aggregates (Student Context status vs progression lifecycle), different authorization |
| `CAN-L3-C-023` (Qualification Eligibility, C04) | `CAN-L3-C-025` (Qualification Completion, C05) | RELATED-BUT-INDEPENDENT | Keep both — the APW-11 → C05 hand-off boundary is correctly drawn |
| `CAN-L3-C-032` (Eligibility Reassessment) | `NEXT-CAN-L3-C-24` (Renewal Evaluation) | RELATED-BUT-INDEPENDENT | Add the renewal candidate; DW-06 and DW-07 are separate workflows with different triggers |
| `CAN-L3-C-012` (Student Communication Preference) | `NEXT-CAN-L3-C-17` (Guardian Communication Authorization) | RELATED-BUT-INDEPENDENT | Keep distinct — preference vs authority, different owners |

---

## 10. Ownership & Architectural Boundary Findings

| Candidate | Claimed Owner | Repository Owner | Finding | Severity |
|---|---|---|---|---|
| `CAN-L3-C-001` | Student Lifecycle | Conflicted: `L2-C01` §6.14 assigns Student Creation to the Admission Domain; `L2-C02` §5 declares Student Lifecycle the exclusive owner of the Student Aggregate | Owner choice is defensible under the Primary Owner Rule (final outcome = Active Student Context) but rests on an unresolved conflict — see §14 | Medium |
| `CAN-L3-C-028`–`C-031` (C06) | Guardian | `L2-C06` §5 ("sole authoritative bounded context", "shall not be duplicated by other domains") vs `L2-C02` WF-C02-13 claiming guardian registration, updates, replacement, emergency contacts, primary designation | Owner choice is correct but duplicated canonical ownership exists in the repository — see §14 | High |
| `CAN-L3-C-031` (C05) / `C-034` (C08) | Qualification Completion / Alumni Transition | `L2-C08` owns the transition; `L2-H06` owns alumni engagement only | Two candidates claim one outcome; `C-031`'s "without transferring ownership" wording is correct and should survive the merge | Medium |
| `CAN-L3-C-015` | Student Lifecycle | Correct for request initiation; Document Services owns generation/issuance/verification | Owner correct, Domains incomplete | Low |
| `CAN-L3-C-017`/`C-018` (C04) | Academic Progression | L1 §20: "Progression only consumes published official results"; "Student Lifecycle never publishes examination results" | **Correct.** No Section I ownership was absorbed | — |
| `CAN-L3-C-018` (C03 Elective) | Academic Registration | Section D retains Course Offering | **Correct.** D listed as participant only | — |
| `CAN-L3-C-024` | Academic Progression | L1 §4: Academic Credit Ledger canonical to Student Lifecycle; ABC is an external integration; APAAR belongs to Identity | Owner correct; Domains omit Section A | Low |
| `CAN-L3-C-033` | Scholarship Eligibility | `L2-C07` §5.2: Finance owns scholarship financial processing, allocation, payment | **Correct.** No Finance behaviour duplicated | — |
| `CAN-L3-C-013`/`C-014` | Student Lifecycle | Person identity remains with `L2-A02` | **Correct.** Domains `A, C` accurate | — |

No candidate silently redefines another domain. The ownership problems are (a) incomplete Domains fields and (b) two pre-existing repository ownership collisions the inventory inherits rather than creates.

---

## 11. Cross-Domain Scenarios Potentially Missing

| Scenario | Why Section C discovery should surface it | Likely final-outcome owner | Participating domains | Phase 1 placement |
|---|---|---|---|---|
| **Institutional Exit Clearance Completion** (`NEXT-C-10`) | `L2-C05` §3 makes clearance mandatory for every exit; C05 aggregates it and publishes `InstitutionalClearanceCompleted` | Qualification Completion & Student Exit (Section C) | C, F, G, H | Stays in the C inventory |
| **Academic Registration Finalization and Publication** (`NEXT-C-04`) | `L2-C03` §14.4 names Academic Delivery, Attendance, Examination and Finance as downstream consumers of the frozen registration | Academic Registration (Section C) | C, D, F, I | Stays in the C inventory |
| **Backlog and Repeat Course Registration** (`NEXT-C-05`) | Triggered by Section I published results, executed by Section C, consuming Section D offerings | Academic Registration (Section C) | C, D, I | Stays in the C inventory |
| **Guardian Financial Responsibility Assignment** (`NEXT-C-18`) | C06 owns the assignment; Finance consumes it to bill the responsible party | Guardian (Section C) | A, C, F | Stays in the C inventory; verify against Varad's Section F inventory in Phase 2 |
| **Student Pickup Authorization** (`NEXT-C-19`) | C06 owns the authority; campus operations enforces it | Guardian (Section C) | C, G | Stays in the C inventory; verify against Varad's Section G inventory |
| **Progression Decision Revision After Appeal** (`NEXT-C-09`) | Triggered by Section I re-evaluation/supplementary results; the revised progression record is a Section C outcome | Academic Progression (Section C) | C, I | Stays in the C inventory; verify against Devang's Section I inventory for a competing "Revaluation and Revised Result" candidate |
| **Conferred Qualification Correction or Revocation** (`NEXT-C-13`) | Downstream transcript, certificate and alumni records must be invalidated | Qualification Completion & Student Exit (Section C) | C, H, I | Stays in the C inventory |

Per Team Instructions §5, discovery area does not determine final ownership — all of the above are provisionally Section C-owned and confirmable in Phase 2.

---

## 12. Lifecycle Gap Analysis

```text
Admission                    COVERED — gap: Expired terminal path
        ↓
Student Creation             COVERED (inside C-001)
        ↓
Activation                   COVERED (inside C-001)
        ↓
Academic Registration        PARTIAL — gaps: window opening/closure, freeze & publication,
                                       backlog/repeat registration
        ↓
Active Study                 PARTIAL — gap: guardian custody/consent/authority regime
                                       (custody, consent, communication, financial, pickup,
                                        guardian suspension); scholarship definition,
                                        eligibility policy, conditional eligibility,
                                        renewal, expiry, bulk recalculation
        ↓
Progression                  PARTIAL — gaps: conditional-promotion fulfilment, probation,
                                       detention & repeat, decision revision after appeal
        ↓
Exceptions / Break /         COVERED (C-021, C-022)
Rejoining
        ↓
Qualification / Exit         PARTIAL — gaps: institutional clearance, administrative exit,
                                       deceased closure, post-conferment revocation,
                                       deferral after failed audit
        ↓
                             GAP — Student Context Archival (WF-C02-10) has no candidate
        ↓
Alumni Transition            COVERED — but duplicated across C-031 and C-034
```

Uncovered terminal outcomes: `Expired` (admission), `Archived` (Student Context), `Administrative Exit`, `Deceased`, `QualificationAwardRevoked`, `Expired`/`Archived` (scholarship eligibility), `Suspended` (guardian relationship), `Frozen` (registration).

---

## 13. Scenario Family Completeness Matrix

| Scenario Family | Expected from Repository | Present | Missing | Status |
|---|---:|---:|---:|---|
| Admissions | 6 | 5 | 1 | PARTIAL |
| Student Activation & Lifecycle | 11 | 10 | 1 | PARTIAL |
| Academic Registration | 8 | 5 | 3 | INCOMPLETE |
| Academic Progression | 12 | 8 | 4 | INCOMPLETE |
| Qualification & Exit | 13 | 8 | 5 | PARTIAL |
| Guardian | 10 | 4 | 6 | INCOMPLETE |
| Scholarship Eligibility | 10 | 3 | 7 | INCOMPLETE |
| Alumni Transition | 3 | 3 | 0 | COMPLETE |
| **Total** | **73** | **46** | **27** | **INCOMPLETE** |

*"Expected" counts reflect independent outcome analysis, not workflow-heading counts: 73 expected outcomes derive from 88 catalogued L2 workflows plus canonical states and events, after composing steps into outcomes and rejecting the 30 items listed in §5. "Present" counts 46 = 45 existing rows + 1 for the recommended split of `C-025`; treating the `C-031`/`C-034` overlap as one outcome nets back to 45 distinct outcomes covered.*

---

## 14. Repository Conflicts Affecting Verification

| Conflict | Sources | Impact on L3 Discovery | Recommendation |
|---|---|---|---|
| **Student Creation has two canonical owners** | `L2-C01` §6.14 Lifecycle Ownership ("Student Creation → Admission Domain") and WF-C01-05 vs `L2-C02` §5 ("exclusive owner of the Student Aggregate") and WF-C02-01 | Determines the Primary Owner of `CAN-L3-C-001`, the P0 entry scenario. Does not block discovery | Escalate to Lead Architect before Phase 2 ownership lock. Do not resolve inside the inventory |
| **Guardian and emergency-contact capabilities have two canonical owners** | `L2-C02` WF-C02-13.3 (guardian registration, updates, replacement, emergency contacts, primary designation, guardian communication preferences) vs `L2-C06` §5 ("sole authoritative bounded context… shall not be duplicated by other domains") | Determines the Primary Owner of `C-028`–`C-031` and all six proposed C06 additions — the single largest ownership exposure in Section C | Escalate. Provisionally keep Guardian as owner (C06 states exclusivity most strongly) and record the conflict against every affected row |
| **Alumni transition ownership language** | `L2-C05` §11.10 ("Transfer ownership of the institutional relationship to the Alumni Domain… Student becomes Alumni") vs `L2-C05` §9.2 and `L2-C08` §7.10 ("workflows shall not modify Student records"; "institutional identity ownership remains within Student Lifecycle; only engagement passes to Section H") | Determines whether the merged alumni candidate may say "ownership transferred". `C-031`/`C-034` currently use the corrected wording | Escalate the stale §11.10 text; keep the "without transferring ownership" phrasing |
| **Registration cancellation after freeze** | `L2-C03` §7.14 ("Registration rollback before freeze") and §11.3 (`Frozen` = "finalized and locked") vs §11.4 state diagram, which permits `Frozen → Cancelled` | Determines whether `C-020` covers post-freeze cancellation or only pre-freeze | Escalate; classify post-freeze cancellation as uncertain until resolved |
| **`AcademicPolicyUpdated` has no publisher** | `DEPENDENCY_MAP.md` §Policy Management: `L2-C08` (10 refs) and `L2-C04` (3 refs) reference policy-change events with no publisher anywhere; recorded as "**Open.** No owner assigned" | Weakens the trigger evidence for policy-revision-driven candidates (`NEXT-C-26` bulk recalculation is safe — C07 owns its own policy lifecycle; the C04 policy-change trigger is not) | Add the candidates; annotate the C04 policy trigger as dependent on an open dependency-map item |

None of these conflicts blocks a candidate from being inventoried. All five affect Phase 2 ownership confirmation and must not be resolved silently.

---

## 15. Final Exact Action List

```text
1.  Add the 27 missing candidates listed in Section 4.
2.  Renumber the duplicated Candidate IDs: reassign the C04 block (currently C-017 to C-020),
    the C06 block (currently C-028 to C-031), and the C07 opening row (currently C-031)
    to unique values, then renumber the whole inventory sequentially from CAN-L3-C-001.
3.  Add the missing section headings "### C07 — Scholarship Eligibility Scenario Discovery"
    and "### C08 — Alumni Transition Scenario Discovery" above their tables.
4.  Fix the file header: add "**Prepared By:** Chetan" and set Status to "Submitted for Review".
5.  Merge CAN-L3-C-031 (C05) and CAN-L3-C-034 (C08) into one alumni transition candidate
    owned by Alumni Transition.
6.  Merge CAN-L3-C-012 into CAN-L3-C-011, or record the decision to retain both.
7.  Split CAN-L3-C-025 into Qualification Completion and Degree Conferment, and
    Academic Record Finalization and Student Exit Completion.
8.  Retitle CAN-L3-C-010 to Intra-Institution Student Transfer.
9.  Retitle CAN-L3-C-018 to Elective and Choice-Based Credit Registration.
10. Retitle CAN-L3-C-020 to Academic Registration Cancellation and remove the Withdrawal claim.
11. Restate the Final Outcome of CAN-L3-C-028 (C06) to reach the verified Active state.
12. Restate the Final Outcome of CAN-L3-C-031 (C07) to include recommendation generation.
13. Correct the Domains field on CAN-L3-C-001, C-006, C-007, C-008, C-009, C-010, C-015,
    C-024, C-025, C-027, C-028 (C05), C-029, C-030 per Section 8.
14. Correct Type on CAN-L3-C-006, C-007 (Cross-domain), and CAN-L3-C-016, C-020 (Critical).
15. Correct Priority on CAN-L3-C-026 and CAN-L3-C-034 from P0 to P1.
16. Escalate the five repository conflicts in Section 14 to the Lead Architect and record
    them against the affected candidates; do not resolve them inside the inventory.
17. Re-run exhaustive verification after the above are applied.
```

---

## Appendix — Verification Method

Twelve independent passes were executed against the authoritative sources in the order defined by the repository authority hierarchy:

| Pass | Focus | Principal finding |
|---|---|---|
| 1 | L2 workflow coverage matrix (88 catalogued workflows across C01–C08) | 27 uncovered outcome families |
| 2 | Lifecycle transition coverage (admission, Student Context, registration, progression, qualification, guardian, scholarship, transition aggregates) | 8 uncovered terminal states |
| 3 | Event and hand-off coverage | `AdmissionExpired`, `StudentArchived`, `QualificationAwardRevoked`, `InstitutionalClearanceCompleted` have no covering scenario |
| 4 | Business rule coverage | Detention, probation, conditional promotion, custody, consent, and eligibility-policy rule families uncovered |
| 5 | Exception / recovery keyword sweep | Expiry, administrative exit, deceased closure, revocation, guardian suspension surfaced; 30 error cases correctly rejected |
| 6 | Actor / persona coverage | Guardian and scholarship-authority personas materially under-represented |
| 7 | Temporal coverage | Pre-registration (window) and post-exit (archival) phases uncovered |
| 8 | CRUD / tiny-scenario detection | `C-012` flagged for merge |
| 9 | Over-broad detection | `C-025` flagged for split |
| 10 | Duplicate / semantic overlap | 1 semantic overlap, 9 ID collisions |
| 11 | Repository boundary verification | 13 ownership/domain defects; 5 repository conflicts |
| 12 | Adversarial reconstruction (inventory titles withheld) | Independent reconstruction produced 73 outcomes against the inventory's 45 |

**Sources inspected:** `L2-C00` … `L2-C08` (original documents, not summaries); `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`; `architecture/ARCHITECTURE_INDEX.md`; `L3_Phase1_Discovery_Rules.md`; `L3_Phase1_Team_Instructions.md`; `L3_Scenario_Type_and_Priority_Rules.md`; `AI_CONTEXT/SECTION_C/C_Canonical_AI_Context.md` (navigation aid only).

---

# End of Report
