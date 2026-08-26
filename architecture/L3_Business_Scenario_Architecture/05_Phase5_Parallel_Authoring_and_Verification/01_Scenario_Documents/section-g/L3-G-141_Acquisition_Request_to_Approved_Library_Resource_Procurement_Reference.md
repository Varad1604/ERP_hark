# Acquisition Request to Approved Library Resource Procurement Reference

**Scenario ID:** L3-G-141  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Library Management  
**Participating Domains:** G, E, F  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-G-141 |
| Canonical Title | Acquisition Request to Approved Library Resource Procurement Reference |
| Primary Owner | Library Management |
| Participating Domains | G, E, F |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the complete institutional outcome in which Library Management identifies and submits a need to acquire a library resource, records the acquisition request and vendor reference, obtains the required approval, and establishes the approved procurement reference for subsequent resource receipt.

The locked L2 states that Acquisition and Cataloguing govern introduction of new resources into the institutional library and that **Procurement remains outside this domain**. The acquisition workflow is:

`Acquisition Requested → Vendor Reference → Approval → Resource Received → Cataloguing → Library Copy Created`.

Acquisition sources may include Purchase, Donation, Exchange, Institutional Publication and Government Distribution, and remain configurable.

## 3. Business Outcome

A valid library acquisition need has been approved and has an authoritative vendor/procurement reference for subsequent receipt processing.

Library Management does not execute procurement or assume ownership of procurement accounting.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

Library Management identifies a resource requirement and initiates an acquisition request.

### 4.2 End Boundary

The acquisition request has received the required approval and the vendor/procurement reference is established for subsequent receipt.

### 4.3 In Scope

- Identify the resource need.
- Record acquisition request.
- Record acquisition source.
- Establish vendor reference where applicable.
- Validate acquisition information.
- Obtain required approval.
- Record approval outcome.
- Establish approved procurement reference.
- Preserve acquisition history.

### 4.4 Out of Scope

- Procurement execution.
- Vendor contracting.
- Purchase-order processing owned elsewhere.
- Payment/accounting execution.
- Resource receipt.
- Cataloguing.
- Library Copy creation.
- Technical procurement integration.

## 5. Ownership and Participating Domains

### Library Management — G

Owns the Library acquisition record and library-side acquisition workflow.

### People & Workforce — E

Participates where personnel/employee context is required by the frozen scenario. E does not own the acquisition.

### Finance — F

Participates in financial/procurement governance as applicable. Finance ownership remains limited to its authoritative financial responsibilities.

The L2 explicitly states Procurement remains outside Library Management.

## 6. Business Actors, Roles and Responsibilities

### Library Acquisition Authority

Identifies the requirement and prepares the acquisition request.

### Library Approver

Provides the required approval according to institutional policy.

### E Authority

Provides authoritative personnel context where required.

### F Authority

Participates in the applicable financial/procurement approval or reference process according to institutional policy.

No unsupported procurement actor or authority is invented.

## 7. Preconditions and Entry Criteria

1. A legitimate library resource requirement exists.
2. The requested resource can be described sufficiently for acquisition processing.
3. Applicable acquisition source is identifiable.
4. Required vendor information can be recorded where relevant.
5. Applicable approval policy is available.
6. Required authority is available.

## 8. Trigger

Library Management initiates an acquisition request for a new library resource.

## 9. Main Business Flow

### Phase A — Establish Acquisition Need

1. Identify the library resource requirement.
2. Record the acquisition request.
3. Identify the proposed acquisition source.
4. Record the required resource/vendor reference information.
5. Preserve the acquisition rationale and relevant business information.

### Phase B — Establish Vendor/Procurement Reference

6. Where applicable, identify the Publisher, Book Supplier, Distributor or Donor reference.
7. Record the vendor/reference information.
8. Validate the acquisition information against applicable library policy.
9. Establish the acquisition record for approval.

The L2 permits operational references for Publisher, Book Supplier, Distributor and Donor.

### Phase C — Approval

10. Submit the acquisition for required approval.
11. Evaluate the request against applicable institutional approval policy.
12. Record the approval decision.
13. If approved, establish the procurement/vendor reference required for subsequent receipt.
14. Preserve the approval history.

### Phase D — Complete

15. Make the approved acquisition available to the subsequent Resource Received workflow.
16. End the scenario.

The resource is not yet received, catalogued or represented as a Library Copy.

## 10. Alternate Business Flows

### 10.1 Acquisition Is Rejected

The acquisition does not progress to the approved procurement reference. The decision and reason are preserved where required.

### 10.2 Additional Approval Is Required

The acquisition remains under the applicable approval process until the required authority decides.

### 10.3 Acquisition Source Is Donation

The vendor/reference concept is applied only where relevant; donor information may be recorded as the operational reference.

### 10.4 Acquisition Source Is Exchange or Institutional Publication

The applicable source is recorded without inventing procurement obligations that do not apply.

### 10.5 Vendor Reference Is Not Applicable

The acquisition may proceed using the applicable source/reference model. No artificial vendor is created.

### 10.6 Finance Review Is Required

The request is evaluated through the applicable institutional financial/procurement governance. Finance does not become owner of the Library acquisition aggregate.

## 11. Failure, Interruption and Business Recovery

### 11.1 Acquisition Need Is Insufficiently Defined

Approval does not proceed until the required business information is established.

### 11.2 Vendor/Source Reference Cannot Be Established

No unsupported vendor is assigned. The acquisition remains unresolved until the appropriate source information is available.

### 11.3 Approval Cannot Be Established

The acquisition does not become approved through an invented substitute authority.

### 11.4 Procurement Is Outside G

Library Management records the approved acquisition reference but does not describe procurement execution mechanics.

### 11.5 Financial Consequence Is Pending

Finance handles the financial process according to its authoritative policy; G preserves the approved acquisition business outcome.

No technical retry, queue, rollback, API or infrastructure behaviour is specified.

## 12. Business Rules and Decision Constraints

The locked L2 establishes:

- Procurement remains outside Library Management.
- Acquisition sources are configurable.
- Vendor references may cover Publisher, Book Supplier, Distributor and Donor.
- Approval is a defined stage of the acquisition workflow.

The acquisition workflow is:

`Acquisition Requested → Vendor Reference → Approval → Resource Received → Cataloguing → Library Copy Created`.

This scenario ends at the approved acquisition/procurement-reference boundary.

No purchase amount, vendor-selection rule, approval threshold, procurement SLA, or payment rule is invented.

## 13. State and Lifecycle Impact

This scenario covers the business progression from Acquisition Requested through Vendor Reference and Approval.

It does not invent an acquisition lifecycle state beyond what is expressed by the L2 workflow.

The next authoritative business outcome is Resource Received.

Catalogue and Library Copy lifecycles do not begin until their respective later outcomes.

## 14. Business Events and Cross-Domain Collaboration

The retrieved L2 material does not expose a dedicated canonical acquisition-request or acquisition-approval event name. Therefore this draft does not invent `AcquisitionRequested` or `AcquisitionApproved`.

### Cross-domain collaboration

- **E:** provides personnel context where required.
- **F:** participates in financial/procurement governance where required.
- **G:** owns the library acquisition record and approval outcome within its boundary.
- Procurement execution remains outside G.

## 15. Business Authorization, Approval and Audit Obligations

Approval is explicitly part of the L2 acquisition workflow.

The request, acquisition source, vendor/reference information, approval decision, approving authority and resulting procurement reference must be auditable.

Exact approval thresholds and authorities remain upstream institutional policy.

No technical access control/IAM implementation is defined.

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Acquisition need recorded.
- Acquisition source recorded.
- Vendor/source reference established where applicable.
- Required validation completed.
- Required approval completed.
- Approval decision recorded.
- Approved procurement reference established.
- Acquisition history preserved.

### Postconditions

- Acquisition is approved within the Library Management boundary.
- A procurement/vendor reference exists for subsequent receipt processing.
- No resource is yet treated as received.
- No catalogue record or Library Copy is created by this scenario.
- Procurement execution remains outside G.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

The locked L2 establishes the acquisition workflow and states that procurement remains outside G, but does not expose a detailed cross-domain procurement contract/event vocabulary in the retrieved source. No such contract/event is invented.

### Assumptions

- Exact approval authority is governed by institutional policy.
- Exact procurement reference semantics remain defined by the participating procurement/Finance architecture.
- Acquisition sources remain configurable.

### Deferred Questions

Confirm the canonical procurement reference/event vocabulary at the relevant upstream/downstream architecture level before Architecture Approval/Lock if mandatory.
