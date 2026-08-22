# Damaged Library Resource to Replacement or Recovery Action

**Scenario ID:** L3-G-140  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Library Management  
**Participating Domains:** G, F, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 2. Scenario Purpose

This scenario governs the complete institutional outcome in which a library resource is determined to be damaged, the damage is recorded and assessed under Library Management policy, and the resulting recovery or replacement obligation is established.

The locked L2 supports recording **Physical Condition** and **Damage Observations** during return processing. Fine Management also supports a **Damaged Resource** fine category and **Replacement Charge**. Fine calculation may consider Resource Category, Membership Category, Grace Period, Number of Days and Institutional Policies, with rules remaining configurable. fileciteturn189file0turn187file0

Library Management owns the operational damage and library-fine outcome. Finance owns accounting/payment. Communication owns notification delivery.

## 3. Business Outcome

The damaged Library Copy/resource has an authoritative damage outcome, the applicable recovery/replacement obligation is established where policy requires it, and the result is recorded for subsequent operational and financial processing.

The scenario does not invent a damage threshold, replacement value, repair procedure, or payment outcome.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A Library Copy/resource is presented, inspected, or otherwise determined to have damage requiring a business decision.

### 4.2 End Boundary

The damage outcome and any applicable recovery/replacement obligation have been recorded, with the resource prevented from being incorrectly treated as ordinarily available where its condition does not permit circulation.

### 4.3 In Scope

- Identify Library Copy/resource.
- Identify associated member/borrowing where applicable.
- Record physical condition.
- Record damage observations.
- Assess operational consequence.
- Determine applicable recovery/replacement treatment.
- Calculate a library fine/replacement charge where policy requires.
- Preserve circulation and damage history.
- Provide Finance reference where applicable.
- Provide H notification outcome where applicable.

### 4.4 Out of Scope

- Payment execution.
- Accounting settlement.
- Physical repair execution.
- Procurement of a replacement.
- New catalogue creation.
- Membership changes.
- Technical implementation.

## 5. Ownership and Participating Domains

### Library Management — G

Owns Library Copy operational handling, circulation condition, damage assessment and library fine/replacement-obligation management.

### Finance — F

Owns accounting and payment processing for financial consequences.

### Communication — H

Owns delivery of configured notifications.

Participation does not transfer ownership from G.

## 6. Business Actors, Roles and Responsibilities

### Library Member / Resource Custodian

Presents or is associated with the damaged resource where the damage occurs during borrowing.

### Librarian / Circulation Authority

Records condition and damage observations and applies the applicable library policy.

### Library Administration

Provides administrative decision-making where exceptional recovery or waiver policy requires it.

### Finance

Processes accounting/payment consequences.

### Communication

Delivers governed notifications.

## 7. Preconditions and Entry Criteria

1. The Library Copy/resource can be identified.
2. The applicable circulation or custody relationship can be established where relevant.
3. The physical condition can be assessed.
4. Applicable damage/recovery policy is available.
5. Required library authority is available.

## 8. Trigger

A Library Copy/resource is determined to be damaged.

## 9. Main Business Flow

### Phase A — Establish the Damage

1. Identify the Library Copy/resource.
2. Identify the associated active borrowing/member where applicable.
3. Inspect or establish the physical condition.
4. Record the Physical Condition.
5. Record Damage Observations.
6. Determine whether the resource remains suitable for ordinary circulation.

### Phase B — Determine Operational Recovery

7. Determine the applicable operational treatment under library policy.
8. If the resource can remain in service after the assessed condition, preserve the applicable operational condition.
9. If the resource requires further recovery/repair/review, prevent it from being treated as ordinarily available until the authoritative business decision permits availability.
10. Preserve the condition history.

### Phase C — Determine Financial Recovery Where Applicable

11. Determine whether a Damaged Resource fine or Replacement Charge applies.
12. Apply the configured fine/recovery policy.
13. Record the applicable obligation and category.
14. Provide the Finance reference where financial processing is required.

### Phase D — Complete

15. Preserve the damage and circulation history.
16. Provide the outcome to H where member notification is required.
17. End the scenario with an authoritative operational and recovery outcome.

## 10. Alternate Business Flows

### 10.1 Damage Is Cosmetic and Does Not Restrict Use

Where policy determines that the resource remains fit for circulation, the resource may continue under its applicable operational condition. No local damage threshold is invented.

### 10.2 Damage Requires Recovery Before Circulation

The resource remains unavailable for ordinary circulation until the authoritative recovery/assessment outcome is established.

### 10.3 Replacement Charge Applies

A Replacement Charge is recorded under the applicable configurable policy.

### 10.4 Damaged Resource Fine Applies

A Damaged Resource fine is established where policy requires it.

### 10.5 No Financial Obligation Applies

The damage remains recorded, but no unsupported financial amount is created.

### 10.6 Resource Is Determined Beyond Recovery

The applicable Library Copy withdrawal/replacement process is followed. This scenario does not invent a new lifecycle state or procurement mechanism.

## 11. Failure, Interruption and Business Recovery

### 11.1 Condition Cannot Be Established

The resource is not incorrectly returned to ordinary availability until its condition is authoritative.

### 11.2 Damage Assessment Requires Administrative Review

The operational outcome remains subject to the configured administrative decision.

### 11.3 Recovery Value Cannot Be Established

No invented replacement amount is created. The damage remains recorded pending the applicable policy decision.

### 11.4 Finance Processing Is Pending

G's library recovery/fine outcome remains authoritative while F handles accounting/payment.

### 11.5 Notification Is Required

H owns communication delivery. No technical retry, queue, rollback, or infrastructure behaviour is specified.

## 12. Business Rules and Decision Constraints

Return processing may record:

- Return Date.
- Return Time.
- Physical Condition.
- Damage Observations.
- Lost Resource Information.
- Outstanding Fine.

Historical circulation records remain immutable. fileciteturn189file0

Fine categories include **Damaged Resource** and **Replacement Charge** among configurable categories. fileciteturn187file0

Fine calculation may consider:

- Resource Category.
- Membership Category.
- Grace Period.
- Number of Days.
- Institutional Policies.

No monetary amount, damage threshold, depreciation rule, repair threshold, or replacement deadline is invented. fileciteturn187file0

## 13. State and Lifecycle Impact

The Library Copy lifecycle is:

`Added → Available → Issued → Returned → Withdrawn`

The scenario does not create a new `Damaged` lifecycle state. Damage is an operational condition/observation that can require a different subsequent Library Copy treatment.

The Fine lifecycle is:

`Calculated → Pending → Paid / Waived → Closed`

where a financial recovery obligation is applicable. fileciteturn192file0

## 14. Business Events and Cross-Domain Collaboration

The available locked L2 material does not expose a dedicated canonical damaged-resource event. Therefore this draft does not invent `ResourceDamaged` or `ReplacementChargeCreated`.

### Cross-domain collaboration

- **F:** receives the financial reference and performs accounting/payment processing.
- **H:** delivers configured communication.
- **G:** remains owner of damage assessment, Library Copy operational consequence and library-fine/recovery outcome.

## 15. Business Authorization, Approval and Audit Obligations

Damage assessment and financial recovery follow configured Library Management authorization.

Where administrative review/approval is required, the decision and reason must be recorded.

The physical condition, damage observations, resulting operational treatment, recovery/fine decision and financial reference must remain auditable.

No technical access control/IAM implementation is defined.

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Resource identified.
- Member/borrowing relationship established where applicable.
- Physical condition recorded.
- Damage observations recorded.
- Operational consequence determined.
- Recovery/replacement treatment established.
- Financial obligation calculated where applicable.
- Finance reference established where applicable.
- H notification outcome established where applicable.
- History preserved.

### Postconditions

- Damage is part of the authoritative resource/circulation history.
- The Library Copy is not incorrectly treated as available when its condition prevents ordinary circulation.
- Any applicable fine follows the canonical Fine lifecycle.
- Finance owns payment/accounting.
- H owns communication.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

The locked L2 does not expose a dedicated damaged-resource event or a fixed recovery decision matrix. No such event or rule is invented.

### Assumptions

- Damage treatment remains configurable/policy driven.
- Repair/replacement execution is a downstream operational process.
- Library Copy lifecycle ownership remains with G.

### Deferred Questions

Confirm the canonical damaged-resource event vocabulary and any mandatory recovery decision matrix before Architecture Approval/Lock if required by the final event/rule register.
