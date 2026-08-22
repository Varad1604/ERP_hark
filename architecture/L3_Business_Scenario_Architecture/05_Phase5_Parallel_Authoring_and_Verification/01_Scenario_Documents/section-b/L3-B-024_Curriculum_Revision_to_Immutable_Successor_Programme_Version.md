# Curriculum Revision to Immutable Successor Programme Version

**Scenario ID:** L3-B-024  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Curriculum Framework  
**Participating Domains:** B, D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Approved
**Architecture Lock:** Locked  
**Version:** 1.0  
**Owner:** Institution & Structure — Curriculum Framework  
**Assigned Author:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`; `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Business_Scenario_Template_v2.0.md`; `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B04_Curriculum_Framework_Workflows.md`  
**Referenced By:** Known downstream consumers identified in `DEPENDENCY_MAP.md`; exact scenario references only where canonically registered  

---

# 2. Scenario Purpose

This scenario exists to publish a successor Programme Version without rewriting the immutable predecessor. It expresses one institutional business outcome at L3 and remains independent of APIs, user interfaces, persistence, messaging technology, and implementation design.

# 3. Business Outcome

The canonical Programme Version / Curriculum reaches **Published immutable successor** through an authorized, validated, evidence-backed decision owned by Curriculum Framework. The result is business-verifiable, historically traceable, and available to material consumers without transferring ownership.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario starts when an approved need for material curriculum revision, the canonical subject and scope are identifiable, and all mandatory entry facts already exist.

## 4.2 End Boundary

The scenario ends when the Curriculum Framework-owned decision is effective, Programme Version / Curriculum has reached **Published immutable successor**, required business evidence is retained, and any exact canonical event or governed hand-off has been produced. It does not wait for every consumer to finish its own downstream processing.

## 4.3 In Scope

- Canonical Programme Version / Curriculum identity, scope, and applicable effective context.
- Business validation, impact evaluation, review, decision, and approval required for this outcome.
- The material lifecycle or version effect from **Published predecessor** to **Published immutable successor**.
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
| Preceded by / Related | L3-B-023 | Adjacent canonical Section B outcome; relationship applies only where its postconditions satisfy this scenario's entry criteria. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Curriculum Framework | Owns Programme Version / Curriculum, its governing rules and lifecycle/version effect, and the final curriculum revision to immutable successor programme version outcome. |
| Section B | Provides the canonical Institution & Structure context applicable to this scenario. |
| D | Contributes only its authoritative scenario-specific facts or evidence; participation does not transfer ownership. |

# 6. Business Actors, Roles and Responsibilities

No scenario-specific canonical actor name can be established for every required responsibility with sufficient authority. Responsibilities are expressed by owning domain in the flow; any material actor gap is recorded in Section 17.

# 7. Preconditions and Entry Criteria

- The canonical Programme Version / Curriculum identity and institutional scope are valid or are validly established by this scenario.
- The owning Institution and required upstream structural context exist in an eligible authoritative state.
- The subject is in a permitted entry lifecycle position: **Published predecessor**.
- Required authoritative rules, versions, relationships, and effective-date context are resolvable.
- Material dependency evidence is available from its owning domain; missing evidence is not assumed.
- Required business authority is established or its absence is explicitly recorded as an Open Issue.

# 8. Trigger

An approved need for material curriculum revision.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish the Curriculum Revision Request, subject identity, scope, proposed effective context, and accountable business intent. | Curriculum | A reviewable governed proposal exists. |
| 2 | Verify entry state, authoritative relationships, required upstream facts, and absence of conflicting effective decisions. | Curriculum Framework | Entry eligibility is confirmed or progression is blocked. |
| 3 | Evaluate material Programme Version / Curriculum rules, lifecycle constraints, dependency evidence, historical impact, and downstream consequences. | B, D | A traceable validation and impact result exists. |
| 4 | Obtain the authoritative review, decision, or approval required by the governing architecture. | Curriculum Framework | An attributable business decision is recorded. |
| 5 | Apply the approved business outcome at its permitted effective point without rewriting predecessor history. | Curriculum Framework | Programme Version / Curriculum reaches Published immutable successor. |
| 6 | Record required evidence and publish or hand off only canonical business facts to material consumers. | Curriculum Framework | The outcome is auditable and available for governed downstream use. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Correctable proposal | Authoritative rules permit correction without changing the final scenario outcome | Return the proposal for correction and re-evaluate it against the same governed baseline. | The proposal may resume toward the defined outcome; no authoritative state is changed prematurely. | Curriculum Framework |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Entry state or identity is invalid | The scenario cannot validly begin. | Reject or return the request according to authoritative governance; preserve the proposal and decision evidence. | Curriculum Framework |
| Mandatory rule, approval, or dependency evidence is missing | Normal progression stops and the current authoritative state remains unchanged. | Obtain authoritative evidence or correct and resubmit where a supported path exists. | Curriculum Framework and evidence-owning participant |
| Approved proposal conflicts with a newer authoritative baseline before effectiveness | Applying it could produce an invalid or historically inconsistent result. | Require revalidation, replacement, cancellation, or renewed approval; do not silently apply stale approval. | Curriculum Framework |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Canonical ownership | Curriculum Framework | Curriculum Framework owns the final outcome; participation and consumption do not transfer ownership. | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B04_Curriculum_Framework_Workflows.md`; locked L1 architecture |
| Authoritative identity and scope | Curriculum Framework | The proposal must reference the canonical Programme Version / Curriculum identity and applicable institutional scope. | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B04_Curriculum_Framework_Workflows.md` |
| Lifecycle integrity | Curriculum Framework | Only the authoritative entry and resulting lifecycle positions may be used; unsupported transitions are prohibited. | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B04_Curriculum_Framework_Workflows.md` |
| Version and historical integrity | Curriculum Framework | Published/effective predecessors remain historically traceable and are not destructively rewritten. | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B04_Curriculum_Framework_Workflows.md` |

## 12.1 Scenario-Specific Applied Decisions

- Material revision creates a successor Programme Version.
- The published predecessor is never overwritten.
- Existing cohorts do not migrate without explicit applicability rules.
- Rejected successor proposals leave the predecessor authoritative.

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Programme Version / Curriculum | Published predecessor | The authorized scenario decision applies only after validation and at the permitted effective point. | Published immutable successor | Curriculum Framework | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B04_Curriculum_Framework_Workflows.md` |

No state synonym, hidden intermediate state, or unsupported transition is introduced by this scenario.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

No exact scenario-specific public event is established with sufficient confidence. The required event-contract decision is recorded in Section 17.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Curriculum Framework | Material downstream domains | Canonical Programme Version / Curriculum identity, lifecycle/version status, effective context, and completed business outcome | Permit governed consumption without transferring ownership | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B04_Curriculum_Framework_Workflows.md` |
| Participating domains (B, D) | Curriculum Framework | Authoritative validation, readiness, physical, academic, workforce, or other scenario-specific evidence | Support the Curriculum Framework-owned final decision while retaining source-domain ownership | `DEPENDENCY_MAP.md`; applicable L2 sources |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate curriculum revision to immutable successor programme version | Canonical actor is used only where explicitly supported upstream; otherwise UNCERTAIN | Must be authorized for the exact subject and institutional scope. | Curriculum Framework |
| Make or approve the final decision | Canonical decision role is used only where explicitly supported upstream; otherwise UNCERTAIN | Decision must evaluate the authoritative proposal, baseline, evidence, and effective context. | Curriculum Framework |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, override, or escalation rule is asserted unless explicitly established by the authoritative source. Terminal states and canonical rules cannot be bypassed by an assumed administrator override.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Trace the governed request and subject | Canonical subject/version, scope, baseline, requested outcome, and effective context | Curriculum Framework |
| Trace validation and decision | Material rule/dependency results, authoritative decision, and required reason where supported | Curriculum Framework and contributing evidence owner |
| Preserve lifecycle and historical interpretation | Prior and resulting lifecycle/version positions and predecessor/successor lineage where applicable | Curriculum Framework |

Section A/A07 retains ownership of authorization and audit infrastructure; Curriculum Framework retains ownership of the business decision and evidence semantics.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The canonical Programme Version / Curriculum and governed scope are unambiguous.
- Every mandatory business rule, dependency condition, and required approval has a satisfactory authoritative result.
- The Curriculum Framework-owned decision is attributable and effective.
- The subject has reached **Published immutable successor** without rewriting predecessor history.
- Required evidence and exact canonical events or hand-offs are available.

## 16.2 Postconditions

- Programme Version / Curriculum is authoritative in **Published immutable successor** for the applicable effective scope and time.
- Historical identities, versions, relationships, decisions, and references remain resolvable where applicable.
- Material consumers may use the outcome for their own governed responsibilities; they do not become owners.
- Later operations remain subject to their own preconditions, authorization, and operation-specific readiness evaluation.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Exact public event contract for Curriculum Revision to Immutable Successor Programme Version | Consumers cannot implement a canonical asynchronous reaction without an approved name and publisher contract. | Confirm against the enterprise event registry and architecture/L2_Domain_Workflow_Architecture/section-b/L2-B04_Curriculum_Framework_Workflows.md; do not invent an event. |

## 17.2 Assumptions

No assumptions. The Draft uses only the accepted 500-decision baseline and authoritative repository sources; uncertainty is recorded as an Open Issue rather than silently resolved.

## 17.3 Deferred Questions

No scenario-specific deferred questions. API, UI, persistence, messaging, deployment, technical RBAC, and implementation recovery belong to later architecture levels by governance and are not repeated here.
