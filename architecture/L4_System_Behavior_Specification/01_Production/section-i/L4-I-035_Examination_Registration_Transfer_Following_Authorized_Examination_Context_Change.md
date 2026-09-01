# L4-I-035 — Examination Registration Transfer Following Authorized Examination Context Change

## 1. Specification Metadata
**Specification ID:** L4-I-035  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination Registration  
**Realizes Scenario:** L3-I-035 — Examination Registration Transfer Following Authorized Examination Context Change  
**Primary Business Owner:** Examination Registration  
**Version:** 1.0  
**Status:** Draft

## 2. Behavioral Objective & Boundary
Govern transfer of an examination registration following an authorised change to its examination context while preserving registration integrity and history.

**Start:** An authoritative examination-context change affects an existing registration.  
**End:** The registration is transferred to the authorised resulting context or a governed non-transfer outcome is recorded.

## 3. Required Business Context
- Existing Examination Registration is authoritative.
- Source and destination examination contexts are identifiable.
- The context change is authorised.
- Applicable registration policy permits the transfer or defines its exception path.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-035-BH-001 | Identify registrations affected by the authorised context change. | L3-I-035 |
| L4-I-035-BH-002 | Validate source and destination examination contexts and candidate eligibility. | L3-I-035 |
| L4-I-035-BH-003 | Determine the permitted registration transfer scope. | L3-I-035 |
| L4-I-035-BH-004 | Establish the registration in the authorised destination context without creating conflicting participation. | L3-I-035 |
| L4-I-035-BH-005 | Preserve source context, destination context, decision authority, and transfer history. | L3-I-035 |

## 5. Cross-Domain & Event Behavior
The originating examination context remains authoritative for the context change; Examination Registration owns the registration consequence. Only canonical registration lifecycle events are used.

## 6. Failure / Rejection / Continuation
- Unauthorised context changes shall not transfer registration.
- Ineligible destination contexts shall prevent transfer.
- Conflicting existing destination registration shall be resolved through the applicable governed process rather than silently duplicated.

## 7. Audit & Evidence Behavior
Preserve source/destination context, eligibility, authority, decision, resulting registration, and historical transfer evidence.

## 8. Observable Completion Conditions
Completion occurs when the registration is authoritative in the permitted destination context and the transfer history is traceable, or a governed non-transfer outcome is recorded.

## 9. Traceability & Provenance
**Parent:** L3-I-035 — Examination Registration Transfer Following Authorized Examination Context Change

## 10. Open Issues / Deferred Design
Exact context-transfer constraints remain governed by applicable examination policy. Technical migration and synchronization mechanisms remain outside L4.