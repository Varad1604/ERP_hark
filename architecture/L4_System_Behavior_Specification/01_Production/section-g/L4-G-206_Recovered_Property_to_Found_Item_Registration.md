# Recovered Property to Found Item Registration

## 1. Specification Metadata


**Specification ID:** ** `L4-G-206`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations — Security Operations (G07)  
**Realizes Scenario:** ** `L3-G-206` — Recovered Property to Found Item Registration  
**Primary Business Owner:** Security Operations (G07)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-G-206_Recovered_Property_to_Found_Item_Registration.md`; `L2-G07_Security_Operations_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** ** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed registration and custody intake of recovered property from surrender at Security Operations through verified cataloging, controlled vault custody, and issuance of the parent-supported intake acknowledgment.

The behavior begins when found property is surrendered to Security Operations. It ends when the Found Property Item Aggregate reaches `FOUND_ITEM_VAULTED`.

This specification does not govern ownership verification, claimant approval, property handover, administrative disposal, vault hardware, or barcode technology.

---

## 3. Required Business Context

- Active Security Operations lost-and-found context.
- Finder / surrendering officer identity.
- Found property article and observable identifying characteristics.
- Lost & Found Vault Custodian.
- Security Post Supervisor participation where required.
- Parent-defined dual-witness vault-storage rule.
- Security Operations ownership of the Found Property Item Aggregate.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-G-206-BH-001` | When found property is surrendered, the system shall establish the Found Property Item Aggregate in `INTAKE_PENDING`. | L3-G-206 §§4, 9, 13 |
| `L4-G-206-BH-002` | The system shall preserve the identity of the finder or surrendering officer and the recovered-property intake context. | L3-G-206 §§2–4 |
| `L4-G-206-BH-003` | The system shall preserve the found item's catalog information required for later identification and claim processing. | L3-G-206 §§2–3 |
| `L4-G-206-BH-004` | The system shall require the parent-defined verification and custody checks before the item is treated as vaulted. | L3-G-206 §§9, 12 |
| `L4-G-206-BH-005` | The system shall require the parent-defined dual-witness custody evidence for vault storage. | L3-G-206 §12 |
| `L4-G-206-BH-006` | The system shall prevent normal registration completion when required intake, custody, or witness evidence is missing. | L3-G-206 §11 |
| `L4-G-206-BH-007` | On successful governed custody intake, the system shall transition the Found Property Item Aggregate from `INTAKE_PENDING` to `FOUND_ITEM_VAULTED`. | L3-G-206 §§9, 13, 16 |
| `L4-G-206-BH-008` | The system shall preserve issuance of the parent-supported intake receipt or acknowledgment to the finder without prescribing its technical format. | L3-G-206 §§3–4 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-G-206-BH-009` | The system shall recognize authoritative identity facts required for intake without transferring identity ownership to Security Operations. | L3-G-206 §§5, 14 |
| `L4-G-206-BH-010` | When the Found Property Item Aggregate reaches `FOUND_ITEM_VAULTED`, the system shall produce the canonical `Found_Property_Registered` business event on behalf of Security Operations. | L3-G-206 §§14, 16 |

No barcode implementation, vault-access technology, persistence design, or event transport is defined.

---

## 6. Failure / Rejection / Continuation

- If finder identity, property verification, or required custody evidence cannot be established, the system shall not represent the item as `FOUND_ITEM_VAULTED`.
- If a custody dispute or security exception occurs, the system shall preserve the last valid business position for authorised escalation.
- Parent-supported manual evidence may be recognized without defining technical synchronization or recovery behavior.
- Ownership verification remains outside this specification.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- surrendering party;
- intake time and context;
- found-item description;
- custody verification;
- dual-witness evidence;
- Vault Custodian decision;
- resulting `FOUND_ITEM_VAULTED` position;
- finder receipt/acknowledgment;
- publication of `Found_Property_Registered`.

---

## 8. Observable Completion Conditions

L4 behavior is complete when:

- the Found Property Item Aggregate is `FOUND_ITEM_VAULTED`;
- required custody and witness evidence is preserved;
- finder acknowledgment is attributable;
- `Found_Property_Registered` has been produced;
- ownership verification and handover remain outside this specification.

---

## 9. Traceability & Provenance

**Parent:** `L3-G-206` — Recovered Property to Found Item Registration  
**Primary verification source:** `L2-G07_Security_Operations_Workflows.md` §8.2.1 (Configuration and Activation), §11 (Failure and Exceptions), §13 (Lifecycle Invariants), §14 (Event Contracts)  
**Higher-level verification:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`

All BHs derive from the Locked L3 parent.

---

## 10. Open Issues / Deferred Design

No material L4 architecture issue remains for this draft.

Barcode technology, vault-access controls, persistence, event transport, audit implementation, and UI/UX remain deferred.
