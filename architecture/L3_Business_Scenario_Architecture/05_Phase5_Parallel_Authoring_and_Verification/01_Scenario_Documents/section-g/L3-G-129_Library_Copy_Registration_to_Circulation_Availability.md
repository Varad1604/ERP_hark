# Library Copy Registration to Circulation Availability

**Scenario ID:** L3-G-129  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Library Management  
**Participating Domains:** G, E  
**Scenario Type:** Critical  
**Priority:** P0  
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
| Final L3 ID | L3-G-129 |
| Canonical Title | Library Copy Registration to Circulation Availability |
| Primary Owner | Library Management |
| Participating Domains | G, E |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the complete business outcome through which a physical library resource is registered as a Library Copy and becomes available for circulation.

The scenario begins from an available Catalogue record and establishes the operational identity and information of the physical copy. It ends when the copy is available for circulation.

The L2 explicitly separates intellectual Catalogue records from physical Library Copies. Each physical copy references exactly one Catalogue record, and the copy workflow is:

`Catalogue Available → Copy Added → Accession Number Assigned → Barcode Assigned → Available For Circulation`.

## 3. Business Outcome

A physical library copy is registered against its authoritative Catalogue record, receives its required operational identity and location/condition information, and becomes **Available** for circulation.

The copy is therefore eligible to participate in subsequent circulation operations, subject to membership and borrowing rules.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A Catalogue record is available and a physical copy is ready to be registered.

### 4.2 End Boundary

The Library Copy reaches the operational condition in which it is Available for Circulation.

### 4.3 In Scope

- Identification of the Catalogue record.
- Registration of the physical copy.
- Accession number assignment.
- Barcode assignment.
- Library branch/section/location information.
- Physical condition recording.
- Operational availability assessment.
- Registration completion.
- Audit/history.

### 4.4 Out of Scope

- Catalogue creation or bibliographic management.
- Circulation/issue.
- Reservation.
- Borrowing.
- Acquisition procurement.
- Finance/accounting.
- Student/personnel lifecycle.
- Technical inventory databases, barcode hardware, APIs, UI or infrastructure.

## 5. Ownership and Participating Domains

Library Management owns Library Copy Management.

Each Library Copy references exactly one Catalogue record. Catalogue ownership remains within G05 but is a separate aggregate from the physical copy.

Section E provides the authoritative personnel context for librarians, library assistants and administrative staff. E does not own the Library Copy.

## 6. Business Actors, Roles and Responsibilities

### Librarian

Authorises or performs library-copy registration according to configured operational authority.

### Library Assistant

May perform registration activities where configured.

### Section E

Provides authoritative personnel information for staff performing the activity; it does not own the copy.

## 7. Preconditions and Entry Criteria

1. A valid Catalogue record exists.
2. The physical resource can be associated with that Catalogue record.
3. The copy can be identified as a distinct physical copy.
4. Required branch/location information is available.
5. Required registration authority is available.

## 8. Trigger

A physical library resource is ready to be registered as a Library Copy against an available Catalogue record.

## 9. Main Business Flow

### Phase A — Establish the Catalogue Relationship

1. Identify the authoritative Catalogue record.
2. Confirm that the physical resource belongs to that intellectual work.
3. Create/register the Library Copy against exactly one Catalogue record.

### Phase B — Establish Copy Identity

4. Assign the copy's Accession Number.
5. Assign the copy's Barcode.
6. Confirm that the copy's operational identity is unique within the applicable library context.

### Phase C — Establish Operational Information

7. Record the Library Branch.
8. Record the Library Section where applicable.
9. Record Rack/Shelf information where applicable.
10. Record the Physical Condition.
11. Establish the applicable Operational Status.

The L2-supported copy statuses include Available, Issued, Reserved, Reference Only, Under Verification, Lost, Damaged and Withdrawn, with status values remaining configurable.

### Phase D — Establish Availability

12. Confirm that the copy satisfies the business conditions for circulation availability.
13. Set the copy to the applicable Available condition.
14. Preserve registration history.
15. End the scenario with the copy available for circulation.

The scenario does not issue the copy to a member; circulation begins separately.

## 10. Alternate Business Flows

### 10.1 Catalogue Record Not Available

Registration cannot proceed because every Library Copy must reference a valid Catalogue record.

### 10.2 Copy Identity Cannot Be Established

The copy does not become available for circulation until the required accession/barcode identity is established.

### 10.3 Copy Is Not Suitable for Circulation

Where the physical condition or operational circumstances do not permit circulation, the copy is not treated as Available. The scenario records the applicable supported/configured operational status rather than inventing a new state.

### 10.4 Reference-Only Copy

If the copy is configured as Reference Only, it is not treated as an ordinary circulation-available copy. The copy remains governed by the applicable operational status.

### 10.5 Copy Requires Verification

A copy requiring verification does not become circulation-available until the verification condition is resolved.

### 10.6 Multiple Copies of One Catalogue Record

Multiple physical copies may reference the same Catalogue record, because the L2 separates intellectual work from physical copies. Each copy nevertheless has its own operational identity.

## 11. Failure, Interruption and Business Recovery

### 11.1 Invalid Catalogue Association

The copy is not registered as available until a valid Catalogue relationship is established.

### 11.2 Missing Operational Identity

The copy remains incomplete until the required accession/barcode identity is established.

### 11.3 Physical Condition Prevents Availability

The copy is kept out of ordinary circulation availability until its condition is resolved under applicable library operations.

### 11.4 Registration Review Rejects the Copy

The copy does not become available and the registration outcome is recorded.

No technical retry, queue, rollback or infrastructure mechanics are described.

## 12. Business Rules and Decision Constraints

- Every Library Copy shall reference exactly one Catalogue record.
- Physical copies are managed separately from Catalogue records.
- Typical copy information includes Accession Number, Barcode, Library Branch, Library Section, Rack, Shelf, Physical Condition and Operational Status.
- Detailed shelf positioning remains intentionally simple.
- Operational status values remain configurable.
- Historical operational information must remain auditable.
- Library Management owns copy management.
- Circulation transactions reference active Library Copies.
- A Library Copy is not a borrowing or circulation aggregate.

The scenario does not invent additional copy identity fields or physical-location requirements.

## 13. State and Lifecycle Impact

The G05 state transition matrix defines the Library Copy lifecycle as:

`Added → Available → Issued → Returned → Withdrawn`

This scenario covers:

`Added → Available`

The status model also supports operational conditions such as Reserved, Reference Only, Under Verification, Lost and Damaged. These are operational statuses and are not converted into invented lifecycle transitions.

## 14. Business Events and Cross-Domain Collaboration

The retrieved G05 event catalogue explicitly lists membership, catalogue, circulation, reservation, fine, acquisition, verification and clearance events, but it does **not** expose a dedicated Library Copy registration/availability event in the retrieved event vocabulary. Therefore no copy-specific event name is invented.

The applicable business collaboration is:

- Catalogue provides the authoritative intellectual-resource reference.
- E provides personnel context for the authorised library operator.
- Library Management owns the resulting Library Copy and its availability.

## 15. Business Authorization, Approval and Audit Obligations

The G05 authorization model identifies Librarian as the typical authority for adding Library Copies.

Accordingly:

- copy registration must follow configured library authority;
- copy identity and physical condition must be traceable;
- lifecycle transitions must be audited;
- historical information must not be silently discarded.

No technical authorization mechanism is defined.

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Valid Catalogue record identified.
- Library Copy registered.
- Accession Number assigned.
- Barcode assigned.
- Applicable branch/location recorded.
- Physical condition recorded.
- Operational status established.
- Copy reaches Available for Circulation.
- Registration history preserved.

### Postconditions

- A distinct Library Copy exists for the physical resource.
- The copy references exactly one Catalogue record.
- The copy is available for eligible circulation.
- Subsequent issue/borrowing workflows may use the copy.
- Catalogue lifecycle is unaffected.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

The locked L2 does not expose a dedicated Library Copy business event for registration/availability in the retrieved event catalogue. No event is invented.

### Assumptions

- Exact uniqueness constraints for accession/barcode identifiers are governed by the applicable library configuration.
- Exact criteria for circulation availability remain governed by configured operational policy.

### Deferred Questions

A dedicated Library Copy event vocabulary should be established upstream if one is required before L3 approval/lock.
