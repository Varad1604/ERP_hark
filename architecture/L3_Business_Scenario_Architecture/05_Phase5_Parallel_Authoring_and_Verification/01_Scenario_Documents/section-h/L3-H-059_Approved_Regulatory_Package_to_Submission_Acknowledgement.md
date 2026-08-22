# Approved Regulatory Package to Submission Acknowledgement

# 1. Scenario Metadata

**Scenario ID:** L3-H-059  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** NEP, NAAC & AISHE Reporting  
**Participating Domains:** H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H08 — NEP, NAAC & AISHE Reporting Workflows  
**Referenced By:** Regulatory correction, resubmission, reporting closure, audit, and historical reporting scenarios

# 2. Scenario Purpose

Govern the external submission of an approved H08 regulatory package and the authoritative recording of the regulatory authority's acknowledgement or response while preserving immutable submission history.

# 3. Business Outcome

The package is submitted and acknowledged, or a controlled external rejection or resubmission requirement is recorded

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized package is due for external submission.

## 4.2 End Boundary

The scenario ends when the approved H08 package has been submitted and the regulatory authority's acknowledgement/response has been authoritatively recorded as accepted, accepted with remarks, rejected, or requiring resubmission.

## 4.3 In Scope

- Submission of an approved and submission-ready H08 Submission Package through the governed institutional submission path.
- Creation/preservation of the H08 Submission Record for the external transmission.
- Recording the regulatory authority's acknowledgement, acceptance, remarks, rejection, or resubmission requirement.
- Preservation of immutable submission and acknowledgement history.

## 4.4 Out of Scope

- Institutional package approval, which precedes this scenario.
- Changing the Reporting Snapshot or source-domain business facts during submission.
- Technical transport, connector, callback, polling, retry, or delivery mechanics.
- Correction/resubmission execution after a regulatory rejection; that belongs to the subsequent correction scenario.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| NEP, NAAC & AISHE Reporting / H | Owns Submission Package status, Submission Record, submission history, regulatory acknowledgement reference/history, and the H08 reporting-submission outcome. |

The external regulatory authority provides the external response but does not own H08 reporting history.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Regulatory Submission Officer | Performs the governed submission of an approved package where authorized. | Submits |
| Regulatory Authority | Receives the institutional submission and issues the external acknowledgement or response represented in H08. | External participant / decision source |

# 7. Preconditions and Entry Criteria

- The Submission Package is institutionally approved.
- The Submission Package is in the H08 source-defined submission-ready position.
- The applicable submission period/window and governing regulatory context permit submission.
- The package references exactly one immutable Reporting Snapshot.
- The submission history is available to preserve the new Submission Record without overwriting prior history.

# 8. Trigger

An authorized package is due for external submission

# 9. Main Business Flow

1. The Regulatory Submission Officer identifies the approved H08 Submission Package due for external submission.
2. H08 confirms the package is approved and ready for submission under the applicable reporting governance.
3. The package is submitted through the institution's approved regulatory submission path.
4. H08 records the external submission as a new Submission Record and preserves the submitted package/snapshot lineage.
5. H08 waits for and receives the regulatory authority's acknowledgement or response at the business level.
6. H08 validates the response against the submitted record and records the acknowledgement history without overwriting prior submission evidence.
7. If accepted, H08 records the accepted regulatory outcome; if accepted with remarks, it records the authority remarks; if rejected, it records the external rejection; if resubmission is required, it records that requirement for subsequent correction governance.
8. The scenario ends with the regulatory response and submission history preserved.

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Accepted with remarks | Authority accepts but provides remarks | Record acceptance and preserve the regulatory remarks | Acknowledged accepted outcome with remarks | NEP, NAAC & AISHE Reporting |
| External rejection | Authority rejects the submission | Record the rejection and preserve the authority response | Controlled external rejection recorded | NEP, NAAC & AISHE Reporting |
| Resubmission required | Authority requires correction and another submission | Record the requirement without altering the historical submission | Resubmission requirement recorded for H060 | NEP, NAAC & AISHE Reporting |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Package is not approved or not submission-ready | External submission must not proceed | Return to H08 package governance before a later valid submission | NEP, NAAC & AISHE Reporting |
| Submission does not reach a completed business submission outcome | Acknowledgement cannot yet be associated with a completed submission | Preserve the failed/incomplete submission history and require a later governed submission action | NEP, NAAC & AISHE Reporting |
| Authority response cannot be matched to the governed submission record | Acknowledgement cannot become authoritative H08 evidence | Resolve the business-record association before recording the final authority outcome | NEP, NAAC & AISHE Reporting |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every submission references exactly one Reporting Snapshot | H08 | Submission lineage remains anchored to an immutable reporting record | L2-H08 BR-010; O-004 |
| Submission requires institutional approval | H08 | Unapproved packages cannot be submitted | L2-H08 BR-011 |
| Only approved Submission Packages may be submitted | H08 | Ready state does not bypass approval | L2-H08 BR-017 |
| Every submission preserves complete history | H08 | New outcomes never overwrite prior Submission Records | L2-H08 BR-018; O-007 |
| Regulatory acknowledgements remain permanently available | H08 | Authority responses become durable reporting evidence | L2-H08 BR-019 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Submission Package | ReadyForSubmission | Governed external submission | Submitted, then Accepted or RejectedByAuthority according to authority outcome | H08 | L2-H08 §§6.11, 18.3 |
| Submission Record | Created | Record transmission and regulatory acknowledgement | Submitted → Acknowledged → Accepted or Rejected; archival may follow later | H08 | L2-H08 §6.12 |
| Reporting Cycle | SubmissionReady | Submission and regulatory outcome | Submitted → Accepted or RejectedByAuthority | H08 | L2-H08 §§6.6, 18.1 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| SubmissionInitiated | H08 | H08 submission governance | Records start of submission | L2-H08 §10.9 |
| ReportSubmitted | H08 | Reporting governance consumers | Records submitted report | L2-H08 §10.9 |
| SubmissionCompleted | H08 | H08 acknowledgement governance | Records completed submission | L2-H08 §10.9 |
| SubmissionFailed | H08 | H08 submission governance | Records failed submission | L2-H08 §10.9 |
| SubmissionAccepted | H08 | Reporting governance consumers | Records external acceptance | L2-H08 §10.9 |
| SubmissionRejected | H08 | Reporting governance consumers | H08 uses this event name for rejection, including the regulatory-acknowledgement workflow | L2-H08 H08-W10; §10.8 |
| RegulatoryRemarksReceived | H08 | Reporting governance consumers | Records authority remarks | L2-H08 §10.9 |
| ResubmissionRequested | H08 | H08 correction governance | Records external requirement for resubmission | L2-H08 §10.9 |

H08 defines a Submission Record `Acknowledged` state and a `RecordAcknowledgement` action, but no distinct `SubmissionAcknowledged` domain event.

## 14.2 Business Contracts and Cross-Domain Hand-offs

The approved Submission Package is handed to the applicable external regulatory authority under the institution's governed reporting relationship. The authority's acknowledgement/response is then recorded by H08 as reporting history. This external exchange does not transfer ownership of H08 reporting records or underlying source-domain facts.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The Regulatory Submission Officer is source-listed for report submission. The external regulatory authority is the source of the acknowledgement/acceptance/rejection/resubmission response. H08 records but does not invent the authority's external decision.

## 15.2 Separation of Duties, Overrides and Exceptions

Institutional submission may occur only after the package is approved. This scenario does not bypass institutional approval or rewrite the regulatory authority's response.

## 15.3 Business Audit and Evidence Obligations

Preserve package identifier/version, immutable Reporting Snapshot reference, submitting authority, submission date/context, Submission Record lineage, acknowledgement reference, external status, authority remarks, rejection/resubmission reason where supplied, and all prior submission history.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The approved package has a preserved H08 Submission Record.
- The external authority's acknowledgement/response is recorded or a controlled rejection/resubmission requirement is preserved.
- Prior submission history remains immutable.

## 16.2 Postconditions

H08 exposes the current governed regulatory submission outcome while retaining full historical lineage. A rejection or resubmission requirement may trigger H060 without altering the prior record.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- H08's Submission Record lifecycle explicitly contains `Acknowledged` and its acknowledgement workflow includes `RecordAcknowledgement`, but the event catalogue contains no distinct `SubmissionAcknowledged` event.
- The event name `SubmissionRejected` is used in H08's institutional approval event set and again in the regulatory acknowledgement workflow, so the same name is semantically overloaded across internal and external rejection contexts. This Draft preserves the source name and does not rename it.

These issues do not prevent the frozen submission/acknowledgement outcome from being represented because the Submission Record lifecycle, acknowledgement action, and external outcome branches are otherwise explicit.

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Upstream H08 event governance should clarify acknowledgement-event publication and distinguish or formally document the two `SubmissionRejected` contexts.
