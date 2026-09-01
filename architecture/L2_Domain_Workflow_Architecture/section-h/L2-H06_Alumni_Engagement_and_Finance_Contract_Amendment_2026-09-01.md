# L2-H06 — Alumni Engagement and Finance Contract — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-H06 — Alumni Workflows |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-09-01 |
| Approved Through | ADR-L4-UPSTREAM-BLOCKER-RESOLUTION-001 |

## 1. Alumni contribution engagement boundary

H06 does not own a generic Alumni Contribution aggregate.

An Alumni offer to contribute is governed through the applicable existing H06 engagement variant:

- Volunteer Assignment;
- Guest Lecture participation;
- Industry/Advisory collaboration participation.

Each variant uses its own existing lifecycle/events. Completion of one variant does not create or mutate C-, D- or E-owned Student, Academic Delivery or Workforce facts. Any source-domain activity uses that source domain's own contracts and authority.

## 2. Finance donation completion contract

The canonical Finance completion event consumed by H06 is:

- `PaymentSettlementCompleted` — published by Payment Management / F02.

Earlier H06 references to `PaymentCompleted` for this Finance settlement fact are superseded and shall not be treated as a second event identity.

On consuming a correlated `PaymentSettlementCompleted`, H06 may record a **Finance Settlement Reference** against the Donation Intent/Alumni relationship context. This reference identifies the Finance-owned Payment outcome and does not create a financial lifecycle, accounting record, receipt, or competing settlement state in H06.

If no authoritative Finance settlement exists, H06 retains only its Donation Intent outcome.

## 3. Downstream effect

This amendment supports the narrowed L3-H-045 variant-specific outcome and resolves the event/contract blocker recorded by L3-H-047.
