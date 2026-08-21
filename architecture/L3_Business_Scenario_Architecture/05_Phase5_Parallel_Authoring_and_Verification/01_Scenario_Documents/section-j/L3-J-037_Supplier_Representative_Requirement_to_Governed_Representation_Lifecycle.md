# Supplier Representative Requirement to Governed Representation Lifecycle

# 1. Scenario Metadata

**Scenario ID:** L3-J-037  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Vendor & Supplier Management  
**Participating Domains:** J  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** supplier registration, profile, sourcing, quotation, tender, negotiation, contract, delivery, invoice, dispute, governance, communication, access, and audit scenarios  

---
# 2. Scenario Purpose

This scenario governs appointment, verification, authorisation, use, change, suspension, expiry, revocation, and historical preservation of supplier representatives. It ensures that communications and actions are attributable to a real person acting within documented scope while preventing a representative from inheriting supplier approval, contract-signing, bank-change, invoice, or payment authority automatically.

---

# 3. Business Outcome

A supplier representation requirement reaches a governed lifecycle record with verified person and supplier linkage, authority source, permitted acts, prohibited acts, effective period, channels, delegation limits, conflicts, privacy, access, review, revocation, and immutable action history.

Outcome labels in this section are business dispositions or supporting-record results, not additional canonical lifecycle states. Section 13 states the only canonical lifecycle impact.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a canonical supplier nominates a person, an institution requires an authorised contact, or an existing representative requires change, renewal, suspension, or revocation.

## 4.2 End Boundary

The scenario ends when representation is approved, rejected, returned, activated, varied, renewed, suspended, expired, or revoked; permitted consumers and access controls receive the current state; and every historical action remains attributable.

## 4.3 In Scope

- verifying person identity, supplier relationship, nomination authority, role, contact channels, conflict, consent, and evidence;
- defining representation scope by process, category, site, contract, sourcing event, communication type, transaction stage, value or time where applicable;
- distinguishing ordinary contact, portal user, bid submitter, negotiator, contract signatory, delivery contact, invoice contact, dispute contact, and governance respondent;
- governing effective dates, expiry, renewal, delegation, substitution, absence, access, change, suspension, revocation, and supplier cessation;
- recording attributable communications, submissions, acknowledgements, and authority checks;
- protecting personal information and preventing unauthorised credential sharing.

## 4.4 Out of Scope

- approving or qualifying the supplier;
- granting contract-signing, bank-account change, beneficiary, invoice approval, payment, tax, or legal authority without the relevant separate mandate;
- authenticating technical sessions or implementing access control;
- deciding employment or agency law beyond received evidence;
- changing supplier legal identity or ownership;
- rewriting actions performed before authority changed.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-J-021 and L3-J-026 / L3-J-027 | Supplier identity and profile support representation. |
| Related | Sourcing and J04 scenarios | Consume authorised submissions and signatures within scope. |
| Related | Finance scenarios | Banking and payment authorities remain independently controlled. |
| Related | L3-J-028 to L3-J-034 | Governance and lifecycle changes may suspend or end representation. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Enterprise Resource Management - Vendor & Supplier Management | Owns representative nomination, verification coordination, scope approval, current-state publication, renewal, revocation, and history. |

Legal, finance, contract, privacy, security, identity, sourcing-event, and communication owners retain specialist authority over signatures, access, banking, payment, and evidentiary effect.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Supplier Representative | Receives notice and supplies permitted response or remediation evidence; cannot remove institutional restrictions. | Responds. |
| Institutional Verifier | Independently verifies evidence material to the governance decision. | Verifies evidence. |
| Supplier Governance Authority | Proposes or approves restriction, suspension, blacklisting, reactivation, inactivation, closure or merge treatment. | Decides governed supplier treatment. |
---

# 7. Preconditions and Entry Criteria

- A canonical supplier or governed pre-registration relationship is identifiable.
- The nominating person or evidence can establish authority to appoint the representative.
- Identity, consent, conflict, scope, signature, access, expiry, renewal, delegation, change, suspension, revocation, privacy, and retention rules are effective.
- High-risk authorities such as contracting, banking and payment can be separated and independently verified.
- Historical actions and authority versions can be retained.

---

# 8. Trigger

A supplier representative is required, nominated, changed, renewed, delegated, substituted, challenged, suspended, expired, or revoked.

---

# 9. Main Business Flow

1. The supplier submits its canonical identity, nominating authority, proposed representative, role, requested scope, channels, effective period, and supporting evidence.
2. The Institutional Verifier verifies supplier status, nominator authority, person identity, contact ownership, consent, role, and duplicate or conflicting representation.
3. Requested acts are classified by ordinary contact, portal use, bid submission, clarification, negotiation, signature, delivery, invoice, dispute, governance, or other approved purpose.
4. High-risk signature, legal, banking-reference, tax, beneficiary, invoice, or payment-related authority is referred for independent specialist verification and cannot be inferred from general representation.
5. Conflicts, former employment, related-party relationships, shared contacts, common credentials, and incompatible duties are declared and assessed.
6. Scope defines supplier entity, branch, site, category, sourcing event, contract, process, permitted acts, prohibited acts, value or other limit, channels, start, expiry, and delegation rules.
7. The representative acknowledges confidentiality, privacy, acceptable use, credential protection, accuracy, communication, and revocation obligations.
8. The Supplier Governance Authority reviews evidence, scope, conflicts, separation, duration, access need, and process-specific clearances.
9. The Supplier Governance Authority rejects, returns, narrows, or approves representation.
10. Approved representation becomes active only from its effective time and after required channel or access controls are applied.
11. Each process owner validates current authority and scope before accepting a material submission, acknowledgement, negotiation, signature, change, or instruction.
12. Actions record representative identity, authority version, supplier, channel, time, scope, and related business record.
13. Changes in role, contact, supplier relationship, nominator authority, legal mandate, conflict, compromise, absence, or governance state trigger review.
14. Renewal requires current verification; expiry ends future authority without deleting previous actions.
15. Revocation or suspension becomes effective prospectively, access is referred for removal, and authorised consumers receive the current state.
16. Actions after expiry or revocation are blocked or referred; prior valid actions remain attributable and are not invalidated automatically.
17. Nominations, evidence, approvals, actions, renewals, changes, suspension, revocation, access referrals, actors, timestamps, and versions remain auditable.

---

# 10. Alternate Business Flows

## 10.1 Multiple Representatives

Each person has an independent scope. Joint, alternative, sequential, or maker-checker authority is recorded explicitly rather than assumed.

## 10.2 Temporary Substitute

The substitute requires verified nomination, limited scope and duration, and separate credentials. Another person's credentials cannot be shared.

## 10.3 Contract Signatory

Legal signing mandate is validated separately for the specific entity and document. General portal or sourcing authority is insufficient.

## 10.4 Banking or Payment Instruction

The representative may communicate a request but cannot approve beneficiary or payment changes; finance-controlled verification remains mandatory.

## 10.5 Supplier Suspended, Blacklisted, Inactive, or Closed

Representation is reviewed, limited, suspended, or retained only for authorised open obligations, disputes, remediation, or records access.

## 10.6 Representative Disputes Revocation

Current supplier and legal mandate evidence controls. The dispute and prior actions remain recorded.

## 10.7 Death, Incapacity, Departure, or Compromise

Authority and access are suspended or revoked promptly, open actions are reviewed, and a replacement requires new approval.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 False or Forged Mandate

Approval is blocked or revoked and the evidence is referred to supplier governance, legal, security, and audit.

## 11.2 Access Exceeds Business Scope

Access is reduced or removed, affected actions are reviewed, and the mismatch is preserved.

## 11.3 Expiry or Revocation Not Propagated

The state is reissued, access and downstream reliance are reconciled, and unauthorised actions are referred.

## 11.4 Shared Credentials

Access is suspended, evidence protected, attributable actions investigated, and new credentials require revalidation.

## 11.5 Incorrect Representative Linkage

An attributable correction is approved; affected communications and transactions are reviewed without deleting history.

## 11.6 System Interruption

The last authoritative representation state controls and is reconciled on recovery.

---

# 12. Business Rules and Decision Constraints

- Representation requires verified person, canonical supplier linkage, nomination authority, consent, explicit scope, effective period, and approval.
- General contact or portal access does not imply bid, negotiation, signature, banking, invoice, legal, or payment authority.
- Each material action is evaluated against the authority version effective at its time.
- Credentials are personal and cannot be shared or inherited.
- Delegation and substitution are prohibited unless expressly authorised and separately recorded.
- Expiry, suspension, and revocation end future authority but do not erase prior actions.
- Supplier restriction, suspension, blacklisting, inactivity, closure, or legal change triggers representative review.
- Representative information follows privacy, confidentiality, purpose, access, correction, and retention rules.
- High-risk banking and payment changes remain under finance-controlled independent verification.
- Representation cannot create supplier approval, qualification, contract, liability, payable, or payment by itself.

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Supplier | Existing canonical J02 state | Updates or records the scenario-specific governed fact without silently changing Supplier lifecycle state. | Existing canonical J02 state remains controlling | Vendor & Supplier Management (J02) | `L2-J02` Section 6 |

No separate lifecycle is defined upstream for the scenario-specific record; this scenario does not create one.
---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

No exact canonical J02 event is defined for supplier representation nomination, approval, suspension, expiry or revocation. This scenario publishes no locally invented substitute; the material event gap is recorded in Section 17.
## 14.2 Cross-Domain Collaboration

| Participant | Collaboration | Boundary |
|---|---|---|
| Supplier Profile / Identity | Supplies supplier and person linkage. | Does not grant process authority. |
| Sourcing / Contract | Validates submission, negotiation, and signature scope. | Retains decision and document authority. |
| Finance / Tax / Banking | Verifies sensitive monetary instructions. | General representation cannot approve them. |
| Privacy / Security / Access | Protects personal data and applies credentials. | Access does not broaden business mandate. |
| Governance / Lifecycle | Supplies supplier restriction, suspension, inactivity, and closure states. | Representative cannot override them. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J02 roles are separated as policy requires: Supplier Representative; Institutional Verifier; and Supplier Governance Authority. The Supplier Representative cannot verify, approve, or remove an institutional restriction on itself, and a conflicted verifier cannot decide the governed outcome.

## 15.2 Required Evidence

- canonical supplier, nominator identity and authority, representative identity, consent, role, contact ownership, relationship, conflicts, and supporting mandate;
- permitted and prohibited acts, entity and branch scope, category, sourcing event, contract, channel, limits, start, expiry, delegation, privacy, confidentiality, and clearances;
- approval, actions, authority version, renewals, changes, suspension, expiry, revocation, access removal, affected actions, actors, timestamps, and versions.

## 15.3 Audit Obligations

The institution must reconstruct who represented which supplier, who nominated and approved the person, what acts and period were authorised, which authority version governed each action, how conflicts and high-risk instructions were controlled, and when access and authority changed or ended.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- a supplier representative requirement has been identified with a verifiable business purpose and institutional authority scope;
- the representative's identity, authority basis, effective dates, scope of representation, and limitations are documented and attributable;
- the representative has been verified and approved by an authority appropriate to the representation scope and institutional risk;
- the representation is linked to the canonical supplier identity and does not create a competing supplier record;
- any termination or expiry of representation is governed, effective from the determined date, and preserved as evidence;
- the representation decision, authority, actors, effective lifecycle, and audit trail are preserved and immutable.


## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- the supplier representative relationship is in an active, expired, or terminated state with an explicit effective date and authority basis permanently on record;
- the representation scope and limitations are the authoritative reference for institutional interactions with the representative on behalf of the supplier;
- the canonical supplier identity is not altered by the establishment or termination of representation;
- the complete representation history, governance decisions, and evidence are permanently auditable;
- downstream sourcing, evaluation, and commitment scenarios may rely on the governed representation relationship to validate interactions with the representative.


---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- The locked J02 architecture defines no Supplier Representative aggregate lifecycle or exact representation events. State and event semantics require an upstream J02 decision if downstream collaboration needs them.
1. Representative types, nomination evidence, duration, delegation, signature mandates, approval levels, and review frequency require confirmation.
2. Treatment of proprietors, partners, directors, authorised signatories, agents, consultants, consortium leads, and power-of-attorney holders requires legal mapping.
3. Process-specific business authority for sourcing, contracting, delivery, invoicing, disputes, banking, and payment requires upstream decisions by the owning domains; technical access mapping follows only after those decisions.

## 17.2 Assumptions

- Representation is person-specific, scoped, time-bound or reviewable, and revocable.
- Access and general contact status do not equal financial or legal authority.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-037**
