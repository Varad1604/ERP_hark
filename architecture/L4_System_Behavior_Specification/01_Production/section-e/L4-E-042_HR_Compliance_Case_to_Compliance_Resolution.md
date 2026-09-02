# HR Compliance Case to Compliance Resolution

## 1. Specification Metadata

**Specification ID:** L4-E-042  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-042 — HR Compliance Case to Compliance Resolution  
**Primary Business Owner:** HR  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-042; applicable locked E architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern an HR compliance case from requirement identification through evidence collection, verification, remediation where verification fails, and completion when the governed compliance requirement is successfully verified.

## 3. Required Business Context

- A compliance requirement exists.
- The affected workforce context can be identified where required.
- Evidence requirements and verification expectations are available from applicable policy or governance.
- The Compliance Case Aggregate remains owned by HR Workflows.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-042-BH-001 | The system shall create the HR compliance case from the identified compliance requirement under HR ownership. | L3-E-042 §§2, 5, 9 |
| L4-E-042-BH-002 | The system shall collect the business evidence required by the applicable compliance requirement without inventing local compliance criteria. | L3-E-042 §§7, 9, 12 |
| L4-E-042-BH-003 | The system shall verify the collected evidence against the applicable compliance requirement and record the verification result. | L3-E-042 §§9, 12, 16 |
| L4-E-042-BH-004 | If verification succeeds, the system shall complete the compliance case and establish the governed compliance resolution. | L3-E-042 §§3, 9, 13, 16 |
| L4-E-042-BH-005 | If verification fails, the system shall record the failed verification, establish the applicable remediation outcome, and return the case to verification when remediation is available. | L3-E-042 §§9, 10, 11, 13 |
| L4-E-042-BH-006 | If remediation does not resolve the requirement, the system shall preserve the failed business result and keep the case governed by the applicable institutional compliance process rather than treating it as a technical failure. | L3-E-042 §§10, 11 |
| L4-E-042-BH-007 | The system shall preserve immutable business history for evidence collection, verification, remediation and completion outcomes. | L3-E-042 §§12, 15 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-042-BH-008 | On successful compliance completion, the system shall publish the canonical **ComplianceCompleted** business event identified by the locked Compliance workflow. | L3-E-042 §§14.1, 16 |
| L4-E-042-BH-009 | The system shall not invent any additional compliance event beyond the parent-defined **ComplianceCompleted** event. | L3-E-042 §14.1 |
| L4-E-042-BH-010 | Evidence collection and verification may involve business handoffs among HR and the relevant verification participant; these handoffs shall remain distinct from the **ComplianceCompleted** event and shall not redefine ownership. | L3-E-042 §§5, 14.2 |

## 6. Failure / Rejection / Continuation

- If required evidence is incomplete, the system shall keep the compliance case pending evidence collection until the required business evidence is available.
- If verification fails, the system shall keep the compliance requirement unresolved and route the case through the parent-defined remediation and re-verification path.
- If remediation does not resolve the requirement, the system shall preserve the failed business result and follow applicable institutional compliance governance.
- Technical retries, queues, rollback and infrastructure recovery remain outside L4.

## 7. Audit & Evidence Behavior

The system shall preserve attributable evidence of the compliance requirement, collected evidence, verification result, remediation outcome, applicable approval or authorization decision where present, and final completion decision.

## 8. Observable Completion Conditions

Completion requires the required evidence to have been collected to the extent required by the governing compliance rule, a successful verification result to be recorded, the compliance case to reach the parent-defined Completed state, and **ComplianceCompleted** to be published. Failed verification remains on the remediation and re-verification path until a governed business outcome is established.

## 9. Traceability & Provenance

**Parent:** L3-E-042 — HR Compliance Case to Compliance Resolution  
**Primary verification source:** Authoritative People & Workforce L2 HR Workflows architecture.

## 10. Open Issues / Deferred Design

Institution-specific compliance criteria, verification authorities and approval roles remain governed by upstream policy/configuration. Technical APIs, database persistence schemas, UI screens, network transport and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
