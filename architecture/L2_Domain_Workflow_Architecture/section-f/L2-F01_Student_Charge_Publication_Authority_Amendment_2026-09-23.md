# L2-F01 — Student Charge Publication Authority — Locked Amendment

| Attribute | Value |
|---|---|
| Amendment Version | 1.1.0 |
| Status | Locked |
| Architecture Level | L2 — Business Domain Architecture |
| Canonical Owner | Finance — Student Charges & Fees |
| Approved Through | Explicit Lead Architect decision, 2026-09-23 |
| Authority | Post-R8 Option A upstream business-architecture remediation |

---

## 1. Purpose

This amendment closes the actor/authority gap affecting L3-F-001 while preserving L2-F01 ownership of Student Charge policy evaluation, creation, validation and publication.

---

## 2. Standard Charge Generation Authority

Standard event-driven Student Charge generation is **domain-authoritative and policy-driven**.

For a valid authoritative source-domain business obligation:

1. the source domain owns the originating business fact/event;
2. L2-F01 resolves the Charge Source and classification;
3. L2-F01 evaluates applicable Charge Policy;
4. L2-F01 calculates and validates the charge under existing Finance architecture;
5. when all required conditions pass, L2-F01 may create and publish the canonical Student Charge.

No mandatory human approval is required merely because a standard source-domain obligation results in a charge.

---

## 3. Ownership Boundary

The source domain does not:

- create Student Charge records;
- own Charge Policy evaluation;
- publish canonical Student Charge lifecycle facts;
- modify the canonical Student Charge.

L2-F01 does not:

- reinterpret the source-domain business fact;
- change source-domain lifecycle state;
- become owner of the originating operational event.

---

## 4. Manual / Exceptional Charge Boundary

A manual or exceptional financial decision may require a separately governed institutional authority where applicable policy requires it.

This amendment does not define one universal manual-charge approver because no single institution-independent role is established by existing architecture.

Such exceptional authority shall be established by the applicable Finance/governance source before use.

This exception does not impose approval on standard event-driven F001 processing.

---

## 5. Publication Event Authority

`ChargePublicationCompleted` remains the canonical L2-F01 publication fact for successful Student Charge publication.

Finance — Student Charges & Fees / L2-F01 is the sole semantic publisher.

Event Infrastructure may distribute the fact but does not own its meaning.

---

## 6. Locked Decisions

1. Standard event-driven Student Charge generation is policy-driven and does not require universal human approval.
2. Source domains retain ownership of originating business facts/events.
3. L2-F01 owns Charge Source resolution, policy evaluation, validation, Student Charge creation and publication.
4. Standard charge publication occurs only after existing canonical policy/validation conditions pass.
5. `ChargePublicationCompleted` is a canonical Student Charge publication fact owned by L2-F01.
6. Manual/exceptional charge authority, when required, must be separately governed and is not generalized by this amendment.
7. No source domain gains authority to create or modify Student Charges.
8. No new Student Charge lifecycle state is introduced.
