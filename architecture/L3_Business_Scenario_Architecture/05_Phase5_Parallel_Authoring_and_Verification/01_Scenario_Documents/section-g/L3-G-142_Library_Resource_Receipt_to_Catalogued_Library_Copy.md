# Library Resource Receipt to Catalogued Library Copy

**Scenario ID:** L3-G-142  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Library Management  
**Participating Domains:** G, E, F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-G-142 |
| Canonical Title | Library Resource Receipt to Catalogued Library Copy |
| Primary Owner | Library Management |
| Participating Domains | G, E, F |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the complete institutional outcome in which an approved library acquisition is received, the received resource is catalogued, and the physical Library Copy is created so that it can proceed through the Library Copy lifecycle toward operational availability.

The locked L2 defines the acquisition workflow:

`Acquisition Requested → Vendor Reference → Approval → Resource Received → Cataloguing → Library Copy Created`.

It also states that Acquisition and Cataloguing govern introduction of new resources into the institutional library and that Procurement remains outside Library Management.

## 3. Business Outcome

An approved acquired resource has been received and catalogued, and one or more physical Library Copy records are created for the received resource under the authoritative Library Copy model.

The scenario does not invent accession numbering, barcode rules, catalogue fields, or procurement execution mechanics.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An approved library acquisition results in the physical resource being received by Library Management.

### 4.2 End Boundary

The received resource has been catalogued and the Library Copy has been created, ready to proceed to the subsequent copy-registration/availability workflow.

### 4.3 In Scope

- Identify approved acquisition.
- Record resource receipt.
- Verify received resource against acquisition reference.
- Establish catalogue information.
- Complete cataloguing.
- Create Library Copy.
- Preserve acquisition/cataloguing/copy history.
- Hand off to subsequent Library Copy operational processing.

### 4.4 Out of Scope

- Procurement execution.
- Vendor payment/accounting.
- Acquisition request approval.
- Detailed catalogue publication policy beyond authoritative L2.
- Circulation issue.
- Reservation.
- Technical identifiers or database design.

## 5. Ownership and Participating Domains

### Library Management — G

Owns Acquisition, Cataloguing, Catalogue and Library Copy business operations.

### People & Workforce — E

Participates where personnel context is required by the frozen scenario. E does not own library resource lifecycle.

### Finance — F

Participates where receipt/financial reference information is required. F retains accounting/payment ownership.

## 6. Business Actors, Roles and Responsibilities

### Library Acquisition/Cataloguing Authority

Receives the resource and performs the library-side verification and cataloguing work.

### Cataloguing Authority

Establishes the authoritative catalogue representation.

### Library Copy Authority

Creates the physical-copy representation.

### E Authority

Provides authoritative personnel context where applicable.

### Finance

Provides/consumes financial information where applicable without taking ownership of the Library Copy or Catalogue.

## 7. Preconditions and Entry Criteria

1. An approved acquisition exists.
2. A procurement/vendor reference exists where applicable.
3. The resource has been received.
4. The received resource can be identified and matched to the acquisition.
5. Required cataloguing information is available.
6. Library Copy creation is authorised.

## 8. Trigger

An approved library resource is received.

## 9. Main Business Flow

### Phase A — Record Receipt

1. Identify the approved acquisition.
2. Identify the vendor/procurement reference where applicable.
3. Record receipt of the resource.
4. Verify the received resource against the approved acquisition.
5. Record receipt outcome and relevant acquisition history.
6. Resolve any business discrepancy before treating the resource as successfully received.

### Phase B — Catalogue the Resource

7. Establish the resource's authoritative catalogue information.
8. Validate required catalogue information.
9. Complete cataloguing.
10. Preserve the relationship between acquisition and catalogue record.
11. Ensure the resource is represented under the authoritative Catalogue lifecycle.

### Phase C — Create Library Copy

12. Determine the physical Library Copy representation required for the received resource.
13. Create the Library Copy.
14. Associate the Library Copy with its authoritative Catalogue record.
15. Preserve acquisition, catalogue and copy lineage/history.
16. Establish the Library Copy in the appropriate initial lifecycle condition.

### Phase D — Complete

17. Make the Library Copy available to the subsequent copy-registration/availability process.
18. End the scenario.

The scenario does not directly perform circulation availability unless that is the next authoritative workflow.

## 10. Alternate Business Flows

### 10.1 Received Resource Does Not Match Acquisition

The resource is not incorrectly catalogued or copied against the wrong acquisition. The discrepancy is resolved under the applicable acquisition process.

### 10.2 Resource Receipt Is Partial

Only the received business outcome is recorded; no unsupported assumption is made about unreceived items.

### 10.3 Cataloguing Information Is Incomplete

Library Copy creation does not proceed as a completed outcome until required catalogue information is authoritative.

### 10.4 Cataloguing Identifies a Required Resource Correction

The catalogue is corrected under the Catalogue lifecycle/policy. No new local catalogue state is invented.

### 10.5 Multiple Physical Copies Are Received

Each physical copy is represented according to the authoritative Library Copy model. No arbitrary copy-count rule is introduced.

### 10.6 Finance Reference Is Required

Financial information is consumed from/through Finance as applicable. Finance does not own Catalogue or Library Copy.

## 11. Failure, Interruption and Business Recovery

### 11.1 Acquisition Reference Cannot Be Matched

Receipt is not converted into an unrelated catalogue/copy outcome.

### 11.2 Resource Cannot Be Reliably Identified

Cataloguing and copy creation remain unresolved until the authoritative resource identity is established.

### 11.3 Catalogue Completion Cannot Be Established

The resource is not treated as fully catalogued until the required catalogue outcome is authoritative.

### 11.4 Copy Creation Is Not Authorised

The resource remains within the acquisition/cataloguing process and does not receive an unsupported Library Copy outcome.

### 11.5 Financial Receipt Information Is Unavailable

The library-side receipt/cataloguing boundary is not redefined; Finance remains authoritative for financial processing.

No technical retry, queue, rollback, API, database or infrastructure behaviour is specified.

## 12. Business Rules and Decision Constraints

The L2 acquisition workflow is:

`Acquisition Requested → Vendor Reference → Approval → Resource Received → Cataloguing → Library Copy Created`.

Acquisition sources include Purchase, Donation, Exchange, Institutional Publication and Government Distribution, and remain configurable.

Vendor references may include Publisher, Book Supplier, Distributor and Donor.

The L2 state transition matrix defines:

- Catalogue: `Draft → Verified → Published → Archived`
- Library Copy: `Added → Available → Issued → Returned → Withdrawn`.

The scenario does not invent catalogue fields, copy identifiers, accession rules, barcode rules, or publication criteria.

## 13. State and Lifecycle Impact

The acquisition progresses through:

`Approved → Resource Received → Cataloguing → Library Copy Created`

within the L2 acquisition workflow.

The Catalogue follows its authoritative lifecycle:

`Draft → Verified → Published → Archived`.

The Library Copy follows:

`Added → Available → Issued → Returned → Withdrawn`.

The scenario's endpoint is **Library Copy Created**. It does not silently advance the copy to Available unless the subsequent authoritative workflow explicitly performs that transition.

## 14. Business Events and Cross-Domain Collaboration

The retrieved L2 material does not expose dedicated canonical event names for resource receipt, cataloguing completion, or Library Copy creation. Therefore this draft does not invent `ResourceReceived`, `Catalogued`, or `LibraryCopyCreated` events.

### Cross-domain collaboration

- **E:** participates where authoritative personnel context is required.
- **F:** participates in financial/receipt reference matters where applicable.
- **G:** owns acquisition, cataloguing, Catalogue and Library Copy outcomes.

The boundary between receipt, cataloguing and copy creation is preserved from the L2 workflow.

## 15. Business Authorization, Approval and Audit Obligations

Receipt, cataloguing and Library Copy creation follow the configured Library Management authority.

The acquisition reference, receipt outcome, cataloguing decision, Catalogue relationship, Library Copy creation and relevant personnel/financial references must remain auditable.

No technical access control/IAM implementation is specified.

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Approved acquisition identified.
- Receipt recorded.
- Received resource matched to acquisition.
- Required catalogue information established.
- Cataloguing completed.
- Catalogue relationship preserved.
- Library Copy created.
- Copy-to-Catalogue relationship established.
- Acquisition/catalogue/copy history preserved.
- Subsequent copy-processing boundary established.

### Postconditions

- The acquired resource is received within Library Management.
- A catalogue representation exists at the appropriate lifecycle condition.
- A Library Copy exists and references its Catalogue record.
- The Library Copy can proceed to subsequent registration/availability processing.
- Procurement and payment remain outside this scenario's ownership.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

The locked L2 does not expose dedicated event names for Resource Received, Cataloguing Completion or Library Copy Creation in the retrieved event vocabulary. No events are invented.

### Assumptions

- Exact catalogue completeness/publication rules remain governed by the authoritative Catalogue policy.
- Exact copy-registration details remain governed by the downstream Library Copy workflow.
- Finance remains authoritative for financial receipt/payment information.

### Deferred Questions

Confirm the canonical event vocabulary for acquisition receipt/cataloguing/copy creation before Architecture Approval/Lock if those events are mandatory in the final event register.
