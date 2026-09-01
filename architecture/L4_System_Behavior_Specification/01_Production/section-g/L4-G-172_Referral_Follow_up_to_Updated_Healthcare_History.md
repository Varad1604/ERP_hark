# Referral Follow-up to Updated Healthcare History

## 1. Specification Metadata

**Specification ID:** L4-G-172  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Institutional Health Services  
**Realizes Scenario:** L3-G-172 — Referral Follow-up to Updated Healthcare History  
**Primary Business Owner:** Institutional Health Services (G06)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** Locked L3-G-172; Locked L2-G06; Locked L1 Business Domain Architecture  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

The system shall support institutional follow-up after referral, preserve the authorised healthcare assessment, and update institutional medical history without acquiring ownership of external provider records. The behavior ends when the follow-up outcome is recorded and the applicable next-care path is established.

## 3. Required Business Context

- G06 owns Medical Encounter, Referral, follow-up healthcare activity and institutional medical history.
- Referral lifecycle is `Recommended → Accepted → Completed`.
- Medical Encounter remains the healthcare context.
- C owns student identity; E owns personnel information.
- External provider records remain externally owned.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-172-BH-001 | When referral follow-up becomes due or otherwise actionable, the system shall identify the patient, originating Medical Encounter and associated Referral context. | L3-G-172 §§3, 7, 8, 9 |
| L4-G-172-BH-002 | The system shall preserve the referral information available to the institution while distinguishing it from externally owned clinical records. | L3-G-172 §§4, 9, 12 |
| L4-G-172-BH-003 | When an authorised healthcare professional completes the follow-up assessment, the system shall record the resulting institutional healthcare outcome. | L3-G-172 §§3, 9, 15 |
| L4-G-172-BH-004 | The system shall update institutional medical history with the authorised follow-up outcome without silently deleting or rewriting prior historical facts. | L3-G-172 §§9, 12, 13, 16 |
| L4-G-172-BH-005 | When further follow-up, observation, referral, prescription or certificate activity is required, the system shall preserve the hand-off to the applicable authoritative healthcare workflow. | L3-G-172 §§9, 10, 16 |

## 5. Cross-Domain & Event Behavior

G06 remains the canonical publisher of Medical Encounter, Referral and healthcare-history facts. C may provide student context and E provides professional context without ownership transfer. The exact event for referral-follow-up completion/history update is not explicitly identified in the retrieved authoritative source; none is invented.

## 6. Failure / Rejection / Continuation

- If Referral cannot be associated with the patient/Medical Encounter, the system shall prevent an unsupported history update.
- If follow-up requirements are unclear, the system shall retain the unresolved condition until authorised healthcare assessment establishes the outcome.
- If external information is unavailable, the system shall record only information actually available to the institution.
- If urgent care is required, the system shall support continuation into the applicable emergency/medical workflow without falsely closing the follow-up.
- If the outcome is not yet determined, the system shall preserve the follow-up activity until authorised assessment is complete.
- Conflicting historical information shall not cause silent destructive rewriting.

## 7. Audit & Evidence Behavior

The system shall preserve patient reference, Medical Encounter, Referral reference, referral reason/provider context, follow-up requirement, assessment, healthcare outcome, responsible professional and resulting follow-up/referral/prescription/certificate action where applicable.

## 8. Observable Completion Conditions

Completion is observable when the authorised follow-up outcome is recorded in institutional medical history, referral history remains traceable, and any subsequent healthcare workflow is established where required.

## 9. Traceability & Provenance

**Parent:** L3-G-172 — Referral Follow-up to Updated Healthcare History  
**Primary verification source:** L2-G06 — Institutional Health Services & Medical Workflows  
**L3 anchors:** §§3, 4, 5, 7, 8, 9, 11, 12, 13, 15, 16

## 10. Open Issues / Deferred Design

The exact canonical event and contract semantics for referral follow-up completion are not explicitly identified in the retrieved L2 material and require upstream verification before Architecture Approval/Lock. Technical exchange, persistence, interfaces and access control remain deferred.