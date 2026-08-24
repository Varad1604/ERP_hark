# L3-I-071 — Marks Collection for Completed Examination Activities

# 1. Scenario Metadata

**Scenario ID:** L3-I-071  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Marks Collection & Evaluation  
**Participating Domains:** Academic Delivery, Examination  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Review  
**Author:** Pallav  
**Version:** 1.1  
**Last Updated:** 2026-08-24  
**Canonical Source Candidate:** CAN-L3-I-071  
**Primary L2 Source:** `architecture/L2_Domain_Workflow_Architecture/section-i/L2-I08_Marks_Collection_and_Evaluation_Workflows.md`  
**Governance Note:** Previous `Locked` status referenced `ACR-I-2026-08-21-002`. Repository search confirms the identifier is repeated in scenario documents, but no independently traceable governance/ADR artifact for that ACR was located. The scenario is therefore returned to **Review** pending traceable approval evidence; this correction does not invalidate the canonical scenario ID or frozen scenario allocation.

# 2. Scenario Purpose

This scenario governs collection of marks arising from completed examination activities. It begins from authoritative completed examination or assessment evidence and concludes when awarded marks have been collected and submitted within Marks Collection & Evaluation for subsequent validation, approval and finalisation.

# 3. Business Outcome

Marks arising from completed examination activities are collected from authorized evaluators, associated with the authoritative assessment context, and submitted as the governed marks-collection outcome without absorbing downstream validation, approval, finalisation or result-processing outcomes.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario starts when an authoritative completed examination or assessment fact and its required assessment evidence are available to Marks Collection & Evaluation.

Applicable upstream facts include the exact L2-I08 consumed events where relevant to the activity:

- `Practical Examination Completed` — published by L2-I06.
- `Theory Examination Completed` — published by L2-I07.
- `Practical Assessment Completed` — published by L2-I06.
- `Answer Books Collected` — published by L2-I07 where theory-script collection is applicable.

## 4.2 End Boundary

The scenario ends when marks awarded through the completed examination activity have been collected from authorized evaluators and the authoritative `Marks Submitted` business event has been published by Marks Collection & Evaluation.

The scenario does **not** include Marks Validation, Marks Approval, Marks Finalisation, Final Approved Marks availability, Result Processing, grade calculation or result publication.

## 4.3 In Scope

- Verification that the examination or assessment activity is authoritatively completed.
- Receipt and traceable association of completed assessment evidence.
- Collection of marks from authorized evaluators.
- Verification that required marks for the applicable collection scope are present for submission.
- Submission of the collected marks for subsequent validation.
- Preservation of evaluation and marks-collection evidence.

## 4.4 Out of Scope

- Examination conduct ownership.
- Examination scheduling or registration ownership.
- Marks validation.
- Marks approval.
- Marks finalisation or establishment of Final Approved Marks.
- Result Processing & Computation.
- Revaluation or rechecking.
- API, UI, database, messaging, access-control, infrastructure or technical recovery design.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Followed by | L3-I-072 — Examination Evaluation to Final Approved Marks | L3-I-072 governs downstream evaluation completion, validation, approval and establishment of Final Approved Marks. L3-I-071 stops at collected/submitted marks. |

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Marks Collection & Evaluation (L2-I08) | Owns assessment-evidence receipt within evaluation, marks collection, marks submission, validation, approval and finalisation lifecycles; this scenario exercises only the evidence/collection/submission portion. |
| Examination Conduct (L2-I06 / L2-I07) | Owns authoritative practical/theory examination completion and conduct evidence supplied to L2-I08. |
| Academic Delivery | Supplies only canonically owned assessment facts or evidence where applicable; ownership does not transfer to Examination. |

The Primary Owner retains the marks-collection outcome. Participating domains retain ownership of their source facts.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility |
|---|---|
| Evaluator | Performs authorized academic evaluation and supplies awarded marks. |
| Chief Evaluator | Supervises evaluation quality where the institutional evaluation model requires it. |
| External Evaluator | Performs independent evaluation where assigned by policy. |
| Internal Evaluator | Performs institutional evaluation where assigned by policy. |
| Evaluation Coordinator / Examination Administrator | Coordinates marks-collection activity where assigned by the institutional operating model. |

Only authorities established by L2-I08 and institutional policy apply. This scenario does not invent a new approval authority.

# 7. Preconditions and Entry Criteria

- The applicable examination or assessment activity has been authoritatively completed.
- Required assessment evidence is available and traceable to its canonical source.
- The evaluator is authorized for the applicable assessment scope.
- Applicable marking scheme, evaluation policy and assessment context are authoritative.
- Marks Collection & Evaluation can identify the exact candidate, assessment/examination activity and evaluation scope.
- No unresolved ownership or identity conflict prevents marks collection.

# 8. Trigger

An authoritative completed examination or assessment activity becomes available for marks collection together with the required assessment evidence.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the authoritative completed examination/assessment fact and required evidence from its owning domain. | Marks Collection & Evaluation / source domain | The marks-collection context is established without transferring source ownership. |
| 2 | Verify the candidate, assessment/examination activity, evaluator assignment and applicable evaluation policy. | Marks Collection & Evaluation | The collection scope is valid and attributable. |
| 3 | Make the authorized assessment evidence available to the assigned evaluator under the applicable evaluation model. | Evaluation Coordinator / Marks Collection & Evaluation | The evaluator has the governed evidence required to award marks. |
| 4 | Perform academic evaluation and determine awarded marks according to the approved marking scheme and policy. | Evaluator | Awarded marks exist as the evaluator's governed academic evaluation outcome. |
| 5 | Collect the awarded marks and associate them with the authoritative candidate, assessment and evaluation context. | Marks Collection & Evaluation | Collected marks are traceable and ready for submission. |
| 6 | Verify that the required marks for the applicable collection scope are present and attributable. | Marks Collection & Evaluation | Marks are ready to enter the downstream validation stage. |
| 7 | Submit the collected marks. | Authorized evaluator / Marks Collection & Evaluation | `Marks Submitted` is published and the scenario completes. |

# 10. Alternate Business Flows

## 10.1 Missing Assessment Evidence

If required assessment evidence is unavailable, evaluation and marks collection do not proceed. The evidence-owning domain must supply or resolve the authoritative evidence before continuation.

## 10.2 Evaluator Unavailable

Where an assigned evaluator becomes unavailable, a replacement may be assigned only through the applicable L2-I08/institutional policy path. Existing evidence and assignment history remain traceable.

## 10.3 Incomplete Marks Collection

If required marks are missing for the applicable collection scope, submission does not occur. Collection remains incomplete until the authoritative missing marks are supplied or a governed exception path resolves the situation.

## 10.4 Evaluation Discrepancy

Where the institutional evaluation model requires multiple evaluations and a material discrepancy exists, the applicable discrepancy-resolution process is followed before marks are submitted as ready for validation.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Completed examination fact cannot be verified | Marks collection cannot validly begin. | Resolve the source fact with the canonical Examination Conduct owner. | L2-I06/L2-I07 and L2-I08 |
| Evidence is missing or compromised | Evaluation cannot validly continue. | Obtain authoritative evidence or follow the applicable governed investigation/exception path. | Evidence owner and L2-I08 |
| Evaluator authority is invalid | Awarded marks cannot be accepted into the governed collection. | Correct evaluator assignment through the applicable authority path. | L2-I08 |
| Marks collection is interrupted before submission | The authoritative submitted-marks outcome has not occurred. | Preserve the last valid business position and continue only when the missing condition is resolved. | L2-I08 |
| Validation later fails | Outside this scenario boundary. | Handled by the downstream Marks Validation workflow; L3-I-071 does not absorb that outcome. | L2-I08 downstream workflow |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Scenario Effect | Owner / Source |
|---|---|---|
| Evaluation begins only after authorized examination conduct has completed. | Prevents marks collection from starting on incomplete or unauthorized examination activity. | L2-I08 §4.1, §9.1 |
| Only authorized evaluators perform academic evaluation. | Prevents unauthorized marks from entering the collection outcome. | L2-I08 §4.5, §9.2 |
| Assessment evidence remains traceable and protected throughout evaluation. | Preserves academic integrity and auditability. | L2-I08 §9.3 |
| Evaluation follows approved marking schemes and evaluation policy. | Awarded marks must be academically defensible and policy-compliant. | L2-I08 §9.4 |
| Collected marks remain provisional until downstream institutional approval. | Prevents L3-I-071 from treating collected/submitted marks as Final Approved Marks. | L2-I08 §5.7, §7.5 |
| `Marks Submitted` represents evaluator submission of collected marks for validation. | Defines the exact scenario completion fact. | L2-I08 §11 |
| Marks Validation, Marks Approval and Final Approved Marks belong to downstream L2-I08 stages and L3-I-072. | Prevents scenario overlap and duplicate outcome ownership. | L2-I08 §§8.9–8.12; L3-I-072 |

No local threshold, deadline, formula, lifecycle state, approval rule or event name is invented.

# 13. State and Lifecycle Impact

This scenario does not introduce a new lifecycle model. It applies the existing L2-I08 evaluation lifecycle through the marks-collection/submission boundary:

```text
Completed assessment evidence available
        ↓
Authorized evaluation
        ↓
Awarded marks collected
        ↓
Marks Submitted
```

Marks Validation, Marks Approval, Marks Finalisation and Ready for Result Processing occur after this scenario ends.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Consumed Business Events

| Event | Canonical Publisher | Business Purpose | Source |
|---|---|---|---|
| `Practical Examination Completed` | L2-I06 — Practical & Viva Examination Conduct | Establishes authoritative completion of a practical/viva examination activity for downstream evaluation. | L2-I08 §12 |
| `Theory Examination Completed` | L2-I07 — Theory Examination Conduct | Establishes authoritative completion of a theory examination activity for downstream evaluation. | L2-I08 §12 |
| `Practical Assessment Completed` | L2-I06 — Practical & Viva Examination Conduct | Provides authoritative completed practical-assessment context. | L2-I08 §12 |
| `Answer Books Collected` | L2-I07 — Theory Examination Conduct | Provides authoritative theory answer-book collection context where applicable. | L2-I08 §12 |

Only events applicable to the actual examination/assessment path are consumed.

## 14.2 Published Business Events

| Event | Canonical Publisher | Published When | Primary Consumer / Purpose | Source |
|---|---|---|---|---|
| `Assessment Evidence Received` | L2-I08 — Marks Collection & Evaluation | Required assessment evidence has been successfully collected into the evaluation context. | Evaluation operations within L2-I08. | L2-I08 §11 |
| `Evaluation Completed` | L2-I08 — Marks Collection & Evaluation | The evaluator has completed academic evaluation. | Marks collection / moderation as applicable. | L2-I08 §11 |
| `Marks Submitted` | L2-I08 — Marks Collection & Evaluation | The evaluator has submitted collected marks. | Marks Validation. This is the scenario completion event. | L2-I08 §11 |

`Marks Validated`, `Marks Approved`, `Final Approved Marks Available`, `Evaluation Closed` and `Evaluation Archived` are valid L2-I08 events but are downstream of this scenario's end boundary and are therefore not published outcomes of L3-I-071.

## 14.3 Business Contracts and Cross-Domain Hand-offs

| Provider | Consumer | Business Hand-off | Purpose |
|---|---|---|---|
| L2-I06 / L2-I07 | L2-I08 | Completed examination/assessment facts and authoritative conduct evidence | Establish legitimate readiness for academic evaluation and marks collection. |
| Academic Delivery, where applicable | L2-I08 | Canonically owned assessment facts/evidence | Supports evaluation without transferring Academic Delivery ownership. |
| L2-I08 marks collection | L2-I08 Marks Validation / downstream L3-I-072 | Submitted marks with applicable evaluation and evidence context | Hands the authoritative collected-marks outcome to downstream validation without declaring the marks final. |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

- Only authorized evaluators may perform academic evaluation and supply awarded marks.
- Evaluation coordination may be performed by the applicable L2-I08 business roles.
- This scenario does not grant institutional Marks Approval authority; approval occurs downstream.

## 15.2 Separation of Duties, Overrides and Exceptions

Any evaluator independence, moderation, multiple-evaluation or approval separation required by institutional policy remains governed by L2-I08. No new override is introduced locally.

## 15.3 Business Audit and Evidence Obligations

The institution must be able to trace:

- the completed examination/assessment source fact;
- candidate and assessment identity;
- assessment evidence received;
- evaluator assignment and authority;
- applicable evaluation policy/marking scheme;
- awarded marks;
- collection completeness;
- submission actor and time;
- `Marks Submitted` as the completed business fact for this scenario.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The examination/assessment completion fact is authoritative.
- Required assessment evidence is available and traceable.
- Evaluation was performed by an authorized evaluator.
- Awarded marks were collected for the applicable scenario scope.
- Required collection completeness checks were satisfied.
- `Marks Submitted` was published by L2-I08.
- No downstream Marks Validation, Marks Approval or Final Approved Marks outcome was absorbed.

## 16.2 Postconditions

- Submitted marks are available to the downstream Marks Validation workflow.
- Collected/submitted marks remain provisional until the authoritative downstream approval/finalisation stages complete.
- Examination Conduct retains ownership of its completion facts.
- Marks Collection & Evaluation retains ownership of the submitted marks/evaluation facts.
- L3-I-072 may continue the governed path toward Final Approved Marks where its own preconditions are satisfied.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| `ACR-I-2026-08-21-002` is referenced as approval authority by this and multiple Section I scenarios, but no independently traceable ACR/ADR governance artifact was located in the repository search. | Formal Architecture Approved/Locked status cannot be independently verified from repository evidence. | Locate and register the original approval record, or re-run the applicable governance approval/lock step under the repository's current change-management process. Until then this scenario remains **Review**. |

## 17.2 Assumptions

No assumption overrides locked architecture. Event names, publishers and boundaries in this revision are taken directly from current locked L2-I08 and adjacent L3 scenario responsibility.

## 17.3 Deferred Questions

System behaviour, solution, integration, security, data, UI/UX and technical recovery design are deferred to later architecture levels.
