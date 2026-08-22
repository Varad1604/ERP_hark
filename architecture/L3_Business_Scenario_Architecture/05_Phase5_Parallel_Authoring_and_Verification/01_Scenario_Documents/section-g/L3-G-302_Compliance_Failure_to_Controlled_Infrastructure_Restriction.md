# Compliance Failure to Controlled Infrastructure Restriction

# 1. Scenario Metadata

**Scenario ID:** L3-G-302  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-21  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Authorized consumers that rely on the usable status and compliance-attention condition of physical Infrastructure

# 2. Scenario Purpose

Govern the Infrastructure response when compliance becomes invalid or expires so that the applicable physical Infrastructure follows the configured G10 restriction or review path, while preserving the locked rule that expiry alone does not automatically disable Infrastructure unless stricter configured enforcement applies.

# 3. Business Outcome

The applicable infrastructure is placed under the configured restriction or review path

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when Infrastructure compliance becomes invalid or expires.

## 4.2 End Boundary

The scenario ends when the applicable Infrastructure has been placed under the configured compliance-attention, review or stricter restriction path required by authoritative G10 policy, with the relevant compliance fact preserved and any source-defined completed event published.

## 4.3 In Scope

- Identification of the affected physical Infrastructure and Compliance Record.
- Authorization of any governed business action required to change Infrastructure availability or restriction status.
- Application of the source-defined default treatment for expired compliance.
- Application of stricter configured enforcement where authoritative institutional policy requires it.
- Preservation of the Compliance Record and historical compliance information.
- Publication of `ComplianceExpired` when the compliance lifecycle reaches `Expired`.
- Publication of other source-defined G10 events only where the corresponding completed source-defined business fact occurs.
- Preservation of immutable audit evidence from canonical completed business events.

## 4.4 Out of Scope

- Creation of a new `Invalid` compliance lifecycle state not defined by G10.
- Automatic Infrastructure shutdown solely because compliance expires where no stricter configured enforcement policy requires restriction.
- Invention of an Infrastructure availability event not defined by locked G10 architecture.
- Regulatory interpretation or creation of new compliance policy.
- Compliance renewal or remediation execution beyond establishing the configured review/restriction outcome.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-G-301 — Compliance Status Change to Infrastructure Compliance Update | Records the current compliance position and preserves historical compliance information. |
| Related | L3-G-289 — Infrastructure Availability Definition to Availability Profile Activation | Provides the authoritative availability governance used when a configured restriction affects Infrastructure availability. |
| Related | L3-G-303 — Infrastructure Inspection Requirement to Closed Risk-Reviewed Outcome | Inspection may be part of a later review/remediation path while remaining a distinct institutional outcome. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns the physical Infrastructure, Compliance Record, compliance-expiry fact, Infrastructure availability/restriction outcome and application of configured Infrastructure compliance policy. |
| Platform Foundation / Section A | Owns authorization decisions, event-governance capability and immutable audit evidence; it does not determine Infrastructure compliance or availability policy. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Compliance Officer | Participates in review of the compliance failure/expiry within authorized institutional responsibility. | Receives or acts on source-defined compliance attention and review responsibilities where authorized. |
| Infrastructure Administrator | Administers the affected Infrastructure and applies the authorized G10 policy outcome. | May perform a governed Infrastructure restriction/change where canonical authorization and policy permit it. |
| Campus Administrator | Participates for affected Infrastructure within the governed campus scope. | May administer campus-scoped restriction or review actions where authorized. |
| Auditor | Reviews preserved compliance and restriction evidence. | Read-only participation; cannot alter compliance or Infrastructure state. |

# 7. Preconditions and Entry Criteria

- The affected physical Infrastructure exists under Infrastructure Management ownership.
- A Compliance Record exists against the Infrastructure.
- The compliance condition has become invalid under the applicable business context or has reached the source-defined `Expired` lifecycle position.
- Applicable configured compliance-enforcement and Infrastructure availability policy is available.
- Any business action that changes Infrastructure availability or restriction state is authorized before execution.

# 8. Trigger

Infrastructure compliance becomes invalid or expires

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the affected authoritative Infrastructure and Compliance Record. | Infrastructure Management | The exact compliance-failure scope is established. |
| 2 | Determine whether the condition is source-defined expiry or another business condition treated as invalid by the applicable configured compliance policy, without inventing an `Invalid` lifecycle state. | Infrastructure Management | The applicable G10 policy path is identified. |
| 3 | For expiry, record/preserve the `Expired` compliance position and apply the G10 default expiry treatment: compliance attention is required and responsible personnel/Compliance Officers are notified according to the source-defined business rule. | Infrastructure Management | Expiry is recognized without automatically disabling the Infrastructure. |
| 4 | Evaluate the configured institutional enforcement policy to determine whether review alone is sufficient or a stricter Infrastructure restriction is required. | Infrastructure Management | The authoritative review/restriction outcome is determined. |
| 5 | Where stricter configured enforcement requires a restriction, confirm authorization for the applicable Infrastructure change and apply only a source-defined/configured Infrastructure availability restriction. | Infrastructure Management using the canonical authorization decision owned by Section A | The Infrastructure enters the configured restriction path without inventing a new availability state. |
| 6 | Where the configured policy requires review rather than immediate restriction, retain the Infrastructure under the source-defined compliance-attention/review path pending the separately governed resolution. | Infrastructure Management | The frozen outcome is satisfied through the configured review path without unsupported automatic shutdown. |
| 7 | Publish `ComplianceExpired` when expiry is the completed compliance fact; publish any other G10 event only if its exact source-defined business fact has actually occurred. | Infrastructure Management | Canonical event semantics remain exact and ownership stays with G10. |
| 8 | Preserve immutable audit evidence from canonical completed business events. | Platform Foundation — Audit Infrastructure | Historical accountability for the compliance failure/restriction outcome is preserved. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Expired compliance with default G10 policy | Compliance reaches `Expired` and no stricter configured enforcement requires immediate restriction. | Mark the Infrastructure as `Compliance Attention Required`, notify responsible personnel and Compliance Officers, publish `ComplianceExpired`, and follow the configured review path. | Infrastructure is not automatically disabled; the compliance issue remains visible and governed. | Infrastructure Management |
| Expired compliance with stricter configured enforcement | Institutional policy requires an Infrastructure restriction when the relevant compliance expires. | After authorization, apply the configured source-defined Infrastructure availability restriction and continue the applicable review/remediation path. | Infrastructure use is restricted according to configured policy rather than a locally invented rule. | Infrastructure Management |
| Non-expiry invalid compliance condition | The frozen trigger is satisfied by an invalid compliance condition that is not the canonical `Expired` lifecycle position. | Apply only the configured G10 review/restriction policy that can be established from authoritative source facts; do not create an `Invalid` Compliance state or invent an event. | The Infrastructure enters the configured review/restriction path while the lifecycle gap remains explicit in Section 17. | Infrastructure Management |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Configured enforcement policy cannot be determined | A valid Infrastructure restriction decision cannot be asserted. | Retain the compliance-attention/review condition that is source-supported and resolve the authoritative policy before applying any stricter restriction. | Infrastructure Management |
| Authorization denies a requested Infrastructure restriction/change | The proposed Infrastructure availability change does not occur. | Continue only with source-supported review/compliance-attention obligations until a later authorized business action is available. | Platform Foundation / Authorization; Infrastructure Management retains the Infrastructure fact. |
| Compliance record or Infrastructure reference is inconsistent | The restriction/review outcome cannot be safely applied to an authoritative target. | Resolve the canonical G10 business reference before a new governed action is attempted. | Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Infrastructure Management owns Compliance Status, Compliance Records and Infrastructure Availability for physical Infrastructure. | Infrastructure Management | Compliance failure consequences remain G10-owned and cannot be re-owned by Section A or consuming domains. | L2-G10 §§1, 3, 8 |
| The canonical Compliance lifecycle is Draft → Active → Expiring → Expired → Renewed. | Infrastructure Management | The scenario may use `Expired` but cannot create an `Invalid` lifecycle state. | L2-G10 Canonical Lifecycle States |
| Expired compliance shall not automatically disable Infrastructure. | Infrastructure Management | Expiry alone cannot be translated into unconditional shutdown. | L2-G10 Expired Compliance |
| On expired compliance, G10 requires notification of responsible personnel and Compliance Officers, publication of the Compliance Expired event, and marking Infrastructure as `Compliance Attention Required`; institutions may configure stricter enforcement. | Infrastructure Management | The default path is review/attention; restriction occurs only when configured policy requires it. | L2-G10 Expired Compliance |
| Source-defined Infrastructure availability states include Available, Reserved, Under Maintenance, Under Renovation, Temporarily Closed, Emergency Closed and Operationally Restricted. | Infrastructure Management | If stricter enforcement changes availability, the scenario uses only an authoritative/configured source-defined availability state and does not invent a new one. | L2-G10 Availability States |
| Every business operation shall be authorized before execution. | Platform Foundation — Authorization | Any governed Infrastructure restriction/change requiring execution must be authorized. | L2-A04 AP-01 |
| Events represent completed business facts and the originating business domain retains ownership. | Infrastructure Management / Platform Foundation — Event Infrastructure | `ComplianceExpired` is published only for actual expiry; no event is invented for the undefined invalid-condition branch. | L2-A06 EIP-01, EIP-02; L2-G10 Compliance Events |
| Audit evidence is generated from canonical business events and remains immutable. | Platform Foundation — Audit | Completed source-defined business facts remain evidentially traceable. | L2-A07 §§7.2–7.4 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Compliance Record | `Expiring` for expiry path; another existing source-defined position for a non-expiry condition | Expiry progresses to `Expired`; no `Invalid` state is introduced for the other frozen trigger branch. | `Expired` for canonical expiry; otherwise unchanged until a source-defined compliance update is established. | Infrastructure Management | L2-G10 Canonical Lifecycle States |
| Infrastructure Availability / compliance-attention condition | Existing source-defined availability state | Default expiry marks `Compliance Attention Required` without automatic disablement; stricter configured policy may require a source-defined restriction. | Review/attention condition under default policy, or an applicable configured source-defined availability restriction when policy requires it. | Infrastructure Management | L2-G10 Expired Compliance; Availability States |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `ComplianceExpired` | Infrastructure Management | Authorized consumers of Infrastructure compliance facts; Audit Infrastructure for evidence generation. | Publishes the completed fact that the Compliance Record has reached `Expired`. | L2-G10 Compliance Events; Expired Compliance |

No canonical G10 event is defined specifically for the frozen trigger branch “compliance becomes invalid” or for a generic compliance-driven Infrastructure restriction. This Draft does not invent one.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Section A supplies canonical authorization, event-governance and audit capabilities. Infrastructure Management retains the decision and business-fact ownership for compliance attention, review and any configured Infrastructure availability restriction.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Apply a configured compliance-driven Infrastructure restriction | Infrastructure Administrator or Campus Administrator within applicable scope | The action requires the applicable configured G10 enforcement policy and a permitting canonical authorization decision. | Infrastructure Management |
| Participate in compliance review | Compliance Officer within applicable institutional responsibility | Participation is governed by configured compliance policy; this L3 scenario does not create universal approval authority. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

G10 states that Infrastructure Compliance supports workflow-driven approvals and configurable compliance policies, but it does not define one universal separation-of-duty or override pattern for compliance failure. Any stricter approval/exception rule must come from the applicable authoritative policy.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve compliance expiry/failure context | The authoritative Compliance Record and current G10 compliance information. | Infrastructure Management |
| Preserve restriction/review outcome | The authoritative Infrastructure availability/review fact established under configured G10 policy. | Infrastructure Management |
| Preserve immutable evidence | Audit evidence generated from canonical completed business events such as `ComplianceExpired`. | Platform Foundation — Audit Infrastructure |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The affected authoritative Infrastructure and Compliance Record have been identified.
- The compliance condition has been evaluated without inventing an `Invalid` lifecycle state.
- For expiry, the source-defined expired-compliance obligations have been applied.
- The configured enforcement policy has determined review versus stricter restriction.
- Any Infrastructure restriction/change has been authorized before execution.
- `ComplianceExpired` has been published when expiry occurred.
- No unsupported event or availability state has been invented.

## 16.2 Postconditions

- The Infrastructure is under the applicable configured restriction or review path.
- Expiry by itself has not been treated as automatic disablement unless stricter configured policy requires restriction.
- Infrastructure Management remains the owner of the Compliance Record, compliance consequence and Infrastructure availability fact.
- Historical compliance and audit evidence remain traceable.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| The frozen trigger includes “compliance becomes invalid,” but the canonical G10 Compliance lifecycle defines only Draft → Active → Expiring → Expired → Renewed and does not define an `Invalid` state. | This Draft can preserve the frozen invalid-condition trigger as a business condition, but cannot assign it a new canonical Compliance lifecycle position. | Upstream G10 architecture may clarify how a non-expiry invalid compliance condition is represented within the canonical Compliance lifecycle. |
| G10 defines `ComplianceExpired` for expiry and the default expired-compliance review treatment, but does not define a canonical event specifically for a non-expiry invalid-compliance condition or a generic compliance-driven Infrastructure restriction. | The exact event for that branch cannot be asserted without invention; business ownership remains clear. | Upstream G10 event governance may define an exact event if that branch requires canonical publication. |

**Architecture assessment:** `PASS WITH ISSUES`

The frozen outcome remains usable because it expressly permits either a configured restriction **or review path**, and the locked G10 default expiry rule provides a source-supported review path. The unresolved invalid-condition lifecycle/event semantics are recorded rather than invented.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions beyond the recorded upstream lifecycle/event gaps.
