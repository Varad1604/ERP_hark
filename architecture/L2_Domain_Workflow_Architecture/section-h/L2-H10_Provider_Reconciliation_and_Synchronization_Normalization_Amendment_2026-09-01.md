# L2-H10 — Provider, Reconciliation and Synchronization Normalization — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-H10 — External Education Integration Workflows |
| Amendment Version | 1.3.0 |
| Status | Locked |
| Effective Date | 2026-09-01 |
| Approved Through | ADR-L4-UPSTREAM-BLOCKER-RESOLUTION-002 |

## 1. External Provider vs Provider Trust

The **External Provider** aggregate and **External Education Provider Trust** aggregate are distinct H10 concepts.

External Provider lifecycle remains:

`Pending Verification → Verified → Certified → Active`

with governed `Restricted`, `Suspended` and terminal `Retired` outcomes as defined by base H10.

Provider Trust remains governed by the separate locked 2026-08-26 H10 amendment:

`Requested → Under Review → Approved → Active → Revoked`

with Rejected, Suspended/restored and Expired branches.

These lifecycles shall not be merged or used interchangeably.

## 2. Provider onboarding rejection

`ProviderOnboardingRejected` is the canonical H10 event for a provider-onboarding rejection fact and is published by the **External Provider aggregate**. It records rejection of the provider onboarding/governance progression and does not create or mutate a Provider Trust record.

## 3. Synchronization recovery semantics

Synchronization recovery remains business-level H10 orchestration. The canonical event vocabulary remains:

- `ExternalProviderSynchronizationStarted`
- `ExternalProviderSynchronizationCompleted`
- `SynchronizationFailed`
- `SynchronizationRetried`
- `SynchronizationRecoveryStarted`
- `SynchronizationRecovered`
- `SynchronizationRecoveryDeferred`
- `SynchronizationRecoveryFailed`

`SynchronizationRecoveryDeferred` means the business recovery cannot yet complete and is deferred to governed reconciliation. No additional Synchronization Job `Quarantined` lifecycle state is introduced by this amendment. Technical retry timing, queues, checkpoints and transport mechanics remain outside business architecture.

## 4. Reconciliation Record lifecycle

The Reconciliation Record canonical business progression is:

`Detected → Assigned → Resolved → Verified → Closed`

Unresolved branch:

`Detected / Assigned / Resolved → Escalated`

An Escalated case remains unresolved until authoritative source state supports a new governed resolution. Closing a Reconciliation Record never authorizes H10 to rewrite canonical source-domain facts.

Canonical events remain:

- `ReconciliationIssueDetected`
- `ReconciliationAssigned`
- `ReconciliationResolved`
- `ReconciliationVerified`
- `ReconciliationClosed`

No new event is required solely for the `Escalated` state by this amendment.

## 5. Downstream effect

This amendment resolves the provider-event/lifecycle issue recorded by L3-H-072, the recovery semantics issue recorded by L3-H-075, and the missing explicit Reconciliation Record lifecycle recorded by L3-H-080.
