# Conflicting Exception Decisions to Single Authoritative Exception Outcome

## 1. Specification Metadata


**Specification ID:** ** `L4-I-181`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Examination Exception & Contingency (I15)  
**Realizes Scenario:** ** `L3-I-181` — Conflicting Exception Decisions to Single Authoritative Exception Outcome  
**Primary Business Owner:** Examination Exception & Contingency (I15)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-I-181`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support reconciliation of contradictory examination-contingency decisions made by affected centres for the same examination, producing one authoritative institutional outcome that preserves inter-centre academic parity.

**Start Boundary:** The CoE detects conflicting conduct dispositions across centres for the same examination code.

**End Boundary:** The Exception Reconciliation Docket reaches `HARMONIZED_OUTCOME_DECREED` and one authoritative remediation decision supersedes conflicting local dispositions.

The specification remains implementation-independent. It does not prescribe APIs, data stores, message brokers, cryptographic mechanisms, hardware, notification technology, infrastructure, technical IAM, or UI/UX.

---

## 3. Required Business Context

- Canonical owner: Examination Exception & Contingency (I15).
- Canonical aggregate: **Exception Reconciliation Docket Aggregate**.
- Entry state: `CONFLICTING_CENTRE_OUTCOMES`.
- Primary parent rule: `BR-I15-010`.
- `BR-I-AUTH` — Dual Executive Sign-off Rule.
- `BR-I-AUDIT` — audit evidence must be durable and tamper-evident; no specific technology is prescribed at L4.
- Parent-defined cross-domain facts remain owned by their canonical source domains.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-181-BH-001` | The system shall establish/preserve the reconciliation docket in `CONFLICTING_CENTRE_OUTCOMES` when contradictory centre decisions are detected. | L3-I-181 §§4, 9, 13 |
| `L4-I-181-BH-002` | The system shall preserve each affected centre's decision and supporting operational evidence as distinct historical inputs. | L3-I-181 §§4, 9, 13 |
| `L4-I-181-BH-003` | The system shall establish that the decisions concern the same examination context before harmonization. | L3-I-181 §§4, 9, 13 |
| `L4-I-181-BH-004` | The system shall enforce `BR-I15-010` so the final resolution protects inter-centre academic parity. | L3-I-181 §§4, 9, 13 |
| `L4-I-181-BH-005` | The system shall preserve the CoE central review and affected-centre attestations used to evaluate the conflict. | L3-I-181 §§4, 9, 13 |
| `L4-I-181-BH-006` | The system shall preserve the authoritative remediation decision, including whether it invalidates unsupported local variances. | L3-I-181 §§4, 9, 13 |
| `L4-I-181-BH-007` | The system shall preserve required governance/legal and independent approval evidence. | L3-I-181 §§4, 9, 13 |
| `L4-I-181-BH-008` | The system shall prevent finalization while material centre evidence, parity impact, or approval authority remains unresolved. | L3-I-181 §§4, 9, 13 |
| `L4-I-181-BH-009` | On valid resolution, the system shall transition the docket to `HARMONIZED_OUTCOME_DECREED`. | L3-I-181 §§4, 9, 13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-181-BH-010` | The system shall consume parent-defined examination, governance, campus, finance, communications, or other participating-domain facts only from their canonical owners. | L3-I-181 §§4, 9, 13 |
| `L4-I-181-BH-011` | When the canonical result is reached, the system shall publish `Conflicting_Exception_Decisions_Harmonized` on behalf of Examination Exception & Contingency. | L3-I-181 §§4, 9, 13 |
| `L4-I-181-BH-012` | Where the Locked parent requires `Examination_Audit_Checkpoint_Sealed`, the system shall preserve the corresponding governed audit checkpoint without prescribing hashing, blockchain, vault, or transport technology. | L3-I-181 §§4, 9, 13 |

---

## 6. Failure / Rejection / Continuation

- If the centre outcomes relate to different examination contexts, they shall not be reconciled through this scenario.
- If parity impact cannot be established, the conflict shall remain unresolved.
- Any grade normalization or universal re-examination must be the explicit content of the authoritative decree; L4 shall not invent which remedy applies.

---

## 7. Audit & Evidence Behavior

The system shall preserve:

- each centre disposition;
- common examination context;
- centre operational evidence;
- parity assessment;
- central review evidence;
- authoritative remediation decree;
- approval/legal evidence;
- state transition and canonical events;

The evidence shall remain attributable and historically traceable; prior decisions or records shall not be silently overwritten.

---

## 8. Observable Completion Conditions

Completion requires:

- the docket is `HARMONIZED_OUTCOME_DECREED`;
- all conflicting centre decisions remain traceable;
- one authoritative outcome is established;
- inter-centre parity basis is evidenced;
- `Conflicting_Exception_Decisions_Harmonized` is published;

---

## 9. Traceability & Provenance

**Realizes Scenario:** `L3-I-181`  
**Canonical Aggregate:** Exception Reconciliation Docket Aggregate  
**Entry State:** `CONFLICTING_CENTRE_OUTCOMES`  
**Resulting State:** `HARMONIZED_OUTCOME_DECREED`  
**Canonical Event:** `Conflicting_Exception_Decisions_Harmonized`  
**Owning Domain:** Examination Exception & Contingency (I15)  
**Primary Rule:** `BR-I15-010`

No new business rule, lifecycle state, event, canonical owner, or business authority is introduced.

---

## 10. Open Issues / Deferred Design

**Upstream scope note:** The parent end boundary allows either uniform grade normalization or a universal re-examination decree. L4 does not choose between these alternatives; the authoritative decision must explicitly state the remedy. Parent references to telemetry, scanners, vaults, blockchain, and message brokers are treated as implementation leakage and deferred.

Implementation details deferred to later levels include persistence, APIs, event transport, communication channels, cryptographic sealing, scanners, physical vault technology, blockchain/ledger implementation, technical access controls, infrastructure, and UI/UX.
