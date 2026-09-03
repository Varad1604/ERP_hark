# Lost or Damaged Transport Pass to Replacement Pass

## 1. Specification Metadata

**Specification ID:** L4-G-115  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-115 — Lost or Damaged Transport Pass to Replacement Pass  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-115; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern a lost or damaged Transport Pass through validation and replacement issuance while preserving the authoritative pass history.

## 3. Required Business Context

- The affected pass and holder context are authoritative.
- Replacement eligibility and policy can be evaluated.
- Transport Management owns the Transport Pass lifecycle.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-115-BH-001 | Register the loss or damage against the authoritative Transport Pass. | L3-G-115 §§3, 8–9 |
| L4-G-115-BH-002 | Validate holder context and applicable replacement eligibility. | L3-G-115 §§7, 9, 12 |
| L4-G-115-BH-003 | Apply the authoritative replacement outcome without inventing a Lost or Damaged lifecycle state. | L3-G-115 §§12–13 |
| L4-G-115-BH-004 | Issue the replacement pass when authorised and establish it as the authoritative operational pass. | L3-G-115 §§3, 9, 16 |
| L4-G-115-BH-005 | Preserve the original pass, replacement relationship and validity history. | L3-G-115 §§12, 15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-115-BH-006 | On successful replacement issuance, Transport Management shall publish `PassReplaced`. | L3-G-115 §§14, 16 |

## 6. Failure / Rejection / Continuation

- Unverified loss/damage or failed eligibility shall prevent replacement issuance.
- Recovery of the original pass shall not create duplicate active authorisation; the applicable upstream pass policy governs it.
- Failed approval shall leave the replacement non-effective.

## 7. Audit & Evidence Behavior

Preserve report, validation, authority, affected-pass outcome, replacement issuance and original-to-replacement history.

## 8. Observable Completion Conditions

The replacement pass is issued and authoritative, `PassReplaced` is published, and the original/replacement history remains traceable.

## 9. Traceability & Provenance

**Parent:** L3-G-115 — Lost or Damaged Transport Pass to Replacement Pass  
**Primary verification source:** L3-G-115 §§3, 9–17; authoritative L2-G04 Transport Management workflow architecture.

## 10. Open Issues / Deferred Design

No unresolved event identifier remains for the core replacement outcome. Technical implementation remains deferred to L5/L6.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
