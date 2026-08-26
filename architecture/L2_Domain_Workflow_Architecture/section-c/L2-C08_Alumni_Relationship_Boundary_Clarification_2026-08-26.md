# L2-C08 — Alumni Transition Workflows — Alumni Relationship Boundary Clarification

| Attribute | Value |
|---|---|
| Target | L2-C08 — Alumni Transition Workflows v1.0 |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-08-26 |
| Approved Through | ADR-C-ALUMNI-BOUNDARY-001 |
| Authority | Explicit Lead Architect remediation authorization recorded 2026-08-26 |

This clarification is canonical L2-C08 architecture and is intended only to remove ambiguity between the C08 Alumni Persona/transition fact and the H06 Alumni engagement relationship. It does not transfer ownership.

## 1. Canonical Meaning of the C08 Alumni Relationship

Where L2-C08 uses the terms **Alumni relationship**, **authoritative institutional Alumni relationship**, or event `AlumniRelationshipActivated`, those terms mean the Student Lifecycle-owned institutional fact that the Person has completed the governed Student-to-Alumni transition for the applicable institution/qualification context.

That fact is represented by the C08 Alumni Transition Record and the Student Lifecycle-owned Alumni Persona. It is not the H06 Alumni Profile and it is not an H06 engagement aggregate.

## 2. Canonical Handoff to H06

C08 remains sole owner of transition eligibility, transition approval coordination, execution, transition history, correction and transition events.

On successful completion C08 publishes `AlumniTransitionCompleted`. H06 may then establish the separate **Alumni engagement relationship** by creating its Alumni Profile and engagement aggregates.

Therefore:

- C08 outcome = authoritative Student-to-Alumni institutional identity/transition fact.
- H06 outcome = post-transition Alumni Profile and governed engagement relationship.

The two outcomes are sequential and must not be treated as duplicate ownership.

## 3. Scenario Boundary Rule

A cross-domain scenario may begin with C05 completion/exit evidence, pass through the mandatory C08 transition, and end with an H06 engagement relationship. Such a scenario is H06-owned only when its final business outcome is explicitly the H06 Alumni Profile/engagement relationship. The scenario must not describe H06 as creating the Alumni Persona or owning the Student-to-Alumni transition.

## 4. Events

No event is renamed by this clarification.

- `AlumniRelationshipActivated` remains a C08 transition fact.
- `AlumniTransitionCompleted` remains the canonical C08 handoff to H06.
- H06 may publish its own Alumni Profile/engagement events only after consuming the completed C08 handoff.

## 5. Downstream Effect

This clarification resolves the ownership-language ambiguity identified by L3-H-039 while preserving all existing C08 and H06 canonical owners.

## 6. Locked Decisions

The semantic distinction and cross-domain boundary in §§1–4 are Locked L2 decisions under ADR-C-ALUMNI-BOUNDARY-001.
