# NEP/MEME Intermediate Qualification Exit

**Architecture Level:** L3 — Business Scenario Architecture  
**Final L3 ID:** L3-C-041  
**Canonical Title:** NEP/MEME Intermediate Qualification Exit  
**Primary Owner:** Qualification Completion & Student Exit  
**Participating Domains:** C  
**Scenario Type:** Critical  
**Priority:** P1  
**Derivation:** KEEP  
**Status:** Locked  
**Version:** 1.1  
**Author:** Varad  
**Last Updated:** 2026-08-22  
**Applicable L2:** `L2-C05_Qualification_Completion_and_Student_Exit_Workflows.md` as amended by `L2-C05_Qualification_Completion_and_Student_Exit_Workflows_Amendment_2026-08-22.md` under `ADR-STU-MEME-EXIT-001`

## 1. Scenario Metadata

The frozen canonical identity is preserved for **L3-C-041 — NEP/MEME Intermediate Qualification Exit**. No title, owner, participating-domain, type, priority, or derivation change is introduced.

## 2. Scenario Purpose

Govern an institutionally approved NEP/MEME intermediate qualification exit using the qualification outcomes and policy boundaries established by locked L1 and the approved L2-C05 amendment, without inventing local credit/year thresholds or a parallel qualification lifecycle.

## 3. Business Outcome

An eligible Student receives the policy-determined qualification outcome and completes the current Student Context exit through the existing Qualification Completion & Student Exit lifecycle. Earned academic credits remain preserved in the authoritative Academic Credit Ledger subject to approved credit-validity policy.

The qualification outcome must be one of the higher-authority L1 outcomes: **Certificate, Diploma, Advanced Diploma, Degree, or Degree with Research**.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
The Student reaches a policy-recognized MEME exit point and becomes eligible for qualification-completion evaluation using authoritative academic evidence.

### 4.2 End Boundary
The scenario ends when the applicable qualification is awarded, the academic record is finalized, and the Student exit is completed through the existing C05 lifecycle.

### 4.3 In Scope
- Determine whether MEME exit is permitted by approved institutional/regulatory policy.
- Evaluate authoritative Academic Credit Ledger and other required evidence.
- Determine the applicable L1-permitted qualification outcome.
- Complete institutional clearance and qualification approval.
- Award the qualification.
- Finalize the academic record and current Student Context exit.
- Preserve earned credits for governed future use/re-entry.

### 4.4 Out of Scope
- Defining numeric credit/year/semester thresholds.
- Defining regulatory policy.
- Creating a new MEME-specific qualification lifecycle or event.
- Certificate or transcript generation.
- Alumni engagement processing.
- Executing future re-entry.
- Technical implementation.

### 4.5 Related Scenarios
| Relationship | Scenario |
|---|---|
| Related | C036 — Qualification Eligibility Determination |
| Related | C039 — Qualification Completion and Degree Conferment |
| Related | C040 — Academic Record Finalization and Student Exit Completion |

## 5. Ownership and Participating Domains

| Capability / Domain | Responsibility |
|---|---|
| Qualification Completion & Student Exit (C05) | Canonical owner of qualification eligibility within its boundary, institutional clearance coordination, approval, award, record finalization and exit completion. |
| Academic Progression (C04) | Owns the Academic Credit Ledger and progression facts consumed as evidence. |
| Examination | Owns Examination evidence where applicable; does not award the qualification. |

The frozen participating-domain set remains Section C only. Cross-capability evidence consumption does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority |
|---|---|---|
| Academic Administrator | Performs Qualification Completion evaluation. | Existing L2-C05 responsibility. |
| Authorized Organizational Units | Complete applicable institutional clearance. | Existing clearance responsibility. |
| Academic Authorities | Approve Qualification Completion. | Existing L2-C05 approval authority. |
| Registrar / Authorized Academic Authority | Confers/awards the applicable qualification. | Existing L2-C05 conferment authority. |
| Authorized Institutional Authority | Approves Student Exit where required. | Existing L2-C05 exit authority. |
| Academic Progression | Supplies authoritative Academic Credit Ledger/progression evidence. | Evidence owner only. |

No MEME-specific actor or approval hierarchy is invented.

## 7. Preconditions and Entry Criteria

- Applicable institution/regulation-approved MEME policy permits an exit pathway.
- Authoritative Academic Credit Ledger and other required academic evidence are available.
- The evidence satisfies the policy-defined requirements for an L1-permitted qualification outcome.
- Existing C05 clearance and approval prerequisites can be satisfied.

No numeric threshold is fixed at L3.

## 8. Trigger

A Student reaches a governed MEME exit point and requests, elects, or otherwise enters the institutionally permitted qualification-exit evaluation under approved policy.

## 9. Main Business Flow

| Step | Business Behaviour | Owner / Responsibility | Result |
|---|---|---|---|
| 1 | Identify the applicable MEME policy and exit eligibility basis. | Qualification Completion & Student Exit | Governing policy established. |
| 2 | Obtain authoritative Academic Credit Ledger and other required evidence. | Academic Progression supplies; C05 consumes | Evidence established. |
| 3 | Evaluate qualification eligibility against policy-defined requirements. | Academic Administrator / C05 | Applicable qualification outcome established or rejected. |
| 4 | Complete required Degree/Qualification Audit and institutional clearance. | C05 / Authorized Organizational Units | Completion prerequisites established. |
| 5 | Obtain Qualification Completion approval. | Academic Authorities | Qualification completion approved. |
| 6 | Award/confer the policy-determined L1-permitted qualification. | Registrar / Authorized Academic Authority | Qualification awarded. |
| 7 | Finalize the academic record. | Qualification Completion & Student Exit | Record finalized. |
| 8 | Complete the current Student Context exit under existing C05 governance. | Authorized Institutional Authority / C05 | Student exit completed. |
| 9 | Preserve earned credits in the authoritative Academic Credit Ledger subject to policy. | Academic Progression retains ownership | Credit history preserved. |

## 10. Alternate Business Flows

| Condition | Business Path | Result |
|---|---|---|
| Policy requirements not satisfied | Do not approve the MEME qualification exit. | Student remains governed by the applicable academic lifecycle. |
| Evidence incomplete | Defer qualification decision until authoritative evidence exists. | No award/exit occurs. |
| Clearance incomplete | Follow existing C05 clearance resolution. | Approval remains pending/unresolved. |
| Student later seeks re-entry | Consume preserved credit/evidence through the appropriate Student Lifecycle re-entry workflow. | Re-entry remains separately governed. |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Resolution |
|---|---|
| Policy does not support an exit at the Student's position | Do not grant an intermediate qualification locally. |
| Academic evidence is disputed/incomplete | Resolve through its canonical owning workflow before qualification completion. |
| Clearance or approval fails | Follow existing C05 rejection/resolution semantics; do not bypass authority. |
| Qualification mapping is ambiguous | Resolve through institution/regulation policy; L3 does not invent a mapping. |

No technical retry, rollback, API or infrastructure behavior is introduced.

## 12. Business Rules and Decision Constraints

- MEME qualification exit is policy governed.
- Allowed qualification outcomes are limited to Certificate, Diploma, Advanced Diploma, Degree, and Degree with Research as established by locked L1.
- Exact credit, year, semester, duration, re-entry, credit-validity and credit-expiry thresholds remain institution/regulation-configured policy.
- Qualification Completion & Student Exit owns the qualification/exit decision.
- Academic Progression retains Academic Credit Ledger ownership.
- Examination evidence does not itself award the qualification.
- Earned credits are preserved after MEME exit subject to approved validity policy.
- Existing C05 approval and audit obligations apply.

## 13. State and Lifecycle Impact

The MEME pathway uses the existing L2-C05 qualification-completion lifecycle rather than creating a parallel state model:

`Eligibility Evaluation → Degree/Qualification Audit → Institutional Clearance → Approval Pending → Qualification Completion Approved → Qualification Awarded → Academic Record Finalized → Student Exit Completed`

The policy-selected qualification outcome is recorded within this existing lifecycle. No `IntermediateQualification` state is invented.

## 14. Business Events and Cross-Domain Collaboration

The existing C05 canonical event vocabulary is reused:

| Business Outcome | Canonical Event | Canonical Publisher |
|---|---|---|
| Eligibility evaluated | `QualificationEligibilityEvaluated` | Qualification Completion Aggregate |
| Qualification audit completed | `DegreeAuditCompleted` | Qualification Completion Aggregate |
| Clearance completed | `InstitutionalClearanceCompleted` | Qualification Completion Aggregate |
| Completion approved | `QualificationCompletionApproved` | Qualification Completion Aggregate |
| Qualification awarded | `QualificationAwarded` | Qualification Completion Aggregate |
| Academic record finalized | `AcademicRecordFinalized` | Qualification Completion Aggregate |
| Student exit completed | `StudentExitCompleted` | Qualification Completion Aggregate |

No `IntermediateQualificationAwarded` or MEME-specific duplicate event is introduced.

## 15. Business Authorization, Approval and Audit Obligations

Existing L2-C05 authorities govern the pathway: Academic Administrator for evaluation, Authorized Organizational Units for clearance, Academic Authorities for completion approval, Registrar / Authorized Academic Authority for qualification conferment, and Authorized Institutional Authority for Student Exit. All decisions and evidence must remain auditable.

No technical IAM/RBAC design is introduced.

## 16. Completion Criteria and Postconditions

C041 completes when:

- MEME exit eligibility is established under approved policy;
- an allowed qualification outcome is selected from authoritative policy/evidence;
- required audit/clearance/approval is complete;
- `QualificationAwarded` is published by the Qualification Completion Aggregate;
- academic record finalization and Student Exit complete through existing C05 lifecycle/events; and
- earned credits remain preserved by Academic Progression subject to validity policy.

Certificate/transcript generation and future re-entry remain separately owned downstream processes.

## 17. Open Issues, Assumptions and Deferred Questions

**Resolved:** The former material gap in L2-C05 is resolved by `ADR-STU-MEME-EXIT-001` and `L2-C05_Qualification_Completion_and_Student_Exit_Workflows_Amendment_2026-08-22.md`.

**Open issues:** None requiring a new L3-owned credential, threshold, lifecycle state, event, or approval authority.

**Assumptions:** Institution/regulation policy supplies the exact MEME threshold and qualification mapping; Academic Progression remains owner of the Academic Credit Ledger.

**Deferred:** Numeric thresholds, policy configuration, credit-validity duration, re-entry execution, and implementation behavior remain outside this scenario or at later architecture levels.

## Production Verification Record

- Frozen canonical identity preserved.
- Locked L1 MEME outcomes and policy boundary respected.
- Approved L2-C05 MEME amendment consumed.
- Existing C05 lifecycle, authority and events reused.
- Academic Credit Ledger ownership preserved with Academic Progression.
- No MEME-specific duplicate event/state invented.
- No numeric policy threshold invented.
- All 17 mandatory sections present.
- Status remains Draft pending normal architecture review.
