# Physical Infrastructure Attribute Change to Effective-Dated Governed Update

## 1. Specification Metadata

**Specification ID:** L4-G-280  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Infrastructure Management  
**Realizes Scenario:** L3-G-280  
**Primary Business Owner:** Infrastructure Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** `L3-G-280_Physical_Infrastructure_Attribute_Change_to_Effective_Dated_Governed_Update.md`; L2-G10 — Infrastructure Management Workflows; L2-A01 — Platform & Tenant Foundation; L2-D03 — Timetable Workflows; `L2-G10_Infrastructure_Management_Workflows_Amendment_2026-08-26.md`; `L2-G10_Infrastructure_Governance_Completion_Amendment_2026-08-26.md`  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

SutraERP shall govern a G10-owned physical-infrastructure attribute change while preserving aggregate identity, source ownership and historical evidence. Attribute-specific effective dating and event publication shall be applied only where the current Locked G10 architecture defines them; no universal attribute-version model is inferred across the frozen scope.

## 3. Required Business Context

- The affected Physical Campus, Building, Floor or Space exists and is not Archived.
- The requested attribute is G10-owned and editable under the current business condition.
- The proposed value, business reason, authorization and applicable validation policy are available.
- Consumers such as D03 remain read-only consumers of resulting G10 facts.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-280-BH-001 | The system shall resolve the affected canonical G10 aggregate, target attribute and current authoritative value before applying a change. | L3-G-280 §§7,9 |
| L4-G-280-BH-002 | The system shall reject a change when the attribute is not G10-owned, authorization is insufficient, or the proposed value violates applicable G10 rules. | L3-G-280 §§5–7,9,11–12 |
| L4-G-280-BH-003 | The system shall preserve immutable aggregate identity and shall not replace the aggregate merely because an attribute value changes. | L3-G-280 §§2,9,12–13 |
| L4-G-280-BH-004 | The system shall apply effective dating only for attribute classes whose current Locked G10 rules explicitly define an effective-date policy; it shall not generalize effective dating to every attribute in the frozen trigger. | L3-G-280 §§3,9,12,16–17 |
| L4-G-280-BH-005 | Where a source-defined successor value becomes authoritative, the system shall make the current G10 fact available to authorized consumers without transferring mutation authority. | L3-G-280 §§3–5,9,12,14,16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-280-BH-006 | For a Capacity Profile establishment/change, the system shall use the later Locked canonical event `CapacityUpdated` from G10 and shall not publish legacy `CapacityChanged` or `RoomCapacityChanged` as separate aliases. | L3-G-280 §§10,14; L2-G10 Governance Completion Amendment §4 |
| L4-G-280-BH-007 | For other attribute classes, the system shall publish only the exact source-defined G10 event applicable to that aggregate/attribute combination and shall not infer an event where the current source is ambiguous or silent. | L3-G-280 §§9,12,14,17 |

Later G10 amendments normalize several neighboring event vocabularies but do not establish one universal event/effective-dating contract for all metadata, capability, sustainability and security-classification combinations covered by this parent.

## 6. Failure / Rejection / Continuation

Unsupported ownership, invalid value or missing authorization shall leave the current fact unchanged. If the effective-date rule or exact completed event is undefined for the requested attribute class, the system shall not claim the frozen governed-update completion.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-280-BH-008 | The system shall preserve aggregate identity, attribute name, predecessor and successor values, effective date where canonically defined, actor/authorization context, business reason and the exact source-defined completed fact where available. | L3-G-280 §§15–16 |

## 8. Observable Completion Conditions

Completion is observable only for an attribute class that has an explicit current G10 validation/effective-date rule and an exact canonical completed event. Because the frozen parent spans combinations not uniformly defined upstream, the specification remains partially blocked for those unsupported combinations.

## 9. Traceability & Provenance

**Parent:** L3-G-280 — Physical Infrastructure Attribute Change to Effective-Dated Governed Update  
**Primary verification sources:** L2-G10 — Infrastructure Management Workflows; L2-A01; L2-D03  
**Current-source checks:** both Locked G10 amendments dated 2026-08-26, including `CapacityUpdated` normalization.

## 10. Open Issues / Deferred Design

No universal effective-dated attribute-version workflow exists across the entire frozen trigger, and not every aggregate/attribute combination has one exact canonical event. Those parent blockers remain except where later G10 amendments explicitly normalize a specific attribute family.
