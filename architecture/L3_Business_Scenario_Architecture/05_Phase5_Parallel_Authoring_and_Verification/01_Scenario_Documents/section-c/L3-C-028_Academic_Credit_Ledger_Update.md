# Academic Credit Ledger Update

**Scenario ID:** L3-C-028  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Progression  
**Participating Domains:** C, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Derivation:** KEEP  
**Status:** Draft  
**Author:** Varad  
**Version:** 1.0  
**Last Updated:** 2026-08-20  
**Exact Filename:** `L3-C-028_Academic_Credit_Ledger_Update.md`

> **Production-authoring note:** This document is authored to production-quality L3 standards and is intended for architecture review. Per Phase 5 governance, a newly authored scenario must remain **Status: Draft** until independently approved; “production-ready” does not mean self-approved or locked.

---


| Attribute | Value |
|---|---|
| Final L3 ID | `L3-C-028` |
| Canonical Title | Academic Credit Ledger Update |
| Primary Owner | Academic Progression |
| Participating Domains | C, I |
| Scenario Type | Critical |
| Priority | P1 |
| Derivation | KEEP |
| Assigned Author | Varad |
| Status | Draft |

The frozen Section C register identifies C028 exactly as **Academic Credit Ledger Update**, owned by Academic Progression, participating in C and I, Critical, P1, KEEP.

**Applicable authoritative L2:** `L2-C04_Student_Academic_Progression_Workflows.md`.

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-C-028 |
| Canonical Title | Academic Credit Ledger Update |
| Primary Owner | Academic Progression |
| Participating Domains | C, I |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs maintenance of the Student's authoritative institutional Academic Credit Ledger following verified academic outcomes.

The Academic Credit Ledger is owned within Academic Progression. It records earned, transferred, recognised, remaining and qualification credits and provides the authoritative credit basis used by progression evaluation.

Examination provides official academic outcomes. Academic Progression evaluates those outcomes and updates its own credit ledger. No source-domain examination record is modified by the ledger update.

---

## 3. Business Outcome

The Student's Academic Credit Ledger accurately reflects the authoritative credit outcome for the applicable evaluation cycle.

The resulting ledger:

- records valid earned credits;
- prevents duplicate recognition;
- follows the applicable curriculum structure;
- preserves auditable credit history;
- provides the credit basis for progression evaluation; and
- may participate in ABC synchronization where applicable.

The ledger remains owned by Academic Progression.

---

## 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authoritative academic outcome requires the Academic Credit Ledger to be evaluated or updated, including progression evaluation and officially published academic outcomes.

## 4.2 End Boundary

The scenario ends when the applicable credit entries and summary are updated, validated and auditable.

Where ABC synchronization is applicable, C04 APW-12 may continue the downstream synchronization process; C028 does not transfer ledger ownership to an external credit ecosystem.

## 4.3 In Scope

- Evaluation of earned credits.
- Recording recognised/approved credits.
- Updating cumulative credit totals.
- Preventing duplicate credit recognition.
- Applying curriculum credit rules.
- Preserving credit history.
- Producing credit summary for progression.
- Preparing the ledger outcome for ABC synchronization where applicable.

## 4.4 Out of Scope

- Examination evaluation.
- Examination result creation/correction.
- Curriculum ownership.
- Academic Registration ownership.
- External ABC connector ownership.
- Qualification conferment.
- Technical persistence, APIs, payloads, queues or infrastructure.

---

## 5. Ownership and Participating Domains

| Domain | Responsibility | Ownership |
|---|---|---|
| **Academic Progression** | Owns Academic Credit Ledger and credit recognition within the progression domain. | **Canonical owner** |
| **Student Lifecycle (C)** | Provides Student lifecycle identity/context and consumes progression consequences. | Retains Student ownership. |
| **Examination (I)** | Provides official academic outcomes from which earned credits may be derived. | Retains examination ownership. |

L2-C04 explicitly identifies Academic Credit Ledger as an Academic Progression aggregate and states that Academic Progression owns the authoritative credit history.

---

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Boundary |
|---|---|---|
| Academic Progression | Evaluates credits and maintains the authoritative ledger. | Sole ledger owner. |
| Examination | Publishes official examination outcomes used as academic evidence. | Does not update the ledger directly. |
| Authorised academic authority | Approves credit recognition where policy requires approval, especially transferred credits. | Cannot bypass ledger rules. |
| Student Lifecycle | Consumes resulting progression/credit consequences where applicable. | Does not own ledger. |

For transferred credits, L2-C04 explicitly requires institutional approval before the credits contribute to progression.

---

## 7. Preconditions and Entry Criteria

- Official academic outcomes are available.
- The Student has an applicable academic context.
- The applicable curriculum/credit structure is available.
- The credit policy is active.
- The Student's existing ledger history is available for duplicate checking.
- Any transferred/recognised credits have the required institutional approval.

Only authoritative academic information may update the ledger.

---

## 8. Trigger

The business trigger is an authoritative academic outcome or progression evaluation requiring credit accumulation to be recalculated/updated.

C04 APW-03 places the credit ledger workflow in the sequence:

**Progression Decision → Calculate Credits → Validate Rules → Evaluate Student Progression → Synchronise ABC.**

---

## 9. Main Business Flow

1. An authoritative academic outcome becomes available for credit evaluation.
2. Academic Progression identifies the applicable Student and academic period.
3. Existing Academic Credit Ledger history is evaluated.
4. Successfully earned credits are identified.
5. Transferred or recognised credits are included only where institutionally approved.
6. Curriculum credit rules are applied.
7. Duplicate credit recognition is prevented.
8. The resulting earned/recognised/remaining/qualification credit position is calculated.
9. The Academic Credit Ledger is updated.
10. The updated ledger is validated.
11. A credit summary is produced for progression evaluation.
12. The ledger change is audited.
13. Where applicable, the outcome becomes available for ABC synchronization through APW-12.

---

## 10. Alternate Business Flows

## 10.1 Ordinary Earned Credit

Official examination/academic outcome confirms successful completion. The corresponding credit is recognised according to curriculum rules.

## 10.2 Transferred Credit

A transferred credit is considered only after institutional approval. The credit is then recorded as recognised/transfer credit.

## 10.3 Duplicate Credit

If an equivalent credit has already been recognised, the duplicate is rejected rather than creating a second credit entry.

## 10.4 Revised Academic Outcome

A permitted revised/re-evaluated official outcome may trigger a new credit evaluation. Historical records are preserved; they are not silently overwritten.

## 10.5 ABC Synchronization

After the authoritative institutional ledger is updated, APW-12 may synchronize eligible credits with the Academic Bank of Credits where applicable.

---

## 11. Failure, Interruption and Business Recovery

| Condition | Outcome | Business Resolution |
|---|---|---|
| Academic outcome is not official | Credit cannot be recognised authoritatively. | Await official publication. |
| Curriculum credit mapping unavailable | Credit calculation cannot complete. | Resolve curriculum dependency. |
| Duplicate credit detected | Duplicate entry is rejected. | Retain the existing authoritative credit. |
| Transferred credit lacks approval | Credit is not recognised. | Complete institutional approval. |
| Credit calculation violates policy | Ledger update does not complete. | Resolve the underlying academic/policy condition. |
| Revised result changes credit outcome | Fresh evaluation is required where policy permits. | Preserve historical ledger history and apply the authorised revision process. |

Technical retries and infrastructure mechanics are excluded.

---

## 12. Business Rules and Decision Constraints

The authoritative C04 rules include:

- only successfully earned credits contribute to progression;
- transferred credits contribute only after institutional approval;
- duplicate academic credits are never recognised;
- credit calculations follow the applicable curriculum structure;
- Credit Ledger balances remain non-negative;
- Academic Credit Ledger entries remain fully auditable;
- historical progression/credit records remain immutable;
- progression uses the authoritative ledger rather than independently recreating credit history.

The ledger therefore remains the canonical credit record for the Student academic progression context.

---

## 13. State and Lifecycle Impact

The Academic Credit Ledger is an authoritative cumulative record rather than a progression-state machine.

Business lifecycle:

```text
Authoritative Academic Outcome
          │
          ▼
Credit Evaluation
          │
          ▼
Credit Validation
          │
          ▼
Ledger Update
          │
          ▼
Credit Summary Available
          │
          ▼
Progression / ABC Consumption
```

No new ledger state is invented.

The ledger records credit entries and cumulative credit position while Academic Progression separately owns the Student's progression state.

---

## 14. Business Events and Cross-Domain Collaboration

C04 APW-03 explicitly produces:

- Updated Credit Ledger
- Credit Summary
- ABC Synchronisation Event

The C04 event architecture must be used as the canonical source for exact event names and publishers.

For C028:

- Examination publishes official academic outcomes.
- Academic Progression evaluates and records the credit result.
- Academic Progression remains the canonical publisher/owner of the resulting progression-domain credit outcome.
- ABC synchronization occurs through APW-12 where applicable.

No new `CreditLedgerUpdated` event is invented merely because the scenario title implies an update.

---

## 15. Business Authorization, Approval and Audit Obligations

Credit recognition must remain policy governed.

Transferred/recognised credits require institutional approval before contributing to progression.

The C04 architecture requires:

- complete auditability;
- immutable historical records;
- authorized manual intervention;
- policy-version traceability;
- auditable credit entries.

No technical authorization model is introduced.

---

## 16. Completion Criteria and Postconditions

The scenario is complete when:

- authoritative academic evidence has been evaluated;
- applicable curriculum/credit rules have been applied;
- duplicate recognition has been prevented;
- approved transferred/recognised credits have been included;
- the Academic Credit Ledger is updated;
- the credit summary is available;
- audit evidence exists.

Postconditions:

- the ledger is the authoritative institutional credit record for the Student;
- cumulative credits remain internally consistent;
- historical credit evidence remains auditable;
- progression evaluation can use the updated credit position;
- ABC synchronization can proceed where applicable.

---

## 17. Open Issues, Assumptions and Deferred Questions

## Open Issues

**OI-C028-01 — Exact canonical event name for the ledger update is not explicitly stated in the available C04 APW-03 excerpt.**  
C04 explicitly defines an “ABC Synchronisation Event” as an output, but the exact ledger-update event name is not safely inferable. Therefore no new event name is invented here.

**OI-C028-02 — Exact Examination-to-Progression credit contract is not redefined at L3.**  
The dependency is authoritative academic outcomes from Examination to Academic Progression. Detailed contract semantics belong to L4.

**OI-C028-03 — Policy Management ownership is unresolved in C04.**  
The progression policy input has no resolved owner in the L2 metadata. C028 does not invent one.

## Assumptions

1. Academic Progression owns the Academic Credit Ledger.
2. Examination remains authoritative for examination outcomes.
3. Curriculum remains authoritative for curriculum credit structure.
4. ABC synchronization does not transfer institutional ledger ownership.
5. Historical ledger evidence remains auditable and is not silently overwritten.

## Deferred Questions

- Exact ledger-update event name, if required.
- Exact ABC event/contract semantics.
- Exact revised-result reconciliation policy.
- Exact policy owner for institutional credit rules.


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
