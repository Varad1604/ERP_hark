# Alumni Profile or Preference Change to Current Governed Record

# 1. Scenario Metadata

**Scenario ID:** L3-H-041  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Alumni  
**Participating Domains:** A, C, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A02 — Person & Identity Workflows; L2-C05 — Qualification Completion & Student Exit Workflows; L2-C08 — Alumni Transition Workflows; L2-H06 — Alumni Workflows  
**Referenced By:** Alumni directory, communications, networking, mentorship, reporting, and engagement scenarios

# 2. Scenario Purpose

Keep permitted H06-owned Alumni profile and engagement preferences current while Section A identity and Section C academic/transition facts remain authoritative references.

# 3. Business Outcome

The permitted alumni engagement record becomes current while source identity and academic evidence remain references

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An alumnus updates allowed profile, consent, directory, or communication preferences.

## 4.2 End Boundary

The permitted H06-owned change is current and audited; Section A and C source facts are unchanged.

## 4.3 In Scope

H06-owned Alumni/profile, professional-profile, communication-preference, and directory-visibility changes.

## 4.4 Out of Scope

Changes to Person/Identity, Student, qualification, exit, Alumni Transition, or Alumni Persona facts; detailed historical employment ownership outside H06's current professional profile.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Platform Foundation / A | Retains canonical Person and Identity facts. |
| Student Lifecycle / C | Retains Student, qualification, exit, transition, and Alumni Persona facts. |
| Alumni / H | Owns permitted Alumni engagement/profile and preference information. |

# 6. Business Actors, Roles and Responsibilities

| Actor | Responsibility | Authority |
|---|---|---|
| Alumni | Maintains permitted self-service profile/preferences. | Initiates / performs |
| Alumni Administrator | Performs H06-supported governed administration where required. | Performs |

# 7. Preconditions and Entry Criteria

The Alumni Profile exists; the requested change concerns H06-owned information; canonical A/C references remain available and externally owned.

# 8. Trigger

An alumnus updates allowed profile, consent, directory, or communication preferences

# 9. Main Business Flow

Resolve the Alumni record and A/C references; confirm H06 ownership of the requested change; apply the valid profile/preference change without changing source facts; publish `ProfessionalProfileUpdated`, `CommunicationPreferencesUpdated`, or `DirectoryVisibilityUpdated` for their defined branches; audit other permitted `UpdateAlumniProfile` changes; make the H06 record current.

# 10. Alternate Business Flows

An opt-out becomes effective immediately where H06 requires it. Employer references may remain unnormalized without creating or changing an enterprise organization record.

# 11. Failure, Interruption and Business Recovery

A change outside H06 ownership remains with its canonical owner. Invalid or non-permitted H06 changes are rejected without changing the current governed record.

# 12. Business Rules and Decision Constraints

Every Alumni references one canonical Person and Student; professional-profile updates do not overwrite enterprise organization records; H06 keeps only the current professional profile; communication honors explicit consent and immediate opt-out; directory visibility follows Alumni privacy preferences; H06 never modifies another bounded context's aggregate.

# 13. State and Lifecycle Impact

Only H06-owned current profile/preference information changes. A/C lifecycle states remain unchanged.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H06 defines `ProfessionalProfileUpdated`, `CommunicationPreferencesUpdated`, and `DirectoryVisibilityUpdated`. It defines `UpdateAlumniProfile` but no canonical generic Alumni Profile update event.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Section A identity and Section C academic/transition facts remain authoritative references; H06 does not become their source of record.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Alumni self-service and Alumni Administrator activity remain limited to responsibilities defined by H06.

## 15.2 Separation of Duties, Overrides and Exceptions

No H06 change alters A/C ownership or expands the fields an alumnus may change.

## 15.3 Business Audit and Evidence Obligations

Preserve Alumni reference, changed H06-owned information, actor/time, applicable consent/privacy evidence, and unchanged source references.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The change is H06-owned, valid, applied to the appropriate Alumni record, and required audit evidence is preserved.

## 16.2 Postconditions

The Alumni engagement record is current; source identity/academic evidence remains externally owned; defined branch events are published.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

H06 requires generic Alumni Profile updates to be audited and defines `UpdateAlumniProfile`, but has no canonical generic `AlumniProfileUpdated` event. This does not prevent the frozen current-record outcome but must be resolved before downstream architecture depends on such an event.

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions beyond the open issue above.
