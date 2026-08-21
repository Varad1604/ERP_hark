# Institutional Campus Activation Decision

**Scenario ID:** L3-B-018  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Institutional Campus  
**Participating Domains:** B, G  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Approved
**Architecture Lock:** Locked  
**Version:** 1.0  
**Owner:** Institution & Structure — Institutional Campus  
**Assigned Author:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`; `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Business_Scenario_Template_v2.0.md`; `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B03_Campus_Workflows.md`  
**Referenced By:** Known downstream consumers identified in `DEPENDENCY_MAP.md`; exact scenario references only where canonically registered  

---

# 2. Scenario Purpose

This scenario exists to decide institutional Campus activation using B03 rules and required G10 evidence. It expresses one institutional business outcome at L3 and remains independent of APIs, user interfaces, persistence, messaging technology, and implementation design.

# 3. Business Outcome

The canonical InstitutionalCampus reaches **Operational** through an authorized, validated, evidence-backed decision owned by Institutional Campus. The result is business-verifiable, historically traceable, and available to material consumers without transferring ownership.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario starts when an eligible Institutional Campus is submitted for activation, the canonical subject and scope are identifiable, and all mandatory entry facts already exist.

## 4.2 End Boundary

The scenario ends when the Institutional Campus-owned decision is effective, InstitutionalCampus has reached **Operational**, required business evidence is retained, and any exact canonical event or governed hand-off has been produced. It does not wait for every consumer to finish its own downstream processing.

## 4.3 In Scope

- Canonical InstitutionalCampus identity, scope, and applicable effective context.
- Business validation, impact evaluation, review, decision, and approval required for this outcome.
- The material lifecycle or version effect from **Approved** to **Operational**.
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
| Preceded by / Related | L3-B-017 | Adjacent canonical Section B outcome; relationship applies only where its postconditions satisfy this scenario's entry criteria. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Institutional Campus | Owns InstitutionalCampus, its governing rules and lifecycle/version effect, and the final institutional campus activation decision outcome. |
| Section B | Provides the canonical Institution & Structure context applicable to this scenario. |
| G | Contributes only its authoritative scenario-specific facts or evidence; participation does not transfer ownership. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Domain Owner | Initiates or performs the B03 institutional Campus action where supported by L2-B03. | Initiates / performs |

# 7. Preconditions and Entry Criteria

- The canonical InstitutionalCampus identity and institutional scope are valid or are validly established by this scenario.
- The owning Institution and required upstream structural context exist in an eligible authoritative state.
- The subject is in a permitted entry lifecycle position: **Approved**.
- Required authoritative rules, versions, relationships, and effective-date context are resolvable.
- Material dependency evidence is available from its owning domain; missing evidence is not assumed.
- Required business authority is established or its absence is explicitly recorded as an Open Issue.

# 8. Trigger

An eligible Institutional Campus is submitted for activation.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish the Institutional Campus Activation Request, subject identity, scope, proposed effective context, and accountable business intent. | Campus | A reviewable governed proposal exists. |
| 2 | Verify entry state, authoritative relationships, required upstream facts, and absence of conflicting effective decisions. | Institutional Campus | Entry eligibility is confirmed or progression is blocked. |
| 3 | Evaluate material InstitutionalCampus rules, lifecycle constraints, dependency evidence, historical impact, and downstream consequences. | B, G | A traceable validation and impact result exists. |
| 4 | Obtain the authoritative review, decision, or approval required by the governing architecture. | Institutional Campus | An attributable business decision is recorded. |
| 5 | Apply the approved business outcome at its permitted effective point without rewriting predecessor history. | Institutional Campus | InstitutionalCampus reaches Operational. |
| 6 | Record required evidence and publish or hand off only canonical business facts to material consumers. | Institutional Campus | The outcome is auditable and available for governed downstream use. |

# 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Entry state or identity is invalid | The scenario cannot validly begin. | Reject or return the request according to authoritative governance; preserve the proposal and decision evidence. | Institutional Campus |
| Mandatory rule, approval, or dependency evidence is missing | Normal progression stops and the current authoritative state remains unchanged. | Obtain authoritative evidence or correct and resubmit where a supported path exists. | Institutional Campus and evidence-owning participant |
| Approved proposal conflicts with a newer authoritative baseline before effectiveness | Applying it could produce an invalid or historically inconsistent result. | Require revalidation, replacement, cancellation, or renewed approval; do not silently apply stale approval. | Institutional Campus |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Canonical ownership | Institutional Campus | Institutional Campus owns the final outcome; participation and consumption do not transfer ownership. | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B03_Campus_Workflows.md`; locked L1 architecture |
| Authoritative identity and scope | Institutional Campus | The proposal must reference the canonical InstitutionalCampus identity and applicable institutional scope. | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B03_Campus_Workflows.md` |
| Lifecycle integrity | Institutional Campus | Only the authoritative entry and resulting lifecycle positions may be used; unsupported transitions are prohibited. | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B03_Campus_Workflows.md` |
| Historical integrity | Institutional Campus | The outcome must preserve prior authoritative identity, decisions, and historical references. | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B03_Campus_Workflows.md` |

## 12.1 Scenario-Specific Applied Decisions

- B03 owns activation; G10 contributes physical evidence without taking ownership.
- Mandatory metadata and lifecycle eligibility must pass.
- Unsatisfactory or conflicting G10 evidence blocks activation.
- CampusActivated does not imply physical capacity or academic scheduling.

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| InstitutionalCampus | Approved | The authorized scenario decision applies only after validation and at the permitted effective point. | Operational | Institutional Campus | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B03_Campus_Workflows.md` |

No state synonym, hidden intermediate state, or unsupported transition is introduced by this scenario.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `CampusActivated` | Institutional Campus | Material downstream consumers | Communicate the completed institutional campus activation decision fact | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B03_Campus_Workflows.md` |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Institutional Campus | Material downstream domains | Canonical InstitutionalCampus identity, lifecycle/version status, effective context, and completed business outcome | Permit governed consumption without transferring ownership | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B03_Campus_Workflows.md` |
| Participating domains (B, G) | Institutional Campus | Authoritative validation, readiness, physical, academic, workforce, or other scenario-specific evidence | Support the Institutional Campus-owned final decision while retaining source-domain ownership | `DEPENDENCY_MAP.md`; applicable L2 sources |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate institutional campus activation decision | Canonical actor is used only where explicitly supported upstream; otherwise UNCERTAIN | Must be authorized for the exact subject and institutional scope. | Institutional Campus |
| Make or approve the final decision | Canonical decision role is used only where explicitly supported upstream; otherwise UNCERTAIN | Decision must evaluate the authoritative proposal, baseline, evidence, and effective context. | Institutional Campus |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, override, or escalation rule is asserted unless explicitly established by the authoritative source. Terminal states and canonical rules cannot be bypassed by an assumed administrator override.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Trace the governed request and subject | Canonical subject/version, scope, baseline, requested outcome, and effective context | Institutional Campus |
| Trace validation and decision | Material rule/dependency results, authoritative decision, and required reason where supported | Institutional Campus and contributing evidence owner |
| Preserve lifecycle and historical interpretation | Prior and resulting lifecycle/version positions and predecessor/successor lineage where applicable | Institutional Campus |

Section A/A07 retains ownership of authorization and audit infrastructure; Institutional Campus retains ownership of the business decision and evidence semantics.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The canonical InstitutionalCampus and governed scope are unambiguous.
- Every mandatory business rule, dependency condition, and required approval has a satisfactory authoritative result.
- The Institutional Campus-owned decision is attributable and effective.
- The subject has reached **Operational** without rewriting predecessor history.
- Required evidence and exact canonical events or hand-offs are available.

## 16.2 Postconditions

- InstitutionalCampus is authoritative in **Operational** for the applicable effective scope and time.
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
