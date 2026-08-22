# L2-C01 — Admissions Workflows — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-C01 — Admissions Workflows |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-08-22 |
| Approved Through | ADR-STU-ADM-EXP-001 |
| Authority | Lead Architect approval recorded 2026-08-22 |

This amendment is part of the canonical L2-C01 architecture. Where this amendment conflicts with the original L2-C01 document, this amendment is authoritative for the amended subject only. Unaffected L2-C01 content remains unchanged.

## 1. Canonical Admission Expiry Lifecycle

The canonical Admission lifecycle includes:

`Admission Initiated → Expired`

The transition occurs when the institution-configured admission expiry policy determines that the Admission has expired.

`Expired` is a terminal Admission state.

Any later L2-C01 lifecycle matrix that omits the expiry transition is superseded by this amendment for the expiry subject only.

## 2. Canonical Event

`AdmissionExpired` remains the canonical business event for this outcome.

**Canonical publisher:** Admissions.

The event is published only after the Admission has conclusively reached `Expired` under the governed Admissions policy.

## 3. Ownership Boundary

Admissions remains the sole canonical owner of the Admission expiry decision and lifecycle state. Other domains may consume the expiry fact but may not directly alter the Admission lifecycle or publish `AdmissionExpired`.

## 4. Policy Boundary

This amendment intentionally does not prescribe:

- a fixed number of days before expiry;
- a fixed calendar mechanism;
- implementation scheduling or timer behavior;
- technical notification or retry behavior.

Expiry timing remains institution-configured business policy within the Admissions domain.

## 5. Downstream Rule

L3-C-006 and any other downstream scenario may rely on `Admission Initiated → Expired`, terminal state `Expired`, and event `AdmissionExpired` as authoritative L2 semantics. Downstream artifacts shall not redefine the state, publisher, or expiry ownership.
