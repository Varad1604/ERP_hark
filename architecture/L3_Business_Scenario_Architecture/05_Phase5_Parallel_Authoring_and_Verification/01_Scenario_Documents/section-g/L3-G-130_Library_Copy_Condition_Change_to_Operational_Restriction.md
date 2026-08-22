# Library Copy Condition Change to Operational Restriction

**Scenario ID:** L3-G-130  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Library Management  
**Participating Domains:** G  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 2. Scenario Purpose

This scenario governs the complete business outcome in which a physical Library Copy's condition changes in a manner that makes ordinary circulation inappropriate or requires an operational restriction.

The scenario preserves the distinction between physical condition and operational status. The L2 identifies both as Library Copy information and provides configurable operational statuses including Available, Issued, Reserved, Reference Only, Under Verification, Lost, Damaged and Withdrawn. fileciteturn163file0

The scenario does not invent a new lifecycle state called `Restricted`. “Operational Restriction” is the business outcome; the actual copy status remains one of the authoritative/configured status values.

## 3. Business Outcome

The affected Library Copy has its physical condition recorded accurately, the applicable operational restriction is established, and the copy is prevented from being treated as ordinarily circulation-available when its condition or operational circumstances do not permit that use.

The condition change and resulting operational decision are preserved in the copy's history.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A physical Library Copy is observed or otherwise identified as having a condition change that may affect its operational use.

### 4.2 End Boundary

The copy's condition and resulting operational status are authoritatively recorded, and subsequent library operations respect the resulting restriction.

### 4.3 In Scope

- Identification of the affected Library Copy.
- Recording physical condition.
- Assessment of operational impact.
- Determination of applicable configured status.
- Restriction from ordinary circulation where required.
- Preservation of condition/status history.
- Audit.

### 4.4 Out of Scope

- Maintenance execution.
- Vendor repair.
- Acquisition/procurement.
- Finance/accounting.
- Catalogue bibliographic changes unless separately required.
- Student/personnel lifecycle.
- Technical inventory/device mechanics.

## 5. Ownership and Participating Domains

Library Management owns the Library Copy and its operational condition/status.

No external domain participates in the frozen scenario. If a later maintenance, acquisition, financial or personnel outcome is required, the relevant canonical owner remains responsible for that outcome.

## 6. Business Actors, Roles and Responsibilities

### Librarian

Assesses or authorises the operational treatment of the affected copy according to configured library authority.

### Library Assistant

May record observed physical condition and support operational assessment where configured.

The scenario does not create a separate maintenance role because maintenance execution is outside the locked G05 scope.

## 7. Preconditions and Entry Criteria

1. The affected Library Copy exists.
2. The copy can be identified.
3. A condition change has been observed or otherwise established.
4. An authorised library operator can assess the operational consequence.

## 8. Trigger

A physical condition change is identified for a Library Copy.

Examples of business observations supported by the L2 model include damage observations, lost-resource information and physical-condition information. The scenario does not assume a particular damage severity.

## 9. Main Business Flow

### Phase A — Identify the Copy and Condition Change

1. Identify the Library Copy.
2. Confirm its current operational status.
3. Record the observed physical condition.
4. Record relevant condition observations.
5. Establish whether the condition affects ordinary circulation.

### Phase B — Determine Operational Treatment

6. Assess the copy against applicable configured operational policy.
7. Determine the appropriate operational status.
8. If ordinary circulation is no longer appropriate, remove the copy from ordinary circulation availability.
9. If the condition requires verification, use the authoritative/configured Under Verification status.
10. If the condition establishes damage, use the authoritative/configured Damaged status.
11. If the copy is no longer operationally usable, use the applicable configured Lost/Withdrawn outcome where supported by the business decision.

### Phase C — Complete the Outcome

12. Record the resulting condition and operational status.
13. Preserve the copy history.
14. Ensure subsequent circulation operations respect the resulting status.
15. End the scenario with the copy under the required operational restriction.

The scenario does not perform repair or maintenance.

## 10. Alternate Business Flows

### 10.1 Condition Change Does Not Affect Circulation

If assessment determines that the copy remains suitable for ordinary circulation, its operational availability remains governed by the applicable status and no restriction is applied.

### 10.2 Condition Requires Verification

The copy is placed in the configured Under Verification status until the business verification outcome is established.

### 10.3 Copy Is Damaged

The copy is recorded as Damaged and is not treated as ordinarily circulation-available while that status applies.

### 10.4 Copy Is Lost

The copy is treated according to the configured Lost status. Any resulting fine/replacement outcome belongs to the relevant G05 fine/acquisition/clearance workflow rather than being invented here.

### 10.5 Copy Is Withdrawn

If the business decision establishes withdrawal, the copy follows the configured Withdrawn status. This scenario does not invent the withdrawal criteria.

### 10.6 Condition Change Occurs During Active Circulation

The condition is recorded and the copy's resulting operational treatment is established according to the applicable library workflow. Any borrowing/return consequence is handled by the relevant circulation scenario rather than being duplicated here.

## 11. Failure, Interruption and Business Recovery

### 11.1 Condition Cannot Be Established

The operational restriction is not asserted until sufficient business evidence is available to make the required decision.

### 11.2 Operational Consequence Is Unclear

The copy may remain under the applicable verification/review condition rather than being assigned an invented status.

### 11.3 Restriction Is Determined Invalid

The copy returns to the applicable authoritative operational status. This scenario does not define a new recovery state.

### 11.4 Physical Repair Is Required

The copy's operational restriction remains in force while the physical-repair outcome is handled by the appropriate downstream/upstream business process. No maintenance workflow is invented here.

No technical retry, queue, rollback or infrastructure behaviour is defined.

## 12. Business Rules and Decision Constraints

- Physical condition is part of Library Copy information.
- Operational status is part of Library Copy information.
- Operational statuses include Available, Issued, Reserved, Reference Only, Under Verification, Lost, Damaged and Withdrawn.
- Status values remain configurable.
- Every Library Copy references exactly one Catalogue record.
- Every lifecycle transition is audited.
- Circulation must respect Library Copy availability/status.
- Historical information must remain preserved.
- Library Management owns Library Copy Management.

The scenario does not establish a fixed severity model, repair SLA, automatic withdrawal threshold, replacement rule or fine amount.

## 13. State and Lifecycle Impact

The canonical Library Copy lifecycle is:

`Added → Available → Issued → Returned → Withdrawn`

The scenario does not invent a `Restricted` lifecycle state.

Operational restriction is represented by the applicable authoritative/configured status, such as:

- `Under Verification`
- `Damaged`
- `Lost`
- `Withdrawn`
- `Reference Only`

The precise status depends on the business assessment. The lifecycle itself remains governed by the L2 state model.

## 14. Business Events and Cross-Domain Collaboration

The retrieved G05 event catalogue does not expose a dedicated Library Copy condition-change or restriction event.

Therefore:

- no `LibraryCopyDamaged`;
- no `LibraryCopyRestricted`;
- no `CopyConditionChanged`

event is invented.

The resulting business fact remains within Library Management and is consumed by subsequent copy/circulation operations through the canonical business model.

## 15. Business Authorization, Approval and Audit Obligations

Library Management governs operational permissions and copy management.

The physical condition observation, assessment, resulting status and business decision must be auditable because G05 requires lifecycle transitions to be audited and maintains immutable operational history.

The exact authority for each operational-status decision remains configurable; no new role is introduced.

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Affected Library Copy identified.
- Physical condition change recorded.
- Operational impact assessed.
- Applicable operational status established.
- Ordinary circulation restricted where required.
- Copy history preserved.
- Subsequent library operations can recognise the resulting status.

### Postconditions

- The Library Copy has an authoritative recorded physical condition.
- Its operational status reflects the business decision.
- The copy is not treated as ordinarily circulation-available when its status prevents that use.
- Catalogue ownership and bibliographic information remain unchanged.
- Any later repair, replacement, fine or withdrawal outcome follows its appropriate library workflow.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

The locked L2 does not provide a dedicated event vocabulary for Library Copy condition/status changes. No event is invented.

### Assumptions

- Exact operational-status selection remains configurable.
- Physical-condition assessment is a library operational responsibility.
- Repair/maintenance execution is outside this scenario.

### Deferred Questions

If a future locked L2 revision introduces dedicated Library Copy condition/status events or a more specific condition lifecycle, the L3 event section should be aligned through architecture change control.
