# Position Definition Retirement and Archive

**Scenario ID:** L3-B-015  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Organisation  
**Participating Domains:** B, E  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Approved
**Architecture Lock:** Locked  
**Version:** 1.0  
**Owner:** Institution & Structure — Organisation  
**Assigned Author:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`; `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Business_Scenario_Template_v2.0.md`; `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B02_Organisation_Workflows.md`  
**Referenced By:** Known downstream consumers identified in `DEPENDENCY_MAP.md`; exact scenario references only where canonically registered  

---

# 2. Scenario Purpose

This scenario exists to end future assignment eligibility and preserve Position Definition history and E02 references. It expresses one institutional business outcome at L3 and remains independent of APIs, user interfaces, persistence, messaging technology, and implementation design.

# 3. Business Outcome

The canonical PositionDefinition reaches **Retired / Archived** through an authorized, validated, evidence-backed decision owned by Organisation. The result is business-verifiable, historically traceable, and available to material consumers without transferring ownership.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario starts when an authorized retirement or subsequent archival decision, the canonical subject and scope are identifiable, and all mandatory entry facts already exist.

## 4.2 End Boundary

The scenario ends when the Organisation-owned decision is effective, PositionDefinition has reached **Retired / Archived**, required business evidence is retained, and any exact canonical event or governed hand-off has been produced. It does not wait for every consumer to finish its own downstream processing.

## 4.3 In Scope

- Canonical PositionDefinition identity, scope, and applicable effective context.
- Business validation, impact evaluation, review, decision, and approval required for this outcome.
- The material lifecycle or version effect from **Active / Retired** to **Retired / Archived**.
- Scenario-specific evidence obligations and canonical cross-domain collaboration.
- Preservation of predecessor and historical references where applicable.

## 4.4 Out of Scope

- Technical RBAC, authentication, APIs, payloads, databases, queues, retries, UI, deployment, or software design.
- Re-ownership or destructive modification of facts belonging to participating or consuming domains.
- Downstream business outcomes represented by other canonical L3 scenarios.
- A global readiness flag; readiness remains operation-specific and dynamically evaluated where applicable.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by / Related | L3-B-014 | Adjacent canonical Section B outcome; relationship applies only where its postconditions satisfy this scenario's entry criteria. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Organisation | Owns PositionDefinition, its governing rules and lifecycle/version effect, and the final position definition retirement and archive outcome. |
| Section B | Provides the canonical Institution & Structure context applicable to this scenario. |
| E | Contributes only its authoritative scenario-specific facts or evidence; participation does not transfer ownership. |

# 6. Business Actors, Roles and Responsibilities

No scenario-specific canonical actor name can be established for every required responsibility with sufficient authority. Responsibilities are expressed by owning domain in the flow; any material actor gap is recorded in Section 17.

# 7. Preconditions and Entry Criteria

- The canonical PositionDefinition identity and institutional scope are valid or are validly established by this scenario.
- The owning Institution and required upstream structural context exist in an eligible authoritative state.
- The subject is in a permitted entry lifecycle position: **Active / Retired**.
- Required authoritative rules, versions, relationships, and effective-date context are resolvable.
- Material dependency evidence is available from its owning domain; missing evidence is not assumed.
- Required business authority is established or its absence is explicitly recorded as an Open Issue.

# 8. Trigger

An authorized retirement or subsequent archival decision.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish the Position Definition Archival Request, subject identity, scope, proposed effective context, and accountable business intent. | Organisation | A reviewable governed proposal exists. |
| 2 | Verify entry state, authoritative relationships, required upstream facts, and absence of conflicting effective decisions. | Organisation | Entry eligibility is confirmed or progression is blocked. |
| 3 | Evaluate material PositionDefinition rules, lifecycle constraints, dependency evidence, historical impact, and downstream consequences. | B, E | A traceable validation and impact result exists. |
| 4 | Obtain the authoritative review, decision, or approval required by the governing architecture. | Organisation | An attributable business decision is recorded. |
| 5 | Apply the approved business outcome at its permitted effective point without rewriting predecessor history. | Organisation | PositionDefinition reaches Retired / Archived. |
| 6 | Record required evidence and publish or hand off only canonical business facts to material consumers. | Organisation | The outcome is auditable and available for governed downstream use. |

# 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Entry state or identity is invalid | The scenario cannot validly begin. | Reject or return the request according to authoritative governance; preserve the proposal and decision evidence. | Organisation |
| Mandatory rule, approval, or dependency evidence is missing | Normal progression stops and the current authoritative state remains unchanged. | Obtain authoritative evidence or correct and resubmit where a supported path exists. | Organisation and evidence-owning participant |
| Approved proposal conflicts with a newer authoritative baseline before effectiveness | Applying it could produce an invalid or historically inconsistent result. | Require revalidation, replacement, cancellation, or renewed approval; do not silently apply stale approval. | Organisation |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Canonical ownership | Organisation | Organisation owns the final outcome; participation and consumption do not transfer ownership. | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B02_Organisation_Workflows.md`; locked L1 architecture |
| Authoritative identity and scope | Organisation | The proposal must reference the canonical PositionDefinition identity and applicable institutional scope. | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B02_Organisation_Workflows.md` |
| Lifecycle integrity | Organisation | Only the authoritative entry and resulting lifecycle positions may be used; unsupported transitions are prohibited. | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B02_Organisation_Workflows.md` |
| Historical integrity | Organisation | The outcome must preserve prior authoritative identity, decisions, and historical references. | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B02_Organisation_Workflows.md` |

## 12.1 Scenario-Specific Applied Decisions

- Retirement ends future assignment eligibility at a deterministic time.
- Existing and historical PositionAssignments remain valid references.
- Archived identifiers are never reused.
- No reactivation path is invented from Retired or Archived.

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| PositionDefinition | Active / Retired | The authorized scenario decision applies only after validation and at the permitted effective point. | Retired / Archived | Organisation | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B02_Organisation_Workflows.md` |

The resulting terminal or retirement position prevents unsupported future operational use. Historical identity and references remain available under authorized resolution.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

No exact scenario-specific public event is established with sufficient confidence. The required event-contract decision is recorded in Section 17.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Organisation | Material downstream domains | Canonical PositionDefinition identity, lifecycle/version status, effective context, and completed business outcome | Permit governed consumption without transferring ownership | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B02_Organisation_Workflows.md` |
| Participating domains (B, E) | Organisation | Authoritative validation, readiness, physical, academic, workforce, or other scenario-specific evidence | Support the Organisation-owned final decision while retaining source-domain ownership | `DEPENDENCY_MAP.md`; applicable L2 sources |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate position definition retirement and archive | Canonical actor is used only where explicitly supported upstream; otherwise UNCERTAIN | Must be authorized for the exact subject and institutional scope. | Organisation |
| Make or approve the final decision | Canonical decision role is used only where explicitly supported upstream; otherwise UNCERTAIN | Decision must evaluate the authoritative proposal, baseline, evidence, and effective context. | Organisation |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, override, or escalation rule is asserted unless explicitly established by the authoritative source. Terminal states and canonical rules cannot be bypassed by an assumed administrator override.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Trace the governed request and subject | Canonical subject/version, scope, baseline, requested outcome, and effective context | Organisation |
| Trace validation and decision | Material rule/dependency results, authoritative decision, and required reason where supported | Organisation and contributing evidence owner |
| Preserve lifecycle and historical interpretation | Prior and resulting lifecycle/version positions and predecessor/successor lineage where applicable | Organisation |

Section A/A07 retains ownership of authorization and audit infrastructure; Organisation retains ownership of the business decision and evidence semantics.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The canonical PositionDefinition and governed scope are unambiguous.
- Every mandatory business rule, dependency condition, and required approval has a satisfactory authoritative result.
- The Organisation-owned decision is attributable and effective.
- The subject has reached **Retired / Archived** without rewriting predecessor history.
- Required evidence and exact canonical events or hand-offs are available.

## 16.2 Postconditions

- PositionDefinition is authoritative in **Retired / Archived** for the applicable effective scope and time.
- Historical identities, versions, relationships, decisions, and references remain resolvable where applicable.
- Material consumers may use the outcome for their own governed responsibilities; they do not become owners.
- Later operations remain subject to their own preconditions, authorization, and operation-specific readiness evaluation.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Exact public event contract for Position Definition Retirement and Archive | Consumers cannot implement a canonical asynchronous reaction without an approved name and publisher contract. | Confirm against the enterprise event registry and architecture/L2_Domain_Workflow_Architecture/section-b/L2-B02_Organisation_Workflows.md; do not invent an event. |

## 17.2 Assumptions

No assumptions. The Draft uses only the accepted 500-decision baseline and authoritative repository sources; uncertainty is recorded as an Open Issue rather than silently resolved.

## 17.3 Deferred Questions

No scenario-specific deferred questions. API, UI, persistence, messaging, deployment, technical RBAC, and implementation recovery belong to later architecture levels by governance and are not repeated here.
