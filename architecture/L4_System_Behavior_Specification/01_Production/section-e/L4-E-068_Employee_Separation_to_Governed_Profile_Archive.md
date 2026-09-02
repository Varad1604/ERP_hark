# Employee Separation to Governed Profile Archive

## 1. Specification Metadata

**Specification ID:** L4-E-068  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-068 — Employee Separation to Governed Profile Archive  
**Primary Business Owner:** Employee Profile  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-068; applicable locked Workforce & Engagement and Employee Profile architecture  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

Apply the governed EmployeeProfile archival consequence of an authoritative completed employee separation while preserving Workforce & Engagement ownership of the employment lifecycle and preserving EmployeeProfile historical information.

## 3. Required Business Context

- Existing canonical EmployeeProfile.
- Workforce & Engagement has completed an authoritative separation outcome for the employee represented by that profile.
- Employee Profile governance defines the applicable archival consequence.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-068-BH-001 | Employee Profile shall act only after an authoritative separation outcome has been completed by Workforce & Engagement. | L3-E-068 §§3, 7–9, 12 |
| L4-E-068-BH-002 | Employee Profile shall confirm that the separation outcome corresponds to the affected canonical EmployeeProfile before applying archival treatment. | L3-E-068 §§7, 9, 11 |
| L4-E-068-BH-003 | Employee Profile shall apply only the governed archival consequence defined by its canonical profile lifecycle. | L3-E-068 §§3, 9, 12–13 |
| L4-E-068-BH-004 | Historical EmployeeProfile information shall remain preserved after archival treatment. | L3-E-068 §§3, 9, 12, 16 |
| L4-E-068-BH-005 | Employee Profile shall not approve, reverse or modify the Workforce & Engagement separation outcome. | L3-E-068 §§4.4, 5, 12 |
| L4-E-068-BH-006 | Payroll settlement, exit-clearance ownership and workforce lifecycle modification shall remain outside this profile-archival behavior. | L3-E-068 §§4.4, 12 |
| L4-E-068-BH-007 | If a later authoritative workforce rejoin occurs, Employee Profile shall follow the canonical rejoin/profile lifecycle rather than independently reversing archival treatment. | L3-E-068 §§10–12 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-068-BH-008 | The separation event/business fact remains published and owned by Workforce & Engagement; the resulting profile archival event/fact remains published and owned by Employee Profile. | L3-E-068 §14 |
| L4-E-068-BH-009 | The business hand-off is Workforce & Engagement → Employee Profile: authoritative separation outcome for the affected employee. | L3-E-068 §14 |
| L4-E-068-BH-010 | L4 shall not invent the exact separation event name, archival event name or archival state vocabulary because the Locked parent explicitly records those Stage 3 mappings as unresolved upstream. | L3-E-068 §§14, 17 |

## 6. Failure / Rejection / Continuation

- Incomplete separation outcomes do not trigger profile archival treatment.
- If the employee/profile relationship cannot be authoritatively established, archival treatment is withheld until the business relationship is resolved.
- A later rejoin follows the canonical rejoin consequence; manual lifecycle bypass is not permitted.
- Technical storage recovery, retries and rollback are outside L4.

## 7. Audit & Evidence Behavior

The separation decision remains traceable under Workforce & Engagement. The profile archival consequence and preservation of historical profile information remain traceable under Employee Profile.

## 8. Observable Completion Conditions

An authoritative separation outcome exists; the corresponding EmployeeProfile has received the governed archival treatment; historical profile information remains preserved; workforce separation ownership remains unchanged; and the profile-side lifecycle event/fact is published only according to confirmed canonical vocabulary.

## 9. Traceability & Provenance

**Parent:** L3-E-068 — Employee Separation to Governed Profile Archive  
**Primary verification source:** L3-E-068 §§3, 5, 9, 11–17 and applicable locked Workforce & Engagement / Employee Profile architecture.

## 10. Open Issues / Deferred Design

The exact frozen Stage 3 separation event, EmployeeProfile archival transition/state and profile archival event remain unresolved upstream according to the Locked parent. L4 preserves those issues and does not invent vocabulary. Technical archival/storage mechanisms remain deferred to L5.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
