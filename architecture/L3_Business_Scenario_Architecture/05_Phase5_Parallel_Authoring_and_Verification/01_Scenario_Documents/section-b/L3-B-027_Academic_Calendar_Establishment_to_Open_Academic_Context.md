# Academic Calendar Establishment to Open Academic Context

**Scenario ID:** L3-B-027  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Calendar  
**Participating Domains:** A, B  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Approved
**Architecture Lock:** Locked  
**Version:** 1.0  
**Owner:** Institution & Structure — Academic Calendar  
**Assigned Author:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`; `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Business_Scenario_Template_v2.0.md`; `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B05_Academic_Calendar_Workflows.md`  
**Referenced By:** Known downstream consumers identified in `DEPENDENCY_MAP.md`; exact scenario references only where canonically registered  

---

# 2. Scenario Purpose

This scenario exists to establish an active published Calendar Version as an open academic-time context without starting downstream processes. It expresses one institutional business outcome at L3 and remains independent of APIs, user interfaces, persistence, messaging technology, and implementation design.

# 3. Business Outcome

The canonical Academic Calendar / Calendar Version reaches **Active** (with **Published** as a required predecessor state) through an authorized, validated, evidence-backed decision owned by Academic Calendar. The result is business-verifiable, historically traceable, and available to material consumers without transferring ownership.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario starts when an authorized Calendar establishment proposal, the canonical subject and scope are identifiable, and all mandatory entry facts already exist.

## 4.2 End Boundary

The scenario ends when the Academic Calendar-owned decision is effective, Academic Calendar / Calendar Version has reached **Active**, required business evidence is retained, and any exact canonical event or governed hand-off has been produced. It does not wait for every consumer to finish its own downstream processing.

## 4.3 In Scope

- Canonical Academic Calendar / Calendar Version identity, scope, and applicable effective context.
- Business validation, impact evaluation, review, decision, and approval required for this outcome.
- The material lifecycle or version effect traversing **Draft**, **Validated / Approved**, **Published**, and finally to **Active**.
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
| Preceded by / Related | L3-B-026 | Adjacent canonical Section B outcome; relationship applies only where its postconditions satisfy this scenario's entry criteria. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Academic Calendar | Owns Academic Calendar / Calendar Version, its governing rules and lifecycle/version effect, and the final academic calendar establishment to open academic context outcome. |
| Section B | Provides the canonical Institution & Structure context applicable to this scenario. |
| A | Contributes only its authoritative scenario-specific facts or evidence; participation does not transfer ownership. |

# 6. Business Actors, Roles and Responsibilities

No scenario-specific canonical actor name can be established for every required responsibility with sufficient authority. Responsibilities are expressed by owning domain in the flow; any material actor gap is recorded in Section 17.

# 7. Preconditions and Entry Criteria

- The canonical Academic Calendar / Calendar Version identity and institutional scope are valid or are validly established by this scenario.
- The owning Institution and required upstream structural context exist in an eligible authoritative state.
- The subject is in a permitted entry lifecycle position: **Draft / Approved / Published**.
- Required authoritative rules, versions, relationships, and effective-date context are resolvable.
- Material dependency evidence is available from its owning domain; missing evidence is not assumed.
- Required business authority is established or its absence is explicitly recorded as an Open Issue.

# 8. Trigger

An authorized Calendar establishment proposal.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish the canonical Create Calendar Request, subject identity, scope, proposed effective context, and accountable business intent. | Academic Calendar | A reviewable governed proposal exists. |
| 2 | Verify entry state, authoritative relationships, required upstream facts, and absence of conflicting effective decisions. | Academic Calendar | Entry eligibility is confirmed or progression is blocked. |
| 3 | Evaluate material Academic Calendar / Calendar Version rules, lifecycle constraints, dependency evidence, historical impact, and downstream consequences. | A, B | A traceable validation and impact result exists. |
| 4 | Obtain the authoritative review, decision, or approval required by the governing architecture. | Academic Calendar | An attributable business decision is recorded. |
| 5 | Apply the approved business outcome at its permitted effective point without rewriting predecessor history. | Academic Calendar | Academic Calendar / Calendar Version reaches Active. |
| 6 | Record required evidence and publish or hand off only canonical business facts to material consumers. | Academic Calendar | The outcome is auditable and available for governed downstream use. |

# 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Entry state or identity is invalid | The scenario cannot validly begin. | Reject or return the request according to authoritative governance; preserve the proposal and decision evidence. | Academic Calendar |
| Mandatory rule, approval, or dependency evidence is missing | Normal progression stops and the current authoritative state remains unchanged. | Obtain authoritative evidence or correct and resubmit where a supported path exists. | Academic Calendar and evidence-owning participant |
| Approved proposal conflicts with a newer authoritative baseline before effectiveness | Applying it could produce an invalid or historically inconsistent result. | Require revalidation, replacement, cancellation, or renewed approval; do not silently apply stale approval. | Academic Calendar |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Canonical ownership | Academic Calendar | Academic Calendar owns the final outcome; participation and consumption do not transfer ownership. | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B05_Academic_Calendar_Workflows.md`; locked L1 architecture |
| Authoritative identity and scope | Academic Calendar | The proposal must reference the canonical Academic Calendar / Calendar Version identity and applicable institutional scope. | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B05_Academic_Calendar_Workflows.md` |
| Lifecycle integrity | Academic Calendar | Only the authoritative entry and resulting lifecycle positions may be used; unsupported transitions are prohibited. | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B05_Academic_Calendar_Workflows.md` |
| Version and historical integrity | Academic Calendar | Published/effective predecessors remain historically traceable and are not destructively rewritten. | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B05_Academic_Calendar_Workflows.md` |

## 12.1 Scenario-Specific Applied Decisions

- Calendar Code is unique within the tenant and Calendar name is mandatory.
- The initial time hierarchy, working days, holidays and windows must resolve coherently.
- Publication/activation does not automatically assign every scope.
- Open academic context does not start downstream business processes.

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Academic Calendar / Calendar Version | Draft / Validated / Approved / Published | The authorized scenario decision applies only after validation and at the permitted effective point. | Active | Academic Calendar | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B05_Academic_Calendar_Workflows.md` |

No state synonym, hidden intermediate state, or unsupported transition is introduced by this scenario.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `CalendarVersionPublished` | Academic Calendar | Material downstream consumers | Communicate the completed academic calendar establishment to open academic context fact | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B05_Academic_Calendar_Workflows.md` |
| `CalendarVersionActivated` | Academic Calendar | Material downstream consumers | Communicate the completed academic calendar establishment to open academic context fact | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B05_Academic_Calendar_Workflows.md` |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Academic Calendar | Material downstream domains | Canonical Academic Calendar / Calendar Version identity, lifecycle/version status, effective context, and completed business outcome | Permit governed consumption without transferring ownership | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B05_Academic_Calendar_Workflows.md` |
| Participating domains (A, B) | Academic Calendar | Authoritative validation, readiness, physical, academic, workforce, or other scenario-specific evidence | Support the Academic Calendar-owned final decision while retaining source-domain ownership | `DEPENDENCY_MAP.md`; applicable L2 sources |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate academic calendar establishment to open academic context | Canonical actor is used only where explicitly supported upstream; otherwise UNCERTAIN | Must be authorized for the exact subject and institutional scope. | Academic Calendar |
| Make or approve the final decision | Canonical decision role is used only where explicitly supported upstream; otherwise UNCERTAIN | Decision must evaluate the authoritative proposal, baseline, evidence, and effective context. | Academic Calendar |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, override, or escalation rule is asserted unless explicitly established by the authoritative source. Terminal states and canonical rules cannot be bypassed by an assumed administrator override.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Trace the governed request and subject | Canonical subject/version, scope, baseline, requested outcome, and effective context | Academic Calendar |
| Trace validation and decision | Material rule/dependency results, authoritative decision, and required reason where supported | Academic Calendar and contributing evidence owner |
| Preserve lifecycle and historical interpretation | Prior and resulting lifecycle/version positions and predecessor/successor lineage where applicable | Academic Calendar |

Section A/A07 retains ownership of authorization and audit infrastructure; Academic Calendar retains ownership of the business decision and evidence semantics.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The canonical Academic Calendar / Calendar Version and governed scope are unambiguous.
- Every mandatory business rule, dependency condition, and required approval has a satisfactory authoritative result.
- The Academic Calendar-owned decision is attributable and effective.
- The subject has reached **Active** (with Published as a required predecessor) without rewriting predecessor history.
- Required evidence and exact canonical events or hand-offs are available.

## 16.2 Postconditions

- Academic Calendar / Calendar Version is authoritative in **Active** for the applicable effective scope and time.
- Historical identities, versions, relationships, decisions, and references remain resolvable where applicable.
- Material consumers may use the outcome for their own governed responsibilities; they do not become owners.
- Later operations remain subject to their own preconditions, authorization, and operation-specific readiness evaluation.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open issues.

## 17.2 Assumptions

No assumptions. The Draft uses only the accepted 500-decision baseline and authoritative repository sources; uncertainty is recorded as an Open Issue rather than silently resolved.

## 17.3 Deferred Questions

No scenario-specific deferred questions. API, UI, persistence, messaging, deployment, technical RBAC, and implementation recovery belong to later architecture levels by governance and are not repeated here.
