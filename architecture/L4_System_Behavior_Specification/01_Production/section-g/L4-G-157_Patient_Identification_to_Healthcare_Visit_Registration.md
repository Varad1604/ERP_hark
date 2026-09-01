# L4-G-157 — Patient Identification to Healthcare Visit Registration

**Architecture Level:** L4 — System Behavior Specification  
**Primary Owner:** Institutional Health Services (G06)  
**Realizes:** L3-G-157 — Patient Identification to Healthcare Visit Registration  
**Status:** Draft

## Objective
Govern identification of an eligible patient and registration of the corresponding healthcare visit within G06 ownership.

## Boundary
**Start:** A person presents for healthcare service and identification is required.  
**End:** A valid healthcare visit is registered, or a governed identification exception prevents registration.

## Required Behaviors
1. Establish the person's authoritative institutional identity/context.
2. Determine whether the person can be treated as the applicable patient for the visit.
3. Record required visit context without inventing clinical facts.
4. Register the healthcare visit under G06.
5. Preserve identity and registration evidence.

## Failure / Recovery
Unresolved identity prevents unsupported patient/visit registration. An identification exception follows the separate applicable resolution scenario. No duplicate patient identity is silently created.

## Ownership & Events
G06 owns patient/visit registration. Only canonical G06 events defined upstream may be published; no new event name is invented.

## Audit & Completion
Completion requires authoritative patient context, registered healthcare visit and preserved traceability.

## L4 Boundary
Clinical diagnosis, treatment, APIs, databases, IAM, infrastructure and UI are outside this specification.

**Traceability:** L3-G-157 — Patient Identification to Healthcare Visit Registration.