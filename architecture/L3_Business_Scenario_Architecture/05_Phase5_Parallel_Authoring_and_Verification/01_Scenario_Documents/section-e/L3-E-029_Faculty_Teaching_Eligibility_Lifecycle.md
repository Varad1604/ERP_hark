# Faculty Teaching Eligibility Lifecycle

**Scenario ID:** L3-E-029  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Faculty Workforce  
**Participating Domains:** B, D, E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Author:** Varad  
**Version:** 1.0  
**Date:** 2026-08-20  
**Depends On:** L2-E03 — Faculty Workforce Workflows; applicable academic requirement and organisational context

## 1. Scenario Metadata
Frozen Stage 3 scenario identity, title, canonical owner, participating domains, Scenario Type and Priority are preserved. This scenario applies locked upstream architecture and does not redesign ownership.

## 2. Scenario Purpose
Determine and govern whether a Faculty Workforce member is approved to perform a specific teaching activity, using the authoritative capability, qualification and academic requirement context.

## 3. Business Outcome
The Faculty Teaching Eligibility reaches an authoritative Approved, Rejected, or applicable Expired outcome, with the decision historically preserved and the appropriate canonical event published.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
A teaching activity requires a governed eligibility decision for a Faculty Workforce member.

### 4.2 End Boundary
The eligibility decision is authoritative for its applicable validity period and downstream academic consumers can rely on the published business fact.

### 4.3 In Scope
- Evaluate faculty capability.
- Evaluate relevant qualification association and verification.
- Evaluate academic requirements.
- Conduct eligibility review.
- Approve or reject eligibility.
- Preserve eligibility validity and historical decisions.

### 4.4 Out of Scope
Implementation details, APIs, databases, services, UI, infrastructure, technical access control/IAM, queues, technical retries and other L4+ concerns are outside this L3 scenario.

## 5. Ownership and Participating Domains
**Canonical Owner:** Faculty Workforce.

Participating domains remain authoritative for their own concepts. Participation in this scenario does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities
- Faculty Workforce owns Teaching Eligibility.
- Academic requirements provide the criteria used by the eligibility decision.
- Organisational context may participate without acquiring ownership of the eligibility lifecycle.

No unsupported technical role is introduced.

## 7. Preconditions and Entry Criteria
- Faculty Workforce is Active.
- Relevant capability information exists.
- Applicable qualification evidence is available.
- Academic requirements are identifiable.

## 8. Trigger
An authorised request or academic requirement requires teaching eligibility to be evaluated.

## 9. Main Business Flow
1. Identify the Faculty Workforce and teaching activity.
2. Gather authoritative faculty capability information.
3. Evaluate relevant qualification association and verification.
4. Evaluate the applicable academic requirements.
5. Move the eligibility request to **UnderReview**.
6. If requirements are met, transition to **Approved**.
7. If requirements fail, transition to **Rejected**.
8. Record the validity period and preserve the historical decision.
9. Publish `FacultyEligibilityApproved` or `FacultyEligibilityRejected`, as applicable.
10. Stop when the eligibility outcome is authoritative.

The main flow stops when the institutional Business Outcome and End Boundary are reached.

## 10. Alternate Business Flows
- Requested → UnderReview begins evaluation.
- UnderReview → Approved when requirements are met.
- UnderReview → Rejected when requirements fail.
- Approved → Expired when validity ends.
- An expired qualification triggers eligibility review where applicable; expired qualifications cannot automatically grant eligibility.
- A non-Active Faculty Workforce cannot retain operational teaching eligibility.

## 11. Failure, Interruption and Business Recovery
If requirements fail, eligibility remains Rejected and the historical decision is preserved. If an approved eligibility reaches expiry, it is no longer the current approved outcome and must follow the applicable renewal/re-evaluation path. No technical retry or rollback is described.

Only business-level recovery is defined.

## 12. Business Rules and Decision Constraints
- Teaching Eligibility is distinct from Faculty Capability.
- Eligibility decisions comply with academic requirements.
- Qualification verification is considered in eligibility evaluation.
- Expired qualifications cannot automatically grant eligibility.
- Previous eligibility decisions remain available for compliance and accreditation.
- Eligibility is not operational for a non-Active Faculty Workforce.

The rules remain owned by the authoritative L2 architecture.

## 13. State and Lifecycle Impact
**Teaching Eligibility:** Requested → UnderReview → Approved or Rejected.

**Validity:** Approved → Expired when validity ends.

No new lifecycle state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Business Events
| Event | Canonical Publisher | Business Meaning |
|---|---|---|
| `FacultyEligibilityApproved` | Faculty Workforce | Teaching eligibility approved. |
| `FacultyEligibilityRejected` | Faculty Workforce | Teaching eligibility rejected. |

### 14.2 Business Contracts / Hand-offs
Capability, qualification association/verification and academic requirements are inputs to the eligibility decision. Faculty Workforce remains the canonical owner of Teaching Eligibility.

Events remain distinct from synchronous business contracts and hand-offs.

## 15. Business Authorization, Approval and Audit Obligations
Eligibility approval follows the institutional approval model. Approval/rejection decisions capture the required decision, actor and reason; no unsupported named approver is invented.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
- Eligibility is evaluated against authoritative inputs.
- A valid Approved or Rejected outcome is established.
- The outcome's validity is respected.
- Historical decisions remain preserved.
- The applicable eligibility event is published.

### 16.2 Postconditions
The resulting business outcome is authoritative for subsequent institutional use and required history remains preserved.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
None identified.

### 17.2 Assumptions
None.

### 17.3 Deferred Questions
None.

**Authoring Classification:** PASS — source-grounded Draft.
