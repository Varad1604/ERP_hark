# Academic Detention and Repeat Determination

**Scenario ID:** L3-C-031  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Progression  
**Participating Domains:** C, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.1  
**Owner:** Varad  
**Last Updated:** 2026-08-22  
**Depends On:** `L2-C04_Student_Academic_Progression_Workflows.md` as amended by `L2-C04_Student_Academic_Progression_Workflows_Amendment_2026-08-22.md`; `L2-C03_Student_Academic_Registration_Workflows.md`; official Examination outcomes  
**Referenced By:** L3-C-027, L3-C-029, L3-C-030, L3-C-024

## 1. Scenario Metadata

Frozen identity: C031 — Academic Detention and Repeat Determination; Primary Owner Academic Progression; Participating Domains C and I; Critical; P1; KEEP.

## 2. Scenario Purpose

Govern the institutional determination that a Student must not progress because applicable academic requirements remain unsatisfied, together with any policy-governed repeat-semester or repeat-academic-year consequence.

## 3. Business Outcome

The Student is authoritatively **Detained** and the applicable repeat consequence, where required by policy, is established. Academic Progression owns detention; Academic Registration owns subsequent registration consequences.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
The scenario begins when authoritative evidence supports the Detention path, including direct progression evaluation or failed Academic Probation review.

### 4.2 End Boundary
The scenario ends when Detained is established and any required repeat determination is recorded. Subsequent registration is outside this scenario.

### 4.3 In Scope
- Determine academic detention.
- Establish Detained.
- Determine repeat semester/year where policy requires it.
- Preserve audit evidence.
- Publish the canonical detention outcome.

### 4.4 Out of Scope
- Examination result creation/correction.
- Academic Probation management.
- Downstream Academic Registration execution.
- Technical workflow or infrastructure design.

### 4.5 Related Scenarios
| Relationship | Scenario |
|---|---|
| Preceded by | L3-C-027 Student Academic Progression Decision |
| Preceded by | L3-C-030 Academic Probation Management and Resolution |
| Related | L3-C-024 Backlog and Repeat Course Registration |

## 5. Ownership and Participating Domains

| Domain / Capability | Responsibility |
|---|---|
| Academic Progression | Owns detention decision, Detained lifecycle position, repeat determination and canonical detention event. |
| Examination (I) | Supplies official examination evidence. |
| Academic Registration (C03) | Owns downstream registration consequences only. |
| Student Lifecycle (C) | Consumes relevant lifecycle consequences without taking detention ownership. |

The frozen participating-domain set remains C, I.

## 6. Business Actors, Roles and Responsibilities

| Role | Responsibility | Authority |
|---|---|---|
| Academic Progression | Determines detention and applicable repeat outcome. | Canonical decision owner. |
| Examination | Supplies authoritative examination evidence. | Evidence owner only. |
| Academic Registration | Applies subsequent registration consequence. | Registration owner only. |

No universal detention-specific human approver is invented.

## 7. Preconditions and Entry Criteria

- Applicable progression policy is active.
- Sufficient authoritative academic evidence exists.
- Detention path is permitted by policy.
- Any repeat determination uses authoritative academic/curriculum facts.

## 8. Trigger

Academic Progression determines that progression requirements are not satisfied and the Detention path applies.

## 9. Main Business Flow

| Step | Business Behaviour | Owner | Result |
|---|---|---|---|
| 1 | Evaluate authoritative basis for non-progression. | Academic Progression | Detention basis established. |
| 2 | Apply approved progression policy. | Academic Progression | Detention confirmed where required. |
| 3 | Establish Detained. | Academic Progression | Student cannot normally progress. |
| 4 | Determine repeat semester/year where policy requires it. | Academic Progression | Repeat consequence established. |
| 5 | Preserve the decision and evidence. | Academic Progression | Audit history complete. |
| 6 | Publish `AcademicDetentionApplied`. | Academic Progression | Downstream consumers may act within their own ownership. |

## 10. Alternate Business Flows

| Condition | Business Path | Result |
|---|---|---|
| Failed probation review | Apply Academic Probation → Detained. | Detained. |
| Requirements later completed | Apply the applicable C04 resolution path. | Progressed where policy and lifecycle permit. |
| Approved appeal changes the basis | Return through the governed fresh-evaluation path. | Historical detention remains preserved. |

## 11. Failure, Interruption and Business Recovery

| Situation | Resolution |
|---|---|
| Academic evidence incomplete | Do not finalize detention solely on incomplete evidence. |
| Repeat consequence cannot yet be determined | Preserve detention where valid and resolve the repeat consequence under policy before downstream registration. |
| Authoritative appeal/review changes evidence | Conduct fresh evaluation; do not rewrite historical evidence. |

No technical recovery design is introduced.

## 12. Business Rules and Decision Constraints

- Academic Detention prevents normal academic progression.
- Only authoritative academic evidence participates in the decision.
- Historical progression decisions remain immutable.
- Repeat-semester/year selection remains policy governed.
- Manual exceptions require upstream-authorized institutional approval.

## 13. State and Lifecycle Impact

| Concept | Entry | Result | Owner |
|---|---|---|---|
| Academic Progression | Evaluation Running | Detained | Academic Progression |
| Academic Probation | Academic Probation | Detained after failed review | Academic Progression |
| Academic Progression | Detained | Evaluation Running after approved appeal, where applicable | Academic Progression |
| Academic Progression | Detained | Progressed after requirements completion, where applicable | Academic Progression |

No repeat-semester/year progression state is invented.

## 14. Business Events and Cross-Domain Collaboration

| Event | Canonical Publisher | Purpose |
|---|---|---|
| Applicable official examination-result event | Examination | Supplies authoritative evidence. |
| `AcademicDetentionApplied` | Academic Progression | Canonical publication that detention is established. |
| `StudentProgressed` | Academic Progression | Used for later governed resolution to Progressed where applicable. |

Under `ADR-STU-PROG-EVT-001`, `StudentDetained` is a superseded alias of `AcademicDetentionApplied` and is not a separate canonical event.

Academic Registration may consume the authoritative detention fact but cannot publish or alter it.

## 15. Business Authorization, Approval and Audit Obligations

The detention basis, repeat determination, exceptions, and any appeal-driven fresh evaluation must remain auditable. Authorization follows the approved progression policy; L3 introduces no technical IAM/RBAC model.

## 16. Completion Criteria and Postconditions

- Detention basis established under approved policy.
- Student is Detained.
- Repeat consequence established where required.
- `AcademicDetentionApplied` published by Academic Progression.
- Historical evidence preserved.
- Downstream registration remains separately owned.

## 17. Open Issues, Assumptions and Deferred Questions

**Open issues:** Exact repeat-semester/year selection criteria, downstream registration hand-off detail, and institution-specific appeal/approval roles remain governed by upstream policy where not universally fixed.

**Resolved upstream issue:** Canonical detention event naming is resolved by `ADR-STU-PROG-EVT-001` and the locked L2-C04 amendment.

**Assumptions:** Academic Progression owns detention; Examination owns official examination evidence; Academic Registration owns subsequent registration execution.

## Production Authoring / Verification Checklist

- [x] Frozen identity preserved.
- [x] Canonical `AcademicDetentionApplied` used.
- [x] Superseded `StudentDetained` not treated as a parallel canonical event.
- [x] Academic Progression publisher ownership preserved.
- [x] Cross-capability registration boundary preserved.
- [x] No implementation-level mechanics introduced.
- [x] All 17 mandatory sections present.
- [x] Status remains Draft.
