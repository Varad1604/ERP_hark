# Position Occupancy Conflict Resolution

**Scenario ID:** L3-E-023  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Position & Assignment  
**Participating Domains:** B, E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Author:** Varad  
**Version:** 1.0  
**Date:** 2026-08-20

## 1. Scenario Metadata
Frozen Stage 3 identity, title, owner, participating domains, Scenario Type and Priority are preserved. This L3 scenario applies the locked upstream architecture and does not redesign it.

## 2. Scenario Purpose
Resolve a conflict involving Position Assignment occupancy so that no ambiguous organisational state becomes operational.

## 3. Business Outcome
The Position Assignment conflict reaches a governed resolution, or the conflicting state remains non-operational until a valid resolution exists.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
A conflict involving Position Assignment occupancy is identified.

### 4.2 End Boundary
The conflict is resolved deterministically and the resulting organisational occupancy state is authoritative, with the resolution subject to the applicable business audit/evidence obligations.

### 4.3 In Scope
Detection and evaluation of Position Assignment conflicts; conflict resolution; occupancy integrity; deterministic resolution; historical integrity; applicable organisational authority consequences.

### 4.4 Out of Scope
Technical implementation, APIs, databases, services, UI, infrastructure, technical access control/IAM, queues, technical retries and other L4+ implementation concerns are outside this L3 scenario. Upstream lifecycle ownership remains with its canonical owner.

### 4.5 Related Scenarios
Only the upstream and downstream relationships explicitly stated by the authoritative architecture are considered related; no new canonical scenario is created.

## 5. Ownership and Participating Domains
**Canonical Owner:** Position & Assignment.

The Position & Assignment domain owns the relevant Position Assignment / organisational authority / occupancy lifecycle. Organisation (B) and Workforce & Engagement (E) provide authoritative business context within their own ownership boundaries. Participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities
- Conflict Resolution governance authority.
- Position & Assignment owns Position Assignment and occupancy conflict resolution.
- Organisation and Workforce & Engagement provide authoritative context where required.

No unsupported technical role or permission model is introduced.

## 7. Preconditions and Entry Criteria
A Position Assignment conflict exists or is identified before the conflicting business state becomes operational.

## 8. Trigger
An occupancy or related assignment conflict requires governed resolution.

## 9. Main Business Flow
1. Identify the conflicting Position Assignment business state.
2. Evaluate the conflict against applicable Position Assignment and organisational constraints.
3. Determine the applicable resolution precedence under governed policy.
4. Resolve the conflict deterministically.
5. Ensure no ambiguous assignment state becomes operational.
6. Preserve the resolution and historical lineage.
7. Publish the applicable canonical domain event(s) for the completed business fact.
8. Stop when the organisational conflict outcome is authoritative.

The flow stops when the institutional business outcome and end boundary are reached.

## 10. Alternate Business Flows
- Conflicts may include concurrent authority collisions, campus scope conflicts, invalid administrative corrections and bulk-processing conflicts.
- A manual override may be used only where the governing architecture permits it and the required governance approval is satisfied.
- A conflict that cannot be validly resolved remains non-operational rather than producing an ambiguous state.

## 11. Failure, Interruption and Business Recovery
Where the conflict cannot be validly resolved, the conflicting state does not become operational. Existing authoritative business state remains in force where applicable, and the matter follows the governed conflict-resolution path. No technical rollback or retry is described.

Recovery is expressed only as a business outcome; no technical retry, queue, rollback or infrastructure behaviour is defined.

## 12. Business Rules and Decision Constraints
- No ambiguous assignment state SHALL become operational.
- Conflict Resolution SHALL execute before the conflicting business state is committed.
- Resolution SHALL be deterministic.
- Manual overrides SHALL be audited.
- Policy engines SHALL determine resolution precedence.
- Every resolution SHALL publish immutable Domain Events.

## 13. State and Lifecycle Impact
Position Assignment remains the canonical aggregate/lifecycle. Conflict Resolution is a governed decision outcome and does not create a competing occupancy lifecycle. The underlying assignment lifecycle remains subject to its canonical states and transitions.

No new state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Business Events
The locked L2 workflow W-020 explicitly requires every resolution to publish immutable Domain Events, but the authoritative material reviewed did not expose a distinct event name for the conflict-resolution outcome. Therefore no event name is invented here.

### 14.2 Business Contracts / Hand-offs
Organisation and Workforce & Engagement provide upstream facts where relevant; conflict resolution remains owned by Position & Assignment. The result is made available to downstream business capabilities through canonical collaboration mechanisms.

Events remain distinct from synchronous business contracts and hand-offs.

## 15. Business Authorization, Approval and Audit Obligations
Execution requires conflict-resolution authority, organisational scope validation, and governance approval for overrides. Manual conflict overrides are audited.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
- Conflict is resolved deterministically or remains non-operational.
- No ambiguous organisational occupancy becomes operational.
- Any permitted override is governed and auditable.
- Historical integrity is preserved.
- Applicable immutable business event publication occurs.

### 16.2 Postconditions
The resulting business state is authoritative for subsequent institutional use and remains historically traceable.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
None identified.

### 17.2 Assumptions
None.

### 17.3 Deferred Questions
None.

**Authoring Classification:** PASS — source-grounded Draft.
