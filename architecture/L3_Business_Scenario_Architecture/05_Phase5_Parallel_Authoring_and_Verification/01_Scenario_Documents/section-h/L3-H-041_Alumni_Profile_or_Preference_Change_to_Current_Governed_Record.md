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
**Last Updated:** 2026-08-23  
**Depends On:** Locked L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-C08 — Alumni Transition Workflows; L2-H06 — Alumni Workflows; applicable locked Section A identity architecture

# 2. Scenario Purpose

Apply a permitted Alumni-owned profile, consent, directory, or communication-preference change while keeping canonical identity, Student, Alumni Persona, and academic evidence under their upstream owners.

# 3. Business Outcome

The permitted alumni engagement record becomes current while source identity and academic evidence remain references

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an alumnus updates allowed profile, consent, directory, or communication preferences.

## 4.2 End Boundary

The scenario ends when the applicable H06-owned current value set or preference state reflects the permitted change and upstream source facts remain unchanged.

## 4.3 In Scope

- Professional Profile updates.
- Communication preference and consent changes.
- Directory visibility changes.
- Newsletter subscription preference where applicable.
- Preservation of upstream identity and academic references.

## 4.4 Out of Scope

- Person identity mutation by H06.
- Student, Alumni Persona, qualification, degree, or academic-evidence mutation.
- Employment-record ownership outside H06's self-declared professional engagement information.
- Student-to-Alumni transition changes.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Section A | Owns canonical Person identity and related identity facts referenced by H06. |
| Student Lifecycle / C | Owns Student, Alumni Persona, qualification-completion and academic-source facts. |
| Alumni / H06 | Owns the Alumni Profile engagement projection, Professional Profile, Communication Preference State, and directory preferences. |

# 6. Business Actors, Roles and Responsibilities

The Alumni initiates permitted self-service changes. Alumni Administrator or Institutional Administrator may perform governed administrative changes where H06 authorizes them.

# 7. Preconditions and Entry Criteria

- The Alumni Profile exists following `AlumniTransitionCompleted`.
- The target information is H06-owned or an H06-owned engagement preference.
- The requested change is permitted for the actor.
- Upstream source records remain references only.

# 8. Trigger

An alumnus updates allowed profile, consent, directory, or communication preferences

# 9. Main Business Flow

1. Identify whether the requested change belongs to Professional Profile, Communication Preference State, directory visibility, or a supported subscription preference.
2. Confirm that the requested field is H06-owned and not an upstream Person, Student, Alumni Persona, qualification, or academic record.
3. Apply the H06 Professional Profile update and publish `ProfessionalProfileUpdated`, where applicable.
4. Apply communication/consent preferences and publish `CommunicationPreferencesUpdated`, where applicable.
5. Apply directory visibility and publish `DirectoryVisibilityUpdated`, where applicable.
6. Apply supported newsletter subscription preference under H06 communication governance where applicable.
7. Make the current H06-owned engagement value set available while preserving upstream references unchanged.

# 10. Alternate Business Flows

- A request may change only professional profile information.
- A request may change only consent/communication preferences.
- A request may change only directory visibility.
- Multiple permitted H06-owned changes may be made while each owning aggregate retains its own lifecycle and event.

# 11. Failure, Interruption and Business Recovery

If the requested change belongs to an upstream owner, H06 does not alter it. If a requested H06 change is not authorized or violates H06 privacy/visibility rules, the current governed H06 record remains unchanged.

# 12. Business Rules and Decision Constraints

- H06 owns only Alumni engagement aggregates.
- Professional information entered by alumni is H06-owned and does not overwrite external employment records.
- Directory visibility must respect consent and privacy.
- Preference updates converge on the latest permitted submitted value.
- Source identity and academic information remain owned upstream.

# 13. State and Lifecycle Impact

This scenario updates current H06-owned engagement values; it does not create a new Student, Alumni Persona, qualification, or Person lifecycle transition. Communication and directory preference states converge on the current permitted setting.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Applicable H06 events include `ProfessionalProfileUpdated`, `CommunicationPreferencesUpdated`, and `DirectoryVisibilityUpdated`. H06 also consumes current authoritative identity/academic facts where required but does not republish them as owned facts.

## 14.2 Business Contracts and Cross-Domain Hand-offs

A and C provide authoritative reference facts. H06 retains only its own engagement/professional/preference information and source references.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The Alumni may initiate supported self-service changes. H06 administrative roles may perform permitted governed changes.

## 15.2 Separation of Duties, Overrides and Exceptions

No local override permits H06 to modify A- or C-owned canonical facts.

## 15.3 Business Audit and Evidence Obligations

Preserve the actor, affected H06-owned information, prior/current governed values as required by H06 audit rules, applicable consent/visibility evidence, and event history.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

A permitted H06-owned profile or preference change has been applied to the correct H06 aggregate and the appropriate H06 business event is recorded.

## 16.2 Postconditions

The Alumni engagement record reflects the current permitted value; identity, Student, Alumni Persona, qualification, and academic-source records remain unchanged and referenced.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

None material to this scenario outcome.

**Architecture assessment:** `PASS`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

None.
