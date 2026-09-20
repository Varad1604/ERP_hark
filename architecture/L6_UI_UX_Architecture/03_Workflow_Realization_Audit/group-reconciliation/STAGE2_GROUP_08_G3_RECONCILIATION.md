# SutraERP L6 Stage 2 — Group 8 Section G3 Reconciliation

**Status:** COMPLETE-FOR-MASTER-CROSS-GROUP-RECONCILIATION  
**Scope:** Section G — Campus Operations G3 (`L3-G-155 … L3-G-251`)  
**Scenario population:** 97  
**Architecture Level:** L6 — UI / UX Architecture Support  
**Branch:** `l6-stage2-full-scenario-realization-audit`  
**Date:** 2026-09-20

> `G8-WF-###` identifiers are temporary Stage-2 normalization keys only. They are not business-architecture IDs and must not appear in ordinary product UX.

## 1. Reconciliation Result

```text
Assigned canonical scenarios       97
Reconciled scenario rows           97 / 97
Unique scenario IDs                97
Missing scenarios                  0
Duplicate scenario assignments     0
Cross-workspace workflow merges    0
Material open issues               0
Distinct normalized workflows      46
User-addressable workflows         46
Tracking / non-mock workflows       0
Upstream-blocked workflows          0
```

Group 8 is fully reconciled. Worker G3 was mechanically complete but materially under-classified operational healthcare, security and housekeeping work as background/system-owned behavior.

## 2. Canonical Population and Ownership Check

The canonical range `L3-G-155 … L3-G-251` contains exactly 97 scenario identities with no gaps or duplicates and aligns to the locked 1,084-scenario ID baseline.

The range resolves to exactly three canonical L2 owners/workspaces:

- `L3-G-155 … L3-G-185` → L2-G06 Institutional Health Services / Medical Workflows → `W-G06`
- `L3-G-186 … L3-G-215` → L2-G07 Security Operations → `W-G07`
- `L3-G-216 … L3-G-251` → L2-G08 Housekeeping Operations & Service Governance → `W-G08`

No normalized workflow crosses these workspace boundaries.

## 3. Worker-Result Audit and Semantic Reconciliation

Worker G3 contained 97 / 97 rows, zero missing IDs and zero duplicate IDs. However, **77 / 97 rows required a disposition and/or mock-interaction correction** after exact L3 review and verification against Locked L2 plus representative/exact L4 behavior.

### 3.1 Institutional Health Services — G06

Evidence-backed corrections include:

- Health Centre configuration/activation and temporary closure expose Health Services operational authority, validation and authorization; they are not background-only.
- Patient-registration exceptions require Medical Staff to resolve identification using source-owned identity facts.
- Walk-in consultation, follow-up, emergency/first-aid response, external referral, ambulance coordination, prescription, medicine dispensing, observation, referral follow-up and programme activity all expose authorised medical professionals and/or Medical Staff performing governed work.
- Medicine inventory threshold/expiry handling is not merely an automated alert: healthcare staff assess medical-operational impact and establish the governed action/alert.
- Medical-event Student/Employee Portal outcomes require G06 to determine the permitted medical information and hand it to the appropriate downstream portal context while retaining medical-data ownership.
- Healthcare operational reporting includes preparation/review and a governance outcome.
- Medical clearance is a governed decision.
- G06 continues to own clinical/operational healthcare facts; C/E/H/F and external providers retain their respective ownership.

### 3.2 Security Operations — G07

Evidence-backed corrections include:

- Security Office configuration/activation/closure, Gate configuration/open/close and person-access override all expose Security Operations execution and supervisory decision roles.
- Visitor, vehicle, courier and contractor movement flows require operational verification and closure.
- Patrol execution, shift handover, incident response, emergency response, missing-person response and fire/safety response all expose explicit security actors, evidence and supervised decisions.
- The medical-security scenario is an owner-preserving G07 → G06 hand-off: Security coordinates the security-side response; G06 retains medical-care ownership.
- Lost & Found and Escort journeys require operational intake, verification, claim/handover or alternative-response decisions.
- Security Policy configuration, temporary delegation and governed reporting contain explicit review/decision responsibilities.

### 3.3 Housekeeping Operations — G08

Evidence-backed corrections include:

- Zone configuration/change/deactivation, schedule planning, task execution, deep/event/emergency/special cleaning, complaint/re-clean/SLA escalation, hygiene inspection, consumable handling, quality inspection, incident handling, waste operations, vendor-service governance and reporting all expose named Housekeeping actors and governed work.
- Housekeeping consumable monitoring is not system-only: Housekeeping Supervisor/Manager assess the condition and establish the operational alert/outcome.
- G235 remains an owner-preserving G08 → G01 maintenance hand-off. G08 initiates the governed maintenance hand-off; G01 retains maintenance execution.
- Vendor operational performance and SLA/service-quality outcomes remain G08 operational governance; vendor-contract/procurement/financial ownership remains outside G08.

## 4. Current Upstream Decisions Consumed

### 4.1 G200 / G212 / G214 controlled authority amendment

The repository contains the 2026-09-01 controlled authority amendment for:

- `L3-G-200`
- `L3-G-212`
- `L3-G-214`

Group 8 consumes the corrected Locked v1.1 successor content:

- G200: Complainant/Witness remains evidence provider; Investigating Security Officer retains supervisory authority.
- G212: Escort Requestor requests/accepts the outcome; Security Dispatch Supervisor retains supervisory authorization.
- G214: Chief Security Officer (Grantor) owns the business delegation decision; Platform administration does not acquire Security business authority.

The former parent-authority contradictions are already resolved and do not block Stage 2.

### 4.2 Shared A01 configuration governance

The 2026-09-20 A01 configuration-governance completion is consumed where the G3 scenarios rely on the generic business-domain configuration framework.

- `L3-G-183` Medical Configuration Change explicitly has no dedicated G06 generic configuration lifecycle/event and therefore consumes the owner-scoped A01 lifecycle, Configuration Approval Authority and generic framework events while G06 retains healthcare-policy semantics.
- `L3-G-249` Housekeeping Policy Configuration likewise consumes the generic A01 governance framework while G08 retains Housekeeping policy semantics.
- `L3-G-213` Security Policy Configuration already defines a G07-owned semantic policy lifecycle and `Security_Policy_Activated`; that domain-specific lifecycle/event remains authoritative and is not replaced by a duplicate A01 semantic event.
- `L3-G-217` Housekeeping Zone Configuration Change uses the explicit G08 Housekeeping Zone lifecycle and `HousekeepingZoneUpdated`; it is not rewritten as a generic configuration aggregate.

A01 remains framework owner, not owner of G06/G07/G08 business semantics.

## 5. Scenario-Level Reconciled Distribution

| Disposition | Rows |
|---|---:|
| INTERACTIVE-WORKFLOW | 37 |
| REVIEW-APPROVAL-WORKFLOW | 33 |
| EXCEPTION-RECOVERY-WORKFLOW | 23 |
| CROSS-WORKSPACE-HANDOFF | 4 |
| **Total** | **97** |

| Mock interaction | Rows |
|---|---:|
| REQUIRED | 97 |
| NOT-REQUIRED | 0 |
| PROHIBITED | 0 |
| **Total** | **97** |

This result is intentional. The current Locked L3 corpus assigns human assessment, performance, review, decision, verification or hand-off responsibility even to several scenarios whose titles initially looked like monitoring/projection outcomes.

## 6. Workflow Normalization Method

Scenarios were merged only when they share the same primary workspace, canonical owner, aggregate/lifecycle context and sustained institutional job.

Normalization specifically avoids:

- merging G06 healthcare work with Security or Housekeeping because all are Section G;
- turning medical, security or housekeeping operational decisions into fake system automation;
- moving external healthcare-provider activity into G06;
- moving Facilities maintenance execution into G08;
- moving portal-service ownership into G06;
- moving Security business authority into Platform Foundation;
- moving vendor/procurement/financial ownership into Housekeeping;
- replacing domain-specific configuration lifecycles/events with generic A01 duplicates; and
- treating scenario titles as workflow identity.

## 7. Normalized Workflow Population

| Working workflow key | Workspace | Workflow family | Scenarios | Canonical scenario IDs | Workflow realization | Mock |
|---|---|---|---:|---|---|---|
| G8-WF-001 | W-G06 | Health Centre configuration, activation and temporary-closure lifecycle | 2 | L3-G-155; L3-G-156 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G8-WF-002 | W-G06 | Patient/medical-encounter registration, identity exception, readiness, consultation and follow-up | 5 | L3-G-157; L3-G-158; L3-G-159; L3-G-160; L3-G-161 | INTERACTIVE-WORKFLOW | REQUIRED |
| G8-WF-003 | W-G06 | Emergency/first-aid response, external referral and ambulance coordination | 3 | L3-G-162; L3-G-163; L3-G-164 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G8-WF-004 | W-G06 | Prescription creation | 1 | L3-G-165 | INTERACTIVE-WORKFLOW | REQUIRED |
| G8-WF-005 | W-G06 | Medicine dispensing and availability-exception resolution | 2 | L3-G-166; L3-G-167 | INTERACTIVE-WORKFLOW | REQUIRED |
| G8-WF-006 | W-G06 | Medicine inventory condition assessment and medical-operations alert | 1 | L3-G-168 | INTERACTIVE-WORKFLOW | REQUIRED |
| G8-WF-007 | W-G06 | Observation-stay admission/completion and capacity-exception resolution | 2 | L3-G-169; L3-G-170 | INTERACTIVE-WORKFLOW | REQUIRED |
| G8-WF-008 | W-G06 | Medical referral creation/completion and institutional follow-up | 2 | L3-G-171; L3-G-172 | INTERACTIVE-WORKFLOW | REQUIRED |
| G8-WF-009 | W-G06 | Medical certificate issuance and revocation | 2 | L3-G-173; L3-G-174 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G8-WF-010 | W-G06 | Health programme planning/completion and screening/camp follow-up | 2 | L3-G-175; L3-G-178 | INTERACTIVE-WORKFLOW | REQUIRED |
| G8-WF-011 | W-G06 | Vaccination campaign and recorded vaccination outcome | 1 | L3-G-176 | INTERACTIVE-WORKFLOW | REQUIRED |
| G8-WF-012 | W-G06 | Annual health-checkup assessment | 1 | L3-G-177 | INTERACTIVE-WORKFLOW | REQUIRED |
| G8-WF-013 | W-G06 | Occupational-health assessment and medical outcome | 1 | L3-G-179 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G8-WF-014 | W-G06 | Medical-event disclosure decision and Student/Employee Portal hand-off | 2 | L3-G-180; L3-G-181 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| G8-WF-015 | W-G06 | Medical-professional unavailability and alternative operational assignment | 1 | L3-G-182 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G8-WF-016 | W-G06 | Medical operational-configuration governance | 1 | L3-G-183 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G8-WF-017 | W-G06 | Healthcare operational reporting and governance review | 1 | L3-G-184 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G8-WF-018 | W-G06 | Medical clearance decision | 1 | L3-G-185 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G8-WF-019 | W-G07 | Security Office activation and temporary-closure lifecycle | 2 | L3-G-186; L3-G-187 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G8-WF-020 | W-G07 | Gate configuration/open/close and person-access decision/override | 4 | L3-G-188; L3-G-189; L3-G-190; L3-G-191 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G8-WF-021 | W-G07 | Visitor/courier/contractor access, check-in, closure and exit | 4 | L3-G-192; L3-G-193; L3-G-196; L3-G-197 | INTERACTIVE-WORKFLOW | REQUIRED |
| G8-WF-022 | W-G07 | Vehicle campus-movement lifecycle | 1 | L3-G-194 | INTERACTIVE-WORKFLOW | REQUIRED |
| G8-WF-023 | W-G07 | Material gate-pass request, decision and movement | 1 | L3-G-195 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G8-WF-024 | W-G07 | Patrol execution and exception escalation | 1 | L3-G-198 | INTERACTIVE-WORKFLOW | REQUIRED |
| G8-WF-025 | W-G07 | Security shift handover and continuity | 1 | L3-G-199 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G8-WF-026 | W-G07 | Security incident, emergency, missing-person and fire/safety response | 4 | L3-G-200; L3-G-201; L3-G-203; L3-G-204 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G8-WF-027 | W-G07 | Medical-security incident hand-off to G06 | 1 | L3-G-202 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| G8-WF-028 | W-G07 | Lost & Found report, registration, claim, handover and unclaimed disposition | 5 | L3-G-205; L3-G-206; L3-G-207; L3-G-208; L3-G-209 | INTERACTIVE-WORKFLOW | REQUIRED |
| G8-WF-029 | W-G07 | Escort request, assignment, completion and exception response | 3 | L3-G-210; L3-G-211; L3-G-212 | INTERACTIVE-WORKFLOW | REQUIRED |
| G8-WF-030 | W-G07 | Security policy configuration governance | 1 | L3-G-213 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G8-WF-031 | W-G07 | Security delegation to temporary operational authority | 1 | L3-G-214 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G8-WF-032 | W-G07 | Security operational reporting and dashboard governance | 1 | L3-G-215 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G8-WF-033 | W-G08 | Housekeeping Zone configuration, change and deactivation lifecycle | 3 | L3-G-216; L3-G-217; L3-G-218 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G8-WF-034 | W-G08 | Cleaning schedule, assignment and recurring/deep/event/emergency/special task execution | 7 | L3-G-219; L3-G-220; L3-G-221; L3-G-223; L3-G-224; L3-G-225; L3-G-226 | INTERACTIVE-WORKFLOW | REQUIRED |
| G8-WF-035 | W-G08 | Cleaning complaint, repeat-corrective and request-SLA escalation lifecycle | 4 | L3-G-227; L3-G-228; L3-G-229; L3-G-230 | INTERACTIVE-WORKFLOW | REQUIRED |
| G8-WF-036 | W-G08 | Hygiene inspection and compliance/corrective outcome | 1 | L3-G-231 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G8-WF-037 | W-G08 | Consumable shortage/availability monitoring, alert and continuity recovery | 3 | L3-G-232; L3-G-241; L3-G-242 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G8-WF-038 | W-G08 | Quality inspection, re-clean and cleanliness-improvement governance | 4 | L3-G-222; L3-G-233; L3-G-247; L3-G-248 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G8-WF-039 | W-G08 | Housekeeping incident resolution and escalation | 2 | L3-G-234; L3-G-236 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G8-WF-040 | W-G08 | Facility issue to G01 maintenance hand-off | 1 | L3-G-235 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| G8-WF-041 | W-G08 | Waste collection, exception, segregation and specialised external-handling coordination | 4 | L3-G-237; L3-G-238; L3-G-239; L3-G-240 | INTERACTIVE-WORKFLOW | REQUIRED |
| G8-WF-042 | W-G08 | Vendor operational assignment and service-performance outcome | 2 | L3-G-243; L3-G-244 | INTERACTIVE-WORKFLOW | REQUIRED |
| G8-WF-043 | W-G08 | Housekeeping SLA monitoring and vendor-breach corrective governance | 2 | L3-G-245; L3-G-246 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G8-WF-044 | W-G08 | Housekeeping policy configuration governance | 1 | L3-G-249 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G8-WF-045 | W-G08 | Housekeeping delegation to temporary operational authority | 1 | L3-G-250 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G8-WF-046 | W-G08 | Housekeeping operational reporting and dashboard governance | 1 | L3-G-251 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |

## 8. Workflow-Level Counts

| Workflow realization | Distinct workflows |
|---|---:|
| INTERACTIVE-WORKFLOW | 18 |
| REVIEW-APPROVAL-WORKFLOW | 20 |
| EXCEPTION-RECOVERY-WORKFLOW | 5 |
| CROSS-WORKSPACE-HANDOFF | 3 |
| **Total** | **46** |

| Workflow mock realization | Distinct workflows |
|---|---:|
| REQUIRED | 46 |
| NOT-REQUIRED | 0 |
| PROHIBITED | 0 |
| **Total** | **46** |

## 9. Cross-Domain / Workspace Observations

- G06 owns healthcare operations and clinical/medical outcomes; C/E/H/F/external providers retain source ownership of their facts/services.
- G180/G181 expose a governed G06 disclosure decision and owner-preserving hand-off to H portal contexts; H does not acquire medical-record ownership.
- G202 exposes G07 security coordination and a hand-off to G06; G07 does not perform medical care.
- G235 exposes G08 maintenance-request initiation/evidence and G01-owned maintenance execution.
- G08 may reference vendors and supplier evidence operationally; contract/procurement/inventory/financial ownership remains with the applicable canonical J/F owners.
- G213 and G217 retain explicit domain lifecycle/event semantics; shared A01 governance is not used to erase those semantic aggregates.
- No workflow family is merged across W-G06, W-G07 and W-G08.

## 10. Canonical Identity Check

```text
Expected first ID        L3-G-155
Expected last ID         L3-G-251
Expected count           97
Observed scenario rows   97
Unique primary IDs       97
Missing IDs              0
Duplicate primary IDs    0
Primary workflow count   46
Scenario assignments     97 / 97 exactly once
Workspace set            W-G06, W-G07, W-G08
```

## 11. Group Exit Gate

| Gate | Result |
|---|---|
| G3 canonical scenarios = 97 | PASS |
| Reconciled realization rows = 97 | PASS |
| Unique IDs = 97 | PASS |
| Missing IDs = 0 | PASS |
| Duplicate scenario assignment = 0 | PASS |
| Every scenario assigned to one normalized workflow | PASS |
| Cross-workspace normalized merge = 0 | PASS |
| Distinct workflows discovered | PASS — 46 |
| User-addressable workflows discovered | PASS — 46 |
| Mock-required workflow population discovered | PASS — 46 |
| Material upstream blockers = 0 | PASS |
| G200/G212/G214 corrected authority consumed | PASS |
| A01 configuration governance consumed without ownership transfer | PASS |
| Domain-specific G07/G08 lifecycle/events preserved | PASS |
| No invented actor/lifecycle/event/publisher | PASS |
| No implementation leakage | PASS |

**GROUP 8 VERDICT:** **PASS — COMPLETE FOR MASTER CROSS-GROUP RECONCILIATION**

Stage 2 may continue to Group 9. The canonical Workflow Realization Register remains intentionally unpopulated until Groups 1–14 and master cross-group reconciliation complete.
