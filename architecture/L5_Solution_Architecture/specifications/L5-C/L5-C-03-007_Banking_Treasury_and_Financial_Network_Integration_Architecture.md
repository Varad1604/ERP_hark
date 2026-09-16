# Banking, Treasury & Financial Network Integration Architecture

## 1. Specification Metadata
**Specification ID:** `L5-C-03-007`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** `L5-C — Contracts, Events & Integration Architecture`  
**Major Area:** `L5-C-03 — External Integration & Connector Architecture`  
**Canonical Technical Responsibility:** Banking/treasury/network integration while Finance retains business ownership.  
**Technical Architecture Owner:** `L5-C-03`  
**Assigned Author:** Pallav  
**Version:** `1.0.2`  
**Status:** `Locked`  
**Last Updated:** `2026-09-16`  
**Depends On:** `L5-C-03-001`; `L5-C-03-002`; `L5-C-01`; `L5-D-03`; `L5-E-01`; applicable Finance/Treasury L5 architecture  
**Applicable ADRs:** `ADR-L5-0002`; `ADR-L5-0009`; `ADR-L5-0014`; `ADR-L5-0016`  
**Governance Authorization:** `L5_Pallav_Stage5_Architecture_Approval_Decision_2026-09-16.md`  
**Governance Lock:** `L5_Pallav_Stage5_Final_Lock_Decision_2026-09-16.md`

## 2. Purpose, Scope & Non-Goals
Defines secure provider-neutral integration with banks, treasury services and financial networks while Finance/Treasury remain authoritative for execution, settlement, Cash Position, reconciliation and ledger outcomes.

## 3. Upstream Applicability & Constraints
### 3.1 Canonical Upstream Reference Matrix
| Source | Version / Status | Exact BHs | C03 Technical Contribution |
|---|---|---|---|
| `L4-F-057` | 1.0.0 / Draft | `BH-001`–`BH-013` | Submit only approved payment instructions, preserve execution/settlement distinction, correlate per-payment outcomes and return evidence to Treasury. |
| `L4-F-061` | 1.0.0 / Draft | `BH-001`–`BH-015` | Compare bank evidence with Treasury facts, keep uncertain outcomes unresolved, prevent false/duplicate settlement, preserve final evidence. |
| `ADR-L5-0016` | Architecture Approved | shared C03 connector pattern | Governs financial-network adapter isolation, correlation, credentials and ambiguous-outcome containment. |

Both cited L4 sources remain Draft, but their identified behavior is explicitly governance-authorized for this Stage-5 approval. Canonical relation state remains `Specified`, not `Validated`, until the locked traceability methodology's normal source-status conditions are met.

### 3.2 Authoritative Constraints
- Provider receipt/acknowledgement is not final settlement.
- Cash Position shall not be changed by C03.
- Bank statement/transaction data is external evidence for Treasury/Finance reconciliation, not direct ledger truth.
- Ambiguous external outcomes shall not be blindly resubmitted where duplicate settlement is possible.
- Each item in a payment population remains separately correlatable.

## 4. Technical Ownership & Dependencies
**Owns:** bank/network adapters, payment API/file submission, status retrieval, statement/transaction normalization, external reference correlation and secure delivery evidence.  
**Must not own:** payment approval, Treasury business lifecycle, settlement truth, Cash Position, reconciliation rules, journal/accounting entries or treasury policy.

## 5. Solution Architecture
### 5.1 Elements
| Element | Responsibility | Boundary |
|---|---|---|
| Financial Network Adapter | Encapsulates bank/network API, secure-file or network-specific interface. | No Treasury business decision. |
| Payment Submission Correlator | Preserves approved batch/item reference and provider reference. | Maintains per-item traceability. |
| Statement/Transaction Importer | Validates/normalizes provider evidence. | Hands to Finance/Treasury reconciliation; no direct ledger mutation. |
| Status Resolver | Retrieves provider execution/settlement evidence. | Returns unresolved disposition where evidence is inconclusive. |
| Secure File Exchange Boundary | Packages and verifies file-based delivery where provider contract requires it. | Reuses C03-008 mechanics rather than duplicating business ownership. |

### 5.2 Primary Flow
```text
Treasury/Finance-approved instruction
→ bank/network adapter + D03 credential
→ secure provider exchange
→ acknowledgement/status/settlement evidence
→ correlate and normalize evidence
→ Treasury/Finance establishes authoritative execution/settlement/cash outcome
```

Inbound statement flow terminates at Finance/Treasury reconciliation intake, not at ledger mutation.

### 5.3 Failure Rules
Failed/returned/uncertain provider outcomes remain evidence until Treasury determines the authoritative disposition. Automatic resubmission is prohibited where duplicate settlement cannot be excluded. A technical file/API receipt shall not be represented as `SettlementCompleted`.

## 6. Architecture Decisions
| ID | Decision | Position | Basis | ADR |
|---|---|---|---|---|
| AD-001 | Integration modes | Common adapter boundary may support API, secure-file and network-specific provider mechanisms. | Frozen C03-007 responsibility + provider variability. | `ADR-L5-0016` |
| AD-002 | Financial authority | Bank/network responses remain evidence; Treasury/Finance owns canonical outcome. | F-057/F-061. | `ADR-L5-0016` ownership-preserving handoff. |
| AD-003 | Reconciliation boundary | C03 normalizes external evidence; Finance/Treasury owns matching/reconciliation. | `L4-F-061-BH-003`,`012`–`015`. | `ADR-L5-0016` |
| AD-004 | Ambiguous outcome | Preserve unresolved correlation; no inferred settlement or blind replay. | `L4-F-061-BH-004`,`007`–`008`. | `ADR-L5-0014`; `ADR-L5-0016` |

## 7. Security, Privacy & Operations
Provider credentials/certificates consume D03. Sensitive bank/account/payment data is minimized in telemetry. Provider health is independently observable and isolated under E01/E05. File-based exchanges consume C03-008 rather than create a second file-transfer architecture.

## 8. Architecture Conformance Requirements
- `CR-001`: C03 shall not create Treasury/Finance settlement, Cash Position, ledger or reconciliation truth from transport acknowledgement alone.
- `CR-002`: Bank statements/transactions shall be evidence inputs to canonical reconciliation, not direct authoritative updates.
- `CR-003`: Ambiguous submissions shall not be blindly resent where duplicate settlement is possible.
- `CR-004`: Every submitted financial operation shall preserve SutraERP/provider correlation and per-item traceability.
- `CR-005`: Provider-specific interfaces shall remain isolated behind C03 adapters.
- `CR-006`: File-based banking integration shall consume C03-008 generic transfer mechanics.

## 9. Open Decisions & Deferred Design
| ID | Item | Classification | Blocking? | Resolution |
|---|---|---|---|---|
| OD-001 | Promotion of F-057/F-061 mappings from `Specified` to `Validated`. | Traceability | No for this Architecture Approved specification. | Revalidate after applicable L4 approval. |
| OD-002 | Specific bank/network/provider protocol/product. | Technology | No. | Bind only from approved provider requirements. |

## 10. Traceability Summary
**Governance-authorized / Specified L4:** `L4-F-057-BH-001`–`013`; `L4-F-061-BH-001`–`015`. **Peers:** C03-001/002/008, C01, D03, E01, Finance/Treasury. **Applicable shared ADR:** `ADR-L5-0016`. **Blocking:** none.
