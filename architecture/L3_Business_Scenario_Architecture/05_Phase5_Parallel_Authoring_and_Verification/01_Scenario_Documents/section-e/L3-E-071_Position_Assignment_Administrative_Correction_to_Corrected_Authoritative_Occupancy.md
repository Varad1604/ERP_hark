# Position Assignment Administrative Correction to Corrected Authoritative Occupancy

**Scenario ID:** L3-E-071  
**Status:** Locked  
**Primary Owner:** Position & Assignment  
**Participating Domains:** B — Organisation; E — People & Workforce  
**Scenario Type:** Critical  
**Priority:** P1  
**Architecture Level:** L3 — Business Scenario Architecture  
**Author:** Varad

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-E-071 |
| Canonical Title | Position Assignment Administrative Correction to Corrected Authoritative Occupancy |
| Primary Owner | Position & Assignment |
| Participating Domains | B — Organisation; E — People & Workforce |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Correct an administrative error in a Position Assignment so that the authoritative organisational occupancy is accurate while preserving assignment ownership, effective dating, lineage and historical traceability.

The scenario corrects Position Assignment information owned by L2-E02. It does not redefine Position Definition, Workforce Engagement, or authorization ownership.

## 3. Business Outcome

The affected Position Assignment reflects the corrected authoritative occupancy, the correction is historically traceable, and the Position Assignment remains the single canonical representation of organisational occupancy for the affected engagement and position.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when an administrative error in an existing Position Assignment is identified and a governed correction is required.

### 4.2 End Boundary

The scenario ends when the corrected Position Assignment becomes the authoritative current representation and the prior business state remains historically reconstructable.

### 4.3 In Scope

- Identification of an incorrect Position Assignment.
- Validation of the correction.
- Correction of assignment information owned by L2-E02.
- Effective dating and version lineage where required by the authoritative assignment lifecycle.
- Preservation of prior assignment history.
- Publication of canonical Position Assignment events.

### 4.4 Out of Scope

- Changing the Position Definition owned by Organisation.
- Changing Workforce Engagement.
- Creating a new person or workforce identity.
- Authorization policy lifecycle.
- Payroll or leave transactions.
- Technical data-repair or database mechanics.

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Position & Assignment | Canonical owner of Position Assignment, assignment lifecycle, historical lineage and authoritative occupancy. |
| Organisation | Owns Position Definition and organisational master structure referenced by the assignment. |

Correction of assignment occupancy does not transfer ownership of Position Definition to E02.

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| HR Administrator | Identifies or administers the assignment correction. |
| Department Administrator | Provides departmental context where required. |
| Authorised organisational administrator | Participates in the governed correction decision. |

Only roles supported by the Position & Assignment architecture are used; no technical actor is introduced.

## 7. Preconditions and Entry Criteria

- The Position Assignment exists.
- Its associated Workforce Engagement remains valid.
- The Position Definition reference is valid.
- An administrative discrepancy has been identified.
- The proposed corrected state can be validated under L2-E02 rules.

## 8. Trigger

An administrative error in an authoritative Position Assignment is identified and a correction request is raised.

## 9. Main Business Flow

| Step | Business Behaviour | Owner | Result |
|---|---|---|---|
| 1 | The incorrect assignment information is identified. | Position & Assignment / authorised HR participant | Correction case exists. |
| 2 | The proposed correction is validated against Position Definition, Workforce Engagement, organisational placement, effective dating and assignment rules. | Position & Assignment | Correction is eligible. |
| 3 | The authoritative Position Assignment is corrected using the governed assignment evolution model. | Position & Assignment | Corrected occupancy becomes authoritative. |
| 4 | Assignment lineage preserves the prior state and effective history. | Position & Assignment | Historical reconstruction remains possible. |
| 5 | The canonical Position Assignment event corresponding to the correction is published. | Position & Assignment | Dependent domains can consume the corrected occupancy fact. |

## 10. Alternate Business Flows

### A1 — Correction Changes Only Effective Dating

The assignment is corrected through its effective-dated/versioned business evolution without altering unrelated assignment information.

### A2 — Correction Would Change Position Definition

If the requested change actually concerns the Position Definition, the correction is redirected to Organisation rather than changing ownership.

### A3 — Correction Would Change Workforce Engagement

If the requested change is actually an employment-lifecycle change, the source is routed to Workforce & Engagement.

## 11. Failure, Interruption and Business Recovery

| Situation | Resolution |
|---|---|
| Proposed correction violates an assignment invariant | Do not apply the correction until the business condition is resolved. |
| Position Definition is invalid or unavailable | Preserve the existing valid assignment and resolve the organisational dependency. |
| Workforce Engagement is not valid for the proposed assignment | Do not create an invalid occupancy relationship. |
| Correction evidence is incomplete | Keep the authoritative assignment unchanged until the evidence is sufficient. |

Technical rollback/retry mechanics are outside L3.

## 12. Business Rules and Decision Constraints

- Position Assignment is canonically owned by L2-E02.
- Position Definition is owned by Organisation.
- Workforce Engagement is owned by L2-E01.
- Every Position Assignment belongs to one Workforce Engagement.
- Every Position Assignment references one Position Definition.
- Historical assignments remain immutable and reconstructable.
- Business evolution produces versioned lineage.
- Organisational authority derives from active Position Assignments.

## 13. State and Lifecycle Impact

| Concept | Impact | Owner |
|---|---|---|
| Position Assignment | Corrected through its authoritative lifecycle/version lineage. | Position & Assignment |
| Position Definition | Referenced, not re-owned or changed. | Organisation |
| Workforce Engagement | Referenced, not re-owned or changed. | Workforce & Engagement |

No new lifecycle state is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

Position & Assignment remains the canonical publisher of assignment lifecycle events.

The exact frozen correction event name must be taken from the authoritative L2-E02 event catalogue and Stage 3 mapping; no new event is invented.

### 14.2 Business Hand-offs

Organisation → Position & Assignment: authoritative Position Definition context.

Workforce & Engagement → Position & Assignment: authoritative Workforce Engagement context.

Position & Assignment → downstream consumers: corrected authoritative occupancy fact.

## 15. Business Authorization, Approval and Audit Obligations

Administrative correction must be performed under the business governance of Position & Assignment and any applicable organisational approval.

The correction decision, supporting evidence, prior assignment state and corrected assignment lineage must remain auditable.

No L3 technical audit mechanism is introduced.

## 16. Completion Criteria and Postconditions

- Corrected Position Assignment is authoritative.
- Position Definition ownership remains with Organisation.
- Workforce Engagement ownership remains with Workforce & Engagement.
- Assignment history remains reconstructable.
- The corrected assignment event is published where defined.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

The repository's L2-E02 architecture clearly establishes canonical ownership, immutable history, versioning and effective dating, but the exact Stage 3 correction trigger/outcome semantics and frozen event name should be confirmed before Architecture Approval.

### 17.2 Assumptions

No assumption overrides locked architecture.

### 17.3 Deferred Questions

Confirm the exact canonical administrative-correction event and transition terminology in the frozen L2-E02 mapping.
