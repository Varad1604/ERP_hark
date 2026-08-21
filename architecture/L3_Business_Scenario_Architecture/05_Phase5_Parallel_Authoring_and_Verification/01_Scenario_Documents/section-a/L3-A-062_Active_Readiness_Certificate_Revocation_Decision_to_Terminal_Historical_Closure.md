# Active Readiness Certificate Revocation Decision to Terminal Historical Closure

# 1. Scenario Metadata

**Scenario ID:** L3-A-062  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Operational Readiness Governance & Certification  
**Participating Domains:** A  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows; L2-A10 — Operational Readiness Workflows; L3-A-046  
**Referenced By:** Governance, regulatory, audit and operation-specific readiness consumers

---

# 2. Scenario Purpose

Permanently revoke an Active Operational Readiness Certificate under recorded regulatory or governance authority and move it to governed historical archival without erasing its assessment, evidence or lifecycle history.

---

# 3. Business Outcome

The certificate becomes Revoked under recorded authority, can no longer return to active use, and proceeds to governed terminal historical retention or archival without erasing its assessment, evidence or lifecycle history

Source assessment: `PASS`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A regulatory or governance authority determines that an active readiness certificate must be permanently revoked.

## 4.2 End Boundary

The Certificate is `Revoked` and then `Archived` under policy, remains permanently queryable with its assessment and evidence lineage, and cannot return to Active use.

## 4.3 In Scope

- Certificate, scope and authority validation.
- Permanent revocation decision and transition.
- Revocation publication and consumer notification.
- Terminal archival/retention transition.
- Preservation of assessment, evidence and lifecycle history.

## 4.4 Out of Scope

- Temporary suspension and restoration.
- Successor certification and supersession.
- Operational remediation or runtime readiness state.
- Technical storage, document, notification or UI implementation.

## 4.5 Related Scenarios

| Relationship | Scenario | Boundary |
|---|---|---|
| Upstream | L3-A-046 — Certificate Availability | Supplies the Active certificate. |
| Related | L3-A-047 — Certificate Suspension and Restoration | Temporary, reversible lifecycle path. |
| Related | L3-A-061 — Certificate Supersession | Replacement by successor rather than withdrawal. |

---

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Operational Readiness Governance & Certification / Section A | Owns Certificate revocation, archival, authority evidence and lineage. |
| Event Infrastructure / Section A | Distributes canonical revocation and archival events. |
| Audit Infrastructure / Section A | Preserves immutable terminal evidence. |

---

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority Boundary |
|---|---|---|
| Governance Authority | Issues the governed permanent revocation decision. | Must act under applicable policy. |
| Regulatory Authority | Supplies binding regulatory revocation basis where applicable. | Does not directly mutate the Certificate aggregate. |
| Responsible Domain | Receives lifecycle facts and may perform separate remediation. | Cannot restore or delete a revoked certificate. |
| System | Executes validated commands and publishes events. | Cannot create the authority decision. |

---

# 7. Preconditions and Entry Criteria

- The Certificate exists and is `Active`.
- Governed scope and originating Assessment are resolvable.
- Revocation authority, rationale and effective time are recorded.
- Applicable policy permits and requires permanent revocation.
- Historical retention and archival requirements are known.

---

# 8. Trigger

A regulatory or governance authority determines that an active readiness certificate must be permanently revoked

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the formal revocation decision and authority evidence. | Governance / Regulatory Authority | Permanent withdrawal is requested. |
| 2 | Resolve the Active Certificate, governed scope, Assessment and evidence lineage. | Certification Service | Complete revocation context is established. |
| 3 | Validate authority, policy, rationale, effective time and current lifecycle state. | Certification Service | Revocation eligibility is confirmed. |
| 4 | Record the immutable governance decision and affected scope. | Operational Readiness | Decision evidence is fixed. |
| 5 | Execute `RevokeOperationalReadinessCertificate`. | Certification Service | Certificate reaches `Revoked`. |
| 6 | Publish `OperationalReadinessCertificateRevoked`. | Operational Readiness Certificate | Consumers receive the permanent withdrawal fact. |
| 7 | Prevent the Revoked Certificate from returning to Active use. | Certification Service | Revocation is terminal for operational certification. |
| 8 | Preserve the Certificate, Assessment, evidence, decisions and lifecycle history through the retention period. | Operational Readiness; Audit Infrastructure | Governance traceability remains intact. |
| 9 | When archival criteria are met, execute `ArchiveOperationalReadinessCertificate`. | Certification Service | Certificate reaches `Archived`. |
| 10 | Publish `OperationalReadinessCertificateArchived`. | Operational Readiness Certificate | Terminal historical closure is available to consumers. |
| 11 | Preserve authority, commands, events, timestamps and correlation evidence. | Operational Readiness; Audit Infrastructure | Completion is auditable. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Path | Outcome |
|---|---|---|---|
| Authority invalid | Decision is not issued by an applicable authority. | Reject revocation. | Certificate remains Active. |
| Temporary condition | Policy supports restoration after resolution. | Use L3-A-047 suspension instead. | No Revoked transition. |
| Archival deferred | Retention criteria are not yet met. | Retain Certificate as Revoked and queryable. | Permanent withdrawal remains effective. |
| Successor required | Governance also requires new certification. | Initiate a separate reassessment/certification cycle. | Revoked history remains unchanged. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Resolution | Owner |
|---|---|---|---|
| Certificate is not Active. | Requested transition may be invalid. | Evaluate applicable current-state lifecycle rule; do not force transition. | Certification Service |
| Authority or rationale is incomplete. | Permanent withdrawal is not governed. | Stop before revocation. | Governance Authority |
| Revocation command fails. | Certificate remains current. | Do not publish revocation; preserve failed attempt evidence. | Certification Service |
| Revocation succeeds but event publication fails. | Consumers may retain stale evidence. | Preserve committed fact and recover delivery through Event Infrastructure without repeating decision. | Event Infrastructure |
| Archival validation fails. | Historical preservation is incomplete. | Retain Revoked state until archival criteria succeed. | Certification Service; Audit Infrastructure |

---

# 12. Business Rules and Decision Constraints

| Rule / Constraint | Effect | Source |
|---|---|---|
| Revocation permanently withdraws certification. | Certificate cannot return to Active. | L2-A10 §20.3 |
| Revocation never deletes certification history. | Assessment and evidence remain available. | L2-A10 §27.2 |
| Revoked certificates remain historically available. | Governance can reconstruct the decision. | L2-A10 §36.6 |
| Archived certificates remain permanently traceable. | Archival is not erasure. | L2-A10 §20.4 |
| Lifecycle transitions are policy-driven and auditable. | Authority and decision evidence are mandatory. | L2-A10 §27.2 |
| Certificate lifecycle does not set runtime readiness. | Business operations require separate evaluation. | L2-A10 domain boundary |

---

# 13. State and Lifecycle Impact

| Concept | Entry | Impact | Result |
|---|---|---|---|
| Operational Readiness Certificate | Active | `Active → Revoked → Archived`. | Terminal historical artifact. |
| Operational Readiness Assessment | Existing | Reference and preserve unchanged. | Immutable. |
| Readiness Evidence | Existing, source-owned | Preserve references and verification history. | Unchanged ownership and traceability. |
| Runtime readiness | Not persisted | No direct change. | Re-evaluated per operation elsewhere. |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Business Events

| Event | Publisher | Condition |
|---|---|---|
| `OperationalReadinessCertificateRevoked` | Operational Readiness Certificate / Certification Service | Revocation succeeds. |
| `OperationalReadinessCertificateArchived` | Operational Readiness Certificate / Certification Service | Governed archival succeeds. |

## 14.2 Synchronous Contracts and Hand-offs

| Contract | Provider | Consumer | Information / Outcome |
|---|---|---|---|
| Revocation authority decision | Governance / Regulatory Authority | Certification Service | Rationale, authority, scope and effective time. |
| Certificate lifecycle query | Certification Service | Governance consumers | Revoked/Archived status and lineage. |
| Lifecycle facts | Operational Readiness | Affected domains | Withdrawal evidence requiring fresh operation-specific evaluation. |
| Audit input | Operational Readiness | Audit Infrastructure | Complete terminal history. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

Revocation and archival require authorized governance context and do not grant or execute business access.

## 15.2 Approval and Separation of Duty

The permanent decision must be issued by the applicable Governance or Regulatory Authority and executed by the Certificate owner; evidence supply does not confer revocation authority.

## 15.3 Audit Obligations

Record Certificate, scope, Assessment, authority, rationale, policy, effective time, prior/resulting states, commands, events, actor, timestamps and correlation while retaining evidence lineage.

---

# 16. Completion Criteria and Postconditions

- Active Certificate and its lineage were resolved.
- Permanent authority and policy were validated.
- Revocation command succeeded and revocation event was published.
- Certificate cannot return to Active use.
- Assessment, evidence and lifecycle history remain immutable.
- Archival occurred only after criteria were satisfied and its event was published.
- No runtime readiness flag was written.

Postconditions: the Certificate remains a terminal historical governance artifact; affected domains independently re-evaluate operation-specific readiness.

---

# 17. Open Issues, Assumptions and Deferred Questions

No open issue prevents this scenario from proceeding to Architecture Approval. No unsupported actor, state, command, event, publisher, deletion or runtime readiness mechanism is assumed.
