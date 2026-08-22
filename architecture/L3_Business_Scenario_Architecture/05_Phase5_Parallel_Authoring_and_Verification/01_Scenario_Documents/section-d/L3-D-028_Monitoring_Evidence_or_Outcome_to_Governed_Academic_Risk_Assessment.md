# Monitoring Evidence or Outcome to Governed Academic Risk Assessment

**Scenario ID:** L3-D-028  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Monitoring  
**Participating Domains:** C, D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D05_Academic_Monitoring_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0749 / CAN-L3-D-029 (final L3-D-028)

---

# 2. Scenario Purpose

Transform monitoring evidence, evaluation results or a published monitoring outcome into a governed Academic Risk Assessment with evidence, category, severity and priority.

# 3. Business Outcome

The academic risk is detected and assessed under approved rules with a traceable severity and priority suitable for monitoring, governance exception or intervention decisions.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A valid monitoring evaluation, health assessment, indicator threshold breach or published outcome indicates a potential academic risk.

## 4.2 End Boundary

The Risk Assessment is Assessed with severity and priority recorded and applicable risk events published.

## 4.3 In Scope

- Load approved risk rules and immutable evaluation context.
- Detect and classify risk conditions.
- Assess severity, impact and priority with evidence traceability.

## 4.4 Out of Scope

- Changing attendance, timetable, registration or teaching records.
- Automatically resolving the risk.
- Bypassing governance exception/review requirements.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D05 | Owns Academic Risk Assessment, detection rule, category, severity, priority and lifecycle. |
| D01–D04 and Section C | Retain ownership of evidence referenced by the assessment. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| D05 Monitoring Engine | Evaluates active risk rules and creates the assessment. | Automated assessment; no governance approval. |
| Monitoring Authority | Reviews identified risks and evidence. | Monitoring review authority. |
| Governance Authority | Handles critical risks and subsequent governed actions. | Governance decision authority. |

# 7. Preconditions and Entry Criteria

- Monitoring evaluation and immutable context exist.
- Risk rules and policy versions are approved and active.
- Required evidence references are available.

# 8. Trigger

Evaluation completes, health assessment is generated, or an academic indicator crosses an approved threshold.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Load evaluation context, health result, evidence references and active risk rules. | D05 Monitoring Engine | Authoritative assessment basis established. |
| 2 | Evaluate indicators and identify risk conditions. | D05 Monitoring Engine | Risk detected or no-risk result retained. |
| 3 | Create Risk Assessment with category, rule, evidence and timestamp. | D05 | Detected risk is traceable. |
| 4 | Analyze academic, scope, compliance, certification and urgency impact. | Monitoring Authority / D05 | Severity basis established. |
| 5 | Assign Critical, High, Medium or Low severity and priority. | D05 under approved rules | Risk reaches Assessed. |
| 6 | Publish completed risk facts and route required governance action. | D05 | Risk events available; source evidence unchanged. |

# 10. Alternate Business Flows

A failed validation, rejection, configured optional approval path, escalation or compensation follows only the alternatives explicitly supported above; it never creates the successful final outcome or rewrites prior facts.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Recovery | Owner |
|---|---|---|---|
| Validation or reference failure | Scenario cannot proceed. | Reject or return for correction without changing the prior authoritative state. | Academic Monitoring |
| Authorization or approval failure | Requested outcome is not permitted. | Retain the prior state and record the decision reason. | Academic Monitoring |
| Interrupted processing | Completion is uncertain. | Resume from the last authoritative state with audit continuity and idempotency. | Academic Monitoring |

# 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| Only the canonical owner may change its owned business state. | Academic Monitoring | Prevents cross-domain ownership leakage. | `L2-D05_Academic_Monitoring_Workflows.md` |
| Each transition requires valid lifecycle position, authority, policy and evidence. | Academic Monitoring | Prevents governance bypass. | `L2-D05_Academic_Monitoring_Workflows.md` |
| Published or historical business facts remain immutable. | Academic Monitoring | Preserves auditability and lineage. | `L2-D05_Academic_Monitoring_Workflows.md` |

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Academic Risk Assessment | Potential condition / Detected | Create assessment and govern severity/priority determination. | Detected → Assessed | D05 | L2-D05 §§8.8,12.3–12.4 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| AcademicRiskDetected / RiskAssessmentCreated / AcademicRiskSeverityAssigned | D05 | Governance review, exception and intervention workflows | Records detection, assessment creation and assigned severity. | L2-D05 §§12.3–12.4 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| D01–D04 and Section C | Academic Monitoring | Retain ownership of evidence referenced by the assessment. | Preserve ownership while enabling the scenario. | `L2-D05_Academic_Monitoring_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Evaluates active risk rules and creates the assessment. | D05 Monitoring Engine | Automated assessment; no governance approval. | Academic Monitoring |
| Reviews identified risks and evidence. | Monitoring Authority | Monitoring review authority. | Academic Monitoring |
| Handles critical risks and subsequent governed actions. | Governance Authority | Governance decision authority. | Academic Monitoring |

## 15.2 Separation of Duties, Overrides and Exceptions

Initiation, validation, review, approval and execution remain distinct wherever source or configured policy requires separation. Automated services execute rules but never make governance decisions. Any override requires authority, justification, evidence and audit history.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Scenario lifecycle and decision | Actor, authority, reason, prior state, validation evidence, decision and resulting state. | Academic Monitoring |
| Source and cross-domain traceability | Source identifiers, versions, evidence references, consumed events and published facts. | Academic Monitoring |
| Failure and recovery | Rejected validations, interruptions, retries, escalation or compensation outcome. | Academic Monitoring |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The academic risk is detected and assessed under approved rules with a traceable severity and priority suitable for monitoring, governance exception or intervention decisions.
- All required authority, validation and audit evidence is retained.
- Applicable completed business facts are published exactly once in business effect.

## 16.2 Postconditions

- Canonical ownership remains unchanged.
- Historical decisions and prior states remain immutable.
- Downstream domains consume facts without modifying the owning aggregate.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open issues.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No deferred questions.

---

**Draft Validation Verdict:** PASS  
**Validation Basis:** Section D canonical register; `L2-D05_Academic_Monitoring_Workflows.md`; Template v2.0; Architecture Level Validation Standards; accepted Section D decisions 001–500.
