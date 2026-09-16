# Payment Gateway Integration Architecture

## 1. Specification Metadata
**Specification ID:** `L5-C-03-003`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** `L5-C — Contracts, Events & Integration Architecture`  
**Major Area:** `L5-C-03 — External Integration & Connector Architecture`  
**Canonical Technical Responsibility:** Multi-gateway payment integration while Finance retains canonical financial/payment state.  
**Technical Architecture Owner:** `L5-C-03`  
**Assigned Author:** Pallav  
**Version:** `1.0.2`  
**Status:** `Draft`  
**Last Updated:** `2026-09-16`  
**Depends On:** `L5-C-03-001`; `L5-C-03-002`; `L5-C-01`; `L5-D-03`; `L5-E-01`; applicable Finance L5 architecture  
**Applicable ADRs:** `ADR-L5-0002`; `ADR-L5-0009`; `ADR-L5-0014`; `ADR-L5-0016`

## 2. Purpose, Scope & Non-Goals
Defines provider-neutral payment-gateway integration for provider initiation, redirect/callback handling, trusted provider verification, refunds and technical reconciliation evidence. Finance remains owner of Payment, refund, settlement, ledger and reconciliation business state.

## 3. Upstream Applicability, Constraints & Assumptions
### 3.1 Canonical Upstream Reference Matrix
| L4 Source | Version / Status | Exact BHs | Required C03 Contribution |
|---|---|---|---|
| `L4-F-007` | 1.0.1 / Draft | `BH-001`–`003`, `BH-007`–`011` | Preserve source/instrument/channel context, support validated provider evidence, immutable Payment/Receipt references and Finance/Accounting ownership. |
| `L4-F-010` | 1.0.1 / Draft | `BH-001`–`007`, `BH-010` | Preserve uncertain payment outcome without claiming settlement; maintain identity/references/evidence; Finance controls authorized lifecycle resolution. |
| `L4-F-061` | 1.0.0 / Draft | `BH-003`–`008`, `BH-012`, `BH-014`–`015` | Compare external evidence, keep uncertainty unresolved, prevent duplicate settlement/automatic reversal, preserve reconciliation evidence. |

These L4 files are Draft, so the relations are `Specified` rather than `Validated`. They cannot support Architecture Approved status without upstream approval or explicit governance authorization.

### 3.2 Authoritative Constraints
- Gateway evidence shall not directly establish canonical Finance state.
- Uncertain provider outcomes shall not be represented as successful settlement.
- No C03-local `Failed`, `Timeout`, `Retrying` or other Payment business state may be invented.
- Technical duplicate/replay handling shall not create duplicate financial effect.
- Accounting, refund authorization, settlement and reconciliation business rules remain with their canonical Finance owners.

## 4. Technical Ownership & Dependencies
**Owns:** gateway abstraction, provider adapter selection, payment-operation correlation, authenticated callback handling, provider status verification, provider refund invocation after Finance authorization and normalized technical evidence.  
**Must not own:** canonical Payment lifecycle/state, fee accounting, refund authorization, Cash Position, ledger state, settlement/reconciliation policy or canonical Finance events.

## 5. Solution Architecture
### 5.1 Elements
| Element | Responsibility | Boundary |
|---|---|---|
| Payment Gateway Router | Selects an approved provider from configuration/policy supplied by authoritative business/institutional capability. | Does not decide financial outcome. |
| Gateway Adapter | Encapsulates provider API/SDK/redirect/callback mechanics. | Provider-specific model remains isolated. |
| Payment Correlator | Links immutable SutraERP payment intent/reference to provider operation/reference. | Preserves identity across retries/status checks. |
| Callback Verifier | Validates provider-supported authenticity/replay protection and correlates evidence. | Redirect/callback alone never mutates Finance state. |
| Status Resolver | Retrieves trusted provider evidence for asynchronous/ambiguous outcomes. | Returns evidence; Finance resolves business state. |
| Refund Executor | Executes Finance-authorized provider refund request and returns evidence. | Cannot authorize refund. |

### 5.2 Primary Flow
```text
Finance-authorized payment intent/reference
→ Payment Gateway Router
→ provider adapter + D03 credential
→ provider operation using C01 idempotency semantics where supported
→ customer/provider interaction where required
→ trusted server-side callback/status evidence
→ normalize + correlate provider evidence
→ Finance applies authoritative Payment/settlement rules
```

### 5.3 Material Failure Model
| Condition | Required Outcome | Forbidden Outcome |
|---|---|---|
| Browser/client redirect | Treat as non-authoritative interaction evidence. | Canonical Payment success from redirect alone. |
| Timeout after provider submission | Preserve ambiguous technical disposition and original correlation. | Blindly create a second charge/payment effect. |
| Duplicate callback | Idempotently recognize/reconcile duplicate evidence. | Duplicate Finance effect. |
| Provider conflict between callback/query | Preserve conflicting evidence for provider-specific authoritative resolution and Finance reconciliation. | Latest-message-wins business rule invented by C03. |
| Failover after submission | Only proceed when original-effect ambiguity/duplicate-charge risk is resolved. | Blind provider switch. |

## 6. Architecture Decisions
| ID | Decision | Position | Basis | ADR |
|---|---|---|---|---|
| AD-001 | Gateway structure | Multi-provider abstraction with provider-specific adapters. | Frozen C03-003 responsibility; provider independence. | `ADR-L5-0016` |
| AD-002 | Authority | Finance owns Payment/refund/reconciliation state; gateway responses are evidence. | F-007/F-010/F-061 ownership. | `ADR-L5-0016` ownership-preserving handoff. |
| AD-003 | Confirmation | Canonical outcome requires trusted server-side provider evidence interpreted by Finance rules; redirect is insufficient. | F-010/F-061 evidence constraints. | `ADR-L5-0016` |
| AD-004 | Ambiguity | Preserve unresolved technical outcome until provider/Finance reconciliation. | `L4-F-010-BH-001`–`007`; `L4-F-061-BH-007`–`008`. | `ADR-L5-0014`; `ADR-L5-0016` |
| AD-005 | Card-data boundary | Provider-hosted/tokenized mechanisms are preferred when provider contracts support them; raw card credential persistence is not required by this architecture. | Data-minimization/security derivation; no card platform selected. | None unless a material payment technology is selected. |

## 7. Security, Privacy & Operations
Machine credentials/certificates consume D03. Sensitive payment data is minimized/redacted. Technical evidence includes operation correlation, provider reference, time and technical outcome without becoming Finance audit truth. Provider health consumes E01/E05.

## 8. Architecture Conformance Requirements
- `CR-001`: Gateway adapters shall not directly mutate Finance persistence or invent Payment lifecycle state.
- `CR-002`: Browser redirects alone shall never establish canonical payment success.
- `CR-003`: Duplicate/ambiguous provider outcomes shall be handled without duplicate financial effect.
- `CR-004`: Refund execution shall require an authoritative Finance instruction.
- `CR-005`: Gateway failover shall not proceed while original provider outcome remains ambiguous and duplicate charge is possible.
- `CR-006`: Technical provider evidence shall preserve SutraERP/provider correlation and remain reconstructable.
- `CR-007`: Named gateway/vendor selection remains non-canonical until governed technology selection.

## 9. Open Decisions & Deferred Design
| ID | Item | Classification | Blocking? | Resolution |
|---|---|---|---|---|
| OD-001 | Applicable F-007/F-010/F-061 L4 specifications remain Draft. | Upstream / Traceability | Yes for Architecture Approved absent explicit authorization. | Approve/review upstream or record approval-level-use authorization, then validate relations. |
| OD-002 | Named payment gateway/provider and provider-specific authentication/profile. | Technology | No. | Govern only when concrete provider selection is required. |

## 10. Traceability Summary
**Specified L4:** `L4-F-007-BH-001`–`003`,`007`–`011`; `L4-F-010-BH-001`–`007`,`010`; `L4-F-061-BH-003`–`008`,`012`,`014`–`015`.  
**Peers:** C03-001/002, C01, D03, E01, Finance architecture. **Applicable shared ADR:** `ADR-L5-0016`. **Blocking:** OD-001.
