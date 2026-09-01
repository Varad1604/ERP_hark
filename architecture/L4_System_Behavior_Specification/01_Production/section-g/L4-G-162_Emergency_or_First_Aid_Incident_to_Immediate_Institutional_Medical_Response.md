# L4-G-162 — Emergency or First-Aid Incident to Immediate Institutional Medical Response

**Architecture Level:** L4 — System Behavior Specification  
**Primary Owner:** Institutional Health Services (G06)  
**Realizes:** L3-G-162 — Emergency or First-Aid Incident to Immediate Institutional Medical Response  
**Status:** Draft

## Objective
Govern an emergency or first-aid incident through immediate institutional medical response within the authorised G06 healthcare boundary.

## Boundary
**Start:** An emergency or first-aid incident is identified.  
**End:** Immediate institutional medical response is established/completed, or a governed escalation/referral boundary is reached.

## Required Behaviors
1. Establish available person/incident context as rapidly as business rules require.
2. Record the emergency or first-aid incident.
3. Assess the incident under authorised healthcare practice.
4. Initiate the applicable immediate institutional medical response.
5. Record the response and resulting healthcare outcome.
6. Preserve incident and response history.
7. Where the scenario requires escalation, hand off through the authoritative healthcare boundary without transferring G06 ownership of the institutional response record.

## Constraints
Do not invent clinical diagnoses, treatment protocols, ambulance arrangements, external referral mechanisms or other emergency details not defined upstream.

## Failure / Recovery
If full identity or contextual information is initially unavailable, response shall follow the applicable emergency business rule rather than inventing identity. Unresolved clinical needs proceed through the authoritative G06 escalation boundary.

## Ownership & Events
G06 owns the institutional medical response. Only canonical upstream healthcare events may be used; no event name is invented here.

## Audit & Completion
Preserve incident context, assessment basis, response, authority, escalation and outcome evidence appropriate to the business scenario.

## L4 Boundary
Clinical technology, APIs, databases, infrastructure, IAM, device integration and deployment are outside L4.

**Traceability:** L3-G-162 — Emergency or First-Aid Incident to Immediate Institutional Medical Response.