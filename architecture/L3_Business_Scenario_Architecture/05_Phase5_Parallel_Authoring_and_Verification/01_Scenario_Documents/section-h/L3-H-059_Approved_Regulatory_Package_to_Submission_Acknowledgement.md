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
**Last Updated:** 2026-08-23  
**Depends On:** Locked L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H08 — NEP, NAAC & AISHE Reporting Workflows  
**Referenced By:** None yet

# 2. Scenario Purpose

Govern submission of an approved H08 regulatory package and preserve the external regulatory authority's acknowledgement, acceptance, rejection, remarks, or resubmission requirement as immutable reporting history.

# 3. Business Outcome

The package is submitted and acknowledged, or a controlled external rejection or resubmission requirement is recorded

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorized package is due for external submission.

## 4.2 End Boundary

The scenario ends when H08 has recorded the submission and the corresponding regulatory acknowledgement/outcome, including acceptance, external rejection, remarks, or a resubmission requirement.

## 4.3 In Scope

- Confirm the Submission Package is approved and ready for submission.
- Submit the approved package within the governed submission window.
- Create/preserve the authoritative H08 Submission Record for the transmission.
- Record the regulatory authority acknowledgement and resulting business outcome.
- Preserve immutable submission and acknowledgement history.

## 4.4 Out of Scope

- Defining implementation-specific submission behavior.
- Altering the approved package or its Reporting Snapshot during submission.
- Correcting source-domain facts after a regulatory rejection.
- Inventing a separate external-rejection event name not defined upstream.

# 5. Ownership and Participating Domains

| Domain / Actor | Responsibility |
|---|---|
| NEP, NAAC & AISHE Reporting / H08 | Owns Submission Package lifecycle, Submission Record, submission history, acknowledgement history, and reporting outcome. |
| External Regulatory Authority | Receives the approved institutional package and issues the acknowledgement, acceptance, rejection, remarks, or resubmission requirement; H08 records but does not own the external authority. |

# 6. Business Actors, Roles and Responsibilities

The external Regulatory Authority is the source of the acknowledgement/outcome. Locked H08 does not define a canonical business actor for executing the institutional submission, although it defines the approval and business preconditions that make a package eligible for submission.

# 7. Preconditions and Entry Criteria

- The Submission Package is approved.
- The package is in `ReadyForSubmission` under the H08 lifecycle.
- The submission window is open.
- The package references exactly one immutable Reporting Snapshot.

# 8. Trigger

An authorized package is due for external submission

# 9. Main Business Flow

1. H08 confirms that the approved Submission Package is eligible for submission and within the governed submission window.
2. H08 initiates the regulatory submission and records the transmission as a Submission Record.
3. The Submission Package progresses to `Submitted` without altering the referenced immutable Reporting Snapshot.
4. The external Regulatory Authority returns an acknowledgement or authority response.
5. H08 records the acknowledgement and preserves the authority response history.
6. If accepted, the reporting submission reaches the corresponding accepted outcome; if rejected or resubmission is required, H08 records that controlled outcome for the subsequent correction/resubmission scenario.

# 10. Alternate Business Flows

- The authority may accept the submission with remarks; H08 records the remarks in the immutable submission history.
- The authority may reject the submission, placing the governed submission path in the H08 external-rejection outcome.
- The authority may require resubmission; H08 records `ResubmissionRequested` and preserves the existing submission record.
- A submission that does not complete does not erase the attempt or alter prior accepted history.

# 11. Failure, Interruption and Business Recovery

If the submission cannot reach a completed business transmission outcome, H08 preserves the Submission Record and package history. Subsequent governed resubmission creates new submission history rather than replacing the prior attempt. No implementation recovery mechanism is specified at L3.

# 12. Business Rules and Decision Constraints

- BR-010 requires every submission to reference exactly one Reporting Snapshot.
- BR-011 requires institutional approval before submission.
- BR-017 allows only approved Submission Packages to be submitted.
- BR-018 requires complete submission history to be preserved.
- BR-019 requires regulatory acknowledgements to remain permanently available.
- Accepted submissions are immutable.
- Resubmissions create new Submission Records.

# 13. State and Lifecycle Impact

Submission Package progresses `Approved → ReadyForSubmission → Submitted`, then to `Accepted` or `RejectedByAuthority`; accepted packages may later be archived. Submission Record progresses `Created → Submitted → Acknowledged → Accepted/Rejected → Archived`.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H08 defines `SubmissionInitiated`, `ReportSubmitted`, `SubmissionCompleted`, `SubmissionFailed`, `SubmissionAccepted`, `RegulatoryRemarksReceived`, and `ResubmissionRequested`. H08 also uses `SubmissionRejected` for an external authority rejection in H08-W10, but the same event name is used for institutional approval rejection in H08-W07; this Draft does not rename or disambiguate it locally.

## 14.2 Business Contracts and Cross-Domain Hand-offs

The business hand-off is the approved H08 Submission Package to the external Regulatory Authority, followed by the authority acknowledgement/outcome back to H08. H08 retains the institutional submission record while the external authority remains external to SutraERP ownership.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Package eligibility is governed by prior H08 institutional approval and the open submission window. The external Regulatory Authority owns the external acknowledgement/acceptance/rejection decision. The locked H08 source does not canonically name the business actor who executes institutional submission.

## 15.2 Separation of Duties, Overrides and Exceptions

Submission cannot bypass package approval or alter the approved immutable snapshot. H08 cannot override or rewrite the regulatory authority's acknowledgement.

## 15.3 Business Audit and Evidence Obligations

Preserve package version/snapshot reference, approval evidence, submission time/history, Submission Record lineage, external acknowledgement, regulatory remarks, acceptance/rejection outcome, and any resubmission requirement.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The approved package has a recorded submission outcome and H08 has preserved the corresponding regulatory acknowledgement, external rejection, remarks, or resubmission requirement.

## 16.2 Postconditions

The original Submission Record and acknowledgement history remain immutable. A resubmission requirement, if any, progresses to governed correction/resubmission without rewriting prior history.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

**Non-blocking business-actor gap:** H08 defines submission preconditions and outcomes but its canonical crosswalk contains no business Actors section, leaving the institutional submission-execution actor unnamed.

**Non-blocking event semantic ambiguity:** `SubmissionRejected` is used both for institutional approval rejection and external regulatory rejection. H08 remains the canonical publisher, but the event meaning is not unique across the reviewed workflows.

**Architecture impact:** the submission and acknowledgement outcome is source-grounded, but actor naming and rejection-event semantics require upstream normalization before locking or downstream event behavior is specified.

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Upstream H08 should define the canonical business submission actor/authority and separate or otherwise normalize institutional versus external rejection event semantics.
