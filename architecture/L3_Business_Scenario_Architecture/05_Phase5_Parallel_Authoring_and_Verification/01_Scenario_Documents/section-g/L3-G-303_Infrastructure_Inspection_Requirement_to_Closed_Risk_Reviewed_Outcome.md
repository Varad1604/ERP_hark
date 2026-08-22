# Infrastructure Inspection Requirement to Closed Risk-Reviewed Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-G-303  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-21  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-G01 — Facilities Management Workflows; L2-G00 — Campus Operations Review & Governance; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Authorized consumers of Infrastructure inspection, condition, compliance, risk and remediation facts

# 2. Scenario Purpose

Govern an Infrastructure inspection from requirement through closure so that the authoritative G10 Inspection Record is established against the applicable physical Infrastructure, the authorized inspection is completed, findings are preserved, the relevant Infrastructure condition is reconciled, and any significant condition is placed onto a source-supported risk or remediation path before closure.

# 3. Business Outcome

An Inspection Record is created against the applicable physical infrastructure, the authorised inspection is completed with findings recorded and the applicable infrastructure condition updated, and any significant condition is recorded with an authorised infrastructure risk or remediation path initiated before the inspection is closed

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an infrastructure inspection becomes due or is authorised.

## 4.2 End Boundary

The scenario ends when the G10 Inspection Record has progressed through the source-defined inspection workflow to closure, inspection findings and relevant Infrastructure condition are preserved, and any significant finding has entered an authorized source-supported risk or remediation path.

## 4.3 In Scope

- Establishment of an Infrastructure-owned Inspection Record against authoritative physical Infrastructure.
- Authorization of the inspection business operation.
- Inspection scheduling, assignment, conduct, finding recording, approval and closure under the G10 inspection workflow.
- Classification of findings using source-defined Infrastructure inspection finding classifications.
- Preservation of inspection history and the latest applicable Infrastructure condition assessment.
- Initiation of a G10 corrective-maintenance remediation requirement where an inspection finding requires corrective maintenance.
- Referral to G00 operational-risk governance where a finding qualifies as a multi-domain Operational Risk and the authoritative governance path is applicable.
- Publication of source-defined Infrastructure inspection and finding events.

## 4.4 Out of Scope

- Facilities-owned operational maintenance execution, Work Order lifecycle, technician execution or vendor execution.
- Creation of a new Infrastructure risk aggregate inside G10.
- Redefinition of G00 Operational Risk ownership.
- Creation of new inspection states, finding classifications, events or approval authorities.
- Automatic Infrastructure closure, availability restriction or compliance-state change unless a separately governed source-defined outcome requires it.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-G-305 — Corrective Maintenance Governance to Infrastructure Remediation Requirement | A source-defined remediation path may be initiated when inspection findings require corrective action. |
| Related | L3-G-302 — Compliance Failure to Controlled Infrastructure Restriction | A compliance-related finding may require a separately governed compliance restriction or review outcome. |
| Related | L3-G-307 — Maintenance Completion to Infrastructure Condition Reconciliation | Completed remediation may later require G10 condition reconciliation. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns physical Infrastructure, the G10 Inspection Record, Infrastructure inspection governance, findings recorded under that inspection, current Infrastructure condition facts and any G10 remediation requirement. |
| Campus Operations Review & Governance / Section G | Owns the Operational Risk aggregate where a significant Infrastructure finding is accepted into the authoritative multi-domain operational-risk path; it does not own the source Infrastructure or Inspection Record. |
| Facilities Management / Section G | Owns operational maintenance execution if a remediation requirement subsequently becomes maintenance work; it does not own physical Infrastructure. |
| Platform Foundation / Section A | Owns authorization decisions, event-governance capability and immutable audit evidence; it does not own Infrastructure inspection semantics or findings. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Administers the Infrastructure inspection under authorized scope. | May initiate or progress G10 inspection governance where authorized. |
| Compliance Officer | Participates where the inspection concerns Infrastructure compliance or safety requirements. | May review compliance-related findings within authorized responsibility; no universal approval authority is invented. |
| Maintenance Coordinator | Participates where a finding requires a G10-governed maintenance requirement. | Coordinates Infrastructure maintenance governance without taking ownership of G01 execution. |
| Campus Administrator | Participates for Infrastructure within the governed campus scope. | May administer campus-scoped Infrastructure inspection actions where authorized. |
| Auditor | Reviews preserved inspection history and evidence. | Read-only participation; does not alter Infrastructure or inspection facts. |

L2-G10 uses the term `Inspector` in its inspection workflow and inspection-audit obligations, but does not define a separate canonical Inspector role or universal authority model. This Draft therefore does not invent one.

# 7. Preconditions and Entry Criteria

- The applicable physical Infrastructure exists under G10 ownership.
- The inspection requirement identifies the Infrastructure and the applicable source-defined or configured inspection type.
- The inspection business operation is authorized before execution.
- Applicable inspection criteria or governance information required for the inspection are available.

# 8. Trigger

An infrastructure inspection becomes due or is authorised

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the authoritative physical Infrastructure and the inspection requirement. | Infrastructure Management | The exact Infrastructure target and inspection context are established. |
| 2 | Confirm that the Infrastructure inspection is authorized for the applicable scope. | Infrastructure Management using the canonical authorization decision owned by Section A | The inspection may proceed. |
| 3 | Establish the G10 Inspection Record against the applicable Infrastructure and schedule the inspection under the source-defined workflow. | Infrastructure Management | The inspection is governed as an Infrastructure-owned inspection record. |
| 4 | Assign the inspection responsibility and conduct the authorized inspection. | Infrastructure Management | The source-defined inspection activity is completed against the applicable Infrastructure. |
| 5 | Record the inspection findings and classify them using the source-defined finding classifications. | Infrastructure Management | Observations, issues, risks, violations or recommendations are preserved without inventing new classifications. |
| 6 | Reconcile the latest applicable Infrastructure condition or health assessment from the completed inspection findings. | Infrastructure Management | The authoritative current Infrastructure condition reflects the supported inspection result. |
| 7 | Where a finding is significant, initiate the applicable source-supported remediation path or refer the finding to the authoritative Operational Risk governance path before closure. | Infrastructure Management; Campus Operations Review & Governance where an Operational Risk is established | The significant condition is not silently closed without a governed follow-up path. |
| 8 | Approve and close the inspection in accordance with the G10 inspection workflow after required significant-condition follow-up has been initiated. | Infrastructure Management | The Inspection Record reaches the source-defined `Closed` lifecycle position with history preserved. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| No significant finding | Inspection findings contain only observations or other outcomes that do not require a risk/remediation path under applicable policy. | Complete finding recording, condition reconciliation, approval and closure without inventing a remediation requirement. | Inspection closes with supported findings and history preserved. | Infrastructure Management |
| Corrective remediation required | A finding requires corrective maintenance. | Establish the G10 corrective-maintenance governance requirement and publish the source-defined corrective-maintenance request fact; Facilities remains responsible for operational execution. | Remediation is initiated without transferring Infrastructure ownership to Facilities. | Infrastructure Management; Facilities Management executes later work. |
| Multi-domain operational risk | A significant finding meets the authoritative G00 meaning of Operational Risk. | Refer the source-domain finding to G00 for risk-governance ownership while preserving the original G10 Inspection Record and Infrastructure fact. | G00 may establish and govern its Operational Risk without modifying G10 source data. | Campus Operations Review & Governance |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Authorization denies the inspection | The inspection does not proceed. | A later inspection may proceed only after a valid authorization decision; no inspection completion is asserted. | Platform Foundation / Authorization; Infrastructure Management retains the requirement. |
| Required Infrastructure reference is invalid or unavailable | The inspection cannot be reliably associated with authoritative physical Infrastructure. | Resolve the G10 Infrastructure reference before the inspection is progressed. | Infrastructure Management |
| Significant finding has no resolved authoritative risk/remediation route | Inspection closure cannot safely assert the frozen outcome. | Preserve the finding and hold closure until an upstream-authorized source-supported path is identified; do not invent a risk owner or contract. | Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Infrastructure Management owns physical Infrastructure and Inspection Records; inspection records remain permanently associated with Infrastructure. | Infrastructure Management | The Inspection Record remains a G10 fact and cannot be re-owned by a consuming domain. | L2-G10 §§8, 32 |
| Infrastructure inspection verifies condition, safety, operational readiness and compliance. | Infrastructure Management | The inspection remains an Infrastructure-governance outcome rather than generic Facilities work. | L2-G10 §32 Inspection Management Workflows |
| G10 inspection workflow is Inspection Scheduled → Inspector Assigned → Inspection Conducted → Findings Recorded → Inspection Approved → Inspection Closed. | Infrastructure Management | The scenario follows the source-defined inspection progression and does not invent states. | L2-G10 §32 Inspection Workflow |
| Finding classifications include Observation, Minor Issue, Major Issue, Critical Risk, Compliance Violation and Recommendation. | Infrastructure Management | Findings use source-defined classifications only. | L2-G10 §32 Inspection Findings |
| Inspection history shall never be deleted. | Infrastructure Management | Closure preserves inspector, date, findings, recommendations, supporting documents and approval history. | L2-G10 §32 Inspection Audit |
| Corrective maintenance planning belongs to Infrastructure; execution belongs to Facilities. | Infrastructure Management / Facilities Management | A remediation requirement may be initiated by G10, but G01 owns operational maintenance execution. | L2-G10 §34; L2-G01 §§1–6 |
| G00 owns the Operational Risk aggregate and never modifies source-domain operational data. | Campus Operations Review & Governance | Any risk referral transfers risk-governance ownership only, not Infrastructure or Inspection ownership. | L2-G00 §§1–8, 13–14 |
| Every business operation shall be authorized before execution. | Platform Foundation — Authorization | Unauthorized inspection actions do not proceed. | L2-A04 AP-01 |
| Events represent completed business facts and business domains retain ownership of those facts. | Infrastructure Management / Platform Foundation — Event Infrastructure | G10 inspection events remain G10 business facts even when distributed through the platform event capability. | L2-A06 EIP-01, EIP-02 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Inspection Record | Not yet established / `Scheduled` after establishment | The inspection progresses through the canonical G10 inspection lifecycle. | `Closed`, after `Scheduled → In Progress → Completed → Approved → Closed` and the corresponding workflow activities. | Infrastructure Management | L2-G10 canonical lifecycle states; §32 Inspection Workflow |
| Infrastructure condition / health | Existing current G10 assessment | The latest supported condition or health assessment is reconciled from inspection findings where applicable. | Updated to a source-supported current assessment; no new condition classification is invented. | Infrastructure Management | L2-G10 Infrastructure condition/health rules and inspection purpose |
| Operational Risk | Not established by this scenario unless the finding qualifies for G00 risk governance | A qualifying finding may be referred to G00. | Governed under G00's `Risk Identified → Assessment → Mitigation → Monitoring → Closed` lifecycle if G00 accepts it. | Campus Operations Review & Governance | L2-G00 Operational Risk lifecycle |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `InfrastructureInspectionScheduled` | Infrastructure Management | Authorized consumers of Infrastructure inspection facts | Publishes the source-defined fact that an Infrastructure inspection has been scheduled. | L2-G10 Inspection Events |
| `FindingRecorded` | Infrastructure Management | Authorized consumers of Infrastructure finding facts | Publishes the source-defined fact that an inspection finding has been recorded. | L2-G10 Inspection Events |
| `InfrastructureInspectionCompleted` | Infrastructure Management | Authorized consumers of completed Infrastructure inspection facts | Publishes the source-defined completed Infrastructure inspection fact. | L2-G10 Inspection Events |
| `CorrectiveMaintenanceRequested` | Infrastructure Management | Facilities Management where corrective maintenance is required | Publishes the source-defined G10 remediation requirement without transferring maintenance-execution ownership. | L2-G10 Maintenance Events; §34 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

The inspection remains owned by G10. Where corrective maintenance is required, G10 hands the governed remediation requirement to G01 while G01 owns Work Orders and operational execution. Where a finding qualifies as a multi-domain Operational Risk, G00 owns any resulting Operational Risk while preserving the G10 source finding. L2-G10 and L2-G00 do not define an exact named synchronous contract for the risk referral; this is recorded in Section 17 rather than invented.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate or progress an Infrastructure inspection | Infrastructure Administrator, Campus Administrator, Compliance Officer or other source-supported G10 responsibility within authorized scope | The action proceeds only when the canonical authorization decision and applicable inspection policy permit it. | Infrastructure Management |
| Establish a corrective-maintenance remediation requirement from a finding | Maintenance Coordinator or other authorized G10 role within applicable scope | The requirement is governed by G10; operational execution remains with G01. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

L2-G10 includes `Inspection Approved` in the inspection workflow and requires preservation of Approval History, but does not define one universal approver, maker-checker pattern or separation-of-duty rule for every Infrastructure inspection type. This Draft does not invent one.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve inspection history | Inspector attribution, inspection date, findings, recommendations, supporting documents and approval history associated with the G10 Inspection Record. | Infrastructure Management |
| Preserve immutable evidence of completed business facts | Audit evidence generated from canonical business events with available actor and context information. | Platform Foundation — Audit Infrastructure |
| Preserve source-domain ownership during risk/remediation follow-up | G10 Inspection Record and finding remain intact while G00 risk or G01 execution records retain their own ownership. | Respective canonical owners |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The G10 Inspection Record exists against authoritative physical Infrastructure.
- The authorized inspection has been conducted and findings recorded.
- The applicable Infrastructure condition or health assessment has been reconciled where supported.
- Any significant finding has an authorized source-supported risk or remediation path initiated before closure.
- The inspection reaches the source-defined `Closed` lifecycle position with history preserved.

## 16.2 Postconditions

- Infrastructure Management retains ownership of the closed Inspection Record, findings and physical Infrastructure facts.
- Any G01 remediation execution remains a separate Facilities-owned lifecycle.
- Any G00 Operational Risk remains a separate G00-owned governance lifecycle.
- Authorized consumers may rely on the source-defined Infrastructure inspection events and current G10 condition facts.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-G10 defines `Critical Risk` as an inspection-finding classification but does not define a G10 Infrastructure Risk aggregate or an exact named hand-off contract from an Infrastructure finding to G00 Operational Risk. | The frozen outcome's “authorised infrastructure risk or remediation path” can use the source-defined remediation route, and G00 clearly owns multi-domain Operational Risk, but the exact cross-context risk-referral contract is not canonicalized. | Upstream architecture should define the governed G10→G00 risk-referral contract if risk-only findings must be transferred without a remediation requirement. |
| G10 owns an `Inspection Record` and defines Inspection Scheduled → Inspector Assigned → Inspection Conducted → Findings Recorded → Inspection Approved → Inspection Closed. However, G10 §9A.254 also states that Maintenance owns Inspection Scheduling, and locked L2-G01 separately owns the Facilities `Inspection` aggregate, Inspection Planning/Recording and operational inspection workflows. The sources distinguish their event names (`InfrastructureInspection...` vs `FacilityInspection...`) but do not resolve the ownership contradiction. | The frozen outcome requires a G10 Inspection Record and completed inspection, but the locked L2 sources disagree on who owns inspection scheduling/operational inspection work. This materially affects ownership and scenario boundary. | Upstream Section G architecture should document the canonical distinction between G10 Infrastructure Inspection and G01 Facility Inspection. |

**Architecture assessment:** `BLOCKED`

The Draft preserves the frozen G10 scenario semantics and all unambiguous inspection rules, but the locked G10/G01 inspection-ownership contradiction materially affects the canonical boundary for inspection scheduling/execution. Upstream Section G resolution is required before this scenario can be source-complete.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
