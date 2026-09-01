# Lost Property Report to Lost Item Registration

## 1. Specification Metadata


**Specification ID:** ** `L4-G-205`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations — Security Operations (G07)  
**Realizes Scenario:** ** `L3-G-205` — Lost Property Report to Lost Item Registration  
**Primary Business Owner:** Security Operations (G07)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-G-205_Lost_Property_Report_to_Lost_Item_Registration.md`; `L2-G07_Security_Operations_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** ** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed registration of a lost-property report from complainant intake through validated cataloging and publication of a searchable lost-item record while protecting distinctive identifying details required by the parent rule.

The behavior begins when an individual lodges a lost-property report. It ends when the Lost Property Dossier Aggregate reaches `LOST_ITEM_REGISTERED`.

This specification does not govern later found-item matching, claimant verification for return, physical storage, portal/UI design, or disposition of unclaimed property.

---

## 3. Required Business Context

- Active Security Operations lost-and-found context.
- Complainant identity.
- Lost item description and distinctive identifying characteristics.
- Lost & Found Security Desk Officer.
- Parent-defined non-disclosure rule for distinctive identifying marks.
- Inquiry-token context where required.
- Security Operations ownership of the Lost Property Dossier Aggregate.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-G-205-BH-001` | When a lost-property report is accepted, the system shall establish the Lost Property Dossier Aggregate in `INTAKE_DRAFT`. | L3-G-205 §§4, 9, 13 |
| `L4-G-205-BH-002` | The system shall preserve sufficient item-description and complainant context to create the authoritative lost-property record. | L3-G-205 §§2–3 |
| `L4-G-205-BH-003` | The system shall retain distinctive identifying marks required for later verification while enforcing the parent-defined non-disclosure constraint on those details. | L3-G-205 §§2, 12 |
| `L4-G-205-BH-004` | The system shall preserve the inquiry-token or equivalent governed reference associated with the lost-property registration where required by the parent. | L3-G-205 §2 |
| `L4-G-205-BH-005` | The system shall prevent registration completion when required complainant or lost-item verification conditions fail. | L3-G-205 §11 |
| `L4-G-205-BH-006` | The system shall require the parent-defined Lost & Found Security Desk Officer review before final registration. | L3-G-205 §§6, 9, 15 |
| `L4-G-205-BH-007` | On successful governed registration, the system shall transition the Lost Property Dossier Aggregate from `INTAKE_DRAFT` to `LOST_ITEM_REGISTERED`. | L3-G-205 §§9, 13, 16 |
| `L4-G-205-BH-008` | The system shall make a searchable lost-property representation available while excluding distinctive identifying details that the parent rule requires to remain undisclosed. | L3-G-205 §§3–4, 12 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-G-205-BH-009` | The system shall recognize authoritative complainant identity facts supplied by their owning domain without transferring identity ownership to Security Operations. | L3-G-205 §§5, 14 |
| `L4-G-205-BH-010` | When the Lost Property Dossier reaches `LOST_ITEM_REGISTERED`, the system shall produce the canonical `Lost_Property_Registered` business event on behalf of Security Operations. | L3-G-205 §§14, 16 |

No portal implementation, search-index technology, message schema, or event transport is defined.

---

## 6. Failure / Rejection / Continuation

- If required identity or lost-item verification fails, the system shall prevent normal registration completion.
- If distinctive identifying details would be exposed contrary to the parent rule, the system shall prevent that disclosure while retaining the governed verification data.
- If an unresolved dispute occurs during intake, the system shall preserve the last valid non-registered position for authorised handling.
- Parent-supported manual evidence may be recognized without defining technical synchronization mechanics.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- complainant and intake context;
- lost-item description;
- distinctive identifying details retained for controlled verification;
- non-disclosure treatment;
- inquiry reference/token where applicable;
- Lost & Found Security Desk Officer review;
- resulting `LOST_ITEM_REGISTERED` position;
- publication of `Lost_Property_Registered`.

---

## 8. Observable Completion Conditions

L4 behavior is complete when:

- the Lost Property Dossier Aggregate is `LOST_ITEM_REGISTERED`;
- required item and complainant information is preserved;
- non-disclosure of distinctive identifying marks is enforced at the business-behavior level;
- `Lost_Property_Registered` has been produced;
- later matching, return, and unclaimed-property disposition remain outside this specification.

---

## 9. Traceability & Provenance

**Parent:** `L3-G-205` — Lost Property Report to Lost Item Registration  
**Primary verification source:** `L2-G07_Security_Operations_Workflows.md` §8.2.1 (Configuration and Activation), §11 (Failure and Exceptions), §13 (Lifecycle Invariants), §14 (Event Contracts)  
**Higher-level verification:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`

All BHs derive from the Locked L3 parent.

---

## 10. Open Issues / Deferred Design

No material L4 architecture issue remains for this draft.

Search technology, portal/UI implementation, physical lost-property storage, persistence, event transport, audit implementation, and UI/UX remain deferred.
