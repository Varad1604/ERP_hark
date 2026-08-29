# Institution Establishment to Final Activation Decision

# 1. Scenario Metadata

**Scenario ID:** L3-B-001  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Institution  
**Participating Domains:** A, B  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Institution & Structure — Institution  
**Assigned Author:** Pallav  
**Last Updated:** 2026-08-24  
**Depends On:** `architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`; `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Business_Scenario_Template_v2.0.md`; `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B01_Institution_Workflow.md`; `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B01_Institution_Activation_Authority_Amendment_2026-08-24.md`; `governance/adr/ADR-B-INST-ACT-001.md`  
**Referenced By:** Known downstream consumers identified in `DEPENDENCY_MAP.md`; exact scenario references only where canonically registered  

---

# 2. Scenario Purpose

This scenario exists to establish a canonical Institution identity, validate its configuration, and reach an auditable final activation decision. It expresses one institutional business outcome at L3 and remains independent of APIs, user interfaces, persistence, messaging technology, and implementation design.

# 3. Business Outcome

The canonical Institution reaches **Active** through an authorized, validated, evidence-backed decision owned by Institution. The result is business-verifiable, historically traceable, and available to material consumers without transferring ownership.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario starts when an authorized Institution establishment and activation request, the canonical subject and scope are identifiable, and all mandatory entry facts already exist.

## 4.2 End Boundary

The scenario ends when the Institution-owned decision is effective, Institution has reached **Active**, required business evidence is retained, and any exact canonical event or governed hand-off has been produced. It does not wait for every consumer to finish its own downstream processing.

## 4.3 In Scope

- Canonical Institution identity, scope, and applicable effective context.
- Business validation, impact evaluation, review, decision, and approval required for this outcome.
- The material lifecycle or version effect from **Configured / Pending Approval** to **Active**.
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
| Related | None | No earlier Section B scenario is required as a universal predecessor. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Institution | Owns Institution, its governing rules and lifecycle/version effect, and the final institution establishment to final activation decision outcome. |
| Section B | Provides the canonical Institution & Structure context applicable to this scenario. |
| A | Contributes only its authoritative scenario-specific facts or evidence; participation does not transfer ownership. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in This Scenario | Authoritative Source |
|---|---|---|
| Institution Administrator | Establishes and configures Institution information and submits the Institution activation request for governed decision. | L2-B01 amendment §1; ADR-B-INST-ACT-001 |
| Institution Activation Authority | Makes the final governed Institution activation decision after the applicable L2-B01 validation and approval prerequisites are evaluated; may approve, reject, or return for correction through existing lifecycle paths. | L2-B01 amendment §1, §4–§6; ADR-B-INST-ACT-001 |
| Institution Domain | Owns the Institution aggregate, lifecycle, business decision semantics, and canonical Institution lifecycle events. | L2-B01; L2-B01 amendment |
| Platform / Authorization capabilities | Provide authorization and audit capabilities without acquiring ownership of the Institution activation business decision. | L2-A04 / Section A boundary preserved by ADR-B-INST-ACT-001 |

`Institution Activation Authority` is an abstract business authority and not a universal Principal, Registrar, Board, Trustee, Tenant Administrator, or other fixed institutional title. Institution-specific mapping is governed upstream; technical role enforcement remains outside L3.

# 7. Preconditions and Entry Criteria

- The canonical Institution identity and institutional scope are valid or are validly established by this scenario.
- The owning Institution and required upstream structural context exist in an eligible authoritative state.
- The subject is in a permitted entry lifecycle position: **Configured / Pending Approval**.
- Required authoritative rules, versions, relationships, and effective-date context are resolvable.
- Material dependency evidence is available from its owning domain; missing evidence is not assumed.
- The Institution Administrator and Institution Activation Authority responsibilities are governed by the approved L2-B01 authority amendment.

# 8. Trigger

An authorized Institution establishment and activation request.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish the Institution Establishment Request, subject identity, scope, proposed effective context, and accountable business intent. | Institution Administrator | A reviewable governed proposal exists. |
| 2 | Verify entry state, authoritative relationships, required upstream facts, and absence of conflicting effective decisions. | Institution | Entry eligibility is confirmed or progression is blocked. |
| 3 | Evaluate material Institution rules, lifecycle constraints, dependency evidence, historical impact, and downstream consequences. | A, B | A traceable validation and impact result exists. |
| 4 | Submit the validated activation request for final governed decision. | Institution Administrator | A decision-ready activation request exists. |
| 5 | Approve, reject, or return the request through the existing governed lifecycle according to the authoritative evidence and constraints. | Institution Activation Authority | An attributable business decision is recorded. |
| 6 | Apply an approved business outcome at its permitted effective point without rewriting predecessor history. | Institution | Institution reaches Active. |
| 7 | Record required evidence and publish or hand off only canonical business facts to material consumers. | Institution | The outcome is auditable and available for governed downstream use. |

# 10. Alternate Business Flows

- A correctable activation request may be returned through the existing L2-B01 `Returned` path for correction and governed resubmission.
- A request that fails the final activation decision may reach the existing `Rejected` outcome and cannot transition directly to `Active`.

No additional alternate lifecycle state or authority is introduced by this scenario.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Entry state or identity is invalid | The scenario cannot validly begin. | Reject or return the request according to authoritative governance; preserve the proposal and decision evidence. | Institution |
| Mandatory rule, approval, or dependency evidence is missing | Normal progression stops and the current authoritative state remains unchanged. | Obtain authoritative evidence or correct and resubmit where a supported path exists. | Institution and evidence-owning participant |
| Approved proposal conflicts with a newer authoritative baseline before effectiveness | Applying it could produce an invalid or historically inconsistent result. | Require revalidation, replacement, cancellation, or renewed approval; do not silently apply stale approval. | Institution |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Canonical ownership | Institution | Institution owns the final outcome; participation and consumption do not transfer ownership. | L2-B01; locked L1 architecture |
| Authoritative identity and scope | Institution | The proposal must reference the canonical Institution identity and applicable institutional scope. | L2-B01 |
| Lifecycle integrity | Institution | Only the authoritative entry and resulting lifecycle positions may be used; unsupported transitions are prohibited. | L2-B01 |
| Activation request authority | Institution | Institution Administrator submits the governed activation request after required configuration. | L2-B01 authority amendment; ADR-B-INST-ACT-001 |
| Final activation authority | Institution | Institution Activation Authority makes the final governed activation decision; local job-title mapping is not defined at L3. | L2-B01 authority amendment; ADR-B-INST-ACT-001 |
| Authorization ownership boundary | A / Institution | Authorization enforcement remains with L2-A04 while Institution retains the activation business decision. | ADR-B-INST-ACT-001 |
| Historical integrity | Institution | The outcome must preserve prior authoritative identity, decisions, and historical references. | L2-B01 |

## 12.1 Scenario-Specific Applied Decisions

- Tenant/platform context already exists; this scenario does not create tenancy.
- Correctable incompleteness may be returned; a failed mandatory activation decision is rejected.
- Rejected cannot transition directly to Active; governed reset/resubmission is required.
- Activation does not equal global readiness or Organisation configuration.
- No universal Principal, Registrar, Board, Trustee, or other local title is made canonical by this scenario.
- No universal separation-of-duties rule is invented beyond the approved upstream amendment.

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Institution | Configured / Pending Approval | The authorized scenario decision applies only after validation and at the permitted effective point. | Active | Institution | L2-B01; L2-B01 authority amendment |

No state synonym, hidden intermediate state, or unsupported transition is introduced by this scenario.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `InstitutionActivated` | Institution | Material downstream consumers | Communicate the completed institution establishment to final activation decision fact | L2-B01; L2-B01 authority amendment |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| L2-A01 Platform / Tenant Foundation | Institution | Active canonical Tenant context and Tenant association | Provide the upstream context in which the Institution is established. | L2-B01 amendment §3 |
| Institution Administrator | Institution Activation Authority | Configured Institution activation request with required validation evidence | Enable the final Institution-owned activation decision without transferring aggregate ownership. | L2-B01 amendment §3–§4 |
| Institution | Material downstream domains | Canonical Institution identity, authoritative lifecycle facts, and completed business outcome | Permit governed consumption without transferring ownership. | L2-B01; L2-B01 amendment §3 |

No API, transport, payload, schema, or implementation contract is introduced.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Establish/configure the Institution and submit activation request | Institution Administrator | Operates within the applicable Institution governance and may submit the configured Institution for governed activation decision. | Institution |
| Make the final activation decision | Institution Activation Authority | Evaluates the authoritative proposal and required validation evidence; may approve, reject, or return through existing L2-B01 paths. | Institution |

## 15.2 Separation of Duties, Overrides and Exceptions

The approved L2-B01 authority amendment introduces no universal separation-of-duties rule between Institution Administrator and Institution Activation Authority. Institution-specific governance may impose stronger separation through authoritative institutional policy and authorization architecture. Terminal states and canonical rules cannot be bypassed by an assumed administrator override.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Trace the governed request and subject | Canonical subject/version, scope, baseline, requested outcome, and effective context | Institution |
| Trace submission and final decision | Institution Administrator submission, material rule/dependency results, Institution Activation Authority decision, and required reason where supported | Institution and contributing evidence owner |
| Preserve lifecycle and historical interpretation | Prior and resulting lifecycle/version positions and predecessor/successor lineage where applicable | Institution |

Section A/A07 retains ownership of authorization and audit infrastructure; Institution retains ownership of the business decision and evidence semantics.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The canonical Institution and governed scope are unambiguous.
- Every mandatory business rule, dependency condition, and required approval has a satisfactory authoritative result.
- The Institution Administrator submission and Institution Activation Authority decision are attributable.
- The Institution-owned decision is effective.
- The subject has reached **Active** without rewriting predecessor history.
- Required evidence and exact canonical events or hand-offs are available.

## 16.2 Postconditions

- Institution is authoritative in **Active** for the applicable effective scope and time.
- Historical identities, versions, relationships, decisions, and references remain resolvable where applicable.
- Material consumers may use the outcome for their own governed responsibilities; they do not become owners.
- Later operations remain subject to their own preconditions, authorization, and operation-specific readiness evaluation.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No material upstream actor/authority issue remains for this scenario. The former activation-authority ambiguity was resolved by `ADR-B-INST-ACT-001` and the locked `L2-B01_Institution_Activation_Authority_Amendment_2026-08-24.md`.

**Governance status:** the scenario remains in **Review**. Resolution of the upstream gap makes the scenario reviewable without local actor invention; it does not itself constitute L3 Architecture Approved or Locked status.

## 17.2 Assumptions

No assumptions. No local job-title mapping, delegation model, universal separation-of-duty rule, authority hierarchy, state, event, or ownership change is inferred beyond the authoritative sources.

## 17.3 Deferred Questions

- Institution-specific mapping of the abstract `Institution Activation Authority` to local organizational roles remains governed by institutional policy and later authorization/implementation architecture.
- API, UI, persistence, messaging, deployment, technical RBAC, and implementation recovery remain deferred to later architecture levels.