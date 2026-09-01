# L4-G-129 — Library Copy Registration to Circulation Availability

## 1. Specification Metadata
**Specification ID:** L4-G-129  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Library Management  
**Realizes Scenario:** L3-G-129 — Library Copy Registration to Circulation Availability  
**Primary Business Owner:** Library Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01

## 2. Behavioral Objective & Boundary
SutraERP shall govern registration and validation of a physical or otherwise governed library copy until it becomes available for circulation.

**Start:** A library copy is identified for registration.  
**End:** The copy is Available for Circulation or reaches a governed non-availability outcome.

## 3. Required Business Context
- An authoritative catalogue resource exists or the applicable resource identity is available.
- Applicable copy-registration and circulation rules are available.
- Library Management owns copy lifecycle.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-129-BH-001 | Register the library copy against its authoritative catalogue resource. | L3-G-129 |
| L4-G-129-BH-002 | Validate copy information required for circulation. | L3-G-129 |
| L4-G-129-BH-003 | Establish the copy in its governed operational condition. | L3-G-129 |
| L4-G-129-BH-004 | When circulation criteria are satisfied, make the copy Available for Circulation. | L3-G-129 |
| L4-G-129-BH-005 | Preserve copy registration and availability history. | L3-G-129 |

## 5. Cross-Domain & Event Behavior
The copy availability outcome shall be available to authorised library consumers without transferring Library Management ownership. No unsupported event name is introduced.

## 6. Failure / Rejection / Continuation
- Invalid copy information shall prevent circulation availability.
- A copy failing required condition checks shall remain unavailable until resolved.
- Registration history shall not be discarded when availability changes.

## 7. Audit & Evidence Behavior
Preserve copy identity, catalogue relationship, validation, condition, authority, and availability evidence.

## 8. Observable Completion Conditions
Completion occurs when the registered copy satisfies circulation criteria and is Available for Circulation, or a governed non-availability outcome is recorded.

## 9. Traceability & Provenance
**Parent:** L3-G-129 — Library Copy Registration to Circulation Availability

## 10. Open Issues / Deferred Design
Technical asset identification, storage, barcode/RFID implementation, and interfaces remain outside L4.