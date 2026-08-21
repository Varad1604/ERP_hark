# Cost Center Request to Active or Closed Cost Center

## 1. Scenario Metadata

**Scenario ID:** L3-F-037  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Budget & Cost Center Management  
**Participating Domains:** B, F  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-21  
**Depends On:** L2-F00; L2-F06; L2-B02; SutraERP L1 Business Domain Architecture (Locked); DEPENDENCY_MAP.md; L3 Business Scenario Template v2.0; Stage 3 Canonical Scenario Register — Section F  
**Referenced By:** None yet

## 2. Scenario Purpose

This scenario governs Finance Cost Center establishment and termination while referencing, but not duplicating, Section B organisational structure.

## 3. Business Outcome

A Cost Center request reaches its governed active or terminal outcome with authoritative organisational references preserved.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A governed request to establish or terminate a Cost Center is submitted.

### 4.2 End Boundary

The Cost Center is active or has reached the locked title’s terminal “Closed” outcome.

### 4.3 In Scope

- Cost Center creation, classification and organisational reference validation
- activation
- governed terminal processing

### 4.4 Out of Scope

- Section B organisation creation or modification
- Budget allocation
- accounting ledger or transaction ownership

### 4.5 Related Scenarios — Optional

No additional relationship is required to define this scenario boundary.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Budget & Cost Center Management | Owns the Finance decision or accounting outcome and its evidence |
| Section B — Institution & Structure | Provides authoritative organisational references without transferring ownership |

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Scenario Responsibility |
|---|---|
| Budget Planning Officer | Prepares the governed Budget proposal, allocation, revision, virement or closure request applicable to this scenario. |
| Budget Control Officer | Performs Budget-control and eligibility assessment under approved Finance policy. |
| Finance Committee | Authorizes governed Budget approval, allocation, structural change or closure where applicable. |
| Cost Center Administrator | Prepares Finance-owned Cost Center lifecycle actions where applicable. |
| Financial Controller | Independently authorizes Cost Center activation, deactivation, reactivation or closure where applicable. |

Roles and authority are established by approved `ACR-F-2026-08-21-001` and do not define technical access control.

## 7. Preconditions and Entry Criteria

- The referenced organisational unit exists under Section B.
- The request identifies the required Cost Center classification and institutional scope.

## 8. Trigger

A valid Cost Center establishment or terminal-status request is presented.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate request and authoritative organisational reference | Finance Administration / Section B | Request is eligible |
| 2 | Create or identify the Cost Center under Finance ownership | Finance Administration | Cost Center identity is governed |
| 3 | Activate the Cost Center when establishment criteria are satisfied | Finance Administration | Cost Center becomes active |
| 4 | For terminal processing, apply only an upstream-authorized lifecycle term | Budget & Cost Center Management | Terminal outcome awaits terminology resolution |

## 10. Alternate Business Flows

No additional scenario-specific alternate flow is defined upstream.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Organisational reference is invalid | Cost Center cannot be established | Correct Section B reference before resubmission | B / F |
| Terminal closure requested | Cost Center must first be Deactivated and pass closure eligibility | Cost Center Administrator prepares; Financial Controller authorizes Deactivated to Closed | Budget & Cost Center Management |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Cost Center references authoritative organisational data | Section B / Budget & Cost Center Management | No Finance copy of organisational structure | L2-F06 §§8.3, 8.6 |
| Cost Center is an independent Finance aggregate | Budget & Cost Center Management | Finance owns Cost Center lifecycle | L2-F06 §§9.5, 10.3 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Cost Center | Created / Active / Deactivated | Apply the approved governed lifecycle action | Active, Deactivated, or terminal Closed | Budget & Cost Center Management | Approved ACR-F-2026-08-21-001 |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| CostCenterActivated | Budget & Cost Center Management | Budget and interested organisational domains | Communicates activation | L2-F06 §§10.7, 14 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Section B | Budget & Cost Center Management | Authoritative organisational unit reference | Anchor Cost Center organisational identity | L2-B02; L2-F06 §8.3 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Create and maintain Cost Center | Finance Administration | Supported Finance capability | Budget & Cost Center Management |
| Authorize activation, deactivation, reactivation or terminal closure | Financial Controller | Cost Center Administrator prepares; closure requires Deactivated state and eligibility | Budget & Cost Center Management |

### 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty rule is authoritatively defined.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve scenario decision traceability | Cost Center request and organisational reference | Budget & Cost Center Management |
| Preserve scenario decision traceability | Activation evidence or terminal decision evidence | Budget & Cost Center Management |

Section A retains ownership of Authorization and Audit infrastructure; Budget & Cost Center Management owns the underlying business decision and evidence.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Activation path: Cost Center is active and `CostCenterActivated` is published.
- Terminal path cannot be completed until upstream terminology is resolved.

### 16.2 Postconditions

- Active Cost Center may receive controlled allocations.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No unresolved material architecture issue remains for this scenario. The former upstream gaps were resolved by approved `ACR-F-2026-08-21-001` and its exact amendment specification.

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**Authoring Validation Classification:** PASS  
**Validation Note:** Source-grounded Draft revalidated against approved ACR-F-2026-08-21-001; no unresolved material L3 issue remains.
