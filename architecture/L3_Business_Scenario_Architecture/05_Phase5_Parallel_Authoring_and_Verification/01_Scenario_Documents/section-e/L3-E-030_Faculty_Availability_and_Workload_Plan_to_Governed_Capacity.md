# Faculty Availability and Workload Plan to Governed Capacity

**Scenario ID:** L3-E-030  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Faculty Workforce  
**Participating Domains:** B, D, E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Author:** Varad  
**Version:** 1.0  
**Date:** 2026-08-20  
**Depends On:** L2-E03 — Faculty Workforce Workflows; applicable organisational, academic and scheduling context

## 1. Scenario Metadata
Frozen Stage 3 scenario identity, title, canonical owner, participating domains, Scenario Type and Priority are preserved. This scenario applies locked upstream architecture and does not redesign ownership.

## 2. Scenario Purpose
Establish a governed view of faculty availability, workload and resulting academic capacity so that workload decisions remain within institutional policy limits.

## 3. Business Outcome
The Faculty Workforce has authoritative effective availability and workload information, and governed capacity visibility supports institutional academic planning.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
Faculty capacity must be determined for academic planning or workload allocation.

### 4.2 End Boundary
The effective availability and workload plan has been established and the resulting capacity view is authoritative for the applicable planning period.

### 4.3 In Scope
- Maintain effective-dated faculty availability.
- Validate availability conflicts.
- Plan and review academic workload.
- Apply capacity constraints.
- Require exception approval where workload exceeds configured capacity.
- Preserve workload and availability history.

### 4.4 Out of Scope
Implementation details, APIs, databases, services, UI, infrastructure, technical access control/IAM, queues, technical retries and other L4+ concerns are outside this L3 scenario.

## 5. Ownership and Participating Domains
**Canonical Owner:** Faculty Workforce.

Participating domains remain authoritative for their own concepts. Participation in this scenario does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities
- Faculty Workforce owns availability, workload and capacity visibility.
- Organisation/Position & Assignment provides organisational context where relevant.
- Academic/scheduling consumers use the resulting capacity information without acquiring ownership.

No unsupported technical role is introduced.

## 7. Preconditions and Entry Criteria
- Faculty Workforce is Active.
- Availability has applicable effective dates.
- Existing workload information is available for capacity evaluation.
- Institutional workload/capacity policy is applicable.

## 8. Trigger
An academic planning or workload requirement requires faculty capacity to be evaluated and governed.

## 9. Main Business Flow
1. Identify the Faculty Workforce and planning period.
2. Capture or update Faculty Availability.
3. Validate availability rules and effective dates.
4. Detect and resolve availability conflicts through the governed business review path.
5. Establish the effective availability outcome.
6. Evaluate existing and proposed workload against faculty capacity.
7. Apply institutional workload policy.
8. Where workload remains within configured capacity, establish the governed workload plan.
9. Where workload exceeds configured capacity, obtain the required exception approval before accepting the exception.
10. Update workload/capacity information and preserve history.
11. Publish the applicable `FacultyAvailabilityChanged`, `FacultyWorkloadUpdated`, and `FacultyCapacityChanged` business facts.
12. Stop when governed capacity visibility is authoritative.

The main flow stops when the institutional Business Outcome and End Boundary are reached.

## 10. Alternate Business Flows
- Availability requires a start date and, where applicable, an end date.
- Availability conflicts are reviewed rather than silently accepted.
- Workload exceeding configured capacity requires exception approval.
- A non-Active Faculty Workforce cannot accept new workload allocations.
- Capacity is derived from the authoritative availability, workload and applicable capability constraints.

## 11. Failure, Interruption and Business Recovery
If availability cannot be validly established, the conflicting availability does not become authoritative. If proposed workload exceeds capacity without the required exception approval, the excess allocation does not become an accepted governed workload outcome. Existing authoritative workload and availability remain preserved. No technical retry or rollback is described.

Only business-level recovery is defined.

## 12. Business Rules and Decision Constraints
- Faculty Availability represents when a faculty member can participate in institutional academic activities.
- Faculty Availability SHALL contain effective dates.
- Workload allocation exceeding configured capacity limits SHALL require exception approval.
- Workload SHALL NOT accept new allocations unless the Faculty Workforce is Active.
- Availability, workload and capability constraints contribute to capacity determination.
- Workload and availability history are preserved.

The rules remain owned by the authoritative L2 architecture.

## 13. State and Lifecycle Impact
**Faculty Availability:** effective-dated availability information with governed changes.

**Faculty Workload:** workload planning and capacity visibility through WF-E03-007.

The scenario does not invent additional lifecycle states beyond those defined by the locked L2 models.

No new lifecycle state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Business Events
| Event | Canonical Publisher | Business Meaning |
|---|---|---|
| `FacultyAvailabilityChanged` | Faculty Workforce | Faculty availability updated. |
| `FacultyWorkloadUpdated` | Faculty Workforce | Faculty workload updated. |
| `FacultyCapacityChanged` | Faculty Workforce | Faculty capacity changed. |

### 14.2 Business Contracts / Hand-offs
Organisation, academic and scheduling consumers use the resulting availability/workload/capacity business facts according to their own responsibilities. Faculty Workforce retains ownership.

Events remain distinct from synchronous business contracts and hand-offs.

## 15. Business Authorization, Approval and Audit Obligations
Availability review and workload exception approval follow the locked institutional governance. Approval decisions require the decision and actor; the source also defines decision-reason requirements. No unsupported named approver is invented.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
- Effective availability is authoritative.
- Workload plan respects configured capacity or has the required approved exception.
- Capacity visibility is established for the planning period.
- Historical availability/workload information is preserved.
- Applicable canonical business events are published.

### 16.2 Postconditions
The resulting business outcome is authoritative for subsequent institutional use and required history remains preserved.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
None identified.

### 17.2 Assumptions
None.

### 17.3 Deferred Questions
None.

**Authoring Classification:** PASS — source-grounded Draft.
