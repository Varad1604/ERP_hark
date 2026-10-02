# L2-F01 — Prospective Admission Charge Context — Proposed Controlled Amendment

| Attribute | Value |
|---|---|
| Status | Proposed — Architecture decision pending; **not canonical** |
| Target | Locked L2-F01 Student Charge & Fee Management and its locked 2026-09-23 publication-authority amendment |
| Proposed version on adoption | Separate additive amendment v1.0.0; no silent change to existing version |
| Date | 2026-10-02 |
| Decision authority | Lead Architect under Architecture Constitution |
| Review participants | Finance domain owner and Finance Review Team; Admissions/C02 owners for cross-domain identity |
| Tracks | Architecture #132, draft PR #135, B18-J02 |
| Reviewed baseline | Architecture `main` `2bd00c68afaa84ec1d513d1f821a9c0feb53aa5b` |

> **No operative effect before explicit approval.** This file is an approval candidate, not a locked amendment, permission to implement, new aggregate, new event/schema/API or L6 mock authority. Any accepted text must be separately governed and checked against locked L1/L2 before activation.

## 1. Exact scope and provenance

The locked L1 Student Activation workflow requires an approved Admission, documents and financial clearance before StudentContext establishment. L2-C01 §9.6 publishes `AdmissionApproved` and §9.12 lists Finance as an authorized consumer. L2-F00 §4 illustrates approval-triggered Finance-created Admission Charges. L2-F01 §§9.4–9.6 recognize Admissions as a Charge Source and include Source Identifier and institutional/session/student scope metadata. L2-F01 §§13.4, 20.4–20.5 list Student Context among **illustrative** calculation/context-validation inputs; §22.4 lists Admission Confirmation among *illustrative* upstream sources. The existing locked 2026-09-23 F01 publication amendment already gives F01 sole validation/publication authority subject to every applicable policy condition.

This proposed amendment resolves only the **missing eligibility/identity boundary for an admission-originated charge before StudentContext exists**. It does not relax F01's other invariants.

## 2. Proposed rules — require formal approval

**PS-001 — Eligible source and purpose.** F01 MAY interpret C01's existing, authoritative `AdmissionApproved` fact as a pre-Student admission financial obligation **only when** a currently effective, applicable F01 Charge Policy requires a qualifying charge before admission confirmation. An approval event alone NEVER guarantees a charge. The source must remain approved and non-cancelled/non-revoked at the financial decision point. No new C01 event is established.

**PS-002 — Prospective financial subject.** For this narrowly defined class of obligation, F01 MAY establish its existing canonical Student Charge against a validated **prospective Admission subject** while no C02 Student/StudentContext yet exists. The mandatory source relationship is the immutable, owner-resolved A02 PersonID within A01 Tenant, C01 Admission ID and correct B01-governed Institution/Institute context, together with the applicable Programme/Session, financial period, admission scope, classification and policy-version references **where required by the specific obligation**. No identifier may be inferred from labels or an unrelated historical StudentContext. **C01's generic event metadata includes Admission ID, Institution ID and Campus ID but does not explicitly require PersonID, Institute ID, Programme or Session fields.** C01/F01 must formally approve the exact event-specific attributes or separate authorized source-context resolution before interpreting this event as a sufficient charge basis; F01 must independently verify A02 PersonID, authorized institutional/Institute linkage and every actually required scope. Missing authoritative context fails closed; no display-label join or assumed schema is allowed.

**PS-003 — Scoped context interpretation.** Student Context among L2-F01 §§13.4 and 20.5 **illustrative** inputs is not presumed to exist for this expressly approved pre-Student Admission-source subtype. Only the requirement for a *not-yet-created C02 StudentContext reference* is conditionally inapplicable **when** the verified prospective-Admission source and all other applicable policy, structural, institutional, financial and publication context requirements are satisfied. An ordinary existing-Student charge is unaffected. This subtype is not a new Student Charge aggregate, lifecycle state, identity owner, default waiver or universal charging rule.

**PS-004 — F01 exclusive creation and publication.** F01 alone determines source qualification, classification, its owned Charge Policy applicability, Fee Structure, monetary amount, financial validation, charge establishment, outstanding position and its own canonical publications, including `ChargePublicationCompleted`. B01 retains separate authority over its versioned institutional business policies where they apply; F01's owned financial Charge Policy must not silently absorb B01 policy definition/version ownership. C01 may identify required admission obligation **types** and confirm institutional clearance conditions but must not create or calculate a Charge, decide F01 Charge Policy, publish F01 milestones or modify Finance aggregates. Standard source-driven F01 issuance remains free of a universal human approver; manual/exceptional charges retain their separate approved authority.

**PS-005 — Source obligation uniqueness and history.** Exactly one originating approved source obligation must identify each Charge Source relationship. Repeat receipt or re-evaluation of the *same* Admission financial obligation must not create a duplicate Student Charge. Distinct duly authorized admission charge classes may have separate charges when supported by applicable F01 policy; a policy/version revision may affect prospective behavior only under existing controlled-change rules and must not rewrite prior published financial history. Cancellation, revocation, error correction and refund must follow their own pre-existing owning-domain contracts, not a newly invented charge state.

**PS-006 — Payment and subsequent Student association.** A prior F02 Unallocated Advance remains a liability until F02 independently confirms valid Payment identity, allocability to this actual published F01 charge and any derived settlement. Publication never implies collection or clearance. After a separately valid C01 `AdmissionConfirmed`, C02 may establish/reuse the permitted canonical Student and StudentContext. Cross-domain consumers MAY subsequently correlate the same immutable Admission/Person/Institute source identity to the later legitimate C02 relationship through **separately approved** owner-safe contracts/read models; they MUST NOT rewrite the original F01 source identity, F02 settlement history, A02 Person or C02 lifecycle. This amendment creates no API, new event or mandatory technical backfill.

## 3. Rejection, exception and audit gates

| Case | Required proposed outcome |
|---|---|
| Unsupported prospective subtype, absent applicable policy/version or absent owner-verified A02 Person/Admission/Institution scope | Do not create or publish Student Charge; retain traceable owner-controlled exception. |
| Mismatched tenant, Institution/Institute, Person, Admission or required Session | Fail source/context validation; no cross-subject allocation or C01 clearance. |
| Duplicate source obligation delivery | Preserve one canonical F01 charge and existing audit history; no duplicate publication outcome. |
| Admission revoked/cancelled while obligation exists | Apply separately governed cancellation/financial adjustment rules and preserve immutable financial lineage; never silently delete settled value. |
| Existing historical Student/StudentContext | No presumed linkage, cross-Institute leakage, duplicated Student or early C02 activation. |
| Received funds without eligible exact charge | F02 Unallocated Advance stays a liability; no F01/F02/C01 fabricated clearance. |

## 4. Approval and cross-level release gates

- [ ] Confirm locked L1 “approved admission + finance clearance before StudentContext” and whether §17 permits the explicit narrow interpretation without upstream L1 amendment; if not, escalate L1 before adopting this L2 text.
- [ ] F01/C01/A02/B01/C02 owners review conceptual identity, institution/Institute semantics, separation of B01 versioned institutional business policy from F01-owned Charge Policy, and **missing C01 event-specific PersonID/Institute/Programme/Session evidence**; resolve all actually required versioned event-specific references or approved authoritative source-context resolution at the proper architecture level.
- [ ] Finance Review Team confirms source-context validation, idempotency, classification, current charging policy, collection and correction invariants.
- [ ] Validate controlled L3-F-001, L3-C-001, L3-F-007/F-009 and affected L4/L5/L6 traces **after** adopting an L2 decision; do not silently edit locked scenarios.
- [ ] Record explicit Lead Architect approval, exact adopted amendment revision, and applicable repository review/merge gates. This proposal must not itself be merged as an approved canonical amendment without that record.

**Companion:** `L2-C01_Admission_Finance_Clearance_Evidence_Amendment_PROPOSED_2026-10-02.md`. The two proposals are independent acceptance gates and neither alone certifies J02.
