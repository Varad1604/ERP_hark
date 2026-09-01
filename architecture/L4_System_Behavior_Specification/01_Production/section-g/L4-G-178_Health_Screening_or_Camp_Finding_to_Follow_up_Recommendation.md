# Health Screening or Camp Finding to Follow-up Recommendation

## 1. Specification Metadata

**Specification ID:** L4-G-178  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Institutional Health Services  
**Realizes Scenario:** L3-G-178 — Health Screening or Camp Finding to Follow-up Recommendation  
**Primary Business Owner:** Institutional Health Services  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** Approved L3-G-178; locked L2-G06 healthcare workflow/event vocabulary  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

The system shall support assessment of a material screening/camp finding and recording the resulting follow-up decision without creating a local finding lifecycle or absorbing the downstream healthcare activity.

## 3. Required Business Context

- Finding originates from a G06 Health Program/screening activity.
- Participant identity is authoritative from C/E where applicable.
- Assessment requires authorised healthcare responsibility.
- Follow-up is a separate G06 healthcare outcome where required.
- No generic screening-finding event is established upstream in the source scenario.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-178-BH-001 | When a material screening/camp finding is identified, the system shall preserve its originating programme and participant context for authorised assessment. | L3-G-178 §§8,9 |
| L4-G-178-BH-002 | The system shall prevent a final follow-up decision when participant identity, finding evidence or authorised healthcare assessment is insufficient. | L3-G-178 §§7,11 |
| L4-G-178-BH-003 | When assessment determines that follow-up is required, the system shall record the authorised follow-up recommendation and preserve its relationship to the originating finding. | L3-G-178 §§9,16 |
| L4-G-178-BH-004 | When no follow-up is required, the system shall record that outcome without creating an unsupported follow-up lifecycle state. | L3-G-178 §§9,13 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-178-BH-005 | Where a resulting healthcare activity has a canonical G06 event, the system shall use that authoritative event and G06 shall remain its publisher. | L3-G-178 §14 |
| L4-G-178-BH-006 | The system shall consume authoritative C/E participant context where required without transferring ownership. | L3-G-178 §§5,14 |

## 6. Failure / Rejection / Continuation

- If the participant cannot be reliably identified, the system shall not attach the finding or recommendation to an uncertain identity.
- If the finding is insufficiently documented, the system shall retain the business context and prevent unsupported final assessment.
- If urgent care is identified, the system shall permit continuation into the applicable emergency/healthcare workflow.
- A recommendation shall not be treated as completion of the recommended future care.

## 7. Audit & Evidence Behavior

The system shall preserve originating programme, participant context, finding, assessment, assessing professional, follow-up decision/recommendation, resulting healthcare activity reference and applicable canonical event evidence.

## 8. Observable Completion Conditions

Completion occurs when the finding has an authorised assessment outcome and either a supported follow-up recommendation is recorded or no follow-up is determined to be required. Subsequent care remains separately governed.

## 9. Traceability & Provenance

**Parent:** L3-G-178 — Health Screening or Camp Finding to Follow-up Recommendation  
**Primary verification source:** L2-G06 — Health Program and healthcare workflow/event definitions  

## 10. Open Issues / Deferred Design

No generic finding event or clinical severity matrix is invented. Detailed clinical criteria, APIs, persistence, notifications and infrastructure are deferred.