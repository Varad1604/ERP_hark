# L2-H10 — External Education Integration Workflows — Enrollment Cancellation and Verification Result Amendment Proposal

| Attribute | Value |
|---|---|
| Target | L2-H10 — External Education Integration Workflows v1.0 plus locked 2026-08-26 H10 governance amendment |
| Proposed Amendment Version | 1.2.0 |
| Status | Review |
| Date | 2026-08-27 |
| Owner | Lead Architect |
| Architectural Level | L2 — Domain Workflow Architecture |
| Domain | H10 — External Education Integration |
| Triggering Gaps | `L3-H-076` cancellation; `L3-H-079` `Expired` / `Inconclusive` verification outcomes |
| Canonical Effect | **None until independently reviewed and approved/locked** |

---

## 1. Purpose

This proposal addresses two remaining H10 gaps discovered during L3 pre-approval validation:

1. `L3-H-076` requires a governed External Enrollment Exchange cancellation outcome, while current locked H10 defines Accepted, Rejected and bounded-validity Expired but no canonical cancellation state/event; and
2. `L3-H-079` requires `Expired` and `Inconclusive` External Credential Verification results, while current locked H10 defines only Verified and Rejected outcomes/events.

This document is **Review-status architecture only**. It does not alter locked H10 until formally approved through repository change control.

---

## 2. Ownership Preservation

Existing ownership remains unchanged:

- H10 owns only external exchange/verification transaction and relationship facts.
- C/D retain canonical Student/Academic enrollment ownership.
- Source credential ownership remains with its canonical source domain.
- A03 retains sole ownership of authentication credentials and Authentication Provider lifecycle.
- H10 outcomes shall not create, cancel, expire, reject or otherwise mutate canonical Student enrollment or source credential lifecycles.

---

## 3. Proposed External Enrollment Exchange Cancellation

Existing lifecycle remains:

`Requested → Validation Pending → Accepted / Rejected`

Existing bounded-validity branch remains:

`Accepted → Expired`

### Proposed cancellation branch

`Requested / Validation Pending → Cancellation Requested → Cancelled`

### Meaning

- `Cancellation Requested` records a governed request to stop the H10 exchange before terminal exchange acceptance/rejection.
- `Cancelled` is terminal for that H10 Enrollment Exchange record.
- Cancellation does not cancel or reverse any canonical Student/Academic enrollment unless the owning domain independently performs its own governed action.
- An Accepted exchange is not locally moved to Cancelled; post-acceptance consequences remain governed by owning-domain decisions and existing expiry rules.

### Proposed authority

The **External Education Integration Governance Authority** owns governed H10 exchange cancellation decisions where institutional approval is required. The originating Student/Academic domain may request cancellation where its own business context requires it but does not become owner of the H10 exchange lifecycle.

### Proposed canonical events

If approved, H10 would be sole semantic publisher of:

- `ExternalEnrollmentExchangeCancellationRequested`
- `ExternalEnrollmentExchangeCancelled`

---

## 4. Proposed External Credential Verification Result Completion

Existing lifecycle remains:

`Requested → Verification Pending → Verified / Rejected`

Proposed additional terminal outcomes:

`Verification Pending → Expired`

`Verification Pending → Inconclusive`

### Proposed meanings

- `Expired` means the verification request/result window ended before a governed conclusive external verification result could be established for that request. This is an H10 verification-request/result outcome and is distinct from expiry of the source credential itself.
- `Inconclusive` means governed external evidence was available but insufficient or contradictory such that neither Verified nor Rejected can be asserted for that request.
- Both outcomes preserve immutable evidence and do not change the source credential lifecycle.
- A later verification requires a successor request/result rather than mutation of the prior Expired/Inconclusive record.

### Proposed canonical events

If approved, H10 would be sole semantic publisher of:

- `ExternalCredentialVerificationExpired`
- `ExternalCredentialVerificationInconclusive`

Existing events remain unchanged:

- `ExternalCredentialVerificationVerified`
- `ExternalCredentialVerificationRejected`

---

## 5. Proposed Business Rules

1. H10 cancellation never substitutes for an owning-domain enrollment decision.
2. Cancelled Enrollment Exchange history remains immutable.
3. H10 verification `Expired` shall not be inferred from source-credential expiry; they are different business facts.
4. `Inconclusive` requires governed evidence that is insufficient or contradictory, not mere technical transport failure.
5. Provider unavailability or transport failure alone remains a failure/interruption condition unless governance determines the verification request window has expired.
6. A later verification creates a successor request/result after Verified, Rejected, Expired or Inconclusive.
7. No new authentication ownership is created.

---

## 6. Downstream Effect

If formally approved and locked:

- `L3-H-076` may be mechanically synchronized to the canonical cancellation branch and exact H10 cancellation events;
- `L3-H-079` may be mechanically synchronized to the complete four-outcome verification model;
- both scenarios may then return to independent Architecture Approval review;
- H10 ownership boundaries with C/D/source credential domains/A03 remain unchanged.

---

## 7. Review Questions

Independent architecture review shall confirm:

1. whether cancellation should be allowed only before Accepted/Rejected terminal outcomes;
2. whether the proposed cancellation authority correctly balances originating-domain intent and H10 exchange ownership;
3. whether verification `Expired` should depend on a configured/governed request validity period;
4. whether `Inconclusive` should be terminal for that request and always require successor verification for later resolution; and
5. whether the proposed event names conflict with any existing base-H10 terminology.

---

## 8. Decision Status

**PROPOSAL STATUS:** REVIEW  
**LOCKED/CANONICAL EFFECT:** NONE  
**L3-H-076:** REMAINS UPSTREAM-BLOCKED UNTIL THIS OR AN ALTERNATIVE RESOLUTION IS APPROVED  
**L3-H-079:** REMAINS UPSTREAM-BLOCKED UNTIL THIS OR AN ALTERNATIVE RESOLUTION IS APPROVED
