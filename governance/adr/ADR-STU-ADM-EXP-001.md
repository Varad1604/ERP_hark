# ADR-STU-ADM-EXP-001 — Canonical Admission Expiry Lifecycle

| Attribute | Value |
|---|---|
| Status | Approved |
| Date | 2026-08-22 |
| Architecture Level | L2 — Business Workflow Architecture |
| Domain | Section C — Student Lifecycle / Admissions |
| Decision Authority | Lead Architect authorization recorded in the architecture working session |
| Trigger | L3 Phase 5 architecture review of L3-C-006 |

## 1. Context

Locked `L2-C01_Admissions_Workflows.md` contains an internal inconsistency for admission expiry:

- `Expired` is defined as a terminal Admission state.
- `AdmissionExpired` is defined as a canonical Admissions event.
- the general Admission transition matrix includes `Admission Initiated → Expired`.
- a later matrix labelled as canonical omits the expiry transition.

Because Level 3 must consume one authoritative lifecycle and must not choose silently between conflicting locked definitions, the omission requires an explicit upstream decision.

## 2. Decision

The canonical Admission lifecycle includes the following expiry transition:

`Admission Initiated → Expired`

The transition occurs only when the institution-configured admission expiry policy determines that the Admission has expired.

`Expired` remains a terminal Admission state.

`AdmissionExpired` remains the canonical business event published by Admissions after the expiry outcome is conclusively established.

Any omission of this transition from a later lifecycle matrix in the original L2-C01 document is superseded by this decision.

## 3. Ownership

Admissions retains canonical ownership of:

- the Admission lifecycle;
- the `Expired` Admission state;
- the expiry decision under institution-configured Admissions policy; and
- publication of `AdmissionExpired`.

No ownership is transferred to Finance, Student Activation, Academic Structure, Platform Foundation, or any downstream consumer.

## 4. Constraints

- This decision does not introduce a new Admission state.
- This decision does not introduce a new business event.
- No numeric expiry duration is fixed at architecture level.
- No additional source state for expiry is introduced without a separate upstream decision.
- Downstream L3 scenarios may consume `Admission Initiated → Expired` and `AdmissionExpired` but may not redefine them.

## 5. Consequences

- L3-C-006 can use one authoritative lifecycle definition.
- The canonical Admissions event vocabulary remains unchanged.
- Existing L2-C01 content is unchanged except where it conflicts with the locked amendment implementing this ADR.

## 6. Implementation in Architecture Repository

This ADR is incorporated through:

`architecture/L2_Domain_Workflow_Architecture/section-c/L2-C01_Admissions_Workflows_Amendment_2026-08-22.md`

The amendment has precedence only for the admission-expiry subject described here. All unaffected L2-C01 content remains authoritative.
