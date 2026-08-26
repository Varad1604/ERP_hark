# Active Budget to Authorized Revision or Virement

## 1. Scenario Metadata

**Scenario ID:** L3-F-034  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Budget & Cost Center Management  
**Participating Domains:** B, F  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Review  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** L2-F00; L2-F06; L2-B02; SutraERP L1 Business Domain Architecture (Locked); DEPENDENCY_MAP.md; L3 Business Scenario Template v2.0; Stage 3 Canonical Scenario Register — Section F  
**Referenced By:** None yet

## 2. Scenario Purpose

This scenario governs a traceable structural change to an Active Budget without transferring accounting or organisational ownership.

## 3. Business Outcome

An authorized revision or virement is applied to the Active Budget, prior versions remain preserved, and the applicable canonical event is published.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A governed need to modify an Active Budget is formally raised.

### 4.2 End Boundary

The authorized change is applied and versioned without violating Budget bounds or ring-fencing.

### 4.3 In Scope

- budget increase, reduction, reallocation or supplementary revision
- virement between eligible Cost Centers or Budget Heads
- maker-checker authorization and historical preservation

### 4.4 Out of Scope

- initial Budget approval
- ordinary allocation creation
- accounting correction, procurement execution or Budget closure

### 4.5 Related Scenarios — Optional

No additional relationship is required to define this scenario boundary.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Budget & Cost Center Management | Owns the Budget structural-change decision, lifecycle impact and evidence |
| Section B — Institution & Structure | Provides authoritative organisational references without transferring ownership |

## 6. Business Actors, Roles and Responsibilities

L2-F06 requires maker-checker governance for Active-Budget structural changes but does not define the canonical institutional actors that perform the maker and checker responsibilities. Those responsibilities remain expressed at owning-domain level until upstream actor mapping is governed; see Section 17.

## 7. Preconditions and Entry Criteria

- The Budget is Active.
- The request identifies the affected Budget, Budget Period, allocations and Cost Centers.
- A business justification is available.

## 8. Trigger

A governed revision or virement request is raised against an Active Budget.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate Budget state, request scope and authoritative organisational references | Budget & Cost Center Management | Request is eligible for decision |
| 2 | Apply Budget bounds, grant ring-fencing and virement constraints | Budget & Cost Center Management | Permissible change is established |
| 3 | Obtain the required independent maker-checker authorization | Budget & Cost Center Management | Change is authorized |
| 4 | Apply the change as a preserved successor version | Budget & Cost Center Management | Active Budget reflects authorized change |
| 5 | Publish `BudgetRevised` or `BudgetVirementApproved` | Budget & Cost Center Management | Outcome is communicated |

## 10. Alternate Business Flows

A revision may increase, reduce, reallocate or supplement the Budget; a virement transfers allocations without changing the parent Budget total.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Request is rejected | Active Budget remains unchanged | Preserve rejection evidence; no new lifecycle state is created | Budget & Cost Center Management |
| Grant ring-fencing or Budget bounds would be violated | Change cannot proceed | Reject or correct the request under governed policy | Budget & Cost Center Management |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Virement does not alter parent Budget TotalAmount | Budget & Cost Center Management | Transfer must remain balanced | L2-F06 §12.2 |
| Grant Budgets are ring-fenced | Budget & Cost Center Management | No virement into general revenue or capital pools | L2-F06 §17.3 |
| Active-Budget structural changes require maker-checker | Budget & Cost Center Management | Initiator cannot approve the same change | L2-F06 §16.1 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Budget | Active | Authorized revision preserves a successor version; no new Budget state is defined | Active | Budget & Cost Center Management | L2-F06 §§6.7–6.8, 13 |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| BudgetRevised | Budget & Cost Center Management | Interested Finance and reporting domains | Communicates authorized revision | L2-F06 §§10.7, 14 |
| BudgetVirementApproved | Budget & Cost Center Management | Interested Finance domains | Communicates authorized virement | L2-F06 §14.1 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Section B | Budget & Cost Center Management | Authoritative organisational and Cost Center context | Validate organisational references | L2-B02; L2-F06 §8.3 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

L2-F06 establishes maker-checker as a business control for Active-Budget structural changes, but it does not name the institutional maker or checker roles. L3 therefore preserves the control while leaving the actor mapping unresolved.

### 15.2 Separation of Duties, Overrides and Exceptions

Maker-checker is mandatory under L2-F06 §16.1: the initiating and approving responsibilities must be exercised independently. No named institutional roles, override authorities or exception roles are introduced at L3.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve scenario decision traceability | Change request and business justification | Budget & Cost Center Management |
| Preserve scenario decision traceability | Prior and successor Budget versions | Budget & Cost Center Management |
| Preserve scenario decision traceability | Independent maker-checker decision and applicable canonical business event | Budget & Cost Center Management |

Section A retains ownership of Authorization and Audit infrastructure; Budget & Cost Center Management owns the underlying business decision and evidence.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Authorized change is reflected in the Active Budget.
- Historical versions are preserved.
- Applicable canonical event is published.

### 16.2 Postconditions

- The revised allocations govern subsequent Budget control.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-F06 records no canonical Actors section. Scenario references to `ACR-F-2026-08-21-001` do not constitute independently traceable approval provenance, and no separate repository governance/ADR/approval artifact establishing the named maker/checker roles was found. | The maker-checker control is authoritative, but the institutional actor mapping cannot be asserted at L3. The scenario remains Review. | Establish the maker/checker actor mapping in authoritative L2/governance and provide independently traceable approval provenance before promotion beyond Review. |

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**Authoring Validation Classification:** REVIEW REQUIRED  
**Validation Note:** L2-F06 maker-checker governance is preserved. Unsupported ACR-based named roles were removed and the upstream actor/provenance gap remains explicit.
