# Supplier Remediation to Restored Eligibility

# 1. Scenario Metadata

**Scenario ID:** L3-J-032  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Vendor & Supplier Management  
**Participating Domains:** J  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** supplier qualification, sourcing, award, commitment, monitoring, performance, restriction, suspension, blacklisting, inactivity, and audit scenarios  

---
# 2. Scenario Purpose

This scenario governs evaluation of supplier remediation and the controlled restoration of eligibility after an approved restriction, suspension, expired or deficient qualification, or other reversible supplier governance condition. It prevents restoration based only on supplier assertion, document submission, elapsed time, or administrative override.

Restoration is prospective, scoped, independently verified, and attributable. It does not erase the original finding, restriction, suspension, blacklisting decision, expiry gap, breach, or transaction history.

---

# 3. Business Outcome

Supplier remediation reaches a verified decision to restore, conditionally restore, partially restore, reject, or defer eligibility, with defined scope, effective time, residual controls, monitoring, qualification requirements, authority, downstream publication, and immutable linkage to the original governance condition.

Outcome labels in this section are business dispositions or supporting-record results, not additional canonical lifecycle states. Section 13 states the only canonical lifecycle impact.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a supplier submits remediation evidence or a review becomes due for a reversible restriction, suspension, qualification deficiency, or legally restorable blacklisting position.

## 4.2 End Boundary

The scenario ends when remediation is rejected, returned, verified, or partially accepted; an authorised restoration decision becomes effective for a defined scope; current consumers are notified; residual controls and monitoring are established; and original history remains preserved.

## 4.3 In Scope

- validating the supplier, original condition, permitted restoration route, remediation plan, evidence, completion, timing, and scope;
- independently verifying corrective action, root-cause control, legal or regulatory compliance, qualification evidence, capability, ownership, security, safety, financial, ethical, and performance conditions;
- determining full, partial, conditional, staged, or no restoration;
- defining effective scope, residual restriction, probation, monitoring, review, and requalification conditions;
- coordinating release of restrictions, lifting of suspension, restored qualification, or restoration after blacklisting only where authorised;
- publishing current eligibility without deleting prior adverse states.

## 4.4 Out of Scope

- rewriting or overturning the original decision without authorised review;
- automatic restoration merely because a duration expired;
- creating new supplier registration or unrelated qualification;
- reinstating where law, external debarment, permanent blacklisting, sanctions, or another mandatory prohibition remains effective;
- reopening or recreating previous sourcing, award, Purchase Order, contract, receipt, payable, accounting, or payment positions;
- technical verification, workflow, monitoring, or rule-engine implementation.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-J-023 to L3-J-031 | Eligibility condition, restriction, suspension, or blacklisting created the restoration need. |
| Related | L3-J-021 and L3-J-022 | Active status and qualification may need current confirmation. |
| Followed by | Sourcing and commitment scenarios | Restored eligibility is only a prospective input. |
| Related | L3-J-033 | A supplier may remain inactive despite restored eligibility. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Enterprise Resource Management - Vendor & Supplier Management | Owns remediation intake, verification coordination, restoration decision, scope publication, monitoring, and history. |

Finding, legal, compliance, finance, safety, security, qualification, contract, and regulatory owners retain their evidence and mandatory clearance authority.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Supplier Representative | Receives notice and supplies permitted response or remediation evidence; cannot remove institutional restrictions. | Responds. |
| Institutional Verifier | Independently verifies evidence material to the governance decision. | Verifies evidence. |
| Supplier Governance Authority | Proposes or approves restriction, suspension, blacklisting, reactivation, inactivation, closure or merge treatment. | Decides governed supplier treatment. |
---

# 7. Preconditions and Entry Criteria

- The canonical supplier and original condition are identifiable.
- The original decision permits review or restoration, or a competent authority has authorised reconsideration.
- Required remediation, evidence, qualification, clearance, verification, review, and approval rules are defined.
- Mandatory prohibitions, external debarment, sanctions, permanent exclusions, and related proceedings can be checked.
- The verifier and restoration authority can act independently and manage conflicts.
- Current supplier profile, ownership, qualification, restrictions, suspension, blacklisting, and inactivity positions are available.

---

# 8. Trigger

The supplier claims completion of remediation, a restriction or suspension review becomes due, a fixed period expires, or an authorised authority requests restoration assessment.

---

# 9. Main Business Flow

1. The supplier or authorised owner opens a restoration case referencing the canonical supplier, original decision, affected scope, remediation requirements, and claimed completion.
2. The Institutional Verifier verifies that a restoration route exists and identifies every controlling restriction, suspension, qualification deficiency, blacklisting decision, external prohibition, and review condition.
3. Supplier identity, ownership, related entities, representatives, sensitive profile changes, qualifications, licences, certificates, insurance, tax, banking references, and current declarations are reconciled as applicable.
4. Submitted evidence is validated for issuer, authority, authenticity, period, scope, completeness, custody, and relation to each exit condition.
5. The institution distinguishes corrective action planned, implemented, operating, independently tested, and proven sustainable.
6. Independent verifiers examine root cause, control design, operating effectiveness, recurrence, outstanding incidents, complaints, obligations, investigations, and residual risk.
7. Specialist owners confirm all mandatory clearances; unresolved legal, regulatory, sanctions, safety, security, tax, qualification, or external debarment conditions block incompatible restoration.
8. The original finding and decision owners confirm whether stated exit, restitution, cooperation, monitoring, time, training, governance, capability, or evidence conditions are satisfied.
9. Conflicts involving supplier, verifier, contract owner, original decision-maker, or restoration authority are declared and mitigated.
10. The recommendation distinguishes full restoration, partial scope restoration, conditional or probationary restoration, deferral for evidence, and rejection.
11. Proposed effective time, eligible scope, continuing restrictions, renewed qualifications, monitoring, transaction limits, review date, and breach consequences are defined.
12. The Supplier Governance Authority reviews evidence sufficiency, independence, mandatory clearances, original decision, supplier history, residual risk, consistency, and proportionality.
13. The authority approves, partially approves, conditionally approves, rejects, returns, or defers the request with reasons.
14. An approved restoration creates a prospective authoritative eligibility state linked to, but not replacing, all prior findings and decisions.
15. Registration, qualification, sourcing, award, commitment, contract, finance, compliance, safety, security, and audit consumers receive the minimum current status needed.
16. Previous sourcing, award, contract, receipt, payable, accounting, or payment outcomes are not reopened automatically; new activity begins only after the effective restoration time.
17. Evidence, verification, clearances, decisions, conditions, monitoring, breaches, reviews, actors, timestamps, and versions remain auditable.

---

# 10. Alternate Business Flows

## 10.1 Partial Remediation

Only independently verified scope is restored. Remaining categories, sites, entities, value bands, or activities stay restricted or suspended.

## 10.2 Conditional or Probationary Restoration

Future use is allowed under explicit monitoring, limits, additional approvals, review dates, and automatic referral conditions. It is not equivalent to unrestricted eligibility.

## 10.3 Fixed Period Expired but Conditions Unmet

Elapsed time creates review eligibility only. The adverse status continues or transitions as the original rule provides until authorised restoration.

## 10.4 External Prohibition Remains Active

Internal remediation may be recorded, but incompatible eligibility is not restored until authoritative external clearance exists.

## 10.5 Permanent or Non-Restorable Blacklisting

The request is rejected or routed to the legally competent reconsideration process. Ordinary remediation cannot override the decision.

## 10.6 New Adverse Evidence

The restoration case is held or rejected and the new finding is referred under L3-J-028 to L3-J-031 as appropriate.

## 10.7 Restored but Inactive Supplier

Eligibility may be restored while relationship state remains inactive under L3-J-033. Reactivation requirements remain separate.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Fraudulent or Unverifiable Remediation Evidence

Restoration is blocked and the evidence is preserved for new governance review.

## 11.2 Verification Incomplete

The case is returned or deferred. Submission, inspection, or partial completion does not equal restoration.

## 11.3 Conflicted Verifier or Approver

The person is replaced or independently reviewed; prior work remains visible.

## 11.4 Incorrect Early Restoration

The status is corrected prospectively under authority, affected activity is reviewed, and the error and impact remain auditable.

## 11.5 Publication Failure

The decision is reissued and current status reconciled. Eligibility does not become usable before reliable authoritative publication where required.

## 11.6 Condition Breach After Restoration

The breach is recorded and referred for restriction or suspension review; status changes only under applicable authority.

## 11.7 System Interruption

The authoritative effective status remains controlling and is reconciled on recovery without backdating.

---

# 12. Business Rules and Decision Constraints

- Remediation evidence, verification, decision, and restored eligibility are distinct.
- No restoration occurs without canonical linkage, original-condition review, independent verification, mandatory clearances, competent authority, reasons, scope, and effective time.
- Expiry of a restriction, suspension, or blacklisting period does not automatically prove eligibility unless the governing decision expressly provides it.
- Mandatory legal, sanctions, regulatory, safety, security, qualification, or external prohibition cannot be overridden by ordinary restoration authority.
- Restoration is no broader than verified remediation and current qualification.
- Conditional restoration must state limits, monitoring, review, and breach consequences.
- Prior findings, restrictions, suspension, blacklisting, expiry gaps, appeals, and breaches remain immutable historical facts.
- Restoration does not recreate or reopen earlier sourcing, awards, contracts, receipts, payables, accounting entries, or payments.
- Restored eligibility does not guarantee invitation, award, commitment, or payment.
- Inactivity and closure remain separate lifecycle states.

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Supplier | Existing canonical J02 state | Applies only an exact J02 supplier-governance transition. | `Restricted`, `Suspended`, `Blacklisted`, `Active`, `Inactive` or `Closed / Archived` as material to the scenario | Vendor & Supplier Management (J02) | `L2-J02` Section 6 |
| SupplierGovernanceCase | No canonical lifecycle defined | Preserves evidence and the attributable governance decision without creating local case states. | Governed case record linked to the exact Supplier state | Vendor & Supplier Management (J02) | `L2-J02` Section 5 |
---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Material Consumer(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `SupplierReactivated` | Vendor & Supplier Management (J02) | J01, J03, J04, J05, Finance and Reporting where material | Records the exact completed business fact material to this scenario. | `L2-J02 Section 13` |
## 14.2 Cross-Domain Collaboration

| Participant | Collaboration | Boundary |
|---|---|---|
| L3-J-023 to L3-J-031 | Supplies original condition, findings, decisions, and exit requirements. | Original history is not rewritten. |
| Legal / Compliance / Specialist Owners | Provide mandatory clearance and verification. | Retain specialist authority. |
| Qualification / Sourcing | Consume current restored scope. | Retain qualification and transaction decisions. |
| Contract / Finance | Reviews continuing obligations and monetary positions. | Restoration does not reopen them. |
| L3-J-033 | Governs inactive relationship status. | Eligibility and activity remain distinct. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J02 roles are separated as policy requires: Supplier Representative; Institutional Verifier; and Supplier Governance Authority. The Supplier Representative cannot verify, approve, or remove an institutional restriction on itself, and a conflicted verifier cannot decide the governed outcome.

## 15.2 Required Evidence

- canonical supplier, original findings and decisions, affected scope, duration, exit conditions, prior breaches, appeals, and current external prohibitions;
- remediation plan, completion evidence, issuer validation, inspections, tests, root-cause correction, operating effectiveness, specialist clearances, conflicts, and residual risk;
- recommendation, approval, reasons, restored and continuing scope, effective time, conditions, monitoring, reviews, breaches, actors, timestamps, and versions.

## 15.3 Audit Obligations

The institution must reconstruct what condition required remediation, what evidence was submitted, who verified it and how, which mandatory clearances existed, why restoration scope and conditions were selected, who approved it, when it became effective, and whether later conditions were satisfied or breached.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- the supplier has completed the required remediation actions as confirmed by authoritative evidence from an acceptable source;
- a restoration decision has been made by an authority appropriate to the original finding type, severity, and institutional risk;
- the remediation evidence, completion confirmation, conditions for restoration, and any ongoing monitoring obligations are documented;
- the restoration scope and any residual conditions are explicit and attributable;
- the restoration decision, evidence, authority, actors, and audit trail are preserved and immutable;
- the supplier's prior finding, restriction, or suspension evidence is preserved and not overwritten by the restoration.


## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- the supplier's eligibility status is restored for the defined scope from the effective date of the restoration decision;
- the prior finding, restriction, or suspension evidence is permanently preserved as historical record;
- the restoration is attributable to specific completed remediation actions and an institutional authority decision;
- any ongoing monitoring obligations, conditions, or reporting requirements are explicitly recorded;
- the complete remediation history, restoration decision, and conditions are permanently auditable.


---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. Remediation evidence standards, independent-verification requirements, restoration authorities, probation periods, and monitoring thresholds require confirmation.
2. Restoration after fixed-term, indefinite, permanent, internal, and external blacklisting requires institution-specific legal mapping.
3. Qualification revalidation and profile-change checks required before restoration require an upstream J02 restoration contract and business rule.

## 17.2 Assumptions

- Restoration is prospective and never erases adverse history.
- Eligibility restoration does not guarantee procurement activity.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-032**
