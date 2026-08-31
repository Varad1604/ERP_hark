# Workflow Definition to Activated Orchestration Availability

## 1. Specification Metadata

**Specification ID:** L4-A-014  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Workflow Engine  
**Realizes Scenario:** L3-A-014  
**Primary Business Owner:** Workflow Engine  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** `L3-A-014_Workflow_Definition_to_Activated_Orchestration_Availability.md`; `L2-A05_Workflow_Engine_Workflows.md`; `L2-A04_Authorization_and_Policy_Workflows.md`; `L2-A06_Event_Infrastructure_Workflows.md`; `L2-A07_Audit_Infrastructure_Workflows.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall turn an approved business-process orchestration requirement into a validated, immutable, published and activated Workflow Version while preserving the Workflow-Owning Domain’s ownership of business rules, decisions, states, commands, events and outcomes. A014 ends at orchestration availability and does not start a Workflow Instance.

## 3. Required Business Context

- One Tenant and identified Workflow-Owning Domain.
- Approved business-process boundary and source-owned business semantics.
- Required workflow metadata, structure, business references, input/output expectations, policy/event references and dependencies.
- Canonical Workflow Definition/Version lifecycle.
- Required governance approval outcome.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-A-014-BH-001 | The system shall prevent definition progression when the Workflow-Owning Domain or approved business-process boundary is unresolved. | L3-A-014 §§7, 9, 11–12 |
| L4-A-014-BH-002 | When a governed definition is created, the system shall place it in `Draft` and publish `WorkflowDefinitionCreated` on behalf of the Workflow Engine. | L3-A-014 §§9, 13–14 |
| L4-A-014-BH-003 | The system shall keep business rules, decisions, states, commands and events owned by the Workflow-Owning Domain while recording only orchestration references required by the definition. | L3-A-014 §§2, 4–5, 9, 12 |
| L4-A-014-BH-004 | Before publication, the system shall perform the source-required structural, policy, event, version, dependency and compatibility validation and prevent progression when any mandatory validation fails. | L3-A-014 §§9, 11–13 |
| L4-A-014-BH-005 | The system shall prevent `Validation → Approved` until the required governance approval outcome exists. | L3-A-014 §§9, 13, 15 |
| L4-A-014-BH-006 | When approval and publication conditions succeed, the system shall transition the version to `Published`, make it immutable and publish `WorkflowVersionPublished`. | L3-A-014 §§9, 12–14 |
| L4-A-014-BH-007 | The system shall prevent activation of a version that is not `Published`. | L3-A-014 §§9, 12–13 |
| L4-A-014-BH-008 | When the published version is validly activated, the system shall transition it to `Activated`, publish `WorkflowVersionActivated`, and make its governed input/output contracts available for new instances. | L3-A-014 §§9, 13–16 |
| L4-A-014-BH-009 | The system shall not start a Workflow Instance or execute the owning domain’s business outcome as part of A014. | L3-A-014 §§4, 16 |

## 5. Cross-Domain & Event Behavior

The system shall preserve the exact Workflow Engine publisher for `WorkflowDefinitionCreated`, `WorkflowVersionPublished` and `WorkflowVersionActivated`. Event Infrastructure governs distribution without acquiring workflow ownership. The Workflow-Owning Domain supplies business semantics and consumes the activated orchestration boundary without transferring business ownership.

## 6. Failure / Rejection / Continuation

Incomplete definition information, failed mandatory validation, absent governance approval, incomplete publication or incomplete activation shall prevent the version from being treated as activated. Partial publication is prohibited. No undefined approval or activation actor is inferred.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-A-014-BH-010 | The system shall preserve evidence of owning domain, definition identity, validation results, governance approval reference, immutable published version, activation outcome and canonical lifecycle events. | L3-A-014 §15.3–16 |

## 8. Observable Completion Conditions

Completion requires a valid Workflow Definition, successful mandatory validation, required approval, immutable `Published` version, `Activated` current version for new instances, available governed input/output contracts, exact canonical workflow events, and preserved business ownership. No instance execution is included.

## 9. Traceability & Provenance

**Parent:** L3-A-014 — Workflow Definition to Activated Orchestration Availability  
**Primary verification source:** L2-A05 — Workflow Engine Workflows  
**Additional verification:** L2-A04 — Authorization & Policy; L2-A06 — Event Infrastructure; L2-A07 — Audit Infrastructure

## 10. Open Issues / Deferred Design

The parent leaves the canonical governance-approval actor, publication/activation authority and definition-lifecycle command contracts undefined. This Draft requires the authoritative outcomes but does not assign unsupported actors or commands.
