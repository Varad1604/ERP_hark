# L4-G-160 — Walk-in Consultation to Completed Medical Encounter

**Architecture Level:** L4 — System Behavior Specification  
**Primary Owner:** Institutional Health Services (G06)  
**Realizes:** L3-G-160 — Walk-in Consultation to Completed Medical Encounter  
**Status:** Draft

## Objective
Govern a walk-in healthcare consultation from eligible encounter establishment through completion and preservation of the medical encounter outcome.

## Boundary
**Start:** An eligible patient presents for walk-in consultation.  
**End:** The medical encounter is completed or reaches a governed non-completion outcome.

## Required Behaviors
1. Establish patient and healthcare-visit context.
2. Confirm consultation eligibility and applicable clinical context.
3. Conduct the consultation within authorised G06 practice boundaries.
4. Record the resulting medical encounter outcome.
5. Preserve relevant clinical and encounter history.
6. Make authorised downstream outcomes available without transferring clinical ownership.

## Constraints
No unsupported diagnosis, prescription, medicine-dispensing, referral or emergency outcome is invented; those remain separate business scenarios where applicable.

## Failure / Recovery
Incomplete patient/visit context prevents unsupported completion. Clinical or operational interruption follows the applicable G06 recovery boundary without technical retry mechanics.

## Ownership & Events
G06 owns the healthcare encounter. Only canonical upstream G06 events may be used; no new event name is invented.

## Audit & Completion
Completion requires a governed completed encounter outcome and preserved clinical/operational evidence.

## L4 Boundary
Clinical-system implementation, APIs, databases, IAM, infrastructure and UI are outside L4.

**Traceability:** L3-G-160 — Walk-in Consultation to Completed Medical Encounter.