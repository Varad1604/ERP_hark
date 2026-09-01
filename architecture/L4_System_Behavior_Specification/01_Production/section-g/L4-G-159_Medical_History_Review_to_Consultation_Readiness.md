# L4-G-159 — Medical History Review to Consultation Readiness

**Architecture Level:** L4 — System Behavior Specification  
**Primary Owner:** Institutional Health Services (G06)  
**Realizes:** L3-G-159 — Medical History Review to Consultation Readiness  
**Status:** Draft

## Objective
Govern review of available medical history so an identified patient is ready for the applicable healthcare consultation without inventing clinical information.

## Boundary
**Start:** A registered healthcare visit requires relevant history review.  
**End:** Consultation readiness is established, or a governed limitation prevents readiness.

## Required Behaviors
1. Identify the authoritative patient and visit context.
2. Retrieve/consider available authoritative medical history within the business scope.
3. Review relevant history according to applicable clinical governance.
4. Record readiness or an authoritative limitation.
5. Preserve the review outcome and relevant provenance.

## Constraints
The system shall not invent diagnoses, treatment decisions, missing history, or unsupported clinical conclusions. Consultation itself remains a separate scenario.

## Failure / Recovery
If required history is unavailable or inconsistent, preserve the limitation and follow the authorised clinical resolution path; do not silently manufacture history.

## Ownership & Events
G06 owns healthcare history and consultation-readiness outcome. Use only upstream canonical G06 events; no new event name is invented.

## Audit & Completion
Completion requires an authoritative readiness outcome or governed limitation with traceable review evidence.

## L4 Boundary
Clinical algorithms, APIs, databases, IAM, infrastructure and UI are outside L4.

**Traceability:** L3-G-159 — Medical History Review to Consultation Readiness.