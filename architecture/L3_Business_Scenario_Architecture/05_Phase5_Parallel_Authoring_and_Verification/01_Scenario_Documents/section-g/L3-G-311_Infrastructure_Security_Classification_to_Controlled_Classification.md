# Infrastructure Security Classification to Controlled Classification

# 1. Scenario Metadata

**Scenario ID:** L3-G-311  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-G07 — Security Operations Workflows; L2-A04 — Authorization & Policy Workflows  
**Referenced By:** Authorized consumers requiring canonical physical-area security classification

# 2. Scenario Purpose

Establish an authorized security classification for physical Infrastructure under G10 ownership while keeping operational security workflows and access-control behaviour outside the classification outcome.

# 3. Business Outcome

The authorised security classification is established without transferring operational security ownership from G07

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a physical infrastructure resource requires a security classification.

## 4.2 End Boundary

The scenario ends when the requested physical-area security classification has passed authorization and policy validation and the authorized classification is established in G10 without creating or changing G07 operational security workflows.

## 4.3 In Scope

- Identification of the physical Infrastructure requiring classification.
- Authorization and policy validation for the classification action.
- Selection of a source-supported/configured physical security classification.
- Establishment of the classification against the canonical Infrastructure fact.
- Preservation of the boundary between physical classification and operational security/access control.

## 4.4 Out of Scope

- Gate operations, visitor management, patrols, incident response or emergency security coordination.
- Operational access-control execution.
- Security personnel management.
- Creation of a new security-classification lifecycle or event not defined upstream.
- Transfer of physical Infrastructure ownership to G07.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-G-286 — Space Classification to Controlled Physical Classification | Covers general physical classification separate from the security-specific outcome. |
| Related | L3-G-299 — Administrative Infrastructure Change to Governed Update | Generic authorized Infrastructure changes remain separately governed. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns physical Infrastructure and the physical security classification recorded against it. |
| Platform Foundation / Section A | Owns authorization and policy-evaluation capability required before the classification change. |

G07 Security Operations is a source boundary reference, not a frozen participating domain in this scenario. It retains operational campus-security workflows and does not own this G10 physical classification outcome.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Initiates or applies an authorized physical security classification. | Must act within authorized Infrastructure scope. |
| Infrastructure Manager | Reviews the proposed classification against G10 physical context and policy. | Does not own G07 operational security. |
| Auditor | Reviews classification history and evidence. | Read-only participation. |

No dedicated “Security Classifier” role is defined by the reviewed G10 source; none is invented.

# 7. Preconditions and Entry Criteria

- The physical Infrastructure exists under G10 ownership.
- A security classification requirement has been identified.
- The requested classification is permitted by institutional configuration/policy.
- The classification action is authorized.

# 8. Trigger

A physical infrastructure resource requires a security classification

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the authoritative physical Infrastructure requiring security classification. | Infrastructure Management | The classification target is established. |
| 2 | Validate authorization for the classification action. | Infrastructure Management using Section A authorization | Unauthorized classification does not proceed. |
| 3 | Validate the requested classification against applicable Infrastructure policy/configuration. | Infrastructure Management / Section A policy capability | Unsupported classification is not applied. |
| 4 | Establish the authorized security classification against the physical Infrastructure. | Infrastructure Management | The canonical physical classification is updated. |
| 5 | Preserve the explicit boundary that this classification does not replace G07 operational access control or security workflows. | Infrastructure Management | Operational security ownership remains unchanged. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Requested classification is not configured | The requested classification is not permitted by institutional configuration. | Do not create a local classification; resolve configuration/policy through its canonical governance path. | No unsupported classification is stored. | Infrastructure Management / Platform Foundation |
| Operational security action is also required | Classification implies a separate gate/access/patrol/incident response need. | Complete this physical classification outcome and use the separately owned G07 operational workflow for the operational action. | Physical and operational security ownership remain distinct. | Infrastructure Management / G07 Security Operations |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Authorization fails | Classification cannot be established. | Preserve the existing classification and obtain a later valid authorization if appropriate. | Platform Foundation / Infrastructure Management |
| Policy/configuration validation fails | Requested classification cannot be accepted. | Correct the request or use a separately governed configuration change; do not invent a classification. | Infrastructure Management |
| Requested action attempts to change operational security behaviour directly | Scenario boundary would be violated. | Keep the G10 classification outcome separate and use G07's canonical operational workflow. | Infrastructure Management / G07 Security Operations |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| G10 may classify physical areas according to security requirements. | Infrastructure Management | Physical security classification is a G10 fact. | L2-G10 §38 |
| Security classification workflow requires Authorization Validation and Policy Validation before update. | Infrastructure Management / Platform Foundation | Classification cannot bypass those checks. | L2-G10 §38 |
| Security classification shall not replace operational access control owned by Security. | Infrastructure Management / G07 Security Operations | Classification does not transfer operational security ownership. | L2-G10 §38; L2-G07 §§1–5 |
| G07 owns gate, visitor, patrol, incident and emergency security operations. | G07 Security Operations | Operational reactions remain outside this scenario. | L2-G07 §§1–5, 12 |
| Every business operation shall be authorized before execution. | Platform Foundation — Authorization | Unauthorized classification does not proceed. | L2-A04 AP-01 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Physical Infrastructure security classification | Existing classification or no established classification as allowed by configuration | Authorized classification is established/updated. | Current G10 security classification reflects the authorized value. | Infrastructure Management | L2-G10 §38 |
| Physical Infrastructure lifecycle | Existing source-defined state | No lifecycle transition is implied merely by classification. | Unchanged unless a separate G10 lifecycle/availability action is required. | Infrastructure Management | L2-G10 lifecycle |
| G07 operational security aggregates | Existing source-defined states | No mutation by this scenario. | Unchanged. | G07 Security Operations | L2-G07 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

L2-G10 §38 defines the security-classification workflow result as “Security Classification Updated” but does not provide an exact canonical event name in the reviewed event catalogue. Generic classification events elsewhere in G10 are not assumed to be the security-classification event. This Draft therefore does not invent or rename an event.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Section A supplies the authorization/policy decision capability. G07 remains the owner of separately executed operational security workflows. No ownership transfer or mandatory G10→G07 command is created by the classification itself.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Establish/update physical security classification | Infrastructure Administrator or Infrastructure Manager within authorized scope | Must satisfy Section A authorization and G10 policy validation. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

No universal security-classification approver or separation-of-duty rule is defined in the reviewed sources. None is invented.

## 15.3 Business Audit and Evidence Obligations

The classification change must remain traceable with the physical Infrastructure reference, prior/current classification where applicable, business reason, authorization context and applicable policy evidence under G10/platform audit rules.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The authoritative physical Infrastructure is identified.
- Authorization and policy validation succeed.
- A permitted security classification is established.
- No G07 operational security workflow is silently executed or re-owned.

## 16.2 Postconditions

- G10 holds the authoritative physical security classification.
- G07 continues to own operational security/access-control workflows.
- Physical Infrastructure ownership remains unchanged.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-G10 §38 defines the completed result but no exact canonical security-classification event name. | A business event cannot be asserted without invention. | Define/register the exact canonical event upstream if event publication is required for this change. |

**Architecture assessment:** `PASS WITH ISSUES`

The frozen outcome and ownership boundary are fully source-grounded; the event-name gap is non-blocking for the Draft.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
