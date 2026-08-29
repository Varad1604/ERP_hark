# Monitoring Evidence or Outcome to Governed Academic Risk Assessment

# 1. Scenario Metadata

**Scenario ID:** L3-D-028  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Monitoring  
**Participating Domains:** C, D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** `L2-D05_Academic_Monitoring_Workflows.md`  
**Governance Basis:** `L2-D05_Business_Architecture_Purity_Amendment_2026-08-24.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0749 / CAN-L3-D-029 (final L3-D-028)

---

# 2. Scenario Purpose

Transform monitoring evidence, evaluation results or a Published monitoring outcome into a governed Academic Risk Assessment with evidence, category, severity and priority.

# 3. Business Outcome

The academic risk is detected and assessed under approved D05 rules with a traceable severity and priority suitable for monitoring, governance-exception or intervention decisions.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A valid monitoring evaluation, health assessment, indicator threshold breach or Published outcome indicates a potential academic risk.

## 4.2 End Boundary

The Risk Assessment reaches Assessed with severity and priority recorded and applicable D05 risk events published.

## 4.3 In Scope

- Apply approved risk rules to authoritative monitoring evidence.
- Detect and classify risk conditions.
- Assess severity, impact and priority with evidence traceability.

## 4.4 Out of Scope

- Changing source-domain records.
- Automatically resolving the risk.
- Bypassing governance-exception/review requirements.
- Engine, service, retry, queue or transaction design.

## 4.5 Related Scenarios — Optional

Governance-review, exception and intervention scenarios may consume the Assessed D05 risk without changing its ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D05 | Owns Academic Risk Assessment, detection rule, category, severity, priority and lifecycle. |
| D01–D04 and Section C | Retain ownership of evidence referenced by the assessment. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority |
|---|---|---|
| Monitoring Authority | Reviews identified risks and monitoring evidence where human review is required. | Monitoring review authority. |
| Governance Authority | Handles critical risks and subsequent governed actions where required. | Governance decision authority. |

Risk-rule evaluation itself is a D05 business capability, not a technical `Monitoring Engine` actor at L3.

# 7. Preconditions and Entry Criteria

- Monitoring evaluation/context exists.
- Risk rules and policy versions are approved and active.
- Required evidence references are available.

# 8. Trigger

Evaluation completes, a health assessment is produced, or an academic indicator crosses an approved threshold.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish the authoritative evaluation/health/evidence context and applicable risk rules. | D05 | Assessment basis is established. |
| 2 | Evaluate indicators and identify risk conditions. | D05 | Risk detected or no-risk result retained. |
| 3 | Establish the Risk Assessment with category, governing rule and evidence references. | D05 | Detected risk is traceable. |
| 4 | Assess academic, scope, compliance, certification and urgency impact. | Monitoring Authority / D05 | Severity basis is established. |
| 5 | Assign the governed severity and priority under approved D05 rules. | D05 | Risk reaches Assessed. |
| 6 | Publish the completed risk facts and make the assessment available for any required governance action. | D05 | Risk events are available; source evidence remains unchanged. |

# 10. Alternate Business Flows

No distinct alternate path reaches Assessed. A missing rule/evidence condition blocks assessment and is handled under Section 11.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Owner |
|---|---|---|---|
| Required evidence/rule reference is missing or invalid | Risk assessment cannot validly complete. | Resolve the authoritative business evidence/rule condition before continuation. | D05 with source owner as applicable |
| Required human monitoring/governance authority cannot be validated | Governed review/action cannot proceed. | Retain the last authoritative D05 position until valid authority exists. | D05 |
| Assessment progression is interrupted | Assessed outcome has not occurred. | Preserve the last authoritative business position/evidence/context and continue when the blocking condition is resolved. | D05 |

# 12. Business Rules and Decision Constraints

- D05 owns Academic Risk Assessment. Source: L2-D05 §§8.8,12.3–12.4.
- Repeated consideration of the same business fact must not create duplicate institutional outcomes; implementation deduplication is deferred. Source: locked D05 Purity Amendment.
- Source-domain evidence ownership remains unchanged.
- No technical engine, queue, retry or persistence behaviour is defined at L3.

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Academic Risk Assessment | Potential condition / Detected | Create assessment and govern severity/priority determination. | Detected → Assessed | D05 | L2-D05 §§8.8,12.3–12.4 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| `AcademicRiskDetected` / `RiskAssessmentCreated` / `AcademicRiskSeverityAssigned` | D05 | Governance review, exception and intervention workflows | Records detection, assessment creation and assigned severity. | L2-D05 §§12.3–12.4 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

D01–D04/Section C provide referenced evidence under their own ownership; D05 owns only the Academic Risk Assessment.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Monitoring and Governance Authorities exercise only the business authority established by D05 policy; risk-rule evaluation carries no governance authority of its own.

## 15.2 Separation of Duties, Overrides and Exceptions

Any separation between monitoring review and governance decision remains governed by D05. Automated execution may apply approved rules but cannot acquire business decision authority.

## 15.3 Business Audit and Evidence Obligations

Retain source evidence references, rule/policy version, assessment basis, severity/priority reasoning, applicable actor authority and published risk events.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Risk reaches Assessed with governed severity/priority.
- Source evidence remains under its canonical owner.
- Required D05 risk events are published without duplicate institutional outcome.

## 16.2 Postconditions

- D05 retains Risk Assessment ownership.
- Required governance/exception/intervention processes may continue using the Assessed risk.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No new open issue identified by the L3-purity correction.

## 17.2 Assumptions

No assumption overrides locked architecture.

## 17.3 Deferred Questions

Technical rule engines, queues, retries, transaction handling, persistence and deduplication are deferred to L4/L5.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** L2-D05; locked D05 Business Architecture Purity Amendment; Template v2.0; Architecture Level Validation Standards.
