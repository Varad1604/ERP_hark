# Finance Configuration Change to Governed Active Configuration

# 1. Scenario Metadata

**Scenario ID:** L3-F-084  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Finance Foundation  
**Participating Domains:** A, F  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-F00 — Finance Domain Model & Foundation  
**Referenced By:** Finance workflows that consume governed Finance configuration

# 2. Scenario Purpose

Govern a change to Finance-owned configuration through the platform configuration hierarchy while preserving Finance ownership of business meaning, controlled version progression, future-effective use, historical references, and auditability.

# 3. Business Outcome

A validated, approved and versioned configuration becomes effective for future processing and can be rolled back

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized change affects Finance calculation, workflow, accounting, settlement, reporting, or retention behavior.

## 4.2 End Boundary

The intended end boundary is reached when the changed Finance configuration has been validated, approved, versioned and activated for future processing and the frozen rollback capability is available. Locked L2 architecture supports the governed activation path but does not define the required rollback lifecycle or authority; exact completion is therefore blocked.

## 4.3 In Scope

- Finance-specific configuration ownership and scope.
- Platform configuration hierarchy and inherited context.
- Versioned Finance configuration change.
- Validation, impact assessment, approval, publication and activation.
- Preservation of predecessor configuration and historical transaction references.
- Auditability of the configuration change.
- Identification of the unresolved rollback and event requirements.

## 4.4 Out of Scope

- Changing Platform-, Tenant-, Institution-, or Organisation-owned configuration from within Finance.
- Changing historical financial transactions after a configuration version changes.
- Redefining Financial Rule lifecycle semantics governed separately in L3-F-085.
- Inventing a rollback state, transition, event, publisher, or approval authority.
- Technical configuration storage, deployment, caching, release, or recovery mechanics.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-F-085 — Financial Rule Change to Governed Active Policy | Governs Finance Financial Rule lifecycle rather than general Finance configuration. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Finance Foundation / F | Owns Finance-specific configuration meaning, categories, versions, validation requirements, lifecycle and effective use. |
| Platform Foundation / A | Owns the Platform Configuration Framework, configuration hierarchy, inheritance and resolution contract; it does not own Finance-specific configuration meaning. |

Participation by Section A does not transfer Finance configuration ownership.

# 6. Business Actors, Roles and Responsibilities

L2-A01 records that its Actors section is absent. L2-F00 references configurable roles and approval structures but does not establish a canonical business actor or institutional role specifically responsible for proposing, approving, activating, or reversing Finance configuration. No placeholder requester, reviewer, approver, or configuration-owner role is invented here.

# 7. Preconditions and Entry Criteria

- The relevant Tenant and institutional context are valid.
- The target configuration item has one canonical owner.
- The proposed item is Finance-specific configuration owned by Finance Foundation.
- Existing parent and predecessor configuration context can be resolved where applicable.
- The proposed change can be evaluated against the Finance configuration validation requirements.
- Historical financial transactions retain their original effective-configuration references.

# 8. Trigger

An authorized change affects Finance calculation, workflow, accounting, settlement, reporting, or retention behavior

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish the Finance configuration item, owning domain, affected behavior and applicable scope. | Finance Foundation / F; Platform Foundation / A | The proposed change is within a valid ownership and configuration boundary. |
| 2 | Create a distinct Finance configuration version without altering the predecessor version. | Finance Foundation / F | A governed candidate exists in the source-defined `Draft` lifecycle position. |
| 3 | Validate institution scope, ownership, version consistency, rule dependencies, approval status, calendar compatibility, accounting compatibility, regulatory compatibility and platform compatibility. | Finance Foundation / F | Invalid configuration is prevented from becoming active. |
| 4 | Assess the impact of the proposed configuration change before approval. | Finance Foundation / F | The candidate is eligible to progress only after governed impact assessment. |
| 5 | Progress the candidate through the source-defined approval stage. | Finance Foundation / F | The configuration is approved or remains outside operational use. |
| 6 | Publish and activate the approved version under the Finance configuration lifecycle and Platform Configuration Framework. | Finance Foundation / F; Platform Foundation / A | The new version becomes the effective configuration for future eligible processing. |
| 7 | Supersede the predecessor where replacement applies while retaining its historical references. | Finance Foundation / F | Prior financial activity remains tied to the configuration version originally used. |
| 8 | Preserve the configuration-change and lifecycle evidence required by Finance governance. | Finance Foundation / F | The governed activation path is auditable. |

The frozen rollback requirement cannot be completed without inventing an upstream-undefined business transition or authority.

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| First configuration version | No predecessor exists | Govern the first version through the same Draft, validation, approval, publication and activation path | A first Active Finance configuration becomes available | Finance Foundation / F |
| Existing predecessor | An Active version is replaced | Keep the predecessor effective until successor activation, then move it to `Superseded` | Historical references remain intact | Finance Foundation / F |

No canonical alternate flow for restoring a prior configuration version is defined upstream.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Ownership or scope validation fails | The candidate cannot safely enter operational use | Keep the candidate outside `Active` until the business configuration issue is corrected | Finance Foundation / F; Platform Foundation / A |
| Finance compatibility validation fails | The change could produce invalid financial behavior | Do not activate the candidate | Finance Foundation / F |
| Required approval is not obtained | The candidate cannot become effective | Retain the current Active version | Finance Foundation / F |
| A governed rollback is required after activation | The frozen recovery outcome cannot be represented by the locked lifecycle | Do not infer reactivation of a Superseded version or another reversal path; upstream architecture must define it | Finance Foundation / F; Platform Foundation / A |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Business Domain Configuration is owned by the respective Level 2 domain | Platform Foundation / A | Finance-specific configuration remains owned by Finance | L2-A01 §6.5 |
| Every configuration item has one owner and ownership shall not overlap | Platform Foundation / A; Finance Foundation / F | A configuration change cannot transfer ownership | L2-A01 §6.5; L2-F00 §82, §89 |
| Finance configuration changes are version controlled | Finance Foundation / F | A change creates a governed version and does not rewrite historical transactions | L2-F00 §80.2, §86, §89 |
| The Finance configuration lifecycle is `Draft → Validation → Approval → Published → Active → Superseded → Archived` | Finance Foundation / F | Only source-defined lifecycle positions may be used | L2-F00 §84 |
| Only Active Finance configuration participates in financial execution | Finance Foundation / F | Candidate and superseded versions do not govern new processing | L2-F00 §84 |
| Activation requires validation and approval | Finance Foundation / F | Governance cannot be bypassed | L2-F00 §§85, 87, 89 |
| Historical transactions retain the configuration version active during execution | Finance Foundation / F | Future changes do not alter historical financial decisions | L2-F00 §§80.2, 86, 89 |
| Configuration changes are auditable | Finance Foundation / F | Change and lifecycle evidence must be preserved | L2-F00 §88–§89 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Finance configuration candidate | Draft | Progresses through governed validation and approval | `Draft → Validation → Approval → Published → Active` | Finance Foundation / F | L2-F00 §84–§87 |
| Predecessor Finance configuration | Active | Replaced only after successor activation | `Active → Superseded` | Finance Foundation / F | L2-F00 §84 |
| Historical financial transaction | Existing historical reference | No mutation; retains original configuration reference | Unchanged | Applicable Finance aggregate / F | L2-F00 §80.2, §86 |

No rollback state or transition is defined in L2-F00 or L2-A01.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

F00 states that financial activities/workflows are represented through domain events, but its published Finance event catalog defines no canonical Finance configuration-change event. A01 likewise establishes configuration governance without defining a Finance configuration lifecycle event or publisher. No event is invented.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Platform Foundation / A | Finance Foundation / F | Platform configuration hierarchy and resolved configuration context | Supplies the common configuration framework while Finance retains domain-configuration ownership | L2-A01 §6 |
| Finance Foundation / F | Finance workflows / F | Effective Finance configuration | Governs future Finance behavior using the Active version | L2-F00 §§83–89 |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The frozen trigger requires an authorized change and the F00 lifecycle requires approval, but the applicable locked sources do not identify the canonical business actor or institutional role with authority to approve or reverse Finance configuration. This authority gap is not resolved locally.

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty or override rule for Finance configuration change is defined by the authoritative L2 sources. A rollback exception path is also undefined.

## 15.3 Business Audit and Evidence Obligations

F00 requires every configuration change to preserve immutable governance history, including predecessor and successor version context, change reason, approval reference, activation context and scope. Historical financial transactions preserve their original configuration references.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The supported activation path requires a valid Finance-owned candidate, successful validation, approval, publication and activation with preserved predecessor/history. The scenario cannot satisfy the complete frozen outcome until rollback semantics, required event publication, and business authority are defined upstream.

## 16.2 Postconditions

A successfully activated version governs future eligible Finance processing, and predecessor/historical references remain preserved. No canonical rollback capability is established by this Draft.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| The frozen outcome requires that the active configuration “can be rolled back,” but neither L2-F00 nor L2-A01 defines rollback eligibility, lifecycle transition, resulting state, or business authority | The final frozen outcome cannot be completed without inventing business recovery semantics | Define the authoritative Finance configuration rollback model and ownership boundary |
| F00 requires financial activities/workflows to be event-represented, but no canonical Finance configuration lifecycle event or publisher exists | Required event semantics and sole publisher cannot be established | Define the canonical configuration-change event(s) and publisher(s), or explicitly exempt this lifecycle from the Finance event requirement |
| The lifecycle requires approval and the trigger requires authorization, but no canonical business actor/authority is assigned for Finance configuration approval or rollback | Business authority is materially unresolved | Define the canonical approval/reversal authority or authoritative policy mapping |

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
