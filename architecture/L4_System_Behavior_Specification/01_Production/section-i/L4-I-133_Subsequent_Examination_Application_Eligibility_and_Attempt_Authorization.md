# L4-I-133 — Subsequent Examination Application, Eligibility and Attempt Authorization

## 1. Specification Metadata
- Specification ID: L4-I-133
- Architecture Level: L4 — System Behavior Specification
- Realizes: L3-I-133 — Subsequent Examination Application, Eligibility and Attempt Authorization
- Primary Owner: Supplementary, Backlog & Improvement Examination
- Participating Domains: C — Student Lifecycle; F — Finance; I — Examination
- Scenario Type: Critical; Priority: P1
- Status: Draft; Version: 1.0

## 2. Behavioral Objective & Boundary
Govern the institutional behavior that converts an eligible subsequent-examination application into an authorized examination attempt. The behavior begins with a valid application context and ends when the attempt is authorized or the application is rejected/held under the governing business rules.

Out of scope: ownership of student lifecycle, Finance fee policy, examination scheduling, hall-ticket production, examination conduct, or technical integration.

## 3. Required Business Context
- Candidate/student identity is authoritative.
- A subsequent examination opportunity is established.
- Eligibility information is available from its canonical owner.
- Applicable Finance clearance, where required, is authoritative.
- Examination owns the resulting attempt authorization.

## 4. Required System Behaviors
1. Identify the application, candidate, subsequent-examination opportunity and requested attempt.
2. Validate the authoritative academic/student eligibility context.
3. Consume the Finance-owned fee/clearance outcome where applicable; do not redefine Finance ownership.
4. Determine whether the application satisfies the applicable subsequent-examination rules.
5. Establish attempt authorization only when all required conditions are satisfied.
6. Preserve rejected, held, and authorized decisions with their governing basis.
7. Prevent authorization outside the approved opportunity or attempt scope.

## 5. Cross-Domain Behavior
Student Lifecycle supplies authoritative candidate context; Finance supplies authoritative financial clearance where applicable. The Supplementary, Backlog & Improvement Examination capability owns subsequent-examination administration and the resulting attempt authorization. No cross-domain ownership is transferred.

## 6. Failure / Rejection / Recovery
Invalid eligibility, missing required clearance, duplicate application, or unsupported attempt scope shall prevent authorization. A held application remains governed until authoritative missing conditions are resolved. No new lifecycle state or event is invented.

## 7. Audit & Evidence
Retain application context, eligibility determination, financial prerequisite outcome, authorization decision, authority, timestamps/business-effective context, and rejection/hold rationale.

## 8. Observable Completion
The application has an authoritative authorized-attempt outcome, or an authoritative rejection/hold outcome is recorded and traceable.

## 9. Traceability
Parent: L3-I-133 — Subsequent Examination Application, Eligibility and Attempt Authorization.

## 10. Deferred Design
Exact eligibility formulae, Finance policy, technical interfaces, persistence structures and notification mechanisms remain governed by their canonical owners and later architecture levels.