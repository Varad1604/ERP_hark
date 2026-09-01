# Library Resource Receipt to Catalogued Library Copy

## 1. Specification Metadata
**Specification ID:** L4-G-142  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Library Management  
**Realizes Scenario:** L3-G-142 — Library Resource Receipt to Catalogued Library Copy  
**Primary Business Owner:** Library Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-G-142 — Library Resource Receipt to Catalogued Library Copy

## 2. Behavioral Objective & Boundary
SutraERP shall govern receipt of an approved library acquisition through verification, cataloguing and creation of the Library Copy without executing procurement or silently advancing downstream circulation.

**Start:** Approved acquisition is received.  
**End:** Resource is catalogued and Library Copy is created.

## 3. Required Business Context
- Approved acquisition and applicable procurement/vendor reference exist where relevant.
- Received resource can be identified.
- Required cataloguing information and copy-creation authority are available.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-142-BH-001 | Record receipt and verify the received resource against the approved acquisition. | L3-G-142 §§7,9 |
| L4-G-142-BH-002 | Prevent completion when the received resource cannot be matched to the authoritative acquisition. | L3-G-142 §10 |
| L4-G-142-BH-003 | Establish and validate the authoritative Catalogue representation for the received resource. | L3-G-142 §§9,12 |
| L4-G-142-BH-004 | Create the Library Copy and associate it with the authoritative Catalogue record. | L3-G-142 §§3,9 |
| L4-G-142-BH-005 | Preserve acquisition, receipt, catalogue and copy lineage/history. | L3-G-142 §§3,15,16 |

## 5. Cross-Domain & Event Behavior
E may provide personnel context and F may provide financial/receipt context where applicable. Procurement remains outside G. The available source does not establish dedicated receipt/cataloguing/copy-creation event names, so none are invented.

## 6. Failure / Rejection / Continuation
- Resource mismatch shall prevent unrelated catalogue/copy creation.
- Incomplete authoritative catalogue information shall prevent completed copy creation.
- Unauthorised copy creation shall not produce a completed Library Copy outcome.
- The Library Copy shall not be silently advanced to Available by this scenario.

## 7. Audit & Evidence Behavior
Preserve acquisition reference, receipt outcome, verification, catalogue decision, Catalogue relationship, copy creation authority and lineage.

## 8. Observable Completion Conditions
Completion occurs when receipt is verified, cataloguing is complete, a Library Copy exists against its Catalogue record, and lineage/history is preserved.

## 9. Traceability & Provenance
**Parent:** L3-G-142 — Library Resource Receipt to Catalogued Library Copy  
**Primary verification source:** L3-G-142; locked L2 Acquisition/Cataloguing workflow

## 10. Open Issues / Deferred Design
Exact catalogue completeness and copy-registration rules remain upstream/downstream policy concerns. Procurement, payment, APIs, persistence and technical implementation remain outside L4.