# Qualification Completion and Degree Conferment

**Architecture Level:** L3 — Business Scenario Architecture  
**Final L3 ID:** L3-C-039  
**Canonical Title:** Qualification Completion and Degree Conferment  
**Primary Owner:** Qualification Completion & Student Exit  
**Participating Domains:** C, I  
**Scenario Type:** Critical  
**Priority:** P0  
**Derivation:** KEEP  
**Status:** Locked  
**Version:** 1.0  
**Author:** Varad  
**Last Updated:** 2026-08-22  

## 1. Scenario Metadata

The frozen canonical identity is preserved for **L3-C-039 — Qualification Completion and Degree Conferment**. No ownership, participating-domain, type, priority, title, or scenario-boundary reclassification is introduced.

## 2. Scenario Purpose

Determine whether the Student has satisfied the complete institutional qualification-completion requirements, obtain the required approval, and record the official academic award.

## 3. Business Outcome

The Student's qualification completion is approved and the academic qualification is officially conferred. `QualificationCompletionApproved` precedes `QualificationAwarded`; qualification completion and degree conferment remain owned by L2-C05, while Examination evidence remains owned by Section I.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
Programme completion and authoritative Examination-completion evidence required for qualification completion are available for the qualification-completion process.

### 4.2 End Boundary
The scenario ends when the official qualification award is recorded and `QualificationAwarded` is published. Academic record finalization and Student Exit are downstream lifecycle steps.

### 4.3 In Scope
- Qualification-completion eligibility evaluation.
- Degree audit.
- Institutional clearance coordination.
- Qualification completion approval.
- Degree conferment.
- Publication of authoritative qualification-completion and award events.

### 4.4 Out of Scope
- Examination execution or result determination.
- Certificate or transcript generation.
- Convocation management.
- Alumni engagement.
- Academic record finalization after conferment.
- Student Exit completion.

### 4.5 Related Scenarios
| Relationship | Scenario |
|---|---|
| Predecessor | C036 — Qualification Eligibility Determination |
| Related | C027 — Student Academic Progression Decision |
| Downstream | C040 — Academic Record Finalization and Student Exit Completion |

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Qualification Completion & Student Exit (C05) | Canonical owner of qualification-completion eligibility, degree audit, qualification approval, degree conferment, and the resulting institutional qualification outcome. |
| Examination (I) | Supplies authoritative Examination completion evidence; does not make or publish the institutional qualification decision. |

Cross-domain participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority |
|---|---|---|
| Qualification Completion & Student Exit | Evaluates qualification completion, coordinates clearances, executes governed approval, and records degree conferment. | Canonical qualification authority. |
| Examination responsibility | Supplies authoritative Examination completion evidence. | Evidence authority for Examination-owned facts. |
| Configured academic approval authority | Provides institutional qualification approval where required by policy. | Authority is configuration/governance-driven as defined in C05; no single fixed role is invented. |

## 7. Preconditions and Entry Criteria

- Programme completion is established.
- Academic progression and applicable academic records are authoritative.
- Required Examination completion evidence is authoritative.
- Applicable qualification-completion policy is known.
- Required institutional clearance information can be evaluated.

## 8. Trigger

Qualification completion processing begins after Programme Completion and required academic/Examination evidence are available.

## 9. Main Business Flow

| Step | Business Behaviour | Owner |
|---|---|---|
| 1 | Validate programme completion and qualification-completion eligibility. | Qualification Completion & Student Exit |
| 2 | Complete the degree audit against applicable curriculum and qualification requirements. | Qualification Completion & Student Exit |
| 3 | Coordinate and evaluate required institutional clearances without re-owning the participating domains' underlying decisions. | Qualification Completion & Student Exit |
| 4 | Complete the configured qualification-completion approval workflow. | Qualification Completion & Student Exit |
| 5 | Record the approved qualification-completion outcome. | Qualification Completion Aggregate |
| 6 | Confer the academic qualification after approval. | Qualification Completion Aggregate |
| 7 | Publish `QualificationCompletionApproved` and then `QualificationAwarded` as completed business facts. | Qualification Completion Aggregate |

## 10. Alternate Business Flows

| Condition | Business Path | Result |
|---|---|---|
| Eligibility is not satisfied | Record the unmet requirement and defer completion processing. | Qualification completion remains unresolved. |
| Degree audit fails | Record deficiencies and defer approval. | No degree conferment. |
| Mandatory institutional clearance is outstanding | Await authoritative clearance resolution from the owning domain. | Approval cannot complete. |
| Approval is rejected | Follow the governed C05 review/rejection path. | No degree conferment. |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Resolution |
|---|---|
| Academic or Examination evidence is incomplete | Obtain authoritative evidence from its canonical owner before continuing. |
| Clearance remains incomplete | Preserve the qualification-completion position and await the owning domain's clearance outcome. |
| Qualification approval is rejected | Return to the governed review state defined by C05. |
| Qualification has already been conferred and later requires change | Use the governed C05 correction/revocation path; ordinary processing must not rewrite history. |

No technical recovery mechanics are introduced.

## 12. Business Rules and Decision Constraints

| Rule | Effect |
|---|---|
| **BR-001** Programme completion is required before Qualification Completion. | Establishes entry boundary. |
| **BR-007** Qualification Completion eligibility is determined exclusively through C05. | Prevents ownership overlap. |
| **BR-012** Every candidate undergoes a complete degree audit. | Makes degree audit mandatory. |
| **BR-022** Mandatory institutional clearances must complete before approval. | Prevents premature approval. |
| **BR-027** Approval requires configured academic authority. | Governs approval. |
| **BR-032** Degree conferment requires successful qualification-completion approval. | Prevents premature award. |
| **BR-033** Degree conferment is the official academic completion decision. | Establishes authoritative outcome. |
| **BR-036** Correction/revocation requires exceptional institutional governance. | Protects conferred awards. |

These rules remain owned by locked L2-C05.

## 13. State and Lifecycle Impact

| Owned Lifecycle | Scenario Impact | Result |
|---|---|---|
| Qualification Completion Aggregate | Eligibility → Degree Audit → Institutional Clearance → Approval Pending | Qualification Completion Approved |
| Qualification Completion Aggregate | Qualification Completion Approved → Degree conferment | Qualification Awarded |

`QualificationAwarded` is followed downstream by Academic Record Finalization; that later transition belongs to C040 rather than this scenario.

## 14. Business Events and Cross-Domain Collaboration

| Event | Canonical Publisher | Purpose |
|---|---|---|
| `QualificationCompletionEvaluationStarted` | Qualification Completion Aggregate | Qualification-completion processing begins. |
| `QualificationEligibilityEvaluated` | Qualification Completion Aggregate | Eligibility evaluation completes. |
| `DegreeAuditCompleted` | Qualification Completion Aggregate | Degree audit completes. |
| `InstitutionalClearanceCompleted` | Qualification Completion Aggregate | Required clearance coordination completes. |
| `QualificationCompletionApproved` | Qualification Completion Aggregate | Institutional qualification approval completes. |
| `QualificationAwarded` | Qualification Completion Aggregate | Official academic qualification is conferred. |

`ExaminationCompletionEvidencePrepared` remains an Examination-owned input and is not republished as a C05 decision event.

## 15. Business Authorization, Approval and Audit Obligations

Qualification Completion approval is an institutional academic decision. C05 permits configurable approval chains, so this scenario does not invent one universal approving role. Eligibility evaluation, audit, clearance coordination, approval, conferment, and exceptions must remain auditable.

No technical access-control model is introduced.

## 16. Completion Criteria and Postconditions

- Programme completion verified.
- Qualification eligibility established.
- Degree audit passed.
- Mandatory clearances complete.
- Qualification completion approved.
- Qualification conferred.
- `QualificationAwarded` published by the Qualification Completion Aggregate.
- Academic record finalization, certificate, transcript, convocation, and alumni processing remain downstream.

## 17. Open Issues, Assumptions and Deferred Questions

**OI-C039-01:** The exact institution-specific approval chain is configurable in locked C05; this document therefore does not invent a single fixed authority.

**OI-C039-02:** Examination completion evidence remains Section I-owned evidence. C05 remains the sole institutional qualification-decision authority.

**Assumption:** Academic record finalization begins after `QualificationAwarded` and is outside C039's end boundary.

## Production Verification Record

- Frozen canonical identity expressed in required `L3-C-039` format.
- Frozen owner, participating domains, type, priority, and title preserved.
- Locked L2-C05 ownership and lifecycle sequence applied.
- Canonical C05 event publishers preserved as Qualification Completion Aggregate.
- Examination evidence ownership preserved.
- No implementation mechanics introduced.
- All 17 mandatory sections present.
- Status remains **Draft** pending architecture review.
