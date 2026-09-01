# Conflicting Responsibility Risk to Enforced Separation of Duties

# 1. Specification Metadata

**Specification ID:** L4-A-051  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Authorization & Policy  
**Realizes Scenario:** L3-A-051 — Conflicting Responsibility Risk to Enforced Separation of Duties  
**Primary Business Owner:** Authorization & Policy  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** SutraERP L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-A02; L2-A03; L2-A04; L2-A06; L2-A07; L3-A-051  
**Referenced By:** None yet

---

# 2. Behavioral Objective & Boundary

Require Separation of Duties semantics to be governed centrally within Authorization & Policy, including platform defaults and only permitted institution overrides, so prohibited responsibility conflicts deny protected attempts unless an approved policy explicitly permits continuation and all remaining authorization controls also allow it.

This specification defines required observable system behavior only. It does not define APIs, database schemas, software classes, user interfaces, infrastructure, deployment, transport mechanisms, or technical orchestration.

---

# 3. Required Business Context

- Tenant, institution, business-process/action/resource context and conflicting responsibility definitions are resolvable.
- Current Role Assignments, Delegations and Temporary Access facts can be resolved.
- Authorization & Policy owns SoD Policy, SoD evaluation, explicit-deny precedence and the final Authorization Decision.
- Authentication establishes authenticated context but does not grant authorization.

---

# 4. System Behavioral Obligations

| Behavior ID | Condition | Required System Behavior | Governed Result | Provenance |
|---|---|---|---|---|
| L4-A-051-BH-001 | A conflicting-responsibility control is required. | Define the conflict semantics and governed process scope using existing responsibility concepts without creating organizational responsibilities. | The SoD requirement is explicit and ownership remains unchanged. | L3-A-051 §§2–4,9; L2-A04 |
| L4-A-051-BH-002 | Platform defaults and institution-specific configuration apply. | Enforce platform defaults and permit an institution override only when the governing policy explicitly allows that scope. | Institution rules cannot exceed platform-governed boundaries. | L3-A-051 §§9–12; L2-A04 Policy Configuration |
| L4-A-051-BH-003 | The SoD configuration is evaluated for governance. | Validate determinism, Tenant isolation, least privilege and explicit-deny precedence before it may be treated as governed. | Ambiguous or weakening configuration cannot progress. | L3-A-051 §§9,11–12; L2-A04 |
| L4-A-051-BH-004 | Policy or override approval is required. | Require source-governed approval evidence without naming an undefined approver, override authority, policy state, command or event. | No unsupported authority is created. | L3-A-051 §§9,15,17; L2-A04 |
| L4-A-051-BH-005 | A protected request reaches centralized authorization. | Resolve current Roles, Delegations, Temporary Access, workflow context and applicable SoD configuration as inputs to the authorization evaluation. | All effective responsibility sources participate in SoD evaluation. | L3-A-051 §§7,9,12; L2-A04 |
| L4-A-051-BH-006 | No prohibited conflict is detected. | Continue the remaining centralized authorization evaluation without treating the SoD result as an independent Allow. | Absence of conflict does not grant access. | L3-A-051 §§10,15; L2-A04 |
| L4-A-051-BH-007 | A prohibited conflict is detected. | Produce Deny under explicit-deny precedence unless an already approved policy explicitly defines a permitted continuation case. | Conflicting execution is blocked by default. | L3-A-051 §§3,9,12; L2-A04 AP-06 |
| L4-A-051-BH-008 | Approved policy explicitly permits continuation for the detected conflict. | Evaluate the policy conditions and all remaining authorization controls; continue only if the complete Authorization Decision is Allow. | An exception never bypasses centralized authorization. | L3-A-051 §§9–10,15; L2-A04 |

---

# 5. Cross-Domain & Event Behavior

- Person & Identity and Authentication supply canonical subject/authenticated context; responsibility facts remain owned by Authorization & Policy aggregates.
- Requesting business domain supplies protected-operation context and consumes the final Authorization Decision only.
- No canonical SoD policy configured/approved/activated event exists; audit activity labels are not promoted to events.
- Audit Infrastructure preserves SoD policy and decision evidence.

---

# 6. Failure / Rejection / Continuation

- Ambiguous scope/responsibility definitions or invalid override → do not treat the policy configuration as governed.
- Required responsibility facts cannot be reliably resolved → do not infer Allow; follow only source-established authorization policy.
- Missing approval or canonical policy event → do not invent authority or publication semantics.

---

# 7. Audit & Evidence Behavior

Preserve applicable policy/configuration version, platform defaults, institution override evidence, approval evidence where defined, subject, effective responsibilities, workflow/resource context, detected conflict, permitted-continuation basis, final Authorization Decision, actor, time and correlation.

---

# 8. Observable Completion Conditions

- Conflict semantics and governed scope are defined.
- Platform defaults and only permitted institution overrides are applied.
- Protected requests evaluate current effective responsibilities centrally.
- Prohibited conflicts produce Deny unless approved policy explicitly permits continuation and the complete authorization evaluation allows it.
- Policy-governance lifecycle/event completion remains deferred where L2-A04 is silent.

---

# 9. Traceability & Provenance

- **Primary parent:** `L3-A-051` — Conflicting Responsibility Risk to Enforced Separation of Duties.
- **Canonical owner:** Authorization & Policy.
- Higher-level ownership, business rules, lifecycle states and canonical event publishers remain governed by the cited L1/L2/L3 sources.
- This L4 specification refines observable system obligations; it does not revise or silently repair upstream architecture.

---

# 10. Open Issues / Deferred Design

| ID | Deferred Design / Upstream Gap | L4 Treatment |
|---|---|---|
| A051-L4-OPEN-01 | SoD Policy configuration workflow, commands and lifecycle states are undefined. | Preserve the gap explicitly; do not invent missing architecture at L4. |
| A051-L4-OPEN-02 | Canonical SoD Policy governance events and publishers are undefined. | Preserve the gap explicitly; do not invent missing architecture at L4. |
| A051-L4-OPEN-03 | Authorities for platform defaults, institution overrides, policy approval and explicitly permitted continuation are unnamed. | Preserve the gap explicitly; do not invent missing architecture at L4. |

No implementation design is introduced by these deferred items.
