# Course Offering Enrollment Eligibility and Restriction Governance

**Scenario ID:** L3-D-006  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Course & Course Offering  
**Participating Domains:** C, D  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D01_Course_and_Course_Offering_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0726 / CAN-L3-D-006

---

# 2. Scenario Purpose

Govern the Course Offering references and availability conditions used when Section C evaluates whether a Student may register, while retaining the underlying eligibility and restriction rules with their authoritative owners.

# 3. Business Outcome

The Course Offering has a governed, effective eligibility-and-restriction reference position available to Section C without D01 owning Student eligibility, restriction logic or registration transactions.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized requirement exists to establish or change the eligibility and restriction references governing enrollment availability for a Course Offering.

## 4.2 End Boundary

The governed eligibility-and-restriction reference position becomes effective and is available to Section C; no Student registration is decided in this scenario.

## 4.3 In Scope

- Govern Programme, period, prerequisite, corequisite, Department, policy and restriction references applicable to the offering.
- Preserve rule ownership outside the Course Offering aggregate.
- Make the governed reference position available to Section C.

## 4.4 Out of Scope

- Student-specific eligibility evaluation and registration decision.
- Creation or change of curriculum rules, Student holds, financial restrictions or compliance restrictions.
- Student registration creation, modification or cancellation.

## 4.5 Related Scenarios — Optional

No related-scenario cross-reference is required to define this scenario boundary.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Course & Course Offering (D01) | Owns eligibility references and offering-level enrollment availability position. |
| Section C — Student Academic Registration | Owns Student-specific eligibility evaluation and registration transactions. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Authorized Course Offering administrator | Maintains offering-level eligibility references and restrictions. | Initiates / performs. |
| Course Offering Governance Authority | Approves material eligibility or restriction changes where policy requires governance review. | Decides. |
| Student Academic Registration authority | Evaluates student-specific eligibility and determines the registration transaction. | Decides within Section C. |

D01 governs offering-level configuration only; Section C exclusively owns student-specific eligibility evaluation and registration decisions.

# 7. Preconditions and Entry Criteria

- The Course Offering exists and is not Archived.
- Authoritative eligibility and restriction rules exist with their owning domains.
- Section C registration ownership is preserved.

# 8. Trigger

An authorized requirement is raised to establish or change Course Offering eligibility or restriction references.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the authoritative eligibility and restriction rules applicable to the offering. | D01 / rule-owning domains | Only canonical rule references enter governance. |
| 2 | Confirm D01 will retain references only and will not own or rewrite rule logic. | D01 | Rule ownership remains upstream. |
| 3 | Evaluate the proposed reference position against offering context and effective dates. | D01 | A governed reference position can be decided. |
| 4 | Make the authorized reference position effective for the Course Offering. | D01 | Offering-level governance is established. |
| 5 | Make the governed position available for Section C Student-specific evaluation. | D01 / Section C | Section C may evaluate registration eligibility without ownership transfer. |

# 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Referenced rule or restriction is not authoritative | Governance cannot safely proceed. | Reject or return the proposal until authoritative ownership is resolved. | D01 |
| Offering is Archived | The offering is immutable. | Reject the governance change. | D01 |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| The Course Offering aggregate owns only eligibility references. | D01 | Prevents D01 from owning Student eligibility logic. | L2-D01 §33 |
| Eligibility and restriction rules remain external to the Course Offering aggregate. | Rule-owning domains | Preserves upstream rule ownership. | L2-D01 §§33,40 |
| Section C is sole owner of academic registration transactions. | Section C | Prevents D01 from deciding or recording registration. | L2-C03 §§5.6,13.7 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Course Offering eligibility configuration | Current offering-level eligibility and restriction references | Replace the governed configuration while retaining its effective position and decision evidence; Course Offering lifecycle state is unchanged. | Effective offering-level eligibility configuration | D01 | L2-D01 §§§38–40; accepted Batch 1 decision 5 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

No separate canonical business event is required for this governed configuration outcome; completion is evidenced by the effective business decision and retained audit record.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Rule-owning domains | D01 | Authoritative eligibility and restriction rules | Allow D01 to govern references without re-owning rules. | L2-D01 §§33,40 |
| D01 | Section C | Governed offering eligibility/restriction references | Support Student-specific registration eligibility evaluation. | L2-D01 §§33,40; L2-C03 §§7.4,9.2 |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Maintain offering-level eligibility configuration | Authorized Course Offering administrator | May change only D01-owned offering configuration. | D01 |
| Approve material configuration change where required | Course Offering Governance Authority | Applies institutional governance without deciding individual student eligibility. | D01 |
| Evaluate student-specific eligibility and registration | Student Academic Registration authority | Uses authoritative D01 configuration but owns the individual outcome. | Section C |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, override or exception authority is established by the authoritative L2 architecture beyond the authorization and policy constraints stated above.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Reference-governance decision | Prior and successor references, effective period, source ownership and decision. | D01 |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Only authoritative rules are referenced.
- The governed reference position is effective.
- Section C can consume it without D01 deciding Student registration.

## 16.2 Postconditions

- D01 owns the offering references only.
- Section C owns Student-specific eligibility and registration.
- Rule logic remains with each canonical rule owner.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open issues.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**Draft Validation Verdict:** PASS  
**Validation Basis:** `Section_D_Canonical_Register.md` and `L2-D01_Course_and_Course_Offering_Workflows.md`; Template v2.0; Architecture Level Validation Standards; accepted Section D decisions 001–500; accepted Batch 1 resolution decisions 1–7 (2026-08-20).
