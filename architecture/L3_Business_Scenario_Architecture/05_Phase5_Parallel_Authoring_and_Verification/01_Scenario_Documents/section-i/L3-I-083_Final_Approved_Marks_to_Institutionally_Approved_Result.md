# Final Approved Marks to Institutionally Approved Result

# 1. Scenario Metadata

**Scenario ID:** L3-I-083  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Result Processing & Computation  
**Participating Domains:** C, D, I  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-26  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-I08 — Marks Collection & Evaluation Workflows; L2-I09 — Result Processing & Computation Workflows; applicable Student Lifecycle and Academic Delivery workflows; applicable Platform Foundation authorization, event and audit architecture  
**Referenced By:** L2-I10 — Result Publication Workflows; Student Lifecycle and Academic Delivery progression consumers

---

# 2. Scenario Purpose

Transform complete Final Approved Marks from I08 into one institutionally approved academic result under I09 ownership, applying configured academic regulations while preserving marks, student and academic-delivery ownership outside Result Processing & Computation.

---

# 3. Business Outcome

Subject results, credit points, grades, SGPA and CGPA, programme result, classification and academic standing are computed, validated and institutionally approved. The resulting Institutionally Approved Result is authoritative for I10 publication and for authorized Student Lifecycle and Academic Delivery progression use.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when Final Approved Marks are available from I08 for every required examination component of the applicable candidate population.

## 4.2 End Boundary

The scenario ends when I09 has completed computation, validation, institutional approval and finalisation of the applicable results and those results are ready for publication and authorized downstream consumption.

## 4.3 In Scope

- Final Approved Marks intake and readiness verification.
- Marks aggregation and component weightage application.
- Academic regulation, pass/fail, grace/benefit and credit rule processing where applicable.
- Subject, grade, grade-point, SGPA, CGPA and programme-result computation.
- Academic standing and classification determination.
- Result validation, institutional approval and finalisation.
- Publication of source-defined I09 business events.
- Preservation of result-processing evidence and source-mark traceability.

## 4.4 Out of Scope

- Creation, correction or approval of marks owned by I08.
- Publication of official results owned by I10.
- Student progression decisions owned by Student Lifecycle.
- Academic-delivery decisions owned by Academic Delivery.
- API, database, UI, technical workflow, queue, retry, deployment or infrastructure design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Boundary Reason |
|---|---|---|
| Upstream | I08 marks evaluation scenarios | Supply Final Approved Marks; I09 does not alter approved marks. |
| Downstream | L3-I-101 — Publish Official Examination Results | Publishes the institutionally approved result without recomputation. |
| Downstream consumer | Student Lifecycle progression scenarios | Consume the approved academic outcome without taking I09 ownership. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Result Processing & Computation / I09 | Canonical owner of academic result computation, validation, approval, finalisation and resulting academic-result facts. |
| Marks Collection & Evaluation / I08 | Supplies immutable Final Approved Marks and retains evaluation ownership. |
| Student Lifecycle / C | Supplies/consumes Student and progression context without owning I09 computation. |
| Academic Delivery / D | Supplies/consumes academic structure or delivery context without owning I09 computation. |
| Platform Foundation / A | Supplies governed authorization, event transport and audit capability without owning result semantics. |

The frozen participating-domain value remains exactly `C, D, I`.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority Boundary |
|---|---|---|
| Result Processing Officer | Coordinates result-processing execution, verifies readiness and performs authorized computation/validation activities. | Cannot modify Final Approved Marks or bypass academic regulations. |
| Examination Administrator | Supports examination-result processing and source completeness within authorized examination scope. | Does not replace institutional approval authority. |
| Department Head | Participates in result review/approval where institutional policy assigns departmental approval. | Authority applies only where the configured institutional approval path requires it. |
| Faculty Dean | Participates in result review/approval where institutional policy assigns faculty-level approval. | Authority applies only where configured. |
| Academic Registrar | Participates in institutional result governance where the applicable policy assigns that responsibility. | Does not alter I08 marks. |
| Controller of Examination | Exercises result approval/finalisation authority where assigned by institutional policy. | Cannot bypass required validation or earlier approvals. |
| Academic Committee | Participates where institutional academic regulations require committee review or approval. | Applies only to the configured governance path. |
| Candidate | Subject of the computed result and authorized consumer of the final published outcome; does not approve their own result. | No result-processing mutation authority is implied. |

L2-I09 defines these as canonical business actors. The exact approval sequence is institution-configurable; this scenario does not invent a universal sequence beyond the source-defined requirement for institutional approval.

---

# 7. Preconditions and Entry Criteria

- Final Approved Marks exist from I08 for all required examination components in scope.
- Candidate and examination-scope references are valid and canonical.
- Applicable grading schemes, credit rules, pass criteria, grace/benefit rules, classification policy and academic regulations are active for the result scope.
- The result population is eligible for computation.
- Any required institutional approval path can be resolved under governing policy.
- No unresolved marks-completeness or candidate-association issue prevents valid computation.

---

# 8. Trigger

Final Approved Marks are available from I08 for all required examination components of the applicable candidate population.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Verify that complete Final Approved Marks are available for the applicable candidates and examination components. | Result Processing Officer / I09 | The population is ready for computation or moves to governed interruption handling. |
| 2 | Resolve the applicable academic regulations, grading scheme, credit rules, pass criteria, benefit rules and classification policy. | Result Processing Officer; Examination Administrator / I09 | The authoritative computation basis is established. |
| 3 | Aggregate approved marks according to the configured assessment-component and weightage rules. | Result Processing Officer / I09 | Governed aggregate marks are available without changing I08 source marks. |
| 4 | Apply academic rules, including mandatory-component, pass/fail, credit and permitted grace/benefit rules. | Result Processing Officer / I09 | Each candidate has a policy-governed academic computation basis. |
| 5 | Compute subject outcomes, grades, grade points and earned credits. | Result Processing Officer / I09 | Subject-level academic results are established. |
| 6 | Compute SGPA, CGPA and applicable semester, academic-year or programme outcomes. | Result Processing Officer / I09 | Aggregate academic performance is established. |
| 7 | Determine classification and academic standing under the applicable regulations. | Result Processing Officer / I09 | The complete computed academic outcome exists. |
| 8 | Validate computational accuracy, credit consistency, grade consistency and regulation compliance. | Result Processing Officer; Examination Administrator / I09 | Only validated results proceed to institutional approval. |
| 9 | Complete the applicable institutional approval path using only the canonical I09 actors assigned by institutional policy. | Controller of Examination; Academic Registrar; Department Head; Faculty Dean; Academic Committee, as applicable / I09 | The validated result becomes institutionally approved. |
| 10 | Finalise the approved result and make it ready for I10 publication and authorized progression consumers. | Controller of Examination or other policy-assigned finalising authority / I09 | The institutionally approved result becomes the authoritative I09 outcome. |
| 11 | Publish the applicable canonical I09 events and preserve immutable processing and approval evidence. | Result Processing & Computation / I09; Event and Audit Infrastructure | Downstream consumers can react without acquiring result ownership. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Grace or academic benefit applies | Approved institutional regulations permit a benefit | Apply the configured benefit before completing the relevant result computation. | A valid policy-governed result reaches the same institutional approval outcome. | I09 |
| Multiple institutional approval stages | Policy requires more than one approval level | Complete the configured Department, Faculty, Registrar, Controller or Academic Committee path as applicable. | The result becomes institutionally approved only after all required stages complete. | I09 |
| No optional benefit applies | Candidate does not satisfy a configured grace/benefit condition | Continue computation using the ordinary academic regulations. | Normal computed result proceeds to validation and approval. | I09 |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required Final Approved Marks are missing | Computation would be incomplete | Withhold computation for the affected scope until I08 supplies the authoritative marks. | I08; I09 |
| Candidate or examination association is inconsistent | Result could be associated incorrectly | Resolve through the applicable candidate-association correction scenario before final approval. | I09 and source owner |
| Academic-rule configuration is unavailable or contradictory | A reliable result cannot be computed | Stop the affected computation and resolve the authoritative policy/configuration issue. | I09 / applicable policy owner |
| Validation fails | Result cannot proceed to approval | Correct the computation basis or source-owned input through the appropriate governed path, then recompute where authorized. | I09; source owner where applicable |
| Required institutional approval is denied or cannot complete | Result is not institutionally approved | Retain the last valid non-approved position and follow the applicable governed resolution path. | I09 |

No technical retry, rollback or implementation recovery mechanism is defined at L3.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Result computation begins only from Final Approved Marks. | I09 / I08 boundary | Provisional or unapproved marks cannot enter authoritative result computation. | L2-I09 §4.1, §5.1 |
| Final Approved Marks are immutable inputs to I09. | I08 | I09 cannot alter evaluation history or awarded marks. | L2-I09 §4.6 |
| Academic computation follows approved institutional regulations. | I09 | Grading, credit, pass/fail, grace and classification logic must use active policy. | L2-I09 §§4.4–4.5, 5.3–5.11 |
| Validation precedes institutional approval. | I09 | Unvalidated results cannot become authoritative approved results. | L2-I09 §§5.12–5.13 |
| Institutional approval precedes finalisation/publication readiness. | I09 | Only approved results may become ready for I10. | L2-I09 §§4.2, 5.13–5.15 |
| I10 publishes results but does not recalculate them. | I09/I10 boundary | Result ownership remains in I09 after publication. | L2-I09 §4.2, §4.7 |

---

# 13. State and Lifecycle Impact

L2-I09 defines a business lifecycle rather than a compact named state enumeration. This scenario preserves the exact source progression without inventing additional states:

`Final Approved Marks Consumption → Marks Aggregation → Academic Rule Processing → Grace & Benefit Processing → Subject Result Computation → Credit Point Calculation → Grade Calculation → SGPA/CGPA Calculation → Programme Result Computation → Academic Standing Determination → Result Validation → Result Approval → Result Finalisation → Ready for Publication`.

| Business Concept | Entry Condition | Scenario Impact | Resulting Lifecycle Position | Canonical Owner |
|---|---|---|---|---|
| Final Approved Marks | Approved and complete | Consumed as immutable inputs | Unchanged | I08 |
| Result Processing outcome | Ready for computation | Progresses through the canonical I09 lifecycle | Finalised / Ready for Publication | I09 |
| Student and academic-delivery facts | Existing authoritative context | Referenced only | Unchanged by I09 | C / D |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Business Events

L2-I09 identifies significant result-processing events including:

- `Result Processing Started`
- `Subject Result Computed`
- `Semester Result Computed`
- `CGPA Calculated`
- `Result Approved`
- `Result Ready for Publication`

The exact event is published only when its corresponding business fact has been established. I09 retains event-semantic ownership; Event Infrastructure owns transport.

## 14.2 Business Hand-offs

| Provider | Consumer | Hand-off | Ownership Effect |
|---|---|---|---|
| I08 | I09 | Final Approved Marks | I08 retains marks/evaluation ownership. |
| I09 | I10 | Institutionally Approved Result ready for publication | I09 retains result-computation ownership; I10 owns publication. |
| I09 | Student Lifecycle / C | Approved academic-result outcome | C may make its own progression decision without rewriting the I09 result. |
| I09 | Academic Delivery / D | Approved academic-result outcome | D consumes academic outcome without acquiring I09 ownership. |

---

# 15. Business Authorization, Approval and Audit Obligations

- Result-processing activities require authorization for the applicable institution and examination scope.
- Institutional approval is mandatory before finalisation; the exact actor sequence follows the institution's configured policy using only L2-I09 canonical actors.
- A candidate cannot approve their own result.
- Any applied grace/benefit, validation outcome and institutional approval must remain auditable.
- Evidence must preserve source marks references, regulations applied, computed outcomes, validation, approvals, finalisation and published event facts.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Complete Final Approved Marks were consumed from I08.
- Applicable academic regulations were resolved and applied.
- Subject, credit, grade, SGPA/CGPA, programme, classification and academic-standing outcomes were computed.
- The result passed source-defined validation.
- The required institutional approval path completed.
- The result was finalised and made ready for I10 publication.
- Applicable I09 events and immutable evidence were preserved.

## 16.2 Postconditions

- The Institutionally Approved Result is authoritative under I09 ownership.
- I08 Final Approved Marks remain unchanged.
- I10 may publish the approved result without recomputation.
- Student Lifecycle and Academic Delivery may consume the result within their own canonical boundaries.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No L3-local architecture issue prevents this scenario from expressing the frozen outcome. L2-I09 defines canonical actors and institutional approval but allows the exact institutional approval sequence to be configured; this scenario therefore does not hard-code one universal sequence.

## 17.2 Assumptions

None.

## 17.3 Deferred Questions

None.

**Architecture assessment:** `PASS`
