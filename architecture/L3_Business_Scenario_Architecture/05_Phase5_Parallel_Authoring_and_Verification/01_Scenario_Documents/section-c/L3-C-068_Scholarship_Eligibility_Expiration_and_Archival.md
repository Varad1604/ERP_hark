# Scholarship Eligibility Expiration and Archival

**Scenario ID:** L3-C-068  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Scholarship Eligibility  
**Participating Domains:** C  
**Scenario Type:** Critical  
**Priority:** P2  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-C07 — Scholarship Eligibility Workflows; applicable authoritative upstream Student Lifecycle, Person & Identity, academic, document, institutional-policy and scholarship-policy sources  
**Referenced By:** Related canonical scenarios listed in Section 4.5

## 1. Scenario Metadata
The frozen Section C identity is preserved. Scholarship Eligibility remains the canonical owner of the Student Scholarship Eligibility Record and eligibility lifecycle.

## 2. Scenario Purpose
Conclude the active applicability of an expired Student Scholarship Eligibility Record and preserve its historical eligibility record for institutional traceability.

## 3. Business Outcome
The eligibility record is no longer treated as an active applicable eligibility position after its governed validity boundary, while its historical decision and evidence remain preserved according to the canonical C07 lifecycle.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
An eligibility record reaches its authoritative validity/expiration boundary.

### 4.2 End Boundary
The record reaches its canonical expired/archived business position and remains historically traceable.

### 4.3 In Scope
- Identify the eligibility record and its governed validity boundary.
- Determine that the eligibility period has expired under the effective scholarship policy.
- Apply the canonical expiration/archival lifecycle decision.
- Preserve historical eligibility information.
- Publish the canonical eligibility lifecycle fact.
- Make the expired position available for historical/audit purposes without treating it as current eligibility.

### 4.4 Out of Scope
- Conditional eligibility resolution/expiry (C065) when the record is specifically conditional.
- Reassessment of a still-applicable record (C066).
- Renewal evaluation (C067).
- Financial settlement or accounting.

### 4.5 Related Scenarios
C065 — Conditional Scholarship Eligibility Resolution or Expiry; C066 — Scholarship Eligibility Reassessment; C067 — Scholarship Renewal Evaluation.

## 5. Ownership and Participating Domains
Scholarship Eligibility owns the eligibility decision and Student Scholarship Eligibility Record. Upstream domains retain ownership of facts consumed for evaluation. Downstream Finance owns financial approval, allocation, disbursement, settlement and accounting; it does not re-evaluate C07 eligibility. This ownership split is explicitly established in locked L2-C07.

## 6. Business Actors, Roles and Responsibilities
The detailed authoritative actor catalogue is defined upstream in L2-C07. This scenario does not invent role names. Business participants are limited to the scholarship-eligibility business responsibility and authoritative upstream information owners as applicable.

## 7. Preconditions and Entry Criteria
- Eligibility record exists.
- Its validity boundary is authoritative.
- Expiration/archival is permitted under C07 policy.
- Historical preservation obligations apply.

## 8. Trigger
An eligibility record reaches its governed validity/expiration boundary.

## 9. Main Business Flow
1. Identify the eligibility record.
2. Confirm the effective validity boundary.
3. Determine that the eligibility position is no longer active under policy.
4. Apply the canonical expiration/archival lifecycle decision.
5. Preserve the complete historical decision record.
6. Publish the canonical lifecycle fact.
7. Ensure the expired position is not represented as current eligibility.

## 10. Alternate Business Flows
- If the record is renewed before expiration, C067 governs the renewal outcome rather than archival.
- If the record requires reassessment before expiry, C066 governs that reassessment.
- A conditional record reaching its condition-specific boundary follows C065 where applicable.

## 11. Failure, Interruption and Business Recovery
An expiration decision is not a deletion of historical eligibility. If the validity boundary is disputed or incorrectly established upstream, the record remains governed by the applicable correction/reassessment process; no local technical recovery is introduced.

Recovery is business-level only; technical retries, queues, rollback and infrastructure behaviour are outside L3.

## 12. Business Rules and Decision Constraints
C07 explicitly owns eligibility validity, expiration and historical traceability. Expiration must not erase the historical eligibility decision. Exact retention/archival policy is upstream and is not invented here.

## 13. State and Lifecycle Impact
The Student Scholarship Eligibility Record moves into the exact expired/archived position defined by L2-C07. No new local state is created.

No local lifecycle state is invented.

## 14. Business Events and Cross-Domain Collaboration
Exact expiration/archival event name and publisher must be taken from locked L2-C07. The accessible source confirms domain event publication but does not expose the detailed event catalogue sufficiently to verify the exact name.

Only canonical L2-C07 events/publishers may be used. No event name is fabricated where the detailed upstream catalogue is not exposed.

## 15. Business Authorization, Approval and Audit Obligations
Expiration/archival must be governed, auditable and consistent with the applicable scholarship policy and record-history obligations.

## 16. Completion Criteria and Postconditions
- Validity boundary confirmed.
- Active eligibility ceases.
- Historical record preserved.
- Canonical expiration/archival fact published.
- Record is available only as historical context, not active eligibility.

## 17. Open Issues, Assumptions and Deferred Questions
**OI-C068-01:** L2-C07 explicitly covers eligibility validity, expiration and historical versioning, but exact archival state/event definitions and retention obligations require detailed C07 verification.

**Classification: PASS WITH ISSUES** for authoring; **not lock-ready** until exact upstream state/event vocabulary is verified.

## Production Verification Record
- [x] Frozen ID/title/owner/domains/type/priority preserved.
- [x] L2-C07 ownership boundary preserved.
- [x] Financial processing remains outside C07.
- [x] No implementation leakage.
- [x] All 17 mandatory sections present.
- [x] Upstream gaps are explicitly recorded.
- [x] Status remains Draft under Phase 5 governance.
