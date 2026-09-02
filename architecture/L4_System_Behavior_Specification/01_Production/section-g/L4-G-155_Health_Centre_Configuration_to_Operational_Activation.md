# Health Centre Configuration to Operational Activation

## 1. Specification Metadata

**Specification ID:** L4-G-155  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-155 — Health Centre Configuration to Operational Activation  
**Primary Business Owner:** Institutional Health Services (G06)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-155; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

The system shall govern health centre configuration to operational activation according to authoritative upstream policy and workflow governance. The behavior ends when the authoritative business outcome is recorded. APIs, persistence, UI, and infrastructure remain outside this specification.

## 3. Required Business Context

- Authoritative Institutional Health Services (G06) context exists.
- Applicable upstream business policy and governance conditions are satisfied.
- Relevant cross-domain facts remain source-owned.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-155-BH-001 | The system shall identify and validate the health centre configuration to operational activation context and authoritative inputs. | L3-G-155 §§3, 9, 12 |
| L4-G-155-BH-002 | The system shall validate required business eligibility and policy constraints under Institutional Health Services (G06) governance. | L3-G-155 §§3, 9, 12 |
| L4-G-155-BH-003 | The system shall execute the canonical health centre configuration to operational activation business evaluation. | L3-G-155 §§3, 9, 12 |
| L4-G-155-BH-004 | The system shall record the resulting authoritative business position and preserve historical audit evidence. | L3-G-155 §§3, 9, 12 |
| L4-G-155-BH-005 | The system shall prevent progression and maintain the last valid business position when governing conditions are not satisfied. | L3-G-155 §§3, 9, 12 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-155-BH-006 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under Institutional Health Services (G06) ownership. | L3-G-155 §14 |

## 6. Failure / Rejection / Continuation

- If required business eligibility or prerequisite conditions are not satisfied, prevent progression and maintain the prior valid state.
- If conflicting or unverified source facts are received, retain the business position as unresolved without inferring missing values.
- Technical retry, database rollback, queue recovery, and infrastructure mechanisms are excluded.

## 7. Audit & Evidence Behavior

The system shall preserve attributable audit evidence of the initiation context, eligibility evaluation, governing policy basis, authorized decisions, resulting lifecycle state transitions, and completed business outcomes.

## 8. Observable Completion Conditions

Completion requires governing conditions validated, the authoritative business position recorded under Institutional Health Services (G06) ownership, attributable audit evidence preserved, and canonical business events published.

## 9. Traceability & Provenance

**Parent:** L3-G-155 — Health Centre Configuration to Operational Activation  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
