# Payment Gateway Integration Architecture

## 1. Specification Metadata
**Specification ID:** `L5-C-03-003`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** `L5-C — Contracts, Events & Integration Architecture`  
**Major Area:** `L5-C-03 — External Integration & Connector Architecture`  
**Canonical Technical Responsibility:** Multi-gateway payment integration while Finance retains canonical financial/payment state.  
**Technical Architecture Owner:** `L5-C-03`  
**Assigned Author:** Pallav  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-16`  
**Depends On:** `L5-C-03-001`; `L5-C-03-002`; `L5-C-01`; `L5-D-03`; `L5-E-01`; applicable Finance L5 architecture  
**Applicable ADRs:** `ADR-L5-0002`; `ADR-L5-0009`; `ADR-L5-0014`

## 2. Purpose, Scope & Non-Goals
Defines provider-neutral payment-gateway integration for initiation, redirect/callback handling, authoritative provider verification, refunds and technical reconciliation evidence. Finance remains owner of payment, refund, settlement, ledger and reconciliation state.

## 3. Upstream Applicability & Constraints
Applicable business behavior includes payment failure/timeout recovery and uncertain external outcomes, including `L4-F-010` and related Finance/Treasury scenarios such as `L4-F-061`; Finance must not represent uncertain or failed external outcomes as settled. Exact BH-level mapping remains `OD-001` until fully extracted.

## 4. Technical Ownership & Dependencies
**Owns:** gateway abstraction, provider adapter selection, payment-operation correlation, authenticated callbacks, provider status verification, provider refund invocation and technical evidence normalization.  
**Must not own:** canonical Payment state, fee accounting, refund authorization, cash/ledger state, settlement/reconciliation policy.

## 5. Solution Architecture
### 5.1 Elements
| Element | Responsibility |
|---|---|
| Payment Gateway Router | Selects an approved provider from configured capability/policy supplied by Finance/institutional configuration. |
| Gateway Adapter | Encapsulates provider API/SDK/redirect/webhook specifics. |
| Payment Correlator | Links SutraERP payment intent/reference to provider references and callback evidence. |
| Callback Verifier | Authenticates webhook/callback evidence and rejects replay/tampering. |
| Status Resolver | Queries provider where asynchronous/ambiguous outcomes require verification. |
| Refund Executor | Executes Finance-authorized refund instructions and returns provider evidence. |

### 5.2 Primary Flow
```text
Finance-approved payment intent
→ Gateway Router
→ provider adapter + D03 credential
→ create provider operation using C01 idempotency semantics where supported
→ customer/provider interaction if required
→ authenticated callback and/or trusted provider status query
→ normalized provider evidence
→ Finance decides canonical payment outcome
```

### 5.3 Material Failure Rules
- Browser/client redirect is not authoritative proof of payment.
- Timeout-after-submit remains technically ambiguous until provider reconciliation.
- Blind failover to a second gateway after submission is forbidden unless duplicate-charge risk is resolved.
- Duplicate callbacks are handled idempotently and shall not create duplicate financial effects.

## 6. Architecture Decisions
| ID | Decision | Position |
|---|---|---|
| AD-001 | Gateway strategy | Multi-provider abstraction with provider-specific adapters. |
| AD-002 | Authority | Finance owns payment/refund/reconciliation state; gateway responses are evidence. |
| AD-003 | Confirmation | Trusted server-side provider evidence is required for external confirmation. |
| AD-004 | Ambiguity | Preserve unknown/pending technical outcome until governed reconciliation. |
| AD-005 | Card data | Prefer hosted/tokenized provider mechanisms; avoid storage of raw card credentials. |

## 7. Security, Privacy & Operations
Machine credentials and certificates are D03-governed. Sensitive payment data is minimized and redacted. Provider evidence includes operation correlation, provider reference, timestamps and technical outcome without treating connector telemetry as Finance audit truth.

## 8. Architecture Conformance Requirements
- `CR-001`: Gateway adapters shall not directly mutate Finance persistence.
- `CR-002`: Browser redirects alone shall never establish canonical payment success.
- `CR-003`: Duplicate/ambiguous provider outcomes shall be resolved without duplicate financial effect.
- `CR-004`: Refund execution shall require Finance-authorized instruction.
- `CR-005`: Gateway failover shall not proceed when the original provider outcome is unresolved and duplicate charge is possible.
- `CR-006`: Raw card credentials shall not be persisted by SutraERP where provider-tokenized/hosted mechanisms are available.

## 9. Open Decisions & Deferred Design
| ID | Item | Blocking? | Resolution |
|---|---|---|---|
| OD-001 | Complete exact Finance L4 BH mapping. | Yes for Architecture Approved. | Trace `L4-F-010`, `L4-F-061` and all materially applicable payment scenarios. |
| OD-002 | Named payment gateway/provider selection. | No. | Govern separately if/when required. |

## 10. Traceability Summary
**Corpus:** `L5-C-03-003`. **Known L4 applicability:** `L4-F-010`, `L4-F-061` plus related payment scenarios pending exact BH extraction. **Peers:** C03-001/002, C01, D03, E01, Finance L5.
