# Capital Renovation Proposal to Infrastructure Change Governance

# 1. Scenario Metadata

**Scenario ID:** L3-G-308  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, J, F, B  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-J00 — Enterprise Resource Management Architecture & Governance; L2-F06 — Budget & Cost Center Management; L2-B03 — Campus Workflows  
**Referenced By:** Authorized institutional processes requiring governed evaluation of a proposed major physical-infrastructure renovation

# 2. Scenario Purpose

Govern a proposed major renovation affecting canonical physical Infrastructure so that the proposed physical change and its impact are recorded under Infrastructure Management ownership while procurement, budget and institutional-campus concerns remain with their canonical domains.

# 3. Business Outcome

The proposed infrastructure change is governed and its impact on physical infrastructure is recorded

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a major infrastructure renovation is proposed.

## 4.2 End Boundary

The scenario ends when Infrastructure Management has evaluated the proposal against the authoritative physical Infrastructure context, recorded the governed proposed change and its identified physical impact, and preserved any required cross-domain references without beginning renovation execution.

## 4.3 In Scope

- Identification of the authoritative physical Infrastructure affected by the proposal.
- Recording the proposed major modification and its physical-infrastructure impact under G10 governance.
- Validation of physical hierarchy, lifecycle and applicable Infrastructure constraints.
- Recognition of Section J procurement/resource-acquisition context where the proposal creates a resource requirement.
- Recognition of Finance budget-planning context where a financial allocation or budget validation is applicable.
- Reference to B03 InstitutionalCampus identity/governance context without transferring physical ownership.
- Preservation of Infrastructure change-governance history.

## 4.4 Out of Scope

- Renovation execution.
- Purchase requisition, sourcing, purchase order, goods receipt or inventory execution.
- Budget creation, revision, accounting, payment or financial posting.
- InstitutionalCampus lifecycle or governance modification.
- Automatic Infrastructure closure, inspection, compliance validation or return to Operational state before an approved renovation proceeds.
- Creation of a new renovation aggregate, state or event not defined by locked sources.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-G-309 — Capital Renovation Completion to Updated Physical Infrastructure | Covers the later completion and physical-infrastructure reconciliation outcome. |
| Related | L3-G-299 — Administrative Infrastructure Change to Governed Update | Handles authorized administrative Infrastructure changes outside the capital-renovation outcome. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns the physical Infrastructure, renovation governance over physical change, hierarchy/lifecycle constraints and the recorded physical impact. |
| Enterprise Resource Management / Section J | Retains canonical ownership of procurement, sourcing, purchase requisitions, purchase orders, goods receipt, inventory and operational asset/resource lifecycles if those capabilities are required by the renovation. |
| Finance / Section F | Retains ownership of budget planning, allocation and financial truth; procurement may request budget validation but does not transfer budget ownership. |
| Institution & Structure / Section B | Provides the canonical InstitutionalCampus identity and governance context; it does not own the physical campus, buildings, floors, spaces or physical lifecycle. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Initiates or records the governed Infrastructure renovation proposal within authorized scope. | May act only where the applicable business authorization permits. |
| Infrastructure Manager | Reviews the proposal against physical Infrastructure context and G10 governance. | Does not gain Finance, Procurement or InstitutionalCampus ownership. |
| Auditor | Reviews preserved governance history and evidence. | Read-only participation. |

The reviewed F06 source contains no canonical actor catalogue, and no renovation-specific J or B actor is defined by the sources reviewed for this scenario. None is invented.

# 7. Preconditions and Entry Criteria

- The affected physical Infrastructure exists under G10 ownership.
- The proposal identifies the physical Infrastructure and the major modification being considered.
- The applicable InstitutionalCampus reference is resolvable where campus context is required.
- Any existing budget or procurement context used for evaluation remains owned by its source domain.
- The business operation is authorized before governed recording or change action proceeds.

# 8. Trigger

A major infrastructure renovation is proposed

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the canonical physical Infrastructure affected by the renovation proposal. | Infrastructure Management | The authoritative physical target is established. |
| 2 | Validate that the proposal may be governed within the applicable Infrastructure scope and lifecycle context. | Infrastructure Management | Invalid physical scope or lifecycle assumptions are not accepted. |
| 3 | Record the proposed major modification and identify the affected physical hierarchy, attributes, capacity, capability, availability or other G10-owned facts as applicable. | Infrastructure Management | The proposed physical change and impact are captured without executing them. |
| 4 | Reference the InstitutionalCampus identity/governance context where the physical proposal affects a campus-level institutional context. | Institution & Structure / Infrastructure Management | Institutional context is preserved without transferring physical ownership. |
| 5 | Where the proposal creates procurement/resource needs, retain those needs as Section J concerns without executing procurement in this scenario. | Enterprise Resource Management | Procurement ownership remains in Section J. |
| 6 | Where budget availability or planned allocation is relevant, retain Finance as the owner of that financial planning context. | Finance | Budget ownership remains in Finance. |
| 7 | Conclude the proposal-governance outcome with the proposed Infrastructure change and physical impact recorded for the next authorized decision or execution stage. | Infrastructure Management | The frozen business outcome is reached without beginning renovation execution. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Proposal has no procurement requirement | The proposed change uses no Section J resource-acquisition capability. | Complete Infrastructure governance without inventing a procurement hand-off. | Physical impact remains governed by G10 only. | Infrastructure Management |
| Proposal has no new budget action | Existing approved financial context is sufficient or no budget validation is required by the relevant Finance policy. | Do not invent a budget revision or commitment. | Finance ownership remains unchanged. | Finance / Infrastructure Management |
| Proposal affects institutional campus context | The change has campus-level institutional significance. | Reference the B03 InstitutionalCampus context while keeping all physical change facts in G10. | Institutional and physical ownership remain separated. | Institution & Structure / Infrastructure Management |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Physical impact cannot be determined from authoritative Infrastructure facts | The proposal cannot be governed reliably. | Resolve the canonical G10 information before concluding the proposal assessment. | Infrastructure Management |
| Required authorization is not available | The governed proposal action does not proceed. | Preserve the proposal context without applying any Infrastructure change; obtain a later valid authorization if appropriate. | Infrastructure Management / Section A authorization capability |
| A procurement or budget action is required but not yet governed by its owning domain | The renovation proposal cannot imply those downstream outcomes. | Preserve the identified need as cross-domain context and use the separately owned J or F workflow. | Enterprise Resource Management / Finance |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Physical Infrastructure is canonically owned by G10. | Infrastructure Management | Renovation governance cannot transfer physical ownership to J, F or B. | L2-G10 §§1–6 |
| Capital Renovation governs major modifications to Infrastructure. | Infrastructure Management | The proposal is handled as physical-infrastructure change governance. | L2-G10 §36 |
| The G10 renovation workflow proceeds from proposal through approval, closure, execution, inspection, compliance validation and return to Operational state. | Infrastructure Management | This proposal scenario stops before those later execution/completion stages. | L2-G10 §36 |
| Renovation preserves historical Infrastructure identity wherever physical identity remains unchanged. | Infrastructure Management | The proposal cannot assume creation of a new physical identity merely because attributes may change. | L2-G10 §36 |
| Section J owns procurement and physical-resource truth; Campus Operations owns the operational reason for resource consumption. | Enterprise Resource Management / Infrastructure Management | A renovation resource need does not transfer the renovation outcome to Section J. | L2-J00 §§1–5, 7.3 |
| Finance owns budgets and planned financial allocation; procurement is out of F06 scope. | Finance | Financial planning remains separate from physical renovation governance. | L2-F06 §§1–5 |
| B03 owns InstitutionalCampus identity/governance while physical Infrastructure is delegated to G10. | Institution & Structure / Infrastructure Management | Campus context does not alter physical ownership. | L2-B03 §§1–4 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Physical Infrastructure aggregate | Existing source-defined state | Proposal and impact are governed; this scenario does not itself perform the renovation lifecycle transition. | Unchanged by this proposal outcome unless a separately authorized lifecycle action occurs. | Infrastructure Management | L2-G10 §§11, 36 |
| InstitutionalCampus | Existing B03 state | Referenced for institutional context only. | Unchanged. | Institution & Structure | L2-B03 |
| Budget / Budget Allocation | Existing F06 lifecycle position if applicable | Referenced for planning context only. | Unchanged by this scenario. | Finance | L2-F06 |
| Procurement/resource aggregates | Existing Section J lifecycle position if applicable | No procurement transaction is executed here. | Unchanged by this scenario. | Enterprise Resource Management | L2-J00 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

L2-G10 defines `RenovationStarted` and `RenovationCompleted` as lifecycle events, but this frozen scenario ends at proposal governance before renovation execution starts. No canonical event specifically named for “Capital Renovation Proposal” or “Infrastructure Change Governance” is defined in the reviewed sources, so this Draft does not invent one.

## 14.2 Business Contracts and Cross-Domain Hand-offs

The reviewed sources establish ownership boundaries among G10, Section J, Finance and B03, but they do not define an exact mandatory contract for this frozen renovation-proposal scenario. Procurement/resource needs, budget context and InstitutionalCampus context therefore remain references to their canonical domains rather than invented synchronous contracts.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Record/govern a proposed physical Infrastructure renovation | Infrastructure Administrator or Infrastructure Manager within authorized scope | Must satisfy the applicable authorization and G10 business rules. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

G10 §36 includes an Approval stage but the reviewed sources do not define one universal renovation approver or a scenario-specific separation-of-duty rule. No approval role or maker-checker requirement is invented.

## 15.3 Business Audit and Evidence Obligations

The proposal-governance history must preserve the authoritative physical Infrastructure reference, the governed proposed change, identified physical impact and applicable authorization/decision evidence. Finance, Section J and B03 retain evidence for their own separately governed actions.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The canonical physical Infrastructure has been identified.
- The proposed major modification has been recorded under G10 governance.
- The physical impact has been identified using authoritative Infrastructure facts.
- Relevant J/F/B context is referenced without ownership transfer or invented execution.
- No renovation execution has been implied by the proposal-governance outcome.

## 16.2 Postconditions

- The proposed Infrastructure change and its physical impact are recorded under Infrastructure Management ownership.
- Physical Infrastructure identity and history remain preserved.
- Any procurement, budget or institutional-campus follow-up remains separately governed by its canonical owner.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Frozen Stage 3 includes J, F and B as participating domains, but L2-G10 §36 does not define exact renovation-proposal contracts or hand-offs to those domains. | Participation can be preserved at ownership-boundary level, but mandatory cross-domain contracts cannot be asserted without invention. | Upstream architecture may define explicit renovation proposal collaboration contracts if required. |
| L2-G10 defines lifecycle events for renovation start/completion but no proposal-governance event. | The Draft cannot publish a proposal event by assumption. | Define a canonical proposal/impact event upstream only if cross-domain event consumption is required. |

**Architecture assessment:** `PASS WITH ISSUES`

The frozen outcome is fully expressible without changing ownership; the missing cross-domain contract/event detail is non-blocking for a Draft.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
