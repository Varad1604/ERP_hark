# Student Credit Transfer and Recognition

**Scenario ID:** L3-C-032  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Progression  
**Participating Domains:** C, H  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-C04_Student_Academic_Progression_Workflows.md`; `L2-H10_External_Education_Integration_Workflows.md` where external-provider orchestration is involved; applicable Curriculum Framework and Academic Registration architecture  
**Referenced By:** L3-C-028 Academic Credit Ledger Update; L3-C-036 Qualification Eligibility Determination

## 1. Scenario Metadata

Frozen identity: **C032 — Student Credit Transfer and Recognition**; Primary Owner **Academic Progression**; Participating Domains **C, H**; Type **Critical**; Priority **P1**; Derivation **KEEP**.

---

## 2. Scenario Purpose

To determine whether academic credits obtained outside the student's current programme may be institutionally recognised and, when approved, incorporate the recognised credits into the authoritative Academic Credit Ledger.

Academic Progression owns the academic equivalency, recognition decision and resulting ledger update. External Education Integration may coordinate external-provider requests and evidence, but does not make the academic recognition decision.

---

## 3. Business Outcome

The credit-transfer request is authoritatively **Approved**, **Rejected**, or **Revision Required**.

For an Approved request, the recognised credit is recorded in the Academic Credit Ledger and the authoritative recognition outcome is published.

A Rejected request creates no recognised-credit ledger entry.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a valid credit-transfer request and its available supporting academic evidence are submitted for institutional recognition.

### 4.2 End Boundary

The scenario ends when Academic Progression establishes the recognition outcome and, for approval, the recognised credit is recorded in the Academic Credit Ledger.

### 4.3 In Scope

- Receipt and verification of transfer-credit evidence.
- Curriculum/equivalency assessment.
- Institutional academic approval or rejection.
- Recognition of approved credits.
- Academic Credit Ledger update.
- Publication of the canonical recognition outcome.
- External hand-off through H10 where applicable.

### 4.4 Out of Scope

- External-provider connector operation.
- External communication mechanics.
- Creation or correction of examination outcomes.
- Curriculum ownership or curriculum redesign.
- Registration of courses after recognition.
- ABC synchronization beyond the resulting ledger outcome.

### 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Followed by | L3-C-028 Academic Credit Ledger Update | Recognition contributes to the authoritative credit ledger. |
| Related | L3-C-036 Qualification Eligibility Determination | Recognised credits may contribute to qualification eligibility. |

---

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Academic Progression (C) | Owns academic credit recognition, equivalency determination, approval/rejection and recognised-credit recording. |
| External Education Integration (H) | Coordinates external request/provider verification and supporting-document exchange where the source is external. It does not decide academic recognition. |

The L2-C04 ownership boundary explicitly assigns academic evaluation, equivalency, institutional approval/rejection, recognised-credit recording and Academic Credit Ledger update to Academic Progression. H10 remains an orchestration boundary.

---

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Participation / Authority |
|---|---|---|
| Academic Progression | Evaluates transferred credits and establishes recognition outcome. | Canonical academic decision authority. |
| External Education Integration | Coordinates external-provider verification and follow-up where applicable. | Integration/orchestration participant; not academic decision authority. |

No additional approval role is created because the authoritative source does not define a universal named role for this scenario.

---

## 7. Preconditions and Entry Criteria

- A Student is eligible to have academic credits considered under institutional policy.
- A credit-transfer request exists.
- Supporting academic evidence is available or its deficiency can be determined.
- The applicable curriculum/equivalency rules are authoritative.
- Where external education is involved, H10 has completed the provider-side verification required before the academic decision.

---

## 8. Trigger

**A credit-transfer request requiring institutional academic recognition is submitted.**

For externally sourced requests, H10 may submit the verified request to Academic Progression.

---

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Receive the transfer request and supporting academic evidence. | Academic Progression / H10 | Recognition case is established. |
| 2 | Verify the academic evidence and source-credit validity. | Academic Progression | Evidence is suitable for academic review. |
| 3 | Map the source learning/credits against the applicable curriculum and equivalency requirements. | Academic Progression | Equivalency position is established. |
| 4 | Establish the institutional recognition decision. | Academic Progression | Approved, Rejected or Revision Required outcome is determined. |
| 5 | Where Approved, record the recognised credit in the Academic Credit Ledger. | Academic Progression | Authoritative recognised credit exists. |
| 6 | Publish the corresponding canonical outcome. | Academic Progression | Downstream consumers receive the authoritative recognition fact. |

---

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result | Responsible Domain |
|---|---|---|---|---|
| A1 | Academic evidence or mapping information is insufficient | Request/coordinate additional evidence and defer final recognition. | Revision Required. | Academic Progression / H10 |
| A2 | Credits fail equivalency or institutional recognition rules | Reject the transfer request. | No ledger recognition. | Academic Progression |
| A3 | External source requires provider follow-up | H10 coordinates external follow-up without deciding academic equivalency. | Case returns for academic decision. | H10 / Academic Progression |

---

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Supporting academic evidence is incomplete | Recognition cannot be finalized. | Obtain the required evidence and return to governed academic review. | Academic Progression / H10 |
| Credits are not equivalent to the applicable curriculum | Recognition is not permitted. | Establish Rejected outcome. | Academic Progression |
| External provider verification is incomplete | Academic decision lacks required external evidence. | H10 coordinates provider follow-up; Academic Progression remains decision owner. | H10 |
| A previously rejected request is resubmitted with new evidence | Prior rejection must not be silently overwritten. | Evaluate the new request as a governed recognition decision while preserving historical outcome. | Academic Progression |

No technical retry, queue or rollback behaviour is defined.

---

## 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| **BR-CTR-001** Credit Transfer requires institutional verification. | Academic Progression | Recognition cannot be granted without verification. | L2-C04 §6.9 |
| **BR-CTR-002** Transferred credits must satisfy curriculum equivalency requirements. | Academic Progression | Equivalency is required before recognition. | L2-C04 §6.9 |
| **BR-CTR-003** Previously rejected credit transfers shall not update the Credit Ledger. | Academic Progression | Rejection cannot create recognised credit. | L2-C04 §6.9 |
| **BR-CTR-004** ABC-imported credits remain subject to institutional recognition policies. | Academic Progression | External/ABC origin does not bypass recognition. | L2-C04 §6.9 |
| **BR-CRD-003** Duplicate academic credits shall never be recognised. | Academic Progression | Duplicate recognition is prohibited. | L2-C04 §6.4 |

---

## 13. State and Lifecycle Impact

| Owned Concept / Aggregate | Entry Position | Scenario Impact | Result | Canonical Owner | Source |
|---|---|---|---|---|---|
| Credit Transfer | Transfer request under review | Equivalency and recognition evaluation | Approved / Rejected / Revision Required | Credit Transfer within Academic Progression | L2-C04 APW-07 |
| Academic Credit Ledger | Existing ledger | Approved recognition adds recognised credit | Updated Credit Ledger | Academic Credit Ledger | L2-C04 APW-03 / APW-07 |

No new lifecycle state is created by this L3 scenario.

---

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers / Interested Domains | Purpose | Source |
|---|---|---|---|---|
| `ExternalCreditTransferSubmitted` | External Education Integration | Academic Progression | Starts academic recognition for an external request. | L2-C04 §7.3 / APW-07 |
| `CreditTransferRecognised` | Credit Transfer aggregate within Academic Progression | Academic Credit Ledger, Qualification Completion & Student Exit, H10 where synchronization is required | Communicates approved recognition. | L2-C04 §7.4–7.7 |
| `CreditTransferRejected` | Credit Transfer aggregate within Academic Progression | H10 where external synchronization is required; Audit/Reporting | Communicates academic rejection. | L2-C04 §7.4–7.7 |
| `CreditLedgerUpdated` | Academic Credit Ledger | Downstream consumers | Communicates the resulting ledger change. | L2-C04 §7.4–7.7 |

### 14.2 Synchronous / Business Hand-off

Where an external request is involved, H10 submits the verified request and supporting evidence to Academic Progression. H10 does not duplicate the academic decision.

---

## 15. Business Authorization, Approval and Audit Obligations

Credit recognition is an institutional academic decision and requires the approved institutional authority defined upstream.

The recognition outcome, academic basis, evidence and resulting ledger change must remain auditable.

No technical IAM/RBAC model is defined.

---

## 16. Completion Criteria and Postconditions

### Approved

- Equivalency is established.
- Institutional recognition is approved.
- Recognised credit is recorded in the Academic Credit Ledger.
- `CreditTransferRecognised` is published.
- Audit evidence is complete.

### Rejected

- Recognition is rejected under applicable rules.
- No recognised-credit ledger entry is created.
- `CreditTransferRejected` is published where applicable.
- Audit evidence is complete.

### Revision Required

- The academic decision is not finalized.
- Required evidence/mapping deficiency is identified.
- The case remains subject to governed follow-up.

---

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

**OI-C032-01 — Exact institutional equivalency criteria are not enumerated in C04.** L3 therefore applies the canonical rule without inventing local equivalency thresholds.

**OI-C032-02 — Exact academic approver role is not universally named.** No new role is introduced.

**OI-C032-03 — H10 is authoritative for external orchestration, but the exact external-provider hand-off contract remains an L4/L5 concern.**

### Assumptions

1. Academic Progression remains the sole owner of academic recognition.
2. H10 participates only for external-provider orchestration.
3. Approved recognition is recorded in the Academic Credit Ledger.

### Deferred Questions

- Institution-specific equivalency matrices.
- Exact approval authority.
- Detailed external hand-off contract.


## Production Verification

- [x] Frozen Stage 3 title, owner, participating domains, type and priority preserved.
- [x] Final L3 identity remains in Section C and filename is derived from canonical title.
- [x] L2-C04 Academic Progression is the primary governing workflow source.
- [x] Cross-domain dependencies preserve canonical ownership.
- [x] No new actors, authorities, rules, states, events or publishers have been invented.
- [x] Business-level recovery only; no technical retry/queue/rollback mechanics.
- [x] L3 remains implementation-independent.
- [x] All 17 locked template sections are present.
- [x] Material source gaps are recorded in Section 17.
