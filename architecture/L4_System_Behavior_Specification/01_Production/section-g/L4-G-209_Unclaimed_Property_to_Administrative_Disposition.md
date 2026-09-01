# Unclaimed Property to Administrative Disposition

## 1. Specification Metadata


**Specification ID:** ** `L4-G-209`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations — Security Operations (G07)  
**Realizes Scenario:** ** `L3-G-209` — Unclaimed Property to Administrative Disposition  
**Primary Business Owner:** Security Operations (G07)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-G-209_Unclaimed_Property_to_Administrative_Disposition.md`; `L2-G07_Security_Operations_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** ** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed administrative disposition of property that remains unclaimed after the parent-defined retention period, from eligibility determination through disposal approval, execution evidence, financial handoff where applicable, and archival closure.

The behavior begins when the statutory unclaimed-property audit identifies eligible items beyond the 90-day threshold. It ends when the Unclaimed Property Batch Aggregate reaches `DISPOSED_AND_ARCHIVED`.

This specification does not define auction platforms, destruction methods, donation logistics, accounting implementation, or physical disposal technology.

---

## 3. Required Business Context

- Unclaimed property retained by Security Operations.
- Parent-defined 90-day retention threshold.
- Disposal Committee Chair.
- Chief Security Officer.
- Finance Officer participation where disposal proceeds arise.
- Vault Custodian participation per the Locked parent.
- Approved disposal method: auction, donation, or destruction.
- Security Operations ownership of the Unclaimed Property Batch Aggregate.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-G-209-BH-001` | When the parent-defined retention threshold is met, the system shall establish or recognize the Unclaimed Property Batch Aggregate in `RETENTION_EXPIRED`. | L3-G-209 §§2, 4, 9, 13 |
| `L4-G-209-BH-002` | The system shall prevent administrative disposition before the parent-defined 90-day unclaimed-property retention requirement is satisfied. | L3-G-209 §12 |
| `L4-G-209-BH-003` | The system shall preserve the Disposal Committee Chair's governed approval of the selected parent-supported disposition path. | L3-G-209 §§2, 6, 15 |
| `L4-G-209-BH-004` | The system shall support only the parent-listed administrative disposition categories—auction, donation, or destruction—unless upstream architecture authorizes another disposition. | L3-G-209 §2 |
| `L4-G-209-BH-005` | Where disposal produces proceeds, the system shall preserve the handoff of the resulting financial fact to the Finance domain without taking ownership of Finance accounting. | L3-G-209 §§3, 5–6 |
| `L4-G-209-BH-006` | The system shall require disposition evidence, including the parent-supported destruction or auction certificate where applicable, before archival completion. | L3-G-209 §§3–4 |
| `L4-G-209-BH-007` | On successful governed disposition, the system shall transition the Unclaimed Property Batch Aggregate from `RETENTION_EXPIRED` to `DISPOSED_AND_ARCHIVED`. | L3-G-209 §§9, 13, 16 |
| `L4-G-209-BH-008` | The system shall preserve the administrative-disposition history after operational closure and shall not silently erase the governing evidence. | L3-G-209 §§3–4, 16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-G-209-BH-009` | The system shall recognize Finance-domain facts related to disposal proceeds without transferring ownership of accounting or fund-crediting behavior to Security Operations. | L3-G-209 §§3, 5–6 |
| `L4-G-209-BH-010` | When the Unclaimed Property Batch reaches `DISPOSED_AND_ARCHIVED`, the system shall produce the canonical `Unclaimed_Property_Disposed` business event on behalf of Security Operations. | L3-G-209 §§14, 16 |

No auction-service integration, finance posting design, document-storage implementation, or event transport is defined.

---

## 6. Failure / Rejection / Continuation

- If the 90-day retention requirement has not been met, the system shall prevent disposal progression.
- If Disposal Committee approval is absent, the system shall not represent the batch as approved for disposition.
- If required disposition evidence is incomplete, the system shall not represent the batch as archived.
- If financial proceeds arise, unresolved Finance handling shall remain a Finance-domain matter and shall not cause Security Operations to redefine Finance ownership.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- retention-expiry basis;
- items included in the batch;
- Disposal Committee approval;
- selected disposal method;
- execution evidence;
- Finance handoff where proceeds arise;
- destruction/auction certificate where applicable;
- resulting `DISPOSED_AND_ARCHIVED` position;
- publication of `Unclaimed_Property_Disposed`.

---

## 8. Observable Completion Conditions

L4 behavior is complete when:

- the Unclaimed Property Batch Aggregate is `DISPOSED_AND_ARCHIVED`;
- retention compliance and disposition approval are attributable;
- disposition evidence is preserved;
- any Finance handoff is recorded without transferring Finance ownership;
- `Unclaimed_Property_Disposed` has been produced.

---

## 9. Traceability & Provenance

**Parent:** `L3-G-209` — Unclaimed Property to Administrative Disposition  
**Primary verification source:** `L2-G07_Security_Operations_Workflows.md` §8.2.1 (Configuration and Activation), §11 (Failure and Exceptions), §13 (Lifecycle Invariants), §14 (Event Contracts)  
**Cross-domain participant:** Section F — Finance  
**Higher-level verification:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`

All BHs derive from the Locked L3 parent.

---

## 10. Open Issues / Deferred Design

No material L4 architecture issue is introduced in this draft.

The parent assigns explicit operational/approval roles to the Disposal Committee Chair, Chief Security Officer, Finance Officer, and Vault Custodian. L4 preserves only the unambiguous business obligations and does not redefine those role labels.

Auction technology, donation logistics, destruction process implementation, Finance posting, persistence, event transport, audit implementation, and UI/UX remain deferred.
