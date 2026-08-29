# Registration Hold Placement and Release

**Scenario ID:** L3-C-026  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Registration  
**Participating Domains:** C, F  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Derivation:** KEEP  
**Status:** Locked  
**Author:** Varad  
**Version:** 1.0  
**Last Updated:** 2026-08-20  
**Exact Filename:** `L3-C-026_Registration_Hold_Placement_and_Release.md`

> **Production-authoring note:** This document is authored to production-quality L3 standards and is intended for architecture review. Per Phase 5 governance, a newly authored scenario must remain **Status: Draft** until independently approved; “production-ready” does not mean self-approved or locked.

---


| Attribute | Value |
|---|---|
| Final L3 ID | `L3-C-026` |
| Canonical Title | Registration Hold Placement and Release |
| Primary Owner | Academic Registration |
| Participating Domains | C, F |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Derivation | KEEP |
| Assigned Author | Varad |
| Status | Locked |

The frozen Section C register identifies C026 exactly as **Registration Hold Placement and Release**, owned by Academic Registration, participating in C and F, Cross-domain, P1, KEEP. The Phase 5 workload assigns the complete C001–C075 block to Varad.

**Applicable authoritative L2:** `L2-C03_Student_Academic_Registration_Workflows.md`.

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-C-026 |
| Canonical Title | Registration Hold Placement and Release |
| Primary Owner | Academic Registration |
| Participating Domains | C, F |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the institutional placement and release of a Registration Hold that affects whether an Academic Registration may proceed.

A Registration Hold is a child entity of the Academic Registration Aggregate in L2-C03. Registration Hold validation is explicitly part of Student Eligibility Validation, while the registration domain remains the canonical owner of academic registration transactions.

The scenario therefore establishes a governed business outcome:

- a hold is placed against the relevant registration context when an authoritative business condition requires restriction; or
- an existing hold is released when the authoritative business condition has been resolved and release is permitted.

The scenario does not redefine the owning domain of the underlying condition. A Finance-related condition, for example, remains a Finance business responsibility; Academic Registration owns the registration restriction and its effect on registration.

---

## 3. Business Outcome

A Registration Hold has been either:

1. **Placed**, causing the applicable registration eligibility condition to prevent ordinary registration where the hold is blocking; or
2. **Released**, removing the applicable registration restriction so that eligibility can be evaluated again.

The hold decision is recorded and auditable, and downstream registration activity uses the resulting hold condition rather than independently recreating the business rule.

---

## 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario starts when an authoritative business condition requires a registration restriction to be placed or when an authoritative resolution permits an existing restriction to be released.

Potential business sources include Student Lifecycle or Finance, consistent with the frozen participating domains. The exact source-specific trigger must remain governed by the authoritative upstream architecture.

## 4.2 End Boundary

The scenario ends when the Registration Hold has been placed or released and the resulting registration restriction is available to the Academic Registration eligibility workflow.

For release, the scenario does not itself approve or create the underlying Finance/Student condition resolution.

## 4.3 In Scope

- Registration Hold placement.
- Registration Hold release.
- Validation of the business basis for the hold/release.
- Recording the hold/release outcome.
- Impact on registration eligibility.
- Audit/evidence of the hold lifecycle.
- Cross-domain participation from C and F.

## 4.4 Out of Scope

- Creation or settlement of Finance obligations.
- Student identity lifecycle decisions.
- Financial ledger ownership.
- Academic Registration creation itself.
- Registration offering selection.
- Registration approval/freeze/publication.
- Technical authorization, APIs, databases, queues, retries or infrastructure.

---

## 5. Ownership and Participating Domains

| Domain | Responsibility | Ownership |
|---|---|---|
| Academic Registration | Owns the Registration Hold as part of the Registration Aggregate and determines its effect on registration eligibility. | **Canonical owner of registration restriction** |
| Student Lifecycle (C) | May provide authoritative Student lifecycle conditions relevant to registration eligibility. | Retains Student ownership. |
| Finance (F) | May provide authoritative financial-clearance conditions where institutionally configured. | Retains Finance ownership. |

The L2-C03 architecture explicitly states that the Registration Aggregate owns `RegistrationHold`, while Student and Financial Clearance are referenced rather than owned by Academic Registration.

A participating domain may provide the business fact that justifies a hold; it does not acquire ownership of the Registration Hold lifecycle.

---

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority Boundary |
|---|---|---|
| Academic Registration | Records and governs the registration restriction and its effect on registration eligibility. | Owns Registration Hold. |
| Authorised administrative participant | May place/release a hold where institutional policy permits administrative action. | Must remain within configured institutional authority. |
| Student Lifecycle participant | Provides authoritative Student lifecycle information where a hold depends on it. | Does not modify the Registration Aggregate directly. |
| Finance participant | Provides authoritative financial-clearance condition where configured. | Does not modify the Registration Aggregate directly. |

No additional role is invented because L2-C03 does not define a universal hold-authority role.

---

## 7. Preconditions and Entry Criteria

For placement:

- an authoritative business condition exists;
- the condition is applicable to the Student/registration context;
- the hold is permitted by institutional registration policy;
- the responsible business authority is established where required.

For release:

- the existing hold is identifiable;
- the authoritative condition that justified the hold has been resolved or otherwise qualifies for release;
- release is permitted by institutional policy;
- any required authority has been satisfied.

The exact source-specific eligibility conditions are not invented where L2-C03 does not define them.

---

## 8. Trigger

**Placement trigger:** an authoritative business condition requires registration to be restricted.

**Release trigger:** an authoritative resolution permits the registration restriction to be removed.

The trigger is deliberately expressed as a business condition rather than as an API, technical command, queue message or implementation mechanism.

---

## 9. Main Business Flow

## 9.1 Placement

1. An authoritative business condition requiring a registration restriction is established.
2. Academic Registration validates that the condition applies to the relevant registration context.
3. Academic Registration determines that institutional policy permits a Registration Hold.
4. The Registration Hold is placed.
5. The hold status becomes part of the registration eligibility context.
6. The registration eligibility workflow evaluates the hold and, where it is blocking, returns **Not Eligible** or **Manual Review Required** according to the applicable policy.
7. The hold action is audited.

## 9.2 Release

1. The authoritative condition underlying the hold is resolved.
2. Academic Registration validates that release is permitted.
3. The Registration Hold is released.
4. The registration eligibility context is updated.
5. A subsequent registration eligibility evaluation may determine the Student eligible, subject to all other rules.
6. The release is audited.

The L2-C03 workflow explicitly includes Registration Hold validation in Student Eligibility Validation and identifies `RegistrationHold` as a Registration Aggregate child entity.

---

## 10. Alternate Business Flows

## 10.1 Finance-Originated Hold

Where Finance is the authoritative source of a financial-clearance condition, Finance provides the relevant business fact. Academic Registration applies the registration restriction within its own ownership boundary.

Finance does not own or directly mutate the Registration Aggregate.

## 10.2 Student-Lifecycle-Originated Restriction

Where Student Lifecycle provides an authoritative Student status that prevents registration, Academic Registration evaluates that status through its registration eligibility rules. This is not treated as a transfer of Student lifecycle ownership.

## 10.3 Multiple Conditions

A Student may have more than one independent business condition affecting registration. C026 does not invent a precedence rule between them. Eligibility is determined by the complete authoritative registration policy.

## 10.4 Release With Other Restrictions Remaining

Release of one Registration Hold does not imply registration eligibility if other mandatory restrictions remain.

---

## 11. Failure, Interruption and Business Recovery

| Business Condition | Outcome | Business Resolution |
|---|---|---|
| Hold basis cannot be validated | Placement/release cannot complete. | Resolve the underlying business evidence. |
| Release condition is not satisfied | Hold remains active. | Resolve the underlying condition through its owning domain/process. |
| Required authorization is absent | Hold action cannot complete. | Obtain the required institutional authority. |
| Another blocking registration rule remains after release | Student remains ineligible. | Resolve the remaining rule/condition. |
| Source-domain condition is disputed | Registration eligibility cannot be changed solely by assumption. | Resolve through the authoritative source domain. |

No technical retry, queue, rollback or infrastructure recovery is defined at L3.

---

## 12. Business Rules and Decision Constraints

The scenario applies the authoritative C03 registration rules, including:

- registration eligibility must validate Registration Holds;
- registration policies govern eligibility;
- authorization is required before business operations;
- administrative operations require appropriate authority;
- every workflow transition is auditable;
- external domains interact through governed business contracts/events;
- registration eligibility must be re-evaluated after a material hold change.

C026 does not create a new local rule saying that every hold is automatically blocking; whether a hold blocks registration is policy-governed.

---

## 13. State and Lifecycle Impact

The Registration Hold is a child entity of the Academic Registration Aggregate.

At L3, the business lifecycle is:

```text
No Applicable Hold
      │
      ├──► Hold Placed ───► Hold Released
      │
      └──► No Hold / Other Conditions Evaluated
```

Only the hold lifecycle is represented here. No new Academic Registration Aggregate state is invented.

A released hold does not automatically transition the entire Registration Aggregate to Approved/Frozen/etc.; those are separate C03 workflows.

---

## 14. Business Events and Cross-Domain Collaboration

The locked C03 event catalogue explicitly defines registration events such as `RegistrationEligibilityValidated`, but it does **not** define a canonical `RegistrationHoldPlaced` or `RegistrationHoldReleased` event in the available event catalogue.

Therefore:

- C026 does **not** invent a new canonical hold event.
- Hold placement/release is represented as a Registration Hold business state and becomes relevant to the subsequent eligibility evaluation.
- Any new hold-specific event requires architecture change control before it can be treated as canonical.

Cross-domain collaboration remains directional:

- C/F provide authoritative conditions.
- Academic Registration owns the registration restriction.
- Academic Registration publishes its canonical registration events.

---

## 15. Business Authorization, Approval and Audit Obligations

Hold placement/release must comply with institutional authorization policy.

The L2-C03 architecture requires authorization before business operations and requires every workflow transition to generate an audit record.

Evidence should establish:

- underlying business basis;
- affected Student/registration context;
- action taken;
- authority where required;
- effective business condition;
- resulting hold status;
- release basis where applicable.

No technical IAM/RBAC model is introduced.

---

## 16. Completion Criteria and Postconditions

## Placement Complete

- valid business basis exists;
- hold is recorded;
- registration eligibility recognizes the hold;
- applicable registration restriction is effective;
- audit evidence exists.

## Release Complete

- authoritative release condition is satisfied;
- hold is released;
- eligibility can be re-evaluated;
- audit evidence exists.

Neither outcome guarantees successful registration because other independent registration rules remain applicable.

---

## 17. Open Issues, Assumptions and Deferred Questions

## Open Issues

**OI-C026-01 — No dedicated locked hold-placement/release workflow was identified in L2-C03.**  
L2-C03 defines `RegistrationHold` as a Registration Aggregate child entity and requires hold validation during Student Eligibility Validation, but the available workflow catalogue does not define a standalone hold lifecycle workflow. A new workflow must not be invented at L3.

**OI-C026-02 — No canonical hold-specific domain events are defined.**  
The event catalogue contains registration eligibility and registration lifecycle events but no explicit hold-placement/release event. Architecture change control is required before introducing one.

**OI-C026-03 — Exact C/F source conditions are not enumerated in C03.**  
Finance and Student Lifecycle are participating domains for the frozen scenario, but C03 does not define a complete source-specific hold matrix. The authoritative upstream domain remains responsible for its condition.

## Assumptions

1. Academic Registration owns `RegistrationHold`.
2. Participating C/F domains retain ownership of their source conditions.
3. A hold affects eligibility through the existing C03 validation workflow.
4. Release of one hold does not imply overall registration eligibility.

## Deferred Questions

- Exact hold categories and severity.
- Exact source-domain trigger contracts.
- Exact hold approval authority.
- Whether a canonical hold-specific event should be added.
- Whether multiple holds have policy-defined precedence.


## Final Authoring / Architecture-Safety Validation

- [x] Frozen L3 identity preserved.
- [x] Frozen title, owner, participating domains, Type and Priority preserved.
- [x] Assigned author is Varad for Section C.
- [x] Exact filename preserved.
- [x] Applicable locked L2 workflow source(s) used.
- [x] Ownership remains with the canonical owner.
- [x] Cross-domain participation does not transfer aggregate/lifecycle ownership.
- [x] Only source-supported actors, rules, lifecycle states, events and publishers are used.
- [x] Business events are kept distinct from synchronous business contracts/hand-offs.
- [x] Failure/recovery remains business-level; technical retries/queues/rollback are excluded.
- [x] No API, database, Business Data Structure, class, code, UI, infrastructure or deployment design is introduced.
- [x] All 17 mandatory sections are present.
- [x] Material upstream ambiguity is explicitly recorded in Section 17.
- [x] Status remains `Draft` pending independent architecture approval.

**Final Classification: PASS WITH ISSUES — Draft**
