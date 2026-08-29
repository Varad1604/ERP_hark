# Academic Probation Management and Resolution

**Scenario ID:** L3-C-030  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Progression  
**Participating Domains:** C, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Varad  
**Last Updated:** 2026-08-22  
**Depends On:** `L2-C04_Student_Academic_Progression_Workflows.md` as amended by `L2-C04_Student_Academic_Progression_Workflows_Amendment_2026-08-22.md`; applicable Student Lifecycle context; official Examination outcomes where probation review requires them  
**Referenced By:** L3-C-027, L3-C-028

## 1. Scenario Metadata

Frozen identity: C030 — Academic Probation Management and Resolution; Primary Owner Academic Progression; Participating Domains C and I; Critical; P1; KEEP.

## 2. Scenario Purpose

Govern the institutional lifecycle of a Student placed on **Academic Probation**, including establishment of probation conditions, review of academic progress, and authoritative resolution of the probation outcome.

The canonical probation-assignment event vocabulary is governed by `ADR-STU-PROG-EVT-001`.

## 3. Business Outcome

The Student's Academic Probation is authoritatively reviewed and resolved as either **Progressed** after successful review or **Detained** after unsuccessful review. Probation history remains auditable.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
The scenario begins when Academic Progression authoritatively assigns Academic Probation.

### 4.2 End Boundary
The scenario ends when the probation review establishes Progressed or Detained.

### 4.3 In Scope
- Establish probation conditions.
- Conduct business-level monitoring and review.
- Evaluate authoritative academic evidence.
- Resolve to Progressed or Detained.
- Preserve probation history and evidence.

### 4.4 Out of Scope
- Original progression evaluation.
- Examination result creation/correction.
- Definition of probation policy.
- Detention workflow after escalation.
- Technical monitoring, scheduling, notifications, retries, APIs or infrastructure.

### 4.5 Related Scenarios
| Relationship | Related Scenario |
|---|---|
| Preceded by | L3-C-027 Student Academic Progression Decision |
| May be preceded by | L3-C-029 Conditional Promotion Condition Fulfilment |
| Followed by | L3-C-031 Academic Detention and Repeat Determination |

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Academic Progression | Owns probation conditions, review, resolution, and canonical progression outcomes. |
| Student Lifecycle (C) | Provides/consumes Student context where required; does not own probation. |
| Examination (I) | Provides official academic outcomes where relevant. |

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Academic Progression | Establishes probation, reviews conditions and determines resolution. | Canonical decision owner. |
| Examination | Supplies official academic evidence. | Evidence owner only. |
| Student Lifecycle | Consumes relevant lifecycle consequences. | Participant only. |

No universal probation-specific human role is invented.

## 7. Preconditions and Entry Criteria

- Student is in Academic Probation.
- Applicable conditions and progression policy are established.
- Required authoritative academic evidence is available or its absence can be determined.

## 8. Trigger

Probation review becomes due under the applicable institutional policy.

## 9. Main Business Flow

| Step | Business Behaviour | Owner | Result |
|---|---|---|---|
| 1 | Identify active probation conditions and review basis. | Academic Progression | Review scope established. |
| 2 | Evaluate authoritative academic evidence. | Academic Progression / Examination | Evidence assessed. |
| 3 | Determine whether probation requirements are satisfied. | Academic Progression | Review decision established. |
| 4 | If passed, establish Progressed. | Academic Progression | Probation successfully resolved. |
| 5 | Preserve review outcome and history. | Academic Progression | Audit evidence complete. |

## 10. Alternate Business Flows

| Condition | Business Path | Result |
|---|---|---|
| Review fails | Establish Detained under the canonical C04 lifecycle. | Detention established. |
| Required evidence is not authoritative | Defer final outcome where policy permits. | Probation remains unresolved. |

## 11. Failure, Interruption and Business Recovery

| Situation | Resolution |
|---|---|
| Required evidence unavailable | Continue governed probation lifecycle until authoritative evidence is available where policy permits. |
| Requirements not satisfied | Transition to Detained through the canonical C04 path. |
| Authoritative evidence changes | Conduct a governed fresh evaluation where policy permits; preserve history. |

No technical recovery mechanics are defined.

## 12. Business Rules and Decision Constraints

- Probation is policy governed by Academic Progression.
- Failed probation review may transition to Detained.
- Historical progression decisions remain immutable.
- Only authoritative academic evidence may determine the outcome.
- L3 does not create a new probation rule or approver.

## 13. State and Lifecycle Impact

| Concept | Entry | Result | Owner |
|---|---|---|---|
| Academic Probation | Academic Probation | Progressed when review passes. | Academic Progression |
| Academic Probation | Academic Probation | Detained when review fails. | Academic Progression |

No new lifecycle state is introduced.

## 14. Business Events and Cross-Domain Collaboration

| Event | Canonical Publisher | Purpose |
|---|---|---|
| Applicable official examination-result event | Examination | Supplies authoritative academic evidence. |
| `AcademicProbationAssigned` | Academic Progression | Canonical publication that Academic Probation has been assigned. |
| `StudentProgressed` | Academic Progression | Communicates successful resolution to Progressed. |
| `AcademicDetentionApplied` | Academic Progression | Canonical publication of an unsuccessful probation outcome that results in detention. |

Under `ADR-STU-PROG-EVT-001`, `StudentPlacedOnProbation` and `StudentOnProbation` are superseded aliases of `AcademicProbationAssigned`; `StudentDetained` is a superseded alias of `AcademicDetentionApplied`. They are not parallel canonical events.

## 15. Business Authorization, Approval and Audit Obligations

Probation decisions remain governed by approved institutional progression policy. The review basis, decision and lifecycle outcome must remain auditable. No technical IAM/RBAC model is introduced.

## 16. Completion Criteria and Postconditions

C030 completes when the probation review is authoritatively resolved as Progressed or Detained, the applicable canonical Academic Progression outcome is recorded/published, and history remains preserved.

## 17. Open Issues, Assumptions and Deferred Questions

**Open issues:** Exact probation review cadence and universal named human approval role remain policy/institution specific where upstream architecture does not fix them.

**Resolved upstream issue:** Canonical probation/detention event vocabulary is resolved by `ADR-STU-PROG-EVT-001` and the locked L2-C04 amendment effective 2026-08-22.

**Assumptions:** Academic Progression owns probation and its outcomes; Examination owns official examination evidence.

**Deferred:** Institution-specific cadence and detailed approval routing.

## Production Authoring / Verification Checklist

- [x] Frozen ID, title, owner, domains, type and priority preserved.
- [x] Locked L2-C04 amendment and ADR consumed.
- [x] Canonical `AcademicProbationAssigned` and `AcademicDetentionApplied` vocabulary used.
- [x] Academic Progression remains canonical publisher.
- [x] No duplicate event publisher or technical implementation detail introduced.
- [x] All 17 mandatory sections present.
- [x] Status remains Draft pending normal architecture review.
