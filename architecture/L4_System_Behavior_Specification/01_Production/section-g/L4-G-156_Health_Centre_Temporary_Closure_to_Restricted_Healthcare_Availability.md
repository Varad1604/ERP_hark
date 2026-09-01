# L4-G-156 — Health Centre Temporary Closure to Restricted Healthcare Availability

**Architecture Level:** L4 — System Behavior Specification  
**Primary Owner:** Institutional Health Services (G06)  
**Realizes:** L3-G-156 — Health Centre Temporary Closure to Restricted Healthcare Availability  
**Status:** Draft

## Objective
Govern a Health Centre temporary closure so its operational status reflects the authorised restriction on healthcare availability while preserving the Health Centre lifecycle owner.

## Boundary
**Start:** An authorised temporary-closure condition is established.  
**End:** The Health Centre is in the applicable temporary-closure condition and healthcare availability is restricted accordingly.

## Required Behaviors
1. Identify the affected Health Centre and scope.
2. Establish the authoritative temporary-closure basis.
3. Apply the applicable G06 review/authority.
4. Transition operational status to the authoritative temporary-closure state defined by L2-G06.
5. Restrict healthcare availability consistent with that state.
6. Preserve prior operational status and closure history.
7. Make the outcome available to authorised consumers.

## Failure / Recovery
No temporary closure is asserted without an authoritative basis. If the closure condition is unresolved, the existing authoritative status remains. Restoration is governed separately and shall not be invented here.

## Events
Only canonical events defined by L2-G06 may be used. No new closure event is invented by this specification.

## Audit & Completion
Preserve closure basis, authority, effective scope, resulting status and historical evidence.

## L4 Boundary
Technical availability controls, APIs, databases, infrastructure, IAM and deployment are outside L4.

**Traceability:** L3-G-156 — Health Centre Temporary Closure to Restricted Healthcare Availability.