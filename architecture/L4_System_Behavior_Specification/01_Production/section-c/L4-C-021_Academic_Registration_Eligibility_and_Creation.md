# L4-C-021 — Academic Registration Eligibility and Creation

**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** `L3-C-021` — Academic Registration Eligibility and Creation  
**Primary Owner:** Academic Registration  
**Participating Domains:** C  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Author:** Varad  

## 1. Purpose
Define implementation-independent system behavior for validating a Student's eligibility and establishing an Academic Registration, without defining APIs, database structures, classes, UI, infrastructure, or deployment.

## 2. Authoritative Inputs
- Active academic session and approved registration window.
- Student lifecycle context.
- Programme/curriculum context.
- Published academic offerings.
- Registration configuration.
- Prerequisite/co-requisite rules.
- Credit limits and applicable academic standing.
- Registration Hold and configured financial-clearance conditions.

Authoritative ownership remains with the source domains; Academic Registration evaluates them for registration purposes.

## 3. Preconditions
1. Registration window is open.
2. Student is in an eligible lifecycle context.
3. Programme and academic context are valid.
4. Applicable offerings and registration configuration are published.
5. Required eligibility information is available.

## 4. Trigger
A valid Student registration request or authorised administrative initiation occurs during the applicable registration window.

## 5. System Behavior
1. Establish the registration context.
2. Validate Student and programme eligibility.
3. Validate session/semester applicability.
4. Validate academic standing and Registration Hold conditions.
5. Validate configured financial clearance where applicable.
6. Determine eligibility as **Eligible**, **Ineligible**, or **Manual Review Required** according to authoritative policy.
7. If eligible, create the Academic Registration business record within the Academic Registration aggregate.
8. Establish the registration lifecycle position supported by L2-C03.
9. Publish only canonical events supported by L2-C03, including `RegistrationEligibilityValidated` and `RegistrationCreated` where their business conditions are met.
10. Preserve audit evidence for the evaluation and creation outcome.

## 6. Alternate Behavior
- Ineligible: do not establish a successful registration; preserve the business reason.
- Manual Review Required: stop normal progression until authorised review resolves the condition.
- Missing authoritative prerequisite: do not infer eligibility.
- Authorised exception: continue only under the applicable institutional policy and audit requirements.

## 7. Business Rules
- Registration occurs only during an approved registration window.
- Student/programme eligibility is mandatory.
- Registration Hold and applicable financial-clearance conditions are evaluated.
- Published offerings are required for selection.
- Duplicate registration and credit-limit constraints apply.
- No downstream domain may become owner of Academic Registration.

## 8. Lifecycle
`Window Open → Eligibility Validated → Registration Created → Academic Offerings Selected → Registration Validated → Approved → Frozen → Published`.

This specification only establishes behavior through the portion relevant to C021 and does not invent lifecycle states.

## 9. Events and Contracts
**Published by Academic Registration:** `RegistrationEligibilityValidated`, `RegistrationCreated` when applicable.  
**Consumed facts:** Student, programme, academic-session, offering, hold and configured financial-clearance facts from authoritative owners.

No new event name is introduced.

## 10. Authorization and Audit
Eligibility evaluation and registration creation require the authority defined by L2-C03 and institutional configuration. Every material decision and lifecycle transition is auditable.

## 11. Business Failure Handling
| Condition | Required behavior |
|---|---|
| Closed registration window | Reject/defer registration. |
| Student ineligible | Return business ineligibility outcome. |
| Hold blocks eligibility | Do not create successful registration until resolved. |
| Missing configuration | Do not invent defaults; resolve the governing configuration. |
| Required authority absent | Stop pending authorised action. |

Technical retry, queue, persistence and infrastructure behavior is outside L4 business/system behavior here.

## 12. Traceability
- L3: `L3-C-021`.
- L2: `L2-C03_Student_Academic_Registration_Workflows.md`.
- Canonical ownership: Academic Registration.
- Related scenarios: C022 modification, C023 cancellation, C024 repeat/backlog, C025 finalization/publication.

## 13. Implementation Boundary
This document specifies observable business/system behavior only. API contracts, schemas, database tables, service decomposition, UI, deployment and technology choices belong to later implementation architecture.

## 14. Open Issues
Institution-specific approval routing and detailed prerequisite configuration remain governed upstream/configuration where not fixed by L2.

**Classification: PASS WITH ISSUES — Draft**
