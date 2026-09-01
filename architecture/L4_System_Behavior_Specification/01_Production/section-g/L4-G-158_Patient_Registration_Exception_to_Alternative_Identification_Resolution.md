# L4-G-158 — Patient Registration Exception to Alternative Identification Resolution

**Architecture Level:** L4 — System Behavior Specification  
**Primary Owner:** Institutional Health Services (G06)  
**Realizes:** L3-G-158 — Patient Registration Exception to Alternative Identification Resolution  
**Status:** Draft

## Objective
Govern an exception to normal patient registration identification through the authorised alternative-identification resolution path.

## Boundary
**Start:** Normal patient identification cannot be completed.  
**End:** An authorised alternative identification resolution is established or registration remains unresolved/rejected.

## Required Behaviors
1. Record the identification exception against the healthcare-registration context.
2. Preserve available identity/context evidence.
3. Apply the authorised alternative-identification rules.
4. Establish the alternative identity resolution only when business authority and evidence permit.
5. Continue or reject patient registration according to the resulting authoritative outcome.
6. Preserve exception and resolution history.

## Constraints
No invented identity, duplicate patient profile, clinical fact or technical identity mechanism may be used to bypass the business exception.

## Events
Use only canonical G06 events defined by authoritative upstream architecture; no event name is invented here.

## Audit & Completion
Completion requires a governed alternative identification resolution and traceable registration consequence, or a recorded unresolved/rejected outcome.

## L4 Boundary
Identity technology, biometric systems, APIs, databases, IAM and infrastructure are outside L4.

**Traceability:** L3-G-158 — Patient Registration Exception to Alternative Identification Resolution.