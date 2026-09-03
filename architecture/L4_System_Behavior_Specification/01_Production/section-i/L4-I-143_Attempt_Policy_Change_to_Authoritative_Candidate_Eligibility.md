# Attempt Policy Change to Authoritative Candidate Eligibility

## 1. Specification Metadata

**Specification ID:** L4-I-143  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination  
**Realizes Scenario:** L3-I-143 — Attempt Policy Change to Authoritative Candidate Eligibility  
**Primary Business Owner:** Supplementary, Backlog & Improvement Examination  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-03  
**Depends On:** L3-I-143; L2-I01; L2-I12; authoritative Student Lifecycle context  
**Referenced By:** Affected subsequent-examination workflows

## 2. Behavioral Objective & Boundary

Re-evaluate current candidate eligibility when an authoritative I01 attempt-policy change becomes effective. I01 owns policy and effective scope; I12 owns subsequent-examination eligibility/attempt administration; Student Lifecycle owns student academic context.

## 3. Required Business Context

- Authoritative effective policy change and scope.
- Affected candidate population and examination scopes.
- Preserved prior attempt and eligibility history.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-143-BH-001 | Identify the effective policy change and affected candidate/examination population. | L3-I-143 §§3,7–9 |
| L4-I-143-BH-002 | Establish authoritative attempt history and required student context for each affected candidate. | L3-I-143 §§9,12 |
| L4-I-143-BH-003 | Apply the effective policy to establish the candidate's current I12 eligibility position. | L3-I-143 §§3,9,12,16 |
| L4-I-143-BH-004 | Preserve prior eligibility decisions and historical attempts rather than rewriting them. | L3-I-143 §§3,9,12,16 |
| L4-I-143-BH-005 | Route downstream schedule/Hall Ticket/conduct consequences to their canonical owners where active processing is affected. | L3-I-143 §§9–11 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-I-143-BH-006 | I01 remains the owner/publisher of policy-change facts and I12 makes the resulting authoritative eligibility position available through its existing lifecycle. The parent does not establish a dedicated policy-change-to-eligibility event; none is invented. | L3-I-143 §14 |

## 6. Failure / Rejection / Continuation

- Unclear policy effective date/scope remains unresolved with I01.
- Incomplete or conflicting attempt history must be resolved before re-evaluation.
- No retroactive rewriting of historical attempts is assumed.
- Exceptional override requires authoritative upstream approval.

## 7. Audit & Evidence Behavior

Preserve policy version/effective scope, affected candidate population, attempt history, prior eligibility, re-evaluation basis and resulting current eligibility.

## 8. Observable Completion Conditions

Completion occurs when every affected candidate has one authoritative current eligibility position under the effective policy while historical attempts and prior decisions remain traceable.

## 9. Traceability & Provenance

**Parent:** L3-I-143  
**Primary verification sources:** L2-I01 and L2-I12.

## 10. Open Issues / Deferred Design

Retroactivity and effective-scope rules remain I01-owned. No dedicated policy-driven eligibility event is defined by the parent.

## Canonical Validation Note

Policy ownership remains with I01; eligibility administration remains with I12.
