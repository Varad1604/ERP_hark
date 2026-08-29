# Expired Supplier Qualification to Governed Eligibility Restriction

# 1. Scenario Metadata

**Scenario ID:** L3-J-024  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Vendor & Supplier Management  
**Participating Domains:** J  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** sourcing eligibility, award, commitment prevention, supplier renewal, performance, profile change, restriction, and audit scenarios  

---
# 2. Scenario Purpose

This scenario governs restriction of supplier eligibility when a qualification or supporting evidence expires without an effective approved renewal. It makes expiry operationally enforceable while preserving supplier identity, Active status where otherwise valid, existing commitments, and historical qualification evidence.

It prevents expired credentials from continuing to authorise new sourcing, award, or commitment and avoids incorrectly treating a scope-level expiry as universal supplier suspension or blacklisting.

---

# 3. Business Outcome

Expired supplier qualification reaches Governed Eligibility Restriction for the exact affected scope, with effective time, reason, downstream blocks, open-position review, supplier notice, renewal path, authority, and audit evidence. Unaffected valid qualifications may remain usable.

Outcome labels in this section are business dispositions or supporting-record results, not additional canonical lifecycle states. Section 13 states the only canonical lifecycle impact.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a qualification or supporting evidence reaches its authoritative expiry and no valid successor eligibility is effective.

## 4.2 End Boundary

The scenario ends when affected eligibility is restricted, new reliance is blocked, pending sourcing and commitments are reviewed by their owners, the supplier is notified, and renewal or other disposition is traceable.

## 4.3 In Scope

- validating the expired evidence, qualification, scope, time, dependency, and absence of successor validity;
- distinguishing category, location, value band, licence, certification, capacity, insurance, regulatory, or other affected scope;
- applying full or partial eligibility restriction at the correct effective time;
- identifying pending sourcing events, recommendations, awards, and commitment attempts that may rely on the expired position;
- communicating restriction to procurement and the supplier;
- preserving unaffected Active status and valid qualifications where permitted;
- handing later renewal to L3-J-023 and recording any restored successor eligibility.

## 4.4 Out of Scope

- renewing evidence or qualification, governed by L3-J-023;
- suspending, blacklisting, or closing the supplier for misconduct or broader risk;
- cancelling an executed Purchase Order or contract;
- reversing receipt, payable, accounting entry, or payment;
- technical eligibility enforcement or notification implementation.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-J-022 and L3-J-023 | Qualification existed and was not renewed before expiry. |
| Related | L3-J-021 | Supplier may remain Active despite scope restriction. |
| Followed by | L3-J-023 | A later approved renewal may establish successor eligibility. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Enterprise Resource Management - Vendor & Supplier Management | Owns expiry validation, eligibility restriction, scope, downstream communication, renewal hand-off, and evidence. |

J04 retains existing commitment lifecycle. Finance retains monetary truth. Specialist domains retain interpretation of expired evidence and any mandatory legal consequence.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Supplier Representative | Supplies permitted qualification or classification evidence; cannot approve its own eligibility. | Submits and responds. |
| Institutional Verifier | Verifies identity and supporting evidence independently. | Verifies evidence. |
| Qualification Authority | Grants, expires or changes category- and scope-specific qualification. | Decides qualification and eligibility. |
| Supplier Governance Authority | Applies supplier restriction or other governance treatment where the outcome requires it. | Decides governed supplier treatment. |
---

# 7. Preconditions and Entry Criteria

- A canonical supplier and qualification position are identifiable.
- The qualification depends on evidence with an authoritative expiry or has its own expiry.
- Current time and effective-date rules are reliable and applicable.
- No approved successor eligibility covers the same scope and effective period.
- Restriction, exception, open-position review, communication, and renewal rules are effective.
- Affected sourcing and commitment references can be identified.

---

# 8. Trigger

The qualification or its required supporting evidence expires without an effective approved renewal or authorised successor position.

---

# 9. Main Business Flow

1. The institution records the supplier, qualification, evidence, expired scope, expiry time, and source of the expiry fact.
2. The expiry date, time zone, issuer, evidence version, qualification dependency, and current supplier identity are verified.
3. The system or control owner confirms that no renewed, overlapping, superseding, or explicitly authorised interim eligibility is effective.
4. The exact affected categories, items, services, works, locations, campuses, value bands, risk classes, funding contexts, and capacities are derived.
5. Unaffected valid qualifications and the supplier's broader Active status are separated from the expired scope.
6. Any mandatory legal, safety, regulatory, insurance, security, or operational consequence is confirmed with the relevant specialist.
7. The qualification is recorded as expired; where J02 governance requires supplier-level restriction, the Supplier transitions to the canonical `Restricted` state at the authoritative effective time.
8. New invitations, participation, evaluation advancement, award approval, and commitment creation relying on the expired scope are blocked or referred according to the applicable stage.
9. Active sourcing events and recommendations are identified and placed into governed review; this scenario does not rewrite their decisions automatically.
10. Existing Purchase Orders, contracts, deliveries, warranties, and obligations are identified for J04 or authoritative-owner review and are not cancelled solely by this state change.
11. The supplier receives notice of affected scope, expiry, consequence, renewal evidence, review channel, and any continuing unaffected status.
12. Internal category, requesting, finance, legal, compliance, safety, security, and commitment owners receive the restriction fact where relevant.
13. Any claimed error, valid successor, regulator extension, or authoritative correction is verified before the restriction is changed.
14. An exceptional interim allowance may exist only under an express rule, with authority, reason, narrow scope, end time, compensating controls, and no mandatory prohibition.
15. A later renewal creates a successor eligibility period; the expired interval remains visible and is not erased or backdated.
16. Unauthorised reliance during the gap is preserved and escalated for corrective review.
17. Expiry, restriction, notifications, affected positions, exceptions, later restoration, actors, timestamps, and versions remain auditable.

---

# 10. Alternate Business Flows

## 10.1 Partial Evidence Expiry

Only qualification positions dependent on the expired evidence are restricted. Independent valid positions remain available.

## 10.2 Issuer Extension or Corrected Expiry

An authoritative issuer or regulator extension is validated and recorded as a new fact. The original record is corrected through governed provenance, not silently edited.

## 10.3 Renewal Approved Before Expiry but Effective Later

The gap remains restricted unless an authorised overlap or interim rule applies. Approval date alone does not eliminate the effective-date gap.

## 10.4 Pending Sourcing Evaluation

The sourcing owner determines whether the supplier was eligible at required checkpoints and whether continued participation is permitted. The expiry fact remains controlling input.

## 10.5 Existing Commitment

The commitment owner reviews legality, safety, performance, insurance, continuity, termination, or remediation under the contract process. No automatic cancellation occurs here.

## 10.6 Mandatory Licence Expiry

Where law prohibits continued performance, specialist and commitment owners receive an urgent fact for immediate governed action beyond new-sourcing restriction.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Expiry Calculation Dispute

The position remains conservatively restricted where required while issuer, time, version, and governing rule are verified.

## 11.2 Restriction Not Propagated

Failed downstream delivery is recorded, reissued, and reconciled. Eligibility follows the authoritative expiry, not notification success.

## 11.3 Incorrect Over-Restriction

Unaffected scope is restored through an attributable correction while preserving the error, impact, and corrective evidence.

## 11.4 Unauthorised Override

The override is blocked or removed and escalated. Any action taken is preserved for investigation.

## 11.5 System Interruption

Expiry remains effective by business time. On recovery, eligibility and affected activity are reconciled without backdating.

## 11.6 Late Renewal

The supplier remains restricted until the new eligibility is approved and effective; the historical gap is retained.

## 11.7 Existing Transaction Risk

The authoritative sourcing, contract, safety, legal, or finance owner receives the fact and decides its record. This scenario does not alter that lifecycle.

---

# 12. Business Rules and Decision Constraints

- Expired qualification cannot authorise new sourcing, award, or commitment activity.
- Restriction becomes effective at the authoritative expiry unless an express valid successor or interim rule applies.
- Scope must be no broader than the qualification dependencies and mandatory consequence.
- Supplier Active status and unrelated valid qualifications remain separate from scope-level expiry.
- Pending applications, uploaded documents, receipts, or review activity do not equal renewed eligibility.
- Later renewal cannot be backdated to conceal an expiry gap.
- Existing commitments are not automatically cancelled, but their owners must receive material expiry facts.
- Overrides require explicit legal basis, authority, narrow scope, end time, and compensating controls and cannot defeat mandatory prohibition.
- Original evidence, qualification, expiry, restrictions, exceptions, and restoration history remain immutable.
- This scenario does not create or cancel a Purchase Order, contract, receipt, payable, accounting entry, or payment.

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Supplier | `Active` | Applies the exact J02 governance transition after the qualifying evidence expires. | `Restricted` | Vendor & Supplier Management (J02) | `L2-J02` Section 6 |
| SupplierQualification | No canonical lifecycle defined | Preserves the expired qualification fact without creating a local renewal or exception state machine. | Expired qualification fact | Vendor & Supplier Management (J02) | `L2-J02` Section 5 and Section 13 |
---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Material Consumer(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `SupplierQualificationExpired` | Vendor & Supplier Management (J02) | J01, J03, J04, J05, Finance and Reporting where material | Records the exact completed business fact material to this scenario. | `L2-J02 Section 13` |
| `SupplierRestricted` | Vendor & Supplier Management (J02) | J01, J03, J04, J05, Finance and Reporting where material | Records the exact completed business fact material to this scenario. | `L2-J02 Section 13` |
## 14.2 Cross-Domain Collaboration

| Participant | Collaboration | Boundary |
|---|---|---|
| L3-J-021 / L3-J-022 | Supply Active status and qualification scope. | Retain approval and qualification history. |
| L3-J-023 | Supplies any effective renewal. | Retains renewal decision. |
| Sourcing | Reviews active events and blocks new reliance. | Retains event lifecycle. |
| J04 | Reviews existing or proposed commitments. | Retains commitment lifecycle. |
| Specialist Owners | Determine mandatory legal, safety, insurance, regulatory, or security consequences. | Retain specialist authority. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J02 roles are separated as policy requires: Supplier Representative; Institutional Verifier; Qualification Authority; and Supplier Governance Authority. The Supplier Representative cannot verify, qualify, approve, or remove an institutional restriction on itself.

## 15.2 Required Evidence

- supplier, qualification, evidence, dependency, scope, expiry, time zone, issuer, and predecessor decision;
- absence or timing of successor renewal, current supplier status, other valid qualifications, and specialist consequence;
- affected sourcing, recommendations, awards, commitments, notices, blocks, exceptions, corrections, and unauthorised reliance;
- restriction and restoration facts, actors, reasons, timestamps, versions, and audit trail.

## 15.3 Audit Obligations

The institution must reconstruct what expired and when, which eligibility depended on it, why the restricted scope was correct, which activities were blocked or reviewed, whether an exception or gap existed, and when any successor eligibility became effective.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- the qualification or evidence has reached or passed its effective expiry date and the supplier's eligibility status has been updated to reflect the restriction;
- no new sourcing events, awards, or commitments referencing the expired qualification have been created after the effective expiry date;
- existing commitments, active contracts, and in-progress sourcing events referencing the qualification have been assessed and dispositioned under their authoritative owners;
- the expiry, restriction decision, affected scope, and disposition of existing obligations are documented and attributable;
- notification of the restriction has been issued to relevant institutional users and governance functions where required;
- the expiry and restriction facts are preserved as immutable historical evidence.


## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- the supplier's qualification status reflects the eligibility restriction for the affected scope from the effective expiry date;
- the restriction prevents new award, sourcing selection, or commitment creation for the affected category or scope;
- existing obligations are governed by their authoritative owners under the disposition established at restriction time;
- the supplier retains its identity, other valid qualifications, and registration status;
- reinstatement of eligibility requires a completed renewal under L3-J-023 or a new qualification under L3-J-022.


---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. Dependency, grace, interim, urgent escalation, and mandatory-consequence rules require institution-specific confirmation.
2. Eligibility checkpoints for open sourcing events require method-specific governance.
3. Required treatment of existing commitments by evidence type requires contract, legal, safety, and regulatory mapping.

## 17.2 Assumptions

- Qualification expiry restricts new eligibility but does not erase identity or history.
- Later renewal creates a successor period without backdating.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-024**
