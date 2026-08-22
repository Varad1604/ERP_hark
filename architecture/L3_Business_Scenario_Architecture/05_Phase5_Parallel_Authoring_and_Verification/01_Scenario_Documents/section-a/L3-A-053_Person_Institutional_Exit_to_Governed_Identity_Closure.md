# Person Institutional Exit to Governed Identity Closure

# 1. Scenario Metadata

**Scenario ID:** L3-A-053  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Person & Identity  
**Participating Domains:** A, VAR:Relationship-Owning Domain  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A02 — Person & Identity Workflows; L2-A03 — Authentication Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Domains retaining historical references to a concluded institutional relationship

---

# 2. Scenario Purpose

Govern closure of a canonical Person and associated Identity after institutional exit or an applicable privacy, legal or retention decision while preserving resolvable downstream references and immutable history.

---

# 3. Business Outcome

The Person progresses through governed deactivation and archival and, where approved, anonymization or permanent deletion, while downstream references remain resolvable and identity audit history is preserved

Source assessment: `PASS WITH ISSUES`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A Person's institutional relationship ends, or a privacy, legal or retention obligation requires closure of a canonical identity.

## 4.2 End Boundary

The Person is no longer operationally active, authentication is unavailable, the Person and Identity are archived, and any approved later anonymization or deletion has followed policy without breaking historical references.

## 4.3 In Scope

- Tenant and Person resolution.
- Relationship-closure evidence from the owning domain.
- Deactivation, session termination and archival.
- Dependency and retention validation.
- Approved anonymization or exceptional deletion where applicable.
- Preservation of identifiers, references and audit history.

## 4.4 Out of Scope

- Ending the source-domain relationship itself.
- Credential, provider or session ownership.
- Role, permission or authorization-policy lifecycle.
- Physical deletion mechanisms, APIs, storage or UI design.

## 4.5 Related Scenarios

| Relationship | Scenario | Boundary |
|---|---|---|
| Upstream | VAR:Relationship-Owning Domain exit scenario | Owns the institutional relationship and supplies the governed exit fact. |
| Related | L3-A-008 — Lost Authentication Factor Recovery | Credential recovery cannot reactivate an archived Person. |

---

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Person & Identity / Section A | Owns Person, Identity and User Account closure lifecycles. |
| Authentication / Section A | Terminates or prevents authenticated sessions under A03 ownership. |
| Platform & Tenant / Section A | Supplies Tenant context and retention configuration. |
| Event Infrastructure / Section A | Distributes canonical Person and Identity events. |
| Audit Infrastructure / Section A | Preserves immutable closure evidence. |
| VAR:Relationship-Owning Domain | Owns the concluded institutional relationship and its exit rationale. |

---

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority Boundary |
|---|---|---|
| Relationship Owner | Supplies the governed relationship-end fact and retained business references. | Cannot modify Person or Identity state directly. |
| Authorized Administrator | Initiates governed deactivation or archival where policy permits. | L2-A02 refers to administrative action but does not define a canonical role name. |
| Privacy / Retention Authority | Supplies an approved privacy, legal or retention decision where applicable. | Exact actor and approval contract remain upstream gaps. |

No more specific administrative or legal authority is invented.

---

# 7. Preconditions and Entry Criteria

- Exactly one Tenant and canonical Person are resolved.
- The institutional relationship-end or applicable legal decision is evidenced.
- Associated Identity and User Account references are known.
- Dependencies and downstream references can be evaluated.
- Authentication sessions can be terminated through A03.
- Retention, anonymization and deletion policies can be evaluated.

---

# 8. Trigger

A Person's institutional relationship ends, or a privacy, legal or retention obligation requires closure of a canonical identity

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive and validate the governed exit or closure requirement. | Relationship Owner; Person & Identity | The closure basis is established. |
| 2 | Resolve the Tenant, Person, Identity, User Account and downstream reference scope. | Person & Identity; Platform & Tenant | One canonical closure subject is fixed. |
| 3 | Validate authority, dependencies, retention obligations and blocking conditions. | Person & Identity | Only an eligible transition proceeds. |
| 4 | Transition an `Active` Person to `Inactive`, disable login and request termination of active sessions. | Person & Identity; Authentication | Operational participation stops and `PersonInactivated` is published. |
| 5 | Preserve all business references and verify archival eligibility. | Person & Identity; Relationship Owner | Historical traceability remains intact. |
| 6 | Archive the Person and associated Identity and keep the records read-only. | Person & Identity | `PersonArchived` and the applicable Identity lifecycle fact are published. |
| 7 | If an approved privacy request applies, validate retention and mask only policy-permitted personal data. | Person & Identity | The Person reaches `Anonymized` and `PersonAnonymized` is published. |
| 8 | If exceptional deletion is approved, revalidate legal retention, dependencies and security confirmation. | Person & Identity | Deletion eligibility is established. |
| 9 | Complete the approved deletion without erasing retained business and audit references. | Person & Identity | The Person reaches `Deleted` and `PersonDeleted` is published. |
| 10 | Preserve actor, authority, decisions, states, events and correlation evidence. | Person & Identity; Audit Infrastructure | Closure remains auditable. |

Steps 7–9 are conditional later paths; archival completes the normal institutional-exit outcome.

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Path | Outcome |
|---|---|---|---|
| Temporary deactivation only | The relationship may resume or archival is not yet eligible. | Stop at `Inactive`. | History is retained and approved reactivation remains possible. |
| Direct archival eligibility | Policy permits `Active → Archived`. | Validate dependencies, terminate sessions and archive. | The Person is read-only and non-authenticating. |
| Privacy anonymization | Retention policy permits masking after archival. | Apply only approved masking. | Business and audit references remain resolvable. |
| Deletion prohibited | Retention or dependencies block deletion. | Retain the archived or anonymized record. | No `PersonDeleted` event is published. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Resolution | Owner |
|---|---|---|---|
| Closure authority is invalid. | Person state cannot change. | Reject and audit the request. | Person & Identity |
| Blocking dependency exists. | Archival, anonymization or deletion could break references. | Stop the affected transition until governed resolution. | Person & Identity; Relationship Owner |
| Session termination is incomplete. | Authentication may remain available incorrectly. | Do not assert completed closure; coordinate A03 termination. | Authentication; Person & Identity |
| Legal retention is unsatisfied. | Deletion is prohibited. | Retain archived or anonymized state. | Person & Identity |
| Closure is interrupted. | Final state is uncertain. | Preserve completed transitions and resume only through governed validation. | Person & Identity |

---

# 12. Business Rules and Decision Constraints

| Rule / Constraint | Effect | Source |
|---|---|---|
| Person & Identity exclusively owns its aggregates. | Other domains supply facts but cannot close identity records. | L2-A02 — INT-301 |
| Person identifiers remain immutable. | Historical references remain resolvable. | L2-A02 — Person invariants |
| Archive precedes deletion and deletion requires approval. | Exceptional deletion cannot bypass governance. | L2-A02 — Business Rules |
| Archived identities cannot authenticate. | Closure prevents new authenticated use. | L2-A02 — Identity state definitions |
| Anonymization preserves business and audit references. | Permitted PII is masked without destroying history. | L2-A02 — Anonymization Workflow |
| Every closure transition is auditable. | Actor, authority, evidence and outcome are retained. | L2-A02 — Complete Auditability |

---

# 13. State and Lifecycle Impact

| Concept | Entry | Impact | Result |
|---|---|---|---|
| Person | Active or Inactive | `Active → Inactive → Archived`; optionally `Archived → Anonymized` or `Archived → Deleted`. | Non-operational governed closure. |
| Identity | Active or otherwise eligible | Disable and archive consistently with Person closure. | Archived; authentication prohibited. |
| User Account | Login-capable where present | Disable login; preserve history. | Disabled or archived under A02 lifecycle. |
| Authentication Session | May be Active | Terminate through A03. | No active closure-subject session. |
| Downstream references | Existing | Preserve against immutable Person identifier. | Resolvable historical references. |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Business Events

| Event | Publisher | Condition |
|---|---|---|
| `PersonInactivated` | Person aggregate | Governed deactivation succeeds. |
| `PersonArchived` | Person aggregate | Archival succeeds. |
| `PersonAnonymized` | Person aggregate | Approved masking succeeds. |
| `PersonDeleted` | Person aggregate | Exceptional governed deletion succeeds. |
| `IdentityDisabled` / `IdentityArchived` | Identity aggregate | Corresponding Identity transition succeeds. |

## 14.2 Synchronous Contracts and Hand-offs

| Contract | Provider | Consumer | Information / Outcome |
|---|---|---|---|
| Relationship-end fact | VAR:Relationship-Owning Domain | Person & Identity | Exit rationale and effective date. |
| Tenant and retention context | Platform & Tenant | Person & Identity | Scope and applicable policy. |
| Session termination | Authentication | Person & Identity | Active authentication context ended. |
| Stable Person reference | Person & Identity | Downstream domains | Historical identity resolution. |
| Audit input | Person & Identity | Audit Infrastructure | Complete closure evidence. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Every state-changing action requires an authorized Tenant-scoped actor.
- Closure does not transfer identity ownership to the relationship domain.

## 15.2 Approval and Separation of Duty

- Permanent deletion requires administrative approval, dependency validation, legal-retention validation and security confirmation.
- L2-A02 does not name the exact approving roles; none are inferred.

## 15.3 Audit Obligations

Record Tenant, Person, Identity, relationship fact, authority, dependencies, retention decision, prior/resulting states, events, actor, timestamp and correlation while protecting PII.

---

# 16. Completion Criteria and Postconditions

- The closure basis and authority were validated.
- Operational login and sessions were disabled.
- Person and Identity reached governed archival.
- Historical identifiers, downstream references and audit history remain resolvable.
- Optional anonymization or deletion occurred only after its distinct approvals and validations.
- Only successful transitions published their canonical events.

Postconditions: the source relationship remains owned by `VAR:Relationship-Owning Domain`; archived identity cannot authenticate; closure history is immutable.

---

# 17. Open Issues, Assumptions and Deferred Questions

| ID | Severity | Issue | Treatment / Required Resolution |
|---|---|---|---|
| A053-ISSUE-01 | Non-blocking | L2-A02 does not define the exact contract or event by which `VAR:Relationship-Owning Domain` supplies institutional exit, nor name privacy/deletion approval actors. | Use a governed hand-off without inventing an event or role; define the authoritative contract and actors before locking. |
| A053-ISSUE-02 | Non-blocking | L2-A02's Archive Workflow says “Archive Identity” but does not specify the exact coordination sequence between `PersonArchived`, `IdentityDisabled` and `IdentityArchived`. | Require consistent completed states and publish only aggregate-owned successful facts; clarify ordering before L4. |

No unsupported actor, event, state, approval authority or implementation mechanism is assumed.
