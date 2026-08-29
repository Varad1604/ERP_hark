# Withdrawn Registration Reinstatement to Authoritative Valid Registration

**Scenario ID:** L3-I-024  
**Participating Domains:** C, I  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Examination Registration  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2-I04 Examination Registration Workflows; applicable Student Lifecycle architecture  
**Referenced By:** Downstream examination workflows consuming Examination Registration; None yet

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-I-024 |
| Canonical Title | Withdrawn Registration Reinstatement to Authoritative Valid Registration |
| Primary Owner | Examination Registration |
| Participating Domains | C, I |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Govern restoration of previously withdrawn examination participation where institutional policy permits reinstatement, without erasing the historical withdrawal.

## 3. Business Outcome

A previously withdrawn registration or eligible subject is reinstated through an authorised business process and becomes the authoritative valid registration position with withdrawal history preserved.

## 4. Scenario Boundary and Scope

**Start:** A withdrawn registration/subject is identified for possible reinstatement.  
**In scope:** reinstatement eligibility, policy/timing review, approval, restoration, audit, and resulting registration lifecycle.  
**Out of scope:** policy change, session reopening, Finance processing, hall-ticket issuance, or technical restoration.  
**End:** registration is restored as valid or remains withdrawn after a governed decision.

## 5. Ownership and Participating Domains

**Examination Registration (I):** owns registration and reinstatement outcome. **Student Lifecycle (C):** provides candidate/academic context and retains ownership of student information.

## 6. Business Actors, Roles and Responsibilities

Candidate may request reinstatement where policy permits; Examination Administrator manages the case; Controller of Examination governs policy/approval where required; Department Administration may verify applicable requirements.

## 7. Preconditions and Entry Criteria

A prior withdrawn registration/subject exists; candidate and examination context are identifiable; reinstatement is a permitted business possibility; examination opportunity remains applicable unless an upstream-approved path says otherwise; current eligibility requirements can be assessed; required authority is available.

## 8. Trigger

A request or institutional determination is raised to restore previously withdrawn examination participation.

## 9. Main Business Flow

1. Identify the withdrawn authoritative registration.  2. Establish candidate, session, category and withdrawn scope.  3. Evaluate current eligibility, withdrawal policy and timing.  4. Obtain required approval.  5. Restore the valid registration position.  6. Apply any remaining review/approval/confirmation/locking requirements.  7. Preserve withdrawal and reinstatement history.  8. Publish applicable existing registration lifecycle events.  9. Make the valid registration available to downstream workflows subject to their prerequisites.

## 10. Alternate Business Flows

**Subject-level reinstatement:** restore only permitted withdrawn subjects.  
**Full reinstatement:** restore complete withdrawn participation where policy permits.  
**Administrative reinstatement:** institution initiates the same governed process.  
**Revalidation:** current requirements are reassessed before restoration.

## 11. Failure, Interruption and Business Recovery

If reinstatement is not permitted, the withdrawn position remains authoritative. If the examination opportunity is no longer applicable, do not restore unless an upstream-approved path establishes eligibility. If current eligibility fails or approval is denied, retain the withdrawn position. Any required correction follows the amendment process.

## 12. Business Rules and Decision Constraints

A withdrawn registration does not automatically become valid; reinstatement requires an approved business basis and authority; restored registration must satisfy current eligibility/policy; withdrawal history remains preserved; registration ownership remains with Examination Registration; confirmation/locking requirements still apply.

## 13. State and Lifecycle Impact

Uses the existing withdrawal and restoration capabilities. No new reinstatement state is invented. A successful reinstatement restores the appropriate existing valid lifecycle position and may require subsequent confirmation or locking.

## 14. Business Events and Cross-Domain Collaboration

L2-I04 does not define **Registration Reinstated**. Where reinstatement is represented as an approved registration modification, **Registration Amended** may apply; **Registration Confirmed** and **Registration Locked** apply when those milestones occur. Examination Registration remains the publisher.

## 15. Business Authorization, Approval and Audit Obligations

Reinstatement must be supported by applicable policy and required examination authority approval. Historical withdrawal and reinstatement decisions remain traceable and auditable. No technical access control/IAM is defined.

## 16. Completion Criteria and Postconditions

**Complete when:** reinstatement eligibility is established, required approval exists, valid registration is restored, history is preserved, and applicable lifecycle events are published. **Afterward:** restored registration continues through remaining lifecycle stages.

## 17. Open Issues, Assumptions and Deferred Questions

L2-I04 explicitly supports reinstatement through approved processes but does not define a dedicated reinstatement event or distinct state. Exact authority/timing remains institution-specific. Financial consequences remain outside Examination Registration.

