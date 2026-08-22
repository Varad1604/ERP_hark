# 1. Scenario Metadata

# Scheduled Non Theory Assessment to Completed Assessment Record

**Scenario ID:** L3-I-043  
**Primary Owner:** Practical & Viva Examination Conduct  
**Participating Domains:** Academic Delivery, Examination  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Author:** Pallav  
**Architecture Approval:** ACR-I-2026-08-21-002 — Approved by Vikram Desai, Enterprise Architecture Governance Lead, 21 August 2026; Conditions: None  
**Post-Approval Lock:** Authorized 21 August 2026 under ACR-I-2026-08-21-002  
**Canonical Source Candidate:** CAN-L3-I-043  
**Primary L2 Source:** L2-I06

# 2. Scenario Purpose

This scenario governs **scheduled non theory assessment to completed assessment record** as one complete institutional business outcome. It applies frozen semantics and locked L2-I06 without redefining upstream architecture.

# 3. Business Outcome

The scenario concludes when **completed Assessment Record** is established as the authoritative governed outcome with required business evidence and hand-offs.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario starts when **scheduled Non Theory Assessment** is established in a valid examination context and source-defined entry conditions are available.

## 4.2 End Boundary

The scenario ends when **completed Assessment Record** is authoritatively determined and recorded for downstream business use.

## 4.3 In Scope

- Verification of source-defined context and entry conditions.
- Application of L2-I06 rules and authority.
- Frozen cross-domain collaboration.
- Establishment and preservation of the final outcome.
- Source-supported events, hand-offs, evidence, and audit obligations.

## 4.4 Out of Scope

- Redefinition of upstream policy, eligibility, academic, workforce, facility, finance, or identity rules.
- Ownership of upstream facts or downstream lifecycles.
- Outcomes beyond this scenario end boundary.
- Technical solution, data, interface, access-control, infrastructure, or recovery design.

## 4.5 Related Scenarios — Optional

Related scenarios provide prerequisites or consume this outcome under locked dependency direction without ownership transfer.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Practical & Viva Examination Conduct | Owns the governed final outcome and applicable aggregate lifecycle. |
| Academic Delivery | Provides only canonically owned facts, decisions, resources, assignments, or confirmations. |

The Primary Owner retains the outcome; participating domains retain their canonical responsibilities.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility |
|---|---|
| Candidate | Participates in authorised assessment activities |
| Internal Examiner | Conducts and supervises assessments |
| External Examiner | Participates where institutional policy requires external assessment |
| Chief Examiner | Oversees assessment governance where applicable |
| Examination Administrator | Coordinates assessment operations |

Only responsibilities supported by L2-I06 apply. No actor or authority is invented.

# 7. Preconditions and Entry Criteria

- The governing examination context exists in the state required by L2-I06.
- Approved policy and source-owned facts are authoritative.
- Required upstream decisions, assignments, eligibility facts, or conduct outcomes are complete where applicable.
- Initiation and decision responsibilities are valid.
- No unresolved ownership, identity, scope, or lifecycle conflict prevents progression.

# 8. Trigger

**Scheduled Non Theory Assessment.**

This is a business occurrence or authoritative condition, not an implementation action.

# 9. Main Business Flow

1. The responsible participant recognizes the authoritative trigger.
2. The Primary Owner verifies source-defined preconditions and upstream facts.
3. Participating domains provide only their canonically owned contributions.
4. Examination participants assess the matter under approved policy and L2-I06.
5. Source-defined conflicts, exceptions, approvals, or continuity obligations are resolved by authorized responsibilities.
6. The Primary Owner establishes **completed Assessment Record**.
7. Required evidence and source-supported events or hand-offs are preserved.
8. The scenario stops at its end boundary.

# 10. Alternate Business Flows

## 10.1 Source-Defined Exception Resolved

An authorized participant applies the resolution defined by L2-I06, preserves its basis, and returns to the main flow.

## 10.2 Authoritative Input Requires Clarification

The owning domain resolves an incomplete or inconsistent input; Examination does not redefine it.

## 10.3 Governed Terminal Outcome

Only a terminal outcome defined by locked policy may conclude the alternate path.

# 11. Failure, Interruption and Business Recovery

- Progression is suspended if required authority, evidence, or authoritative input is unavailable.
- Disputed facts return to their canonical owner.
- The last valid business position and evidence are preserved.
- Recovery occurs only after the missing condition is satisfied or an authorized terminal decision is reached.
- Technical exception and recovery mechanisms are outside L3.

# 12. Business Rules and Decision Constraints

| Constraint | Application | Owner / Source |
|---|---|---|
| Applicable L2-I06 constraint | L2-I08 Marks Collection & Evaluation — Completed assessment records, attendance status and examiner completion | L2-I06 |
| Applicable L2-I06 constraint | Assessment Prepared — Assessment readiness completed — Examination Administration | L2-I06 |
| Applicable L2-I06 constraint | Assessment Completed — Assessment execution completed — L2-I08 | L2-I06 |
| Applicable L2-I06 constraint | Assessment Attendance — Record candidate participation — I08 | L2-I06 |

No local threshold, deadline, formula, authority, state, or rule is created.

# 13. State and Lifecycle Impact

| Lifecycle State / Impact | Meaning |
|---|---|
| No new lifecycle state | Only lifecycle impacts explicitly defined by L2-I06 apply. |

Activities are not lifecycle states. Transitions require source-defined authority and conditions.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

### Published Events

| Event | Canonical Publisher | Published When | Consumers |
|---|---|---|---|
| L2-I08 Marks Collection & Evaluation | L2-I06 / Examination | Completed assessment records, attendance status and examiner completion | Authoritative consumers |
| Assessment Prepared | L2-I06 / Examination | Assessment readiness completed | Examination Administration |
| Assessment Completed | L2-I06 / Examination | Assessment execution completed | L2-I08 |
| Assessment Attendance | L2-I06 / Examination | Record candidate participation | I08 |

### Consumed Events

| Event | Canonical Publisher | Business Purpose |
|---|---|---|
| L2-I08 Marks Collection & Evaluation | Completed assessment records, attendance status and examiner completion | Authoritative prerequisite |
| Assessment Attendance | Record candidate participation | Authoritative prerequisite |
| Assessment Completion | Completed Assessment Activity | Authoritative prerequisite |
| Completed | Assessment execution finished | Authoritative prerequisite |

No event is invented where no exact matching canonical event is supported.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider | Consumer | Business Hand-off |
|---|---|---|
| Academic Delivery | Practical & Viva Examination Conduct | Provides only canonically owned facts, decisions, resources, assignments, or confirmations. |

These are business hand-offs only, with no technical contract design.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Initiation, review, recommendation, approval, and final decision remain distinct where L2-I06 distinguishes them. Only source-supported actors exercise them.

## 15.2 Separation of Duties, Overrides and Exceptions

No control, override, or exceptional authority is created locally.

## 15.3 Business Audit and Evidence Obligations

The institution must demonstrate the trigger, context, contributing facts, responsible participants, decision basis, applied policy, exceptions, prior position where changed, and final outcome.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Trigger and preconditions were valid.
- Authoritative facts and contributions were considered.
- Required authority and constraints were applied.
- **Completed Assessment Record** was established without transferring ownership.
- Required evidence, events, and hand-offs were preserved where supported.
- No downstream outcome was absorbed.

## 16.2 Postconditions

- The final outcome is authoritative for its scope.
- Prior evidence remains traceable where an earlier position is revised, consolidated, reopened, or corrected.
- Downstream capabilities may consume the outcome through source-supported collaboration.
- All domains retain canonical ownership.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No scenario-specific material conflict is introduced by this draft. Later-discovered conflicts require upstream change control.

## 17.2 Assumptions

No assumption overrides locked architecture. Referenced locked sources are assumed unchanged.

## 17.3 Deferred Questions

System behaviour, solution, integration, security, data, and user-experience design are deferred to L4–L6.
