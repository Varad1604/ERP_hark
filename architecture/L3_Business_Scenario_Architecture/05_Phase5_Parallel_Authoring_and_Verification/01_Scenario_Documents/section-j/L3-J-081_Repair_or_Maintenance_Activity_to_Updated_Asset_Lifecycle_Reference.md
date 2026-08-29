# Repair or Maintenance Activity to Updated Asset Lifecycle Reference

## 1. Scenario Metadata

**Scenario ID:** L3-J-081  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Asset Lifecycle Management  
**Participating Domains:** G, J  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-22  
**Depends On:** L2-J00; L2-J08; applicable L2-J05, L2-J06, L2-J07, L2-J09, L2-J10 and L2-F12 references; frozen Stage 3 Section J canonical row  
**Referenced By:** None yet

## 2. Scenario Purpose

This scenario defines the complete institutional business outcome represented by **Repair or Maintenance Activity to Updated Asset Lifecycle Reference** while preserving the locked Section J ownership, lifecycle, authority, evidence, event, and dependency boundaries.

## 3. Business Outcome

J08 preserves the maintenance reference and any supported Asset condition or lifecycle consequence.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An authorised repair or maintenance activity produces an operational outcome relevant to an Asset.

### 4.2 End Boundary

The scenario ends when the stated Business Outcome is recorded with required authority and evidence, or when a material unresolved issue prevents that outcome from being claimed.

### 4.3 In Scope

- The business decisions, evidence, and lifecycle impact necessary for this outcome.
- Material cross-domain facts and handoffs defined by the governing sources.

### 4.4 Out of Scope

- Receipt, inventory, stock movement, audit conclusions, disposal execution, and financial Asset accounting remain with their canonical owners.
- APIs, data design, software components, UI behaviour, technical access control, deployment, transport, and technical recovery.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Asset Lifecycle Management | Owns the scenario aggregate, governed lifecycle impact, evidence, and final outcome. |
| G, J | Supplies or consumes only its canonically owned facts, decisions, or consequences; participation does not transfer ownership. |

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Asset Registrar | Participates in the governed business outcome within the authority defined by Asset Lifecycle Management. | Performs only the source-supported responsibility; applicable independent approval and segregation of duties remain enforced. |
| Asset Approver | Participates in the governed business outcome within the authority defined by Asset Lifecycle Management. | Performs only the source-supported responsibility; applicable independent approval and segregation of duties remain enforced. |
| Asset Custodian / Holder | Participates in the governed business outcome within the authority defined by Asset Lifecycle Management. | Performs only the source-supported responsibility; applicable independent approval and segregation of duties remain enforced. |
| Transfer Requester / Approver | Participates in the governed business outcome within the authority defined by Asset Lifecycle Management. | Performs only the source-supported responsibility; applicable independent approval and segregation of duties remain enforced. |
| Independent Verifier | Participates in the governed business outcome within the authority defined by Asset Lifecycle Management. | Performs only the source-supported responsibility; applicable independent approval and segregation of duties remain enforced. |
| Retirement Recommender / Approver | Participates in the governed business outcome within the authority defined by Asset Lifecycle Management. | Performs only the source-supported responsibility; applicable independent approval and segregation of duties remain enforced. |

## 7. Preconditions and Entry Criteria

- Tenant and institution context is established.
- The trigger and all mandatory upstream references are authoritative and current for the business decision.
- The affected aggregate is in a source-supported lifecycle condition.
- Applicable actor authority and separation-of-duty requirements can be satisfied.

## 8. Trigger

An authorised repair or maintenance activity produces an operational outcome relevant to an Asset.

## 9. Main Business Flow

1. The applicable Campus Operations domain supplies the authoritative repair or maintenance outcome and retains ownership of maintenance execution.
2. J08 validates the Asset, external activity reference, warranty/AMC context, tenant, institution, and current lifecycle state.
3. J08 appends the maintenance/service reference to protected Asset history without requiring a dedicated event.
4. If the operational Asset condition changes, J08 records the governed condition consequence and publishes AssetConditionChanged.
5. Finance accounting and Campus Operations maintenance state remain outside J08 ownership.

## 10. Alternate Business Flows

- Where a source-supported alternative outcome applies, the responsible actor records that outcome with its reason and evidence.
- A path owned by another domain ends with a governed handoff; the receiving domain determines its own outcome.

## 11. Failure, Interruption and Business Recovery

- If required authority, evidence, lifecycle eligibility, or a mandatory external fact is absent, the scenario does not claim successful completion.
- Confirmed business evidence remains intact while the matter is returned, referred, corrected, or held in its supported business state.
- Recovery follows only an upstream-defined business path; no technical retry or rollback behaviour is specified.

## 12. Business Rules and Decision Constraints

- `Asset, AssetAssignment, AssetTransfer, or AssetLifecycleEvent as applicable` remains within Asset Lifecycle Management ownership.
- Receipt, inventory, stock movement, audit conclusions, disposal execution, and financial Asset accounting remain with their canonical owners.
- Conditional policy statements retain their upstream qualifiers.
- Tenant and institution context, historical integrity, and canonical references are preserved.
- No local threshold, state, actor, event, publisher, or contract is introduced.

## 13. State and Lifecycle Impact

The scenario applies only the lifecycle stages and transitions explicitly supported by its governing L2 sources. Historical evidence is preserved; amendment, cancellation, correction, reversal, closure, or archival is used only where authoritative semantics exist.

## 14. Business Events and Cross-Domain Collaboration

| Business Event / Fact | Canonical Publisher | Scenario Use |
|---|---|---|
| `AssetConditionChanged` | Asset Lifecycle Management (J08) | Published or consumed only when the corresponding business fact occurs. |

Synchronous validations, acknowledgements, approval outcomes, and governed handoffs remain distinct from business events. Consumers do not acquire publishing or aggregate ownership.

## 15. Business Authorization, Approval and Audit Obligations

- Business authority is evaluated against tenant, institution, lifecycle, scope, value or quantity, classification, and delegated authority where the source makes them applicable.
- Source-supported maker-checker and segregation-of-duty obligations are enforced.
- The owning domain produces business evidence; enterprise authorization, workflow routing, and audit persistence remain with their canonical platform owners.

## 16. Completion Criteria and Postconditions

- The Business Outcome is institutionally verifiable.
- The owned aggregate and evidence reflect the supported final business interpretation.
- Required cross-domain references are preserved without transferring ownership.
- No material open issue remains.

## 17. Open Issues, Assumptions and Deferred Questions

**Validation Classification:** PASS

**Open Issue:** No open architecture issue was identified after applying the cited locked sources.

No assumption in this Draft overrides locked architecture. Any authoritative source change requires affected-content revision and revalidation.
