# Closed Audit Evidence to Governed Supplemental Determination

## 1. Specification Metadata

**Specification ID:** L4-J-093  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Enterprise Resource Management — Physical Verification, Audit & Reconciliation  
**Realizes Scenario:** L3-J-093  
**Primary Business Owner:** Physical Verification, Audit & Reconciliation  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-093_Closed_Audit_Evidence_to_Governed_Supplemental_Determination.md`; `L2-J09_Physical_Verification_Audit_and_Reconciliation.md`; `L4_Governance_and_Architecture_Standard.md`; `L4_Validation_Standard.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern new, corrected, recovered or previously unavailable evidence received after an audit has closed or reached a preserved incomplete outcome.

The original audit record shall remain immutable. New evidence may produce no change, clarification, supplemental finding, corrected successor determination or a separately authorised reopening referral, but shall never silently edit, delete, overwrite or backdate the original finding.

Any stock, asset, Finance, J10, Legal, disciplinary, reporting or other consequence remains separately owned.

## 3. Required Business Context

- identifiable governed original audit outcome and immutable evidence set;
- new evidence linked to audit/population/item/location/lot/serial/asset/custody/variance/finding/referral;
- evidence source, discovery date, event date, custody and late-availability reason;
- original scope, cut-off, counts, findings, limitations, responses and closure;
- original downstream adjustments/J10/asset/Finance/legal/reporting links;
- supplemental-review, materiality, response, reopening, period, appeal, retention and legal-hold rules;
- Audit Reviewer and Audit Approver participation.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-093-BH-001 | When post-closure evidence is submitted, the system shall require original audit linkage, affected finding/item, source, discovery/event dates, custody, reason for late availability, claimed effect and confidentiality to be attributable. | L3-J-093 §§4.1, 7–9 step 1 |
| L4-J-093-BH-002 | The system shall preserve the new evidence and require validation of submitter authority, authenticity, completeness, chronology, provenance and relation to the original audit before it can affect a determination. | L3-J-093 §9 step 2, 12 |
| L4-J-093-BH-003 | Duplicate, previously considered, fabricated, altered, privileged, restricted or unrelated material shall remain explicitly classified and shall not be silently excluded or accepted. | L3-J-093 §9 step 3 |
| L4-J-093-BH-004 | The system shall reconstruct the relevant original audit scope, cut-off, counts, evidence, limitations, findings, responses, downstream outcomes and closure without modifying them. | L3-J-093 §§9 step 4, 12 |
| L4-J-093-BH-005 | The system shall distinguish corroborating, contradictory, corrective, explanatory, recovered physical evidence, subsequent-event, external-determination and irrelevant evidence as applicable. | L3-J-093 §§9 steps 5–6, 12 |
| L4-J-093-BH-006 | A subsequent event shall not be represented as proof of the original cut-off condition merely because it was discovered after closure. | L3-J-093 §§9 step 6, 12 |
| L4-J-093-BH-007 | The system shall preserve materiality assessment across physical quantity, condition, location, ownership, custody, cut-off, cause, responsibility, adjustment, loss, asset, Finance, grant, legal, reporting and control findings. | L3-J-093 §9 step 7 |
| L4-J-093-BH-008 | Where urgent safety, security, fraud, legal-hold, evidence-integrity or continuing-stock risk exists, the system shall expose the governed urgent referral without prejudging the supplemental determination. | L3-J-093 §9 step 8 |
| L4-J-093-BH-009 | Where fairness/law requires, the system shall preserve response opportunity for original finding owners, specialists and affected parties and shall control conflicts/self-review. | L3-J-093 §§9 steps 9–10, 12 |
| L4-J-093-BH-010 | The system shall preserve the Audit Approver's source-supported disposition: reject, retain with no change, clarify, supplemental finding, corrected successor determination or formal reopening referral. | L3-J-093 §§3, 9 steps 11–14 |
| L4-J-093-BH-011 | Any approved successor determination shall receive its own effective time, reasons, scope, evidence, authority and version while the original audit remains unchanged. | L3-J-093 §§9 step 15, 12–13 |
| L4-J-093-BH-012 | The system shall expose every affected downstream record for separately authorised correction rather than automatically reversing or rewriting adjustments, J10, asset, Finance, recovery, legal, disciplinary or reporting outcomes. | L3-J-093 §§4.4, 9 steps 12, 16, 12 |
| L4-J-093-BH-013 | Formal reopening shall require separate competent authority and defined scope; acceptance/review of new evidence shall not itself reopen the original audit. | L3-J-093 §§3–4, 10.6, 12 |
| L4-J-093-BH-014 | The original Inventory Audit shall remain `Closed` or `Archived` when a supplemental determination is linked; any separately authorised Supplemental Audit follows its own J09 lifecycle. | L3-J-093 §13 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-093-BH-015 | The system shall preserve J09 publication of `SupplementalAuditReviewRequested`, `SupplementalAuditEvidenceRecorded`, `SupplementalAuditLinked` and `SupplementalDeterminationRecorded` at their source-supported business points. | L3-J-093 §14.1 |
| L4-J-093-BH-016 | J06, J08, J10, Finance, Legal, HR/disciplinary, Reporting, Records, Privacy and Security owners shall receive governed impact facts/referrals without transferring correction or consequence authority to J09. | L3-J-093 §§5, 14.2 |

## 6. Failure / Rejection / Continuation

- Unresolved evidence authenticity shall prevent the new evidence from changing the closed outcome.
- Unavailable original records shall hold or qualify review pending governed archive recovery/integrity checks.
- Unauthorised silent edits shall be contained, original versions restored and integrity concerns escalated.
- Failed propagation of a downstream correction shall be reissued/reconciled while original and successor decisions remain distinct.
- Conflicted reviewer participation shall require source-supported replacement or independent supervision.
- Business interruption shall preserve original versions and new-evidence custody until continuation.

**Provenance:** L3-J-093 §11.

## 7. Audit & Evidence Behavior

The system shall preserve:

- original audit identity, scope, cut-off, counts, findings, limitations, responses and closure;
- original adjustments/J10/asset/Finance/downstream links;
- new evidence source, discovery/event dates, custody, late-availability reason and confidentiality;
- authenticity/provenance/materiality analysis;
- responses and conflicts;
- Audit Reviewer analysis and Audit Approver determination;
- successor determination scope/effective time/version;
- reopening/correction referrals and returned outcomes;
- canonical supplemental-event history;
- actors, timestamps and versions.

**Provenance:** L3-J-093 §15.

## 8. Observable Completion Conditions

Completion is observable when the new evidence has a governed disposition; the original audit remains immutable and in its original closed/archived position; any supplemental determination is separately attributable and versioned; canonical supplemental events are available where applicable; and all downstream impacts are referred to their authoritative owners rather than silently rewritten.

## 9. Traceability & Provenance

**Parent:** L3-J-093 — Closed Audit Evidence to Governed Supplemental Determination  
**Primary verification source:** L2-J09 — Physical Verification, Audit & Reconciliation

Material anchors: L3-J-093 §§2–17.

## 10. Open Issues / Deferred Design

Supplemental review authority, materiality, response periods, reopening thresholds, appeal/finality rules, closed-period inventory/Finance correction and specialist treatment of recovered stock/external decisions/forensic evidence remain upstream governance matters.

Technical versioning, evidence ingestion, replay, notification and persistence remain deferred.
