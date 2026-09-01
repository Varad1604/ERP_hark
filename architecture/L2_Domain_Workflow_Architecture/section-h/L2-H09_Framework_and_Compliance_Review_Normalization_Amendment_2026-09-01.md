# L2-H09 — Framework and Compliance Review Normalization — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-H09 — Accreditation and Compliance Workflows |
| Amendment Version | 1.2.0 |
| Status | Locked |
| Effective Date | 2026-09-01 |
| Approved Through | ADR-L4-UPSTREAM-BLOCKER-RESOLUTION-002 |

## 1. Accreditation Framework review state

For the Accreditation Framework lifecycle, `Under Review` is the canonical state label. Historical H09 wording `Review` denotes the same lifecycle position and shall not create a second state.

The relevant governed path is:

`Draft → Under Review → Approved → Published → Active`

Historical framework versions remain immutable and only one applicable framework version may be Active for the governed scope.

## 2. Compliance Review event timing

The Compliance Review lifecycle remains:

`Scheduled → In Progress → Reviewed → Action Assigned → Closed`

`Compliance Review Completed` records completion of evidence evaluation/review activity and occurs when the review reaches the `Reviewed` milestone. It does **not** mean the Compliance Review aggregate is `Closed`.

`Compliance Review Assigned` records required governed action assignment, and `Compliance Review Closed` records final review closure.

## 3. Compliance-status vocabulary

`Compliant` is a governed H09 compliance-status outcome that may be recorded for an applicable Active Compliance Requirement only when updated evidence and review confirmation satisfy BR-H09-035.

`Compliant` is not a replacement lifecycle state for the Compliance Requirement aggregate. The canonical Requirement lifecycle defined by existing H09 amendments remains authoritative.

`Compliance Status Updated` is published by the Compliance Requirement aggregate when its governed compliance-status assessment is updated.

## 4. Downstream effect

This amendment resolves the lifecycle vocabulary issue recorded by L3-H-061 and the event/status ambiguity recorded by L3-H-070.
