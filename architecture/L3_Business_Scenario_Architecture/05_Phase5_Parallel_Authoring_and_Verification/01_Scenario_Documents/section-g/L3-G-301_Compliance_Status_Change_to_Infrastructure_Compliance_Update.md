# Compliance Status Change to Infrastructure Compliance Update

# 1. Scenario Metadata

**Scenario ID:** L3-G-301  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-21  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Authorized consumers requiring the current and historical Infrastructure compliance position

# 2. Scenario Purpose

Govern a change in the compliance condition of physical Infrastructure so that the current G10-owned Compliance Record reflects the applicable source-defined compliance state or version while prior compliance history remains immutable and traceable.

# 3. Business Outcome

The current compliance state is recorded while historical compliance information remains traceable

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a compliance condition changes.

## 4.2 End Boundary

The scenario ends when the applicable Compliance Record has been updated in accordance with the source-defined compliance lifecycle and version-governance rules, the current compliance position is recorded, the appropriate canonical compliance event has been published, and prior compliance history remains preserved.

## 4.3 In Scope

- Identification of the affected Infrastructure and current Compliance Record.
- Authorization of the compliance update business operation.
- Validation of the changed compliance condition against applicable Infrastructure and compliance policy rules.
- Progression of the Compliance Record only through source-defined lifecycle positions where a lifecycle change applies.
- Version/effective-date preservation where compliance information changes require historical traceability.
- Publication of the exact G10 compliance event appropriate to the completed change.
- Preservation of immutable audit evidence from the completed business fact.

## 4.4 Out of Scope

- Initial establishment of a Compliance Record.
- Creation of new compliance lifecycle states or transitions.
- Automatic Infrastructure shutdown merely because a compliance record expires; that is governed separately by L3-G-302 and applicable configured policy.
- Regulatory interpretation outside the configured G10 compliance governance.
- Infrastructure inspection execution.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-G-300 — Infrastructure Compliance Requirement to Compliance Record Establishment | Establishes the Compliance Record whose current status may later change. |
| Related | L3-G-302 — Compliance Failure to Controlled Infrastructure Restriction | Governs the configured restriction or review outcome when compliance becomes invalid or expires. |
| Related | L3-G-303 — Infrastructure Inspection Requirement to Closed Risk-Reviewed Outcome | Inspection findings may constitute authoritative evidence relevant to a later compliance change, while remaining a distinct business outcome. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns the physical Infrastructure, Compliance Record, compliance lifecycle, compliance versions, current compliance state and completed compliance-change business fact. |
| Platform Foundation / Section A | Owns authorization decisions, event-governance capability and immutable audit evidence generated from the completed G10 business fact; it does not own or reinterpret compliance. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Administers the Infrastructure Compliance Record within authorized scope. | May perform a governed compliance update when authorization and G10 validation permit it. |
| Compliance Officer | Participates in review or maintenance of Infrastructure compliance information according to applicable institutional responsibility. | May participate in the compliance update within authorized scope; no universal approval authority is invented. |
| Campus Administrator | Participates in compliance updates for Infrastructure within the governed campus scope. | May act where authorized for that campus scope. |
| Auditor | Reviews preserved compliance history and immutable evidence. | Read-only participation; cannot alter current or historical compliance facts. |

# 7. Preconditions and Entry Criteria

- The applicable physical Infrastructure exists under Infrastructure Management ownership.
- A current Compliance Record exists against the Infrastructure.
- A compliance condition has changed and the change can be evaluated under the applicable G10 compliance governance.
- The compliance update business operation is authorized before execution.

# 8. Trigger

A compliance condition changes

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the authoritative Infrastructure and current Compliance Record affected by the changed compliance condition. | Infrastructure Management | The exact compliance context is established. |
| 2 | Confirm that the compliance update operation is authorized for the applicable Infrastructure scope. | Infrastructure Management using the canonical authorization decision owned by Section A | The update is permitted to proceed. |
| 3 | Validate the changed compliance condition against the applicable Infrastructure facts and configured compliance policy. | Infrastructure Management | The proposed compliance update is evaluated against authoritative rules. |
| 4 | Determine the applicable source-defined compliance lifecycle position and/or compliance-record version impact without inventing a new state. | Infrastructure Management | The permitted business change is identified. |
| 5 | Record the current compliance state or updated compliance information while preserving effective dates, expiry dates, superseded versions and historical versions where applicable. | Infrastructure Management | The current compliance position is authoritative and prior history remains traceable. |
| 6 | Publish the exact canonical compliance event applicable to the completed change: `ComplianceUpdated`, `ComplianceExpired` or `ComplianceRenewed` as appropriate to the source-defined outcome. | Infrastructure Management | The completed G10 compliance fact is published without transferring ownership. |
| 7 | Preserve immutable audit evidence from the canonical completed business event. | Platform Foundation — Audit Infrastructure | Historical evidence of the compliance change is preserved. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Compliance reaches expiry | The permitted lifecycle progression results in `Expired`. | Record the `Expired` compliance position, preserve history, and publish `ComplianceExpired`. Any Infrastructure restriction or review consequence is handled under the configured policy and L3-G-302. | The compliance lifecycle is updated without automatically disabling Infrastructure in this scenario. | Infrastructure Management |
| Compliance is renewed | An expired compliance obligation is renewed in accordance with the source-defined lifecycle. | Record the renewed compliance position, retain superseded/historical versions as applicable, and publish `ComplianceRenewed`. | Current compliance reflects renewal while history remains immutable. | Infrastructure Management |
| Compliance information changes without expiry or renewal | A permitted compliance modification changes current information but does not produce the specific expiry or renewal events. | Update the Compliance Record under version-management rules and publish `ComplianceUpdated`. | Current compliance information is updated and prior history remains traceable. | Infrastructure Management |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Authorization denies the update | The current Compliance Record remains unchanged. | A later update may proceed only after a valid authorization decision. | Platform Foundation / Authorization; Infrastructure Management retains the compliance fact. |
| Proposed change is not a permitted source-defined lifecycle progression | The requested compliance-state change cannot be recorded as proposed. | Resolve the authoritative business condition and use only a permitted G10 lifecycle position or transition. | Infrastructure Management |
| Required compliance or Infrastructure facts are inconsistent | A trustworthy current compliance outcome cannot be asserted. | Resolve the authoritative G10 information before attempting the compliance update again. | Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Infrastructure Management owns Compliance Records and Compliance Status for physical Infrastructure. | Infrastructure Management | Current and historical compliance remain G10 facts. | L2-G10 §§1, 3, 8 |
| The canonical Compliance lifecycle is Draft → Active → Expiring → Expired → Renewed. | Infrastructure Management | The scenario cannot invent `Invalid`, `Suspended` or other compliance lifecycle states. | L2-G10 Canonical Lifecycle States |
| Compliance records support multiple versions, effective dates, expiry dates, superseded versions and historical versions; historical versions remain immutable. | Infrastructure Management | Updating the current compliance position must not overwrite prior compliance history. | L2-G10 Compliance Version Management |
| Infrastructure Compliance supports effective dating, expiry management, auditability and configurable compliance policies. | Infrastructure Management | The current state/version is governed by the applicable policy and time context. | L2-G10 Compliance Principles |
| Every business operation shall be authorized before execution. | Platform Foundation — Authorization | Unauthorized compliance updates do not proceed. | L2-A04 AP-01 |
| Events represent completed business facts and the originating domain retains ownership. | Infrastructure Management / Platform Foundation — Event Infrastructure | Compliance events describe completed G10 facts and do not transfer ownership to A06. | L2-A06 EIP-01, EIP-02 |
| Audit evidence is generated from canonical business events and is immutable. | Platform Foundation — Audit | Prior and current compliance changes remain evidentially traceable without duplicating the operational Compliance Record. | L2-A07 §§7.2–7.4 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Compliance Record | `Draft`, `Active`, `Expiring` or `Expired`, as applicable to the source-defined lifecycle | A valid compliance condition change updates the current lifecycle position and/or current version while retaining historical versions. | Only a permitted source-defined position: `Active`, `Expiring`, `Expired` or `Renewed`, according to the applicable transition and business condition. | Infrastructure Management | L2-G10 Canonical Lifecycle States; Compliance Version Management |
| Infrastructure | Existing source-defined Infrastructure lifecycle and availability position | The Compliance Record is updated; Infrastructure lifecycle/availability is not automatically changed solely by this scenario. | Unchanged unless a separately governed Infrastructure restriction/review outcome applies. | Infrastructure Management | L2-G10 Expired Compliance; Infrastructure lifecycle/availability rules |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `ComplianceUpdated` | Infrastructure Management | Authorized consumers of Infrastructure compliance facts; Audit Infrastructure for evidence generation. | Publishes a completed compliance modification that is not represented by a more specific expiry or renewal event. | L2-G10 Compliance Events; L2-A06 EIP-02 |
| `ComplianceExpired` | Infrastructure Management | Authorized consumers of Infrastructure compliance facts; Audit Infrastructure for evidence generation. | Publishes the completed fact that Compliance has reached `Expired`. | L2-G10 Compliance Events; Expired Compliance |
| `ComplianceRenewed` | Infrastructure Management | Authorized consumers of Infrastructure compliance facts; Audit Infrastructure for evidence generation. | Publishes the completed fact that Compliance has been renewed. | L2-G10 Compliance Events |

## 14.2 Business Contracts and Cross-Domain Hand-offs

Section A provides canonical authorization, event-governance and audit capabilities. No hand-off changes ownership of the Compliance Record, compliance state, compliance version or physical Infrastructure.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Update the current Infrastructure compliance position | Infrastructure Administrator, Compliance Officer or Campus Administrator within applicable authorized scope | The update must be authorized and comply with the applicable G10 lifecycle and compliance policy. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

G10 supports workflow-driven approvals and configurable compliance policies but does not define one universal approval, separation-of-duty or override rule for every compliance status change. Category-specific governance must therefore come from the applicable authoritative policy.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve current compliance | The current G10 Compliance Record state/version and the applicable completed compliance event. | Infrastructure Management |
| Preserve historical compliance | Superseded and historical Compliance Record versions with their source-defined date information. | Infrastructure Management |
| Preserve immutable business evidence | Audit evidence generated from the canonical completed compliance event. | Platform Foundation — Audit Infrastructure |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The affected Infrastructure and current Compliance Record have been identified.
- The compliance update has been authorized and validated.
- The current compliance state/version has been recorded using only source-defined lifecycle positions and version rules.
- Historical compliance information remains immutable and traceable.
- The appropriate canonical compliance event has been published.
- Immutable audit evidence is preserved through the canonical audit architecture.

## 16.2 Postconditions

- Infrastructure Management remains the owner of the current and historical compliance facts.
- Authorized consumers may rely on the current compliance position.
- Expiry-related Infrastructure restriction or review remains governed separately and does not occur automatically inside this scenario.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No blocking architecture issue was identified. The locked G10 compliance lifecycle, version-management rules and compliance event catalogue directly support the frozen scenario outcome.

**Architecture assessment:** `PASS`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
