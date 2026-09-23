# Health, Alerting, Operational Diagnostics & Service Management Architecture

## 1. Specification Metadata
**Specification ID:** `L5-E-05-003`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-E — Reliability, Deployment & Operations Architecture  
**Major Area:** L5-E-05 — Observability & Operations Architecture  
**Canonical Technical Responsibility:** Health signals, alerting, diagnostics and service-operations integration.  
**Technical Architecture Owner:** L5-E-05 — Observability & Operations Architecture  
**Assigned Author:** Pallav  
**Version:** `1.1.0`  
**Status:** `Architecture Approved`  
**Last Updated:** `2026-09-23`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; Candidate Document Corpus Register 105; `L5-E-05-001`; `L5-E-05-002`; `L5-E-04-*`; `L5-E-01-*`; `L5-E-02-*`; `L5-C-02-*`; `L5-D-02-*`; `L5-D-03-*`; `L5-D-04-*`.  
**Referenced By:** `L5-E-05-004`, operational support/readiness consumers and service-management integrations.  
**Applicable ADRs:** `ADR-L5-0014`; `ADR-L5-0029`; `ADR-L5-0040`; `ADR-L5-0044`; `ADR-L5-0045`.

## 2. Purpose, Scope & Non-Goals
Defines the L5 technical architecture for service health semantics, alert evaluation, incident context, safe diagnostics, runbook/service-catalog linkage and bounded automated remediation.

It does not own business recovery policy, business-state correction, deployment internals, staffing/escalation policy, audit truth or vendor products.

## 3. Upstream Applicability, Constraints & Assumptions
`L5-E-05-001/002` provide telemetry/correlation. `L5-E-04-*` provides runtime/deployment identity. `L5-E-01-*` owns resilience/failure-isolation principles. No direct L4/BH relation is claimed by default. Exact links shall be recorded only when approved L4 behavior materially requires this capability directly; current R4B review found no separate direct material BH.

## 4. Technical Ownership, Boundaries & Dependencies
### 4.1 Owned technical elements
- **Health Contract:** defines liveness, readiness, dependency-health and bounded functional-health semantics.
- **Health Aggregator:** combines component and dependency status without erasing signal provenance.
- **Alert Evaluation Point:** evaluates governed signal conditions against declared severity/actionability rules.
- **Alert Router/Deduplicator:** routes, groups and suppresses duplicate/maintenance-expected notifications without deleting source evidence.
- **Incident Context Builder:** creates a technical context identifier linking relevant telemetry, version/configuration and dependency information.
- **Diagnostic Snapshot Service:** exposes bounded, authorized, normally read-only diagnostic views.
- **Runbook & Service Ownership Link:** resolves service/component ownership and operational action guidance.
- **Remediation Control Boundary:** permits only explicitly pre-authorized, bounded technical actions.

### 4.2 Boundary rules
Diagnostics and remediation cannot directly rewrite business state outside canonical owning-domain contracts/workflows. Health status cannot redefine business lifecycle state. Alert severity cannot redefine security incident classification owned elsewhere.

## 5. Solution Architecture
### 5.1 Health model
- **Liveness:** process/runtime can continue and is not irrecoverably stuck.
- **Readiness:** component can safely accept its intended workload in its current runtime context.
- **Dependency health:** status of required/optional technical dependencies, represented separately from process liveness.
- **Functional health:** bounded, non-destructive probe of a materially important technical path where justified.

A dependency outage does not automatically imply process non-liveness. Readiness may degrade when a required dependency prevents safe workload acceptance. Optional dependency failure may produce degraded readiness/functional state without declaring the process dead.

### 5.2 Health publication flow
```text
component/runtime
→ local health evaluators
→ dependency health inputs
→ health contract normalization
→ minimal public health view
→ privileged detailed diagnostic view
```

Public health responses reveal only the minimum information needed by runtime/platform consumers. Detailed dependency, version/configuration and error information requires authorization.

### 5.3 Alerting flow
```text
telemetry/health signal
→ alert evaluation point
→ correlation + deduplication
→ maintenance/suppression check
→ severity/actionability classification
→ ownership/runbook resolution
→ notification/service-management handoff
```

Paging alerts should represent actionable service/user impact, security risk, material SLO risk or imminent capacity/failure risk. Symptom-oriented alerts are preferred for paging; cause/detail signals may enrich incident context. Queue/backlog conditions should consider age/growth and delivery impact rather than raw depth alone.

### 5.4 Incident diagnostic flow
```text
alert or authorized operator query
→ Incident Context ID
→ logs + metrics + traces + health
→ deployment/change/version metadata
→ safe configuration fingerprint
→ dependency map
→ bounded read-only diagnostic checks
→ runbook/service owner
→ optional authorized remediation request
```

Diagnostic checks must be bounded in scope/time and must not become hidden production mutation paths. Configuration fingerprints exclude secret values.

### 5.5 Remediation model
Automated remediation is permitted only where the action is pre-approved, technically bounded, attributable and reversible where feasible. Restart is valid only when restart is an accepted recovery mechanism from E01/runtime architecture. Actions affecting business state must go through the owning business/domain mechanism rather than E05 tooling.

### 5.6 Failure model
Alerting or service-management tool outage does not alter business transaction outcome. Lost/delayed alert delivery is itself operationally detectable where feasible. Diagnostic failure returns an explicit unavailable/partial result rather than fabricating health. Maintenance suppression affects notification behavior, not underlying evidence collection.

## 6. Architecture & Technology Decisions
- **AD-001 — Distinct health classes:** liveness, readiness, dependency health and functional health remain separate concepts.
- **AD-002 — Minimal public exposure:** public health surfaces disclose only minimum operational status.
- **AD-003 — Actionability-first alerting:** paging is reserved for actionable impact/risk conditions.
- **AD-004 — Correlated incident context:** alerts link to multi-signal and change/dependency context rather than isolated log text.
- **AD-005 — Read-only diagnostics by default:** privileged mutation is outside normal diagnostic behavior.
- **AD-006 — Controlled remediation:** automated remediation must be pre-authorized and bounded.
- **AD-007 — Maintenance suppression semantics:** suppression changes notification, not evidence retention.
- **AD-008 — Tool neutrality:** no incident/service-management product is selected here.

## 7. Security, Privacy, Quality & Operations
Detailed diagnostics are privileged. Break-glass access is time-bound, attributable and audited by the canonical audit architecture. Public health surfaces never expose secrets, stack traces, unrestricted topology or sensitive configuration. Diagnostic probes use least privilege and bounded resource consumption. Service ownership/runbook references must be maintainable independently of vendor tooling.

## 8. Architecture Conformance Requirements
- **CR-001:** Liveness and readiness shall be represented separately.
- **CR-002:** Required dependency failure may affect readiness but shall not automatically imply non-liveness.
- **CR-003:** Every production paging alert shall resolve to an accountable owner and action path.
- **CR-004:** Maintenance suppression shall not delete underlying telemetry/evidence.
- **CR-005:** Detailed diagnostics shall require authorization and be read-only by default.
- **CR-006:** Public health output shall not expose secrets or internal diagnostic detail.
- **CR-007:** Automated remediation shall be explicitly pre-approved and bounded.
- **CR-008:** E05 operations tooling shall not modify business state outside owning-domain contracts/workflows.
- **CR-009:** Diagnostic/alert tool failure shall not redefine business success/failure.
- **CR-010:** Product selection shall require the applicable ADR/governance decision.

## 9. Open Decisions & Deferred Design
- **OD-001:** Alert/incident/service-management product — unresolved.
- **OD-002:** Numeric alert thresholds, response times and escalation timing — consumed from canonical operational requirements/configuration profiles.
- **OD-003:** Remediation action catalog remains profile/governance controlled. Production enablement of any automated remediation action requires explicit pre-approval under the owning technical/security authority; this is an enablement gate, not an architecture-approval blocker.
- **OD-004:** R4B review found no direct material L4/BH contribution requiring a separate E05-003 relation. Do not create artificial mappings.
- **OD-005:** Stage-3 closure approved downstream consumption of peer responsibility surfaces; material peer changes require impact revalidation. No direct L4 relation is required unless future material applicability is established.

## 10. Traceability Summary
Candidate Corpus Register 105 and `L5-E-05-001/002` establish the technical area. Explicit dependencies include E04 runtime identity, E01/E02 reliability/performance context, C02 event signals and D02/D03/D04 controls. No direct material L4/BH link was established in R4B; architecture approval rests on the corpus responsibility, approved E05 baseline/telemetry peers, locked ADRs and explicit remediation-authorization guardrails.


## 2026-09-23 R4B Targeted Revalidation

**Result:** PASS — Architecture Approved.

This specification was revalidated against R4A-approved operational-readiness evidence parents where directly material, the Locked L5 governance/validation standards, applicable Locked ADRs, current E05 peer boundaries, and the R4B Technical Service Acceptance Authority decision.

No HF-01…HF-15 blocker remains for this specification.

Direct L4/BH relations are recorded only where material. Absence of an artificial direct BH mapping is not treated as a traceability defect for reusable technical responsibilities whose authority is established by the canonical corpus and approved peer architecture.

Final L5 Corpus Lock remains separately governed and is not authorized by this approval.
