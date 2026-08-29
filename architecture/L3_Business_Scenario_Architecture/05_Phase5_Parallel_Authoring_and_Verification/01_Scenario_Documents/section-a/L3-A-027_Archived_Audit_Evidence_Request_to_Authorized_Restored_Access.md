# Archived Audit Evidence Request to Authorized Restored Access

# 1. Scenario Metadata

**Scenario ID:** L3-A-027  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Audit Infrastructure  
**Participating Domains:** A, VAR:Evidence-Requesting Domain  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A02 — Person & Identity Workflows; L2-A03 — Authentication Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows; L3-A-026 — Retention Decision to Governed Audit Evidence Archival  
**Referenced By:** Authorized investigation and governance activities requiring archived evidence

---

# 2. Scenario Purpose

Temporarily restore authorized availability of immutable archived Audit Evidence for a governed investigation or governance activity while preserving archived storage, evidence integrity and traceable closure.

---

# 3. Business Outcome

Access to the archived evidence is temporarily restored under governed authorization while the evidence itself remains unchanged, and the restoration activity and its expiry remain traceable.

L2-A07 defines restoration and closure lifecycle transitions and the restoration event, but not the expiry rule, duration, closure workflow, closure authority or closure event required by the frozen outcome. The scenario is therefore classified `BLOCKED`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorized investigation or governance activity requires access to audit evidence held in archived storage.

## 4.2 End Boundary

The intended end boundary is reached when access is restored only for the authorized scope, the activity is completed or its access period expires, Availability Status returns to `Restricted Access`, and restoration/closure history is traceable.

## 4.3 In Scope

- Restoration request, purpose, evidence scope and authorization.
- Archived-evidence location and integrity confirmation.
- `Restricted Access → Restored Access` transition.
- Read-only authorized access.
- Intended expiry/closure and return to `Restricted Access`.
- Restoration and access traceability.

## 4.4 Out of Scope

- Moving evidence back to Active Storage.
- Modifying, duplicating or deleting evidence.
- Defining technical restoration/storage mechanisms.
- Performing the requesting domain's investigation or governance decision.
- Inventing an expiry period, closure actor or event.

## 4.5 Related Scenarios

| Relationship | Scenario | Reason |
|---|---|---|
| Upstream | L3-A-026 — Retention Decision to Governed Audit Evidence Archival | Supplies immutable evidence in archived/restricted status. |
| Related | L3-A-023 — Investigation Request to Authorized Historical Evidence Outcome | Consumes restored evidence within an approved investigation scope. |

---

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Audit Infrastructure / Section A | Owns Restoration Operation, Availability Status, evidence query boundary, restoration result and event. |
| VAR:Evidence-Requesting Domain | Owns the investigation/governance need and consumes authorized read-only access. |
| Platform & Tenant / Section A | Supplies Tenant and institution isolation. |
| Person & Identity / Authentication | Supply requester identity and authenticated context. |
| Authorization & Policy / Section A | Owns Access and Restoration Policy decisions. |
| Event Infrastructure / Section A | Distributes `Evidence Restoration Completed`. |

The frozen Participating Domains value remains exactly `A, VAR:Evidence-Requesting Domain`.

---

# 6. Business Actors, Roles and Responsibilities

L2-A07 requires an authorized restoration requester but defines no universal Requestor, Restoration Approver or Restoration Closure actor. The requesting domain is a participant rather than a substitute actor; none is invented.

---

# 7. Preconditions and Entry Criteria

- Immutable Audit Evidence exists in `Archived Storage` with `Restricted Access`.
- An investigation or governance activity has a legitimate purpose and explicit evidence scope.
- Requester identity, authentication, Tenant, institution and classification are available.
- Access and Restoration Policy decisions can be obtained.
- The request does not seek evidence modification or Storage Status change.

---

# 8. Trigger

An authorized investigation or governance activity requires access to audit evidence held in archived storage.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the restoration request, purpose and evidence scope. | VAR:Evidence-Requesting Domain; canonical requestor undefined | A governed restoration request begins. |
| 2 | Validate requester identity, authentication, Tenant, institution, classification and purpose. | Audit Infrastructure; A01–A04 providers | The access boundary is established. |
| 3 | Obtain the Access and Restoration Policy decision for the exact scope. | Authorization & Policy | Restoration is authorized or denied. |
| 4 | Locate the archived evidence and confirm immutable identity, archived Storage Status and current Restricted Access. | Audit Query Service; Audit Infrastructure | Eligible archived evidence is resolved. |
| 5 | Complete the authorized Restoration Operation and transition `Restricted Access → Restored Access`. | Audit Infrastructure | Evidence becomes temporarily accessible without Storage Status change. |
| 6 | Record the Restoration Result and publish `Evidence Restoration Completed`. | Audit Infrastructure | Restoration availability becomes traceable. |
| 7 | Make the immutable evidence available read-only to the authorized requesting activity. | Audit Infrastructure; VAR:Evidence-Requesting Domain | Authorized use may proceed within scope. |
| 8 | On expiry or governed closure, end restored access and transition `Restored Access → Restricted Access`. | Closure authority and rule undefined; Audit Infrastructure owns status | Temporary availability ends. |
| 9 | Preserve request, authorization, access, expiry/closure and final restricted outcome. | Audit Infrastructure | The frozen outcome is fully traceable. |

Step 8 cannot be finalized because L2-A07 names the transition trigger `Restoration Closure` but defines no rule, workflow, authority, timing contract or event for it.

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Path | Outcome | Owner |
|---|---|---|---|---|
| Investigation use | Authorized scope belongs to A023. | Restore only the referenced evidence and hand access to that investigation. | Evidence remains archived and read-only. | Audit Infrastructure |
| Governance use | Authorized internal governance activity requires evidence. | Apply the same restoration and closure controls. | Same temporary restored-access outcome. | Audit Infrastructure |
| Partial scope authorization | Policy approves fewer evidence references than requested. | Restore only the approved subset and return the governed scope result. | Unauthorized evidence remains restricted. | Authorization & Policy; Audit Infrastructure |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Resolution | Owner |
|---|---|---|---|
| Requester, purpose or scope is invalid. | Restoration cannot be governed. | Reject without changing Availability Status. | Audit Infrastructure; Authorization & Policy |
| Tenant/institution boundary is invalid. | Isolation would be violated. | Deny restoration. | Platform & Tenant; Authorization & Policy |
| Evidence is not archived/restricted. | Restoration precondition is absent. | Return the current governed availability; do not force a transition. | Audit Infrastructure |
| Evidence cannot be located or integrity is uncertain. | Trustworthy access cannot be supplied. | Do not restore; obtain verification/governance resolution. | Audit Infrastructure |
| Authorization is denied or expires before use. | Access is prohibited. | Keep/return Availability Status to `Restricted Access`. | Authorization & Policy; Audit Infrastructure |
| Restoration closure cannot be determined. | Temporary access could remain open indefinitely. | The scenario cannot complete canonically; preserve the unresolved outcome and escalate the upstream gap. | Audit Infrastructure |

No technical restoration mechanism is defined.

---

# 12. Business Rules and Decision Constraints

| Rule | Owner | Effect | Source |
|---|---|---|---|
| Restoration requires authorization and governed purpose/scope. | Authorization & Policy | Unauthorized archived access is prohibited. | L2-A07 §§7.7, 28; A07-W13 |
| Restoration changes Availability Status only. | Audit Infrastructure | Storage remains `Archived Storage`; evidence is unchanged. | L2-A07 §§21.8, 22.1 |
| Restored access is read-only and Tenant/institution isolated. | Audit Infrastructure; Platform & Tenant | Requester cannot modify or cross boundaries. | L2-A07 security principles |
| Closure returns `Restored Access → Restricted Access`. | Audit Infrastructure | Temporary access must not become permanent. | L2-A07 lifecycle matrix |
| Every restoration transition produces additional audit evidence. | Audit Infrastructure | Request, access and closure remain accountable. | L2-A07 §22.2 |

---

# 13. State and Lifecycle Impact

| Concept | Entry | Impact | Result | Owner |
|---|---|---|---|---|
| Audit Evidence | Created Once; immutable | Read-only access only. | Unchanged. | Audit Infrastructure |
| Storage Status | `Archived Storage` | No transition. | Remains `Archived Storage`. | Audit Infrastructure |
| Availability Status | `Restricted Access` | Authorized Restoration. | `Restored Access` | Audit Infrastructure |
| Availability Status | `Restored Access` | Restoration Closure. | `Restricted Access` | Audit Infrastructure |
| Governance Status | Existing governed value | No transition. | Unchanged. | Audit Infrastructure |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Events

| Event | Publisher | Condition | Effect |
|---|---|---|---|
| `Evidence Restoration Completed` | Audit Infrastructure | Authorized restoration succeeds and evidence reaches `Restored Access`. | Requesting/governance consumers learn temporary availability. |

L2-A07 defines no canonical event for restoration expiry or closure.

## 14.2 Contracts

| Provider | Consumer | Hand-off |
|---|---|---|
| VAR:Evidence-Requesting Domain | Audit Infrastructure | Purpose, requested evidence scope and requester context. |
| Authorization & Policy | Audit Infrastructure | Access/Restoration Policy decision and approved scope. |
| Audit Infrastructure | Authorized requesting activity | Read-only restored evidence access and Restoration Result. |
| Audit Infrastructure | Audit/event consumers | Restoration and intended closure evidence. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

Restoration is limited to authenticated requester, approved purpose, Tenant/institution, classification, evidence scope and temporary access conditions.

## 15.2 Approval and Separation of Duty

Access and Restoration Policy supply approval. L2-A07 defines no universal approver, closure authority, SoD or override role.

## 15.3 Audit Obligations

Requester, purpose, scope, Tenant/institution, classification, authorization, evidence references, prior/new statuses, restoration time, access activity, expiry/closure and final status must remain traceable.

---

# 16. Completion Criteria and Postconditions

The intended completion criteria are authorized restoration scope, immutable archived evidence resolution, `Restricted Access → Restored Access`, read-only availability, publication of `Evidence Restoration Completed`, governed expiry/closure, `Restored Access → Restricted Access`, and complete traceability.

These criteria cannot be validated fully until the expiry and Restoration Closure authority/workflow/event are defined.

Postconditions: evidence remains immutable in Archived Storage; Governance Status is unchanged; access returns to Restricted Access; the requesting domain owns only its investigation/governance need.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Impact | Required Resolution |
|---|---|---|
| **A027-BLOCKER-01:** The frozen outcome requires temporary restored access with traceable expiry, but L2-A07 defines no expiry basis, duration, policy attribute or timing rule. | The end of temporary access cannot be determined without invention. | Define the canonical expiry/closure rule and required policy inputs. |
| **A027-BLOCKER-02:** L2-A07 names `Restoration Closure` as the trigger for `Restored Access → Restricted Access` but defines no closure workflow, command, authority or completion event. | The scenario cannot complete its required final restricted-access outcome. | Add the closure workflow/contract, authorized actor or policy trigger, and event publisher where required. |
| **A027-ISSUE-01:** L2-A07 does not define universal restoration requestor or approving roles. | Later RBAC cannot map exact human responsibilities. | Define role-to-request/approval authority where manual restoration is supported. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
