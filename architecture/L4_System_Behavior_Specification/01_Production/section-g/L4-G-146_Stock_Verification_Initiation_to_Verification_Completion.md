# Stock Verification Initiation to Verification Completion

## 1. Specification Metadata

**Specification ID:** L4-G-146  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-146 — Stock Verification Initiation to Verification Completion  
**Primary Business Owner:** Library Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-146; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern an authorised library stock-verification cycle from scheduled commencement through physical verification, difference analysis, reporting and **Administrative Closure**, while leaving downstream missing-resource investigation, write-off, procurement and Finance settlement outside this boundary.

## 3. Required Business Context

- An authorised/scheduled verification cycle exists.
- Verification scope and a supported verification type are established.
- Applicable collection/catalogue context and responsible personnel context are available.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-146-BH-001 | Identify the verification cycle, scope, verification type, applicable collection and responsible personnel context. | L3-G-146 §§7,9 |
| L4-G-146-BH-002 | Progress through the authoritative workflow `Verification Scheduled → Physical Verification → Difference Analysis → Missing Resources → Verification Report → Administrative Closure` without inventing additional states. | L3-G-146 §§2,9,13 |
| L4-G-146-BH-003 | During physical verification and difference analysis, record supported findings including available, missing, damaged, misplaced and duplicate-record outcomes as applicable. | L3-G-146 §§2,9,12 |
| L4-G-146-BH-004 | Establish the verification report/outcome for the authorised scope and do not assert completion while required verification or difference analysis remains incomplete. | L3-G-146 §§9,11,16 |
| L4-G-146-BH-005 | Establish Administrative Closure only when the verification cycle is authoritatively complete and preserve verification history. | L3-G-146 §§3,9,13,15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-146-BH-006 | On authorised commencement, Library Management shall publish the canonical `LibraryVerificationStarted` event. On Administrative Closure/completion, Library Management shall publish the canonical `LibraryVerificationCompleted` event. E supplies personnel context where applicable and H owns notification delivery where required. | L3-G-146 §§2–3,9,14,16 |

## 6. Failure / Rejection / Continuation

- Verification shall not commence with an invented scope or unsupported verification type.
- Incomplete physical verification or difference analysis shall prevent false completion.
- Administrative Closure and `LibraryVerificationCompleted` shall not be asserted prematurely.
- Downstream investigation/write-off shall remain separate even when discrepancies are found.

## 7. Audit & Evidence Behavior

Preserve verification identity, scope, type, responsible personnel context, physical-verification outcome, difference analysis, classified findings, verification report, Administrative Closure and canonical event history.

## 8. Observable Completion Conditions

Completion occurs when the authorised scope has been verified, differences analysed, findings classified, the verification report established, Administrative Closure recorded and `LibraryVerificationCompleted` published by Library Management.

## 9. Traceability & Provenance

**Parent:** L3-G-146 — Stock Verification Initiation to Verification Completion  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

The upstream source does not define a detailed discrepancy-resolution matrix or mandatory verification-report approval rule. Those gaps are preserved. Technical scanning, devices, APIs, persistence, UI and infrastructure are deferred to L5.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
