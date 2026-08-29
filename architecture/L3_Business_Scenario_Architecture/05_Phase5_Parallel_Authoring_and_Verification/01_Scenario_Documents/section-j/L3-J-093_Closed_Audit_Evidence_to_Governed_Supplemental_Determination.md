# Closed Audit Evidence to Governed Supplemental Determination

# 1. Scenario Metadata

**Scenario ID:** L3-J-093  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Physical Verification, Audit & Reconciliation  
**Participating Domains:** J  
**Scenario Type:** Critical  
**Priority:** P2  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J09_Physical_Verification_Audit_and_Reconciliation.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** audit history, inventory adjustment, loss or damage, asset finding, finance reconciliation, legal or disciplinary review, reporting correction, records retention, and oversight scenarios  

---
# 2. Scenario Purpose

This scenario governs new, corrected, recovered, or previously unavailable evidence received after an inventory audit has closed or reached a preserved incomplete outcome. It permits a reasoned supplemental determination while protecting the finality, chronology, and immutability of the original audit record.

Supplemental determination does not silently reopen, delete, overwrite, or backdate the original finding. Any required stock, asset, finance, J10, legal, disciplinary, or reporting consequence follows its own authorised successor process.

---

# 3. Business Outcome

Post-closure evidence reaches an approved decision to reject, retain without change, clarify, supplement, correct prospectively, refer, or formally reopen under separate authority, with evidence provenance, materiality, affected findings, downstream impact, reasons, authority, and immutable linkage to the original audit.

Outcome labels in this section are business dispositions or supporting-record results, not additional canonical lifecycle states. Section 13 states the only canonical lifecycle impact.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when credible evidence related to a closed, cancelled, incomplete, reconciled, or referred audit outcome is submitted or discovered after that outcome's effective closure.

## 4.2 End Boundary

The scenario ends when the evidence is rejected, accepted as non-material, appended as clarification, used for an approved supplemental determination, or referred for formal reopening or successor action; affected consumers are notified; and original and supplemental records remain linked.

## 4.3 In Scope

- receiving and preserving new documents, recovered stock, corrected source records, statements, external confirmations, surveillance, forensic evidence, movement records, valuation references, legal decisions, or downstream outcomes;
- validating provenance, authenticity, custody, timing, relevance, availability at original closure, and reason for late submission;
- assessing materiality to counts, variances, causes, responsibility, adjustment, loss, damage, asset, finance, reporting, legal, or other findings;
- obtaining original owner and affected-party response where appropriate;
- approving no change, clarification, supplemental finding, corrected successor determination, or formal reopening referral;
- tracking downstream corrections without rewriting historical decisions.

## 4.4 Out of Scope

- silently editing the closed audit or deleting original evidence;
- automatically reversing stock, asset, finance, J10, recovery, disciplinary, legal, or reporting outcomes;
- accepting unsupported late assertion as fact;
- bypassing appeal, legal, period, or competent-authority requirements;
- technical versioning, evidence ingestion, notification, or replay implementation.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-J-087 / L3-J-088 | Original audit has closed or preserved an incomplete outcome. |
| Related | L3-J-089 to L3-J-092 | Supplemental evidence may affect variance, adjustment, J10, or asset findings. |
| Followed by | Relevant successor consequence scenario | Each correction remains independently authorised. |
| Related | Finance, Legal and Records scenarios | Retain periods, appeals, holds, and authoritative consequences. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Enterprise Resource Management - Physical Verification, Audit & Reconciliation | Owns supplemental-evidence intake, validation, impact review, determination, notification, and history. |

Inventory, asset, finance, J10, legal, security, disciplinary, grant, records, and audit owners retain their authoritative decisions and correction authority.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Variance Investigator | Investigates and records attributable findings where required. | Investigates. |
| Reconciliation Officer | Determines the J09 reconciliation position, issues governed referrals and records returned outcomes. | Reconciles; does not execute J06, J08 or J10 state changes. |
| Audit Approver | Approves the J09 reconciliation or supplemental determination within established authority. | Decides the J09 outcome. |
| Audit Reviewer | Reviews evidence, separation and lineage without modifying the outcome. | Reviews evidence. |
| Inventory Custodian | Supplies custody and operational evidence without controlling the finding. | Supports and responds. |
---

# 7. Preconditions and Entry Criteria

- An identifiable governed audit outcome and immutable evidence set exist.
- New evidence can be linked to the audit, population, item, location, lot, serial, asset, custody, variance, finding, or referral.
- Supplemental evidence, authenticity, materiality, response, authority, reopening, correction, period, appeal, retention, legal-hold, and notification rules are effective.
- Original and downstream records can be read without being overwritten.
- Conflicts and sensitive evidence can be controlled.

---

# 8. Trigger

New, recovered, corrected, externally confirmed, or previously unavailable evidence may materially affect a governed audit outcome or its downstream consequences.

---

# 9. Main Business Flow

1. The submitter records original audit, affected item or finding, evidence source, discovery date, event date, custody, reason for late availability, claimed effect, and confidentiality.
2. The Audit Reviewer protects the evidence and validates submitter authority, authenticity, completeness, chronology, provenance, and relation to the closed audit.
3. Duplicate, previously considered, fabricated, altered, privileged, legally restricted, or unrelated material is identified without silent exclusion.
4. The original audit scope, cut-off, counts, evidence, limitations, findings, responses, adjustments, J10 referrals, asset findings, finance links, and closure decision are reconstructed.
5. The new evidence is classified as corroborating, contradictory, corrective, explanatory, recovered physical evidence, new subsequent event, external determination, or irrelevant.
6. Availability at original closure and any intentional concealment, negligence, system failure, delayed external source, or ordinary later event are distinguished.
7. Materiality to physical quantity, condition, location, ownership, custody, cut-off, cause, responsibility, adjustment, loss, asset, finance, grant, legal, reporting, or control findings is assessed.
8. Immediate safety, security, fraud, legal-hold, evidence-integrity, or continuing stock risk is referred under urgent authority.
9. Original finding owners, specialists, and affected parties receive sufficient material and response opportunity where lawful.
10. Conflicts and potential self-review are controlled through independent participation.
11. The recommendation distinguishes reject, retain with no change, append clarification, issue supplemental finding, issue corrected successor determination, or refer for formal reopening.
12. Proposed impact lists every original finding and downstream adjustment, J10, asset, finance, recovery, legal, disciplinary, reporting, or control record potentially affected.
13. The Audit Approver reviews provenance, credibility, materiality, fairness, finality, legal constraints, period treatment, and downstream implications.
14. The Audit Approver rejects, returns, approves clarification, approves supplemental or corrected determination, or authorises a formal reopening referral.
15. The successor determination receives its own effective time, reasons, scope, evidence, authority, and version while the original audit remains unchanged.
16. Downstream owners receive impact referrals and decide corrections under their own authority; completed actions are linked and reconciled.
17. Submission, evidence, analyses, responses, decisions, downstream corrections, actors, timestamps, and versions remain auditable.

---

# 10. Alternate Business Flows

## 10.1 Evidence Is Immaterial

It is appended or retained with a no-change determination and reasons. Original outcome remains controlling.

## 10.2 Recovered Missing Stock

Physical identity, custody, condition, location, and recovery time are verified; loss, adjustment, J10, asset, and finance owners receive successor facts.

## 10.3 External Legal or Regulatory Decision

The decision's authority, finality, scope, and effect are validated before it influences the supplemental determination.

## 10.4 Evidence Indicates Concealment or Tampering

Evidence is protected and referred to competent legal, security, disciplinary, fraud, or audit authority without prejudging the case.

## 10.5 Original Finding Remains Correct but Reason Changes

A clarification or supplemental cause finding is issued without reversing correct quantity or stock consequences.

## 10.6 Formal Reopening Required

The competent authority defines reopened scope, evidence, independence, cut-off treatment, and relation to the original audit; reopening is not implied by review alone.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Evidence Authenticity Unresolved

The evidence remains quarantined or pending and cannot change the closed outcome.

## 11.2 Original Record Unavailable

Review is held or qualified while archive recovery and integrity checks occur.

## 11.3 Unauthorised Silent Edit

The alteration is contained, original version restored, and integrity event escalated.

## 11.4 Downstream Correction Not Propagated

Impact referral is reissued and inconsistent states reconciled; original and successor decisions remain distinct.

## 11.5 Conflicted Reviewer

The reviewer is replaced or independently supervised and prior work remains visible.

## 11.6 System Interruption

Evidence custody and original versions remain protected and reconciliation occurs before continuation.

---

# 12. Business Rules and Decision Constraints

- Closed audit records are immutable; later evidence creates append-only clarification, supplemental determination, correction, or formal reopening.
- Evidence must have provenance, authenticity, custody, relevance, timing, and materiality assessment.
- A subsequent event is not misrepresented as evidence of the original cut-off condition.
- New evidence does not automatically reverse adjustment, J10, asset, finance, recovery, disciplinary, legal, or reporting outcomes.
- Every affected downstream record requires its own authorised correction.
- Original and successor findings, effective times, reasons, evidence, and authorities remain distinguishable.
- Reopening requires competent authority and defined scope; it is not implied by accepting evidence.
- Affected parties receive response opportunity where fairness and law require.
- Legal holds, appeals, closed periods, privilege, privacy, confidentiality, and statutory finality remain controlling.
- Concealment or tampering indicators require separate competent investigation.

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Original Inventory Audit | `Closed` or `Archived` | Links supplemental evidence and determination without rewriting the closed lifecycle history. | Remains `Closed` or `Archived` | Physical Verification, Audit & Reconciliation (J09) | `L2-J09` Section 12 and Section 31.10 |
| Supplemental Audit | `Draft` where a separate audit is authorized | Follows its own canonical J09 lifecycle. | Existing J09 lifecycle position | Physical Verification, Audit & Reconciliation (J09) | `L2-J09` Section 12 |
---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Material Consumer(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `SupplementalAuditReviewRequested` | Physical Verification, Audit & Reconciliation (J09) | J06, J08, J10, Finance and Audit where material | Records the exact completed business fact material to this scenario. | `L2-J09 Section 31` |
| `SupplementalAuditEvidenceRecorded` | Physical Verification, Audit & Reconciliation (J09) | J06, J08, J10, Finance and Audit where material | Records the exact completed business fact material to this scenario. | `L2-J09 Section 31` |
| `SupplementalAuditLinked` | Physical Verification, Audit & Reconciliation (J09) | J06, J08, J10, Finance and Audit where material | Records the exact completed business fact material to this scenario. | `L2-J09 Section 31` |
| `SupplementalDeterminationRecorded` | Physical Verification, Audit & Reconciliation (J09) | J06, J08, J10, Finance and Audit where material | Records the exact completed business fact material to this scenario. | `L2-J09 Section 31` |
## 14.2 Cross-Domain Collaboration

| Participant | Collaboration | Boundary |
|---|---|---|
| L3-J-087 / L3-J-088 | Supplies original audit, evidence, limitations, and closure. | Original record remains immutable. |
| L3-J-089 to L3-J-092 | Supplies findings and downstream links. | Each correction remains separately authorised. |
| Finance / Asset / J10 / Legal / HR / Reporting | Reviews impact and executes successor outcomes. | Supplemental audit does not post them. |
| Records / Privacy / Security | Protects versions, holds, access, and sensitive evidence. | Retains specialist authority. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J09 roles are separated as policy requires: Reconciliation Officer; Audit Approver; Audit Reviewer; Variance Investigator; and Read-Only Auditor. Original decision-makers cannot suppress material contrary evidence, and the Audit Reviewer cannot silently replace the Audit Approver decision.

## 15.2 Required Evidence

- original audit, scope, cut-off, counts, findings, limitations, responses, closure, adjustments, J10 referrals, asset findings, finance links, and downstream outcomes;
- new evidence, source, discovery, event date, late-submission reason, custody, authenticity, relevance, availability, confidentiality, and materiality;
- responses, conflicts, impact analysis, recommendation, determination, effective time, reopening or correction referrals, completed outcomes, actors, timestamps, and versions.

## 15.3 Audit Obligations

The institution must reconstruct what new evidence arrived, why it was late, how authenticity and materiality were assessed, what original findings and downstream outcomes were affected, who approved the successor determination, and how corrections were completed without rewriting history.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- new evidence has been identified that is materially relevant to a closed audit and could not have been reasonably included at closure;
- the evidence has been assessed by an authority appropriate to its nature, materiality, and institutional consequence;
- a determination has been made on whether to append the evidence, reopen the audit finding, issue a supplemental finding, or record the evidence as a post-closure note;
- the supplemental determination has been approved with explicit authority, effective date, and connection to the original closed audit identity;
- the original closed audit evidence and closure decision are preserved and not overwritten;
- the supplemental determination, evidence, authority, actors, and audit trail are preserved and immutable.


## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- the closed audit has a supplemental determination record permanently linked to the original closed audit identity;
- the original closed audit evidence and closure decision are preserved, unaltered, and independently auditable;
- the supplemental finding or note does not retrospectively change the original closure decision without explicit authority to do so;
- any material consequence of the supplemental determination is referred to its authoritative governing process;
- the complete evidence chain from original count through closure to supplemental determination is permanently auditable and reconstructable.


---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. Supplemental review authority, materiality, response periods, reopening thresholds, appeal interaction, and finality rules require confirmation.
2. Closed-period inventory and finance corrections require cross-domain treatment.
3. Recovered stock, external decisions, forensic evidence, concealed records, and legal proceedings require specialist rules.

## 17.2 Assumptions

- Supplemental determinations are append-only successor records.
- Downstream corrections retain separate authority.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-093**
