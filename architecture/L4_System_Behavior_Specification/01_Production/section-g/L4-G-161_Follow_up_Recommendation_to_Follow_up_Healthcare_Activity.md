# L4-G-161 — Follow-up Recommendation to Follow-up Healthcare Activity

**Architecture Level:** L4 — System Behavior Specification  
**Primary Owner:** Institutional Health Services (G06)  
**Realizes:** L3-G-161 — Follow-up Recommendation to Follow-up Healthcare Activity  
**Status:** Draft

## Objective
Govern an authorised healthcare follow-up recommendation through establishment and completion of the corresponding follow-up healthcare activity.

## Boundary
**Start:** A follow-up recommendation is established.  
**End:** The follow-up healthcare activity is completed or reaches its governed unresolved/cancelled outcome.

## Required Behaviors
1. Identify authoritative patient and prior healthcare context.
2. Record the follow-up recommendation and its business basis.
3. Establish applicable follow-up timing or condition where governed.
4. Register the follow-up healthcare activity when due/eligible.
5. Record completion or governed non-completion outcome.
6. Preserve recommendation, activity and outcome history.

## Constraints
Do not invent clinical recommendations, diagnoses, schedules or treatment obligations not supported by authoritative healthcare context.

## Failure / Recovery
If follow-up context cannot be established, the activity remains unresolved rather than receiving an invented clinical basis. A separately governed clinical outcome remains under its own scenario.

## Ownership & Events
G06 owns the healthcare follow-up outcome. Only canonical G06 events defined upstream may be used.

## Audit & Completion
Preserve recommendation basis, patient/encounter context, follow-up decision, activity and final outcome.

## L4 Boundary
Scheduling technology, APIs, databases, notifications, IAM and infrastructure are outside L4.

**Traceability:** L3-G-161 — Follow-up Recommendation to Follow-up Healthcare Activity.