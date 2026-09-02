# Learning Opportunity Establishment to Governed Active Availability

## 1. Specification Metadata

**Specification ID:** L4-D-040  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Academic Delivery — Open Electives & MOOCs  
**Realizes Scenario:** L3-D-040  
**Primary Business Owner:** Open Electives & MOOCs  
**Version:** 1.0.0  
**Status:** Architecture Approved  
**Owner:** Pallav  
**Last Updated:** 2026-09-02  
**Depends On:** `L3-D-040_Learning_Opportunity_Establishment_to_Governed_Active_Availability.md`; `L2-D06_Open_Electives_and_MOOCs_Workflows.md`; Locked L4 governance/template/validation baseline  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

Establish an Open Elective or approved external/MOOC Learning Opportunity and activate governed availability using canonical academic/provider references.

**Start boundary:** An approved D01 Course Offering or approved mapped provider course is designated for flexible-learning use.

**End boundary:** The Learning Opportunity reaches Active and is discoverable within its governed audience, capacity and enrollment window.

This specification is limited to observable system behavior required by the Locked parent. It does not absorb adjacent domain outcomes or prescribe implementation mechanisms.

---

## 3. Required Business Context

- Approved source Course Offering/provider course
- Authoritative session/mapping/provider context
- Audience/capacity/waitlist/window configuration
- Applicable institutional readiness/activation authority

All source-owned facts remain owned by their canonical business domains.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-D-040-BH-001 | The system shall classify the opportunity using the governed D06 opportunity type. | L3-D-040 §§3–13 |
| L4-D-040-BH-002 | The system shall validate authoritative offering/provider, academic session, mapping and approval facts. | L3-D-040 §§3–13 |
| L4-D-040-BH-003 | The system shall prevent establishment/activation when source/provider references are invalid. | L3-D-040 §§3–13 |
| L4-D-040-BH-004 | The system shall establish and publish the configured opportunity without changing source-master ownership. | L3-D-040 §§3–13 |
| L4-D-040-BH-005 | The system shall, for external opportunities, record only the approved canonical D06 provider relationship and configuration business fact. | L3-D-040 §§3–13 |
| L4-D-040-BH-006 | The system shall establish governed capacity, reservation, waitlist, audience and enrollment window. | L3-D-040 §§3–13 |
| L4-D-040-BH-007 | The system shall validate business readiness before activation. | L3-D-040 §§3–13 |
| L4-D-040-BH-008 | The system shall recognize the authoritative D06 activation decision without making the system the decision-maker. | L3-D-040 §§3–13 |
| L4-D-040-BH-009 | The system shall transition the opportunity to Active only when readiness/authority conditions are satisfied. | L3-D-040 §§3–13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-D-040-BH-010 | The system shall publish `OpenElectiveOfferingPublished` or `MOOCOpportunityPublished` according to the governed opportunity type. | L3-D-040 §14; L2-D06_Open_Electives_and_MOOCs_Workflows.md exact event semantics |
| L4-D-040-BH-011 | The system shall publish `ProviderOfferingConfigured` where the external-provider configuration fact applies. | L3-D-040 §14; L2-D06_Open_Electives_and_MOOCs_Workflows.md exact event semantics |
| L4-D-040-BH-012 | The system shall publish `ElectiveCapacityConfigured` where the capacity configuration fact applies. | L3-D-040 §14; L2-D06_Open_Electives_and_MOOCs_Workflows.md exact event semantics |
| L4-D-040-BH-013 | The system shall publish `LearningOpportunityActivated` when the opportunity becomes Active. | L3-D-040 §14; L2-D06_Open_Electives_and_MOOCs_Workflows.md exact event semantics |

### Ownership-Preserving Collaboration

- Consume D01/Section B academic facts and external provider evidence without ownership transfer.

Event names and publisher direction remain exactly those established by the parent/upstream architecture. No message schema, broker, queue, delivery or retry design is defined here.

---

## 6. Failure / Rejection / Continuation

- Invalid source/provider/session/mapping facts block activation.
- Missing activation authority retains the prior D06 opportunity position.

In every failure or interruption path, the system shall preserve the last valid business position and expose only the source-supported business continuation/rejection outcome. Technical rollback, retry and infrastructure recovery are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to demonstrate this specification: Preserve source references, opportunity type/scope, audience/capacity/window configuration, approval/readiness evidence and activation fact.

Evidence behavior remains implementation-independent and does not prescribe tables, logs, storage engines or retention technology.

---

## 8. Observable Completion Conditions

- the opportunity is Active/discoverable
- availability is constrained by governed audience/capacity/window
- source ownership is unchanged
- `LearningOpportunityActivated` is published

The specification is not complete merely because a technical action occurred; the applicable governed business outcome above must be objectively observable.

---

## 9. Traceability & Provenance

**Parent:** `L3-D-040` — Learning Opportunity Establishment to Governed Active Availability  
**Primary verification source:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
**Architecture boundary:** Locked L4 Governance and Architecture Standard; Locked L4 Validation Standard  

Every BH is derived from the Locked L3 parent. The L2 source is used only to verify exact owner, rule, lifecycle and event semantics already represented by the parent; this L4 specification does not silently repair or expand the L3 business scenario.

---

## 10. Open Issues / Deferred Design

No L4-specific open issue remains under the current Locked parent baseline.

The following remain deferred to L5/L6: API/interface design, persistence/data model, service/component design, event transport, technical access control, transaction/retry/rollback mechanics, infrastructure/deployment and UI/UX design.

---

**Canonical Validation Note:** This specification is **Architecture Approved** under the final Architecture Approval issued by Dhruv / Lead Architect on 2026-09-02, as reported for this Section D batch. The specification is not `Locked`; movement to `Locked` requires separate explicit governance authorization.
