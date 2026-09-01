# Vaccination Programme to Recorded Vaccination Outcome

## 1. Specification Metadata

**Specification ID:** L4-G-176  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Institutional Health Services  
**Realizes Scenario:** L3-G-176 — Vaccination Programme to Recorded Vaccination Outcome  
**Primary Business Owner:** Institutional Health Services  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** Approved L3-G-176; locked L2-G06 vaccination lifecycle/event definitions  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

The system shall support a G06 Vaccination Campaign from planning through running activity to completion and shall record each applicable vaccination outcome without inventing vaccine-specific clinical policy.

## 3. Required Business Context

- Vaccination Campaign is G06-owned.
- Lifecycle: `Planned → Running → Completed`.
- Participant context comes from authoritative C/E sources where applicable.
- Vaccination outcomes are institutional healthcare records.
- Individual follow-up remains separately governed.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-176-BH-001 | When an authorised vaccination requirement is established, the system shall establish the Vaccination Campaign in `Planned` with applicable programme and participant context. | L3-G-176 §§8,9 |
| L4-G-176-BH-002 | When the campaign is ready for operational conduct, the system shall transition it to `Running`. | L3-G-176 §§9,13 |
| L4-G-176-BH-003 | For each participant receiving the vaccination activity, the system shall record the applicable vaccination outcome against the authoritative participant and healthcare context. | L3-G-176 §§9,16 |
| L4-G-176-BH-004 | The system shall prevent an unsupported completed vaccination outcome when participant identity or healthcare evidence is not established. | L3-G-176 §§7,11 |
| L4-G-176-BH-005 | The system shall transition the campaign to `Completed` only when its configured completion condition is satisfied. | L3-G-176 §§9,13 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-176-BH-006 | On each authoritative recorded vaccination outcome, the system shall publish the canonical `VaccinationRecorded` event with G06 as publisher. | L3-G-176 §14 |
| L4-G-176-BH-007 | Where participant context is required, the system shall consume authoritative C/E context without transferring identity ownership. | L3-G-176 §§5,14 |

## 6. Failure / Rejection / Continuation

- If participant identity cannot be verified, the system shall not attach a vaccination outcome to that participant.
- If vaccination activity is interrupted, the campaign shall remain in its last valid business position.
- If a participant does not receive vaccination, the system shall record only an outcome supported by authoritative programme policy.
- The system shall not complete the campaign before its completion condition is met.

## 7. Audit & Evidence Behavior

The system shall preserve campaign identity, participant context, responsible healthcare professional, vaccination activity/outcome, lifecycle transitions and `VaccinationRecorded` evidence where applicable.

## 8. Observable Completion Conditions

Completion occurs when the campaign reaches `Completed` with materially required vaccination outcomes recorded and applicable individual follow-up separately established. A participant-level outcome does not imply campaign completion.

## 9. Traceability & Provenance

**Parent:** L3-G-176 — Vaccination Programme to Recorded Vaccination Outcome  
**Primary verification source:** L2-G06 — Vaccination Campaign lifecycle and `VaccinationRecorded` event  

## 10. Open Issues / Deferred Design

Vaccine eligibility, dosage, consent, contraindication and public-health reporting rules are not invented here. Technical scheduling, persistence, APIs, portal/UI and infrastructure are deferred.