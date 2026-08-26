# Cost Center Request to Active or Closed Cost Center

## 1. Scenario Metadata

**Scenario ID:** L3-F-037  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Budget & Cost Center Management  
**Participating Domains:** B, F  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Review  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** L2-F00; L2-F06; L2-B02; SutraERP L1 Business Domain Architecture (Locked); DEPENDENCY_MAP.md; L3 Business Scenario Template v2.0; Stage 3 Canonical Scenario Register — Section F  
**Referenced By:** None yet

## 2. Scenario Purpose

This scenario governs Finance Cost Center establishment and terminal processing while referencing, but not duplicating, Section B organisational structure.

## 3. Business Outcome

A Cost Center request reaches the source-supported governed outcome while authoritative organisational references and Finance ownership are preserved.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A governed request to establish or terminate a Cost Center is submitted.

### 4.2 End Boundary

For establishment, the Cost Center is Active. The frozen scenario title also names a terminal “Closed” outcome, but the exact terminal lifecycle term is not established by the authoritative L2 source and remains unresolved in Section 17.

### 4.3 In Scope

- Cost Center creation, classification and organisational reference validation
- activation
- governed terminal processing to the extent explicitly supported upstream

### 4.4 Out of Scope

- Section B organisation creation or modification
- Budget allocation
- accounting ledger or transaction ownership

### 4.5 Related Scenarios — Optional

No additional relationship is required to define this scenario boundary.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Budget & Cost Center Management | Owns the Finance Cost Center aggregate and its governed lifecycle actions |
| Section B — Institution & Structure | Provides authoritative organisational references without transferring ownership |

## 6. Business Actors, Roles and Responsibilities

L2-F06 does not define canonical institutional actors for Cost Center lifecycle actions. Finance owns the business responsibilities; named administrator/controller roles are not introduced at L3 and the actor/decision-authority mapping remains an upstream gap.

## 7. Preconditions and Entry Criteria

- The referenced organisational unit exists under Section B.
- The request identifies the required Cost Center classification and institutional scope.

## 8. Trigger

A valid Cost Center establishment or terminal-status request is presented.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate the request and authoritative organisational reference | Budget & Cost Center Management / Section B | Request is eligible |
| 2 | Create or identify the Cost Center under Finance ownership | Budget & Cost Center Management | Cost Center identity is governed |
| 3 | Activate the Cost Center when establishment criteria are satisfied | Budget & Cost Center Management | Cost Center becomes Active |
| 4 | For terminal processing, apply only a lifecycle action explicitly governed by authoritative upstream architecture | Budget & Cost Center Management | Unsupported terminal terminology is not manufactured at L3 |

## 10. Alternate Business Flows

No additional scenario-specific alternate flow is defined upstream.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Organisational reference is invalid | Cost Center cannot be established | Correct the authoritative Section B reference before resubmission | B / F |
| Requested terminal outcome depends on a lifecycle state not defined upstream | Terminal processing cannot be authoritatively completed at L3 | Preserve the last valid Cost Center lifecycle position and obtain the upstream lifecycle decision | Budget & Cost Center Management |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Cost Center references authoritative organisational data | Section B / Budget & Cost Center Management | No Finance copy of organisational structure | L2-F06 §§8.3, 8.6 |
| Cost Center is an independent Finance aggregate | Budget & Cost Center Management | Finance owns Cost Center lifecycle | L2-F06 §§9.5, 10.3 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Cost Center | Source-supported pre-activation position | Establish and activate the Cost Center | Active | Budget & Cost Center Management | L2-F06 Cost Center lifecycle semantics |

The frozen title’s terminal `Closed` term is not treated as a canonical L2 state unless and until authoritative upstream architecture establishes that lifecycle position.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| CostCenterActivated | Budget & Cost Center Management | Budget and interested organisational domains | Communicates activation | L2-F06 §§10.7, 14 |

No terminal event is invented for the unresolved terminal path.

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Section B | Budget & Cost Center Management | Authoritative organisational unit reference | Anchor Cost Center organisational identity | L2-B02; L2-F06 §8.3 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

L2-F06 does not define the canonical institutional actors that initiate or authorize Cost Center activation or terminal processing. L3 therefore keeps those responsibilities with the owning Finance domain pending upstream role mapping.

### 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, override or exception authority is authoritatively defined for this Cost Center lifecycle scenario.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve scenario decision traceability | Cost Center request and organisational reference | Budget & Cost Center Management |
| Preserve scenario decision traceability | Source-supported activation or terminal-processing decision | Budget & Cost Center Management |

Section A retains ownership of Authorization and Audit infrastructure; Budget & Cost Center Management owns the underlying business decision and evidence.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Establishment path: Cost Center is Active and `CostCenterActivated` is published.
- Terminal path remains incomplete until the upstream terminal lifecycle term and authority are established.

### 16.2 Postconditions

- Active Cost Center may receive controlled allocations.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-F06 records no canonical Actors section, so named `Cost Center Administrator` / `Financial Controller` authority cannot be established from authoritative L2. | L3 cannot assign named initiation/approval authority for Cost Center lifecycle actions. | Establish Cost Center actor/decision-authority mapping in authoritative L2/governance. |
| The frozen L3 title names an `Active or Closed Cost Center` outcome, while the current L2-F06 source does not independently establish the terminal `Closed` lifecycle state relied on by the prior ACR-based wording. | L3 must preserve the frozen title but cannot manufacture the terminal lifecycle transition or completion evidence. The scenario remains Review. | Resolve the Cost Center terminal lifecycle terminology/state upstream and trace the decision before Architecture Approval. |
| Scenario references to `ACR-F-2026-08-21-001` do not provide independently traceable governance provenance for either the actor mapping or terminal-state addition. | Prior ACR-based claims cannot be treated as authoritative repository facts. | Provide an independent governance/ADR/approval artifact or remove the unsupported approval reference repository-wide. |

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**Authoring Validation Classification:** REVIEW REQUIRED  
**Validation Note:** Source-supported Cost Center ownership and activation semantics are preserved. Unsupported ACR-based roles and terminal lifecycle claims were removed without changing the frozen scenario ID, filename or title.
