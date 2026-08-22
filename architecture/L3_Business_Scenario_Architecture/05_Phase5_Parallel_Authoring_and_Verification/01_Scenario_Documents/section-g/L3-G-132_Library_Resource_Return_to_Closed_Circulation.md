# Library Resource Return to Closed Circulation

**Scenario ID:** L3-G-132  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Library Management  
**Participating Domains:** G, F, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 2. Scenario Purpose

This scenario governs the complete business outcome in which an actively borrowed Library Copy is returned, its physical condition and any overdue/loss/damage implications are assessed, the borrowing is closed, and the copy receives the appropriate subsequent operational treatment.

The L2 defines Return Processing as capable of recording Return Date, Return Time, Physical Condition, Damage Observations, Lost Resource Information and Outstanding Fine. Historical circulation records remain immutable. fileciteturn170file0

Library Management owns the circulation closure. Finance owns accounting/payment processing if a fine or replacement charge results. H owns notification delivery.

## 3. Business Outcome

The active borrowing is closed through a valid return outcome, the returned Library Copy receives its appropriate operational status based on the return condition, any applicable fine business outcome is established within Library Management, and the return is recorded in immutable circulation history.

The scenario does not automatically make every returned copy Available; the copy's actual condition determines its subsequent operational status.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A Library Copy associated with an Active borrowing is presented for return.

### 4.2 End Boundary

The borrowing reaches the canonical Closed condition and the returned copy's resulting operational status is established.

### 4.3 In Scope

- Identify Active borrowing.
- Identify member and Library Copy.
- Record return date/time.
- Assess physical condition.
- Record damage/lost-resource information where applicable.
- Determine overdue/fine implications.
- Close circulation.
- Establish copy operational status.
- Publish `ResourceReturned`.
- Preserve immutable circulation history.
- Notify through H where required.

### 4.4 Out of Scope

- Fine accounting/payment.
- Membership changes.
- New reservation creation.
- Catalogue modification.
- Physical repair.
- Student/personnel lifecycle.
- Technical implementation.

## 5. Ownership and Participating Domains

### Library Management — G

Owns circulation, Library Copy operational status and Library Fine operations.

### Finance — F

Owns accounting and payment processing. G may calculate a library fine and provide the financial reference; F owns settlement.

### Communication — H

Owns notification delivery.

## 6. Business Actors, Roles and Responsibilities

### Librarian / Circulation Desk Operator

Processes the return, records condition, and establishes the resulting circulation/copy outcome.

### Member

Returns the resource.

### Finance

Handles accounting/payment when the library fine outcome requires financial processing.

### H

Consumes the return/fine business outcome for notification.

The G05 authorization matrix identifies Librarian / Circulation Desk as typical authorization for return processing. fileciteturn173file0

## 7. Preconditions and Entry Criteria

1. An Active borrowing exists.
2. The borrowed Library Copy can be identified.
3. The member can be identified.
4. Return processing authority is available.
5. The physical copy is presented or the return condition can otherwise be established.

## 8. Trigger

A borrowed Library Copy is returned.

## 9. Main Business Flow

### Phase A — Identify Active Circulation

1. Identify the returned Library Copy.
2. Identify the associated Active borrowing.
3. Confirm the borrowing/member relationship.
4. Establish the expected due date.

### Phase B — Process Return

5. Record the return date and time.
6. Inspect/record physical condition.
7. Record damage observations where applicable.
8. Determine whether the resource is returned complete and usable.
9. Determine whether the return is overdue under configured borrowing policy.
10. Determine whether a fine is applicable.

### Phase C — Establish Circulation Outcome

11. Close the borrowing.
12. Record the completed circulation history.
13. Publish `ResourceReturned`.
14. Determine the Library Copy's subsequent operational status.

### Phase D — Complete Copy Treatment

15. If the copy is suitable for ordinary circulation, establish the applicable Available condition.
16. If the copy requires verification, damage handling, loss handling or another operational restriction, establish the applicable configured status.
17. Preserve the return and condition history.
18. Make the outcome available to subsequent reservation/circulation workflows.

### Phase E — Complete

19. Provide relevant business information to H for governed notification.
20. Provide fine/payment reference information to Finance where applicable.
21. End the scenario with the circulation closed.

## 10. Alternate Business Flows

### 10.1 Overdue Return

If the due date has been exceeded, the configured fine policy is evaluated. G05 defines `ResourceOverdue` as the canonical event for a due date being exceeded and supports overdue fine calculation. fileciteturn167file0turn170file0

### 10.2 Damaged Resource

The damage observation is recorded. The Library Copy receives the applicable operational status and is not automatically treated as Available.

### 10.3 Lost Resource

Where the resource is identified as lost, the loss information is recorded and the applicable fine/replacement process is initiated through the appropriate G05 workflow.

### 10.4 Return Requires Verification

The copy may remain under the configured `Under Verification` operational status until the business assessment is complete.

### 10.5 Return Is Suitable for Circulation

The copy returns to Available when the business assessment establishes that it is suitable for ordinary circulation.

### 10.6 Fine Requires Finance Processing

G calculates/manages the library fine operationally; Finance owns accounting and payment processing. G does not close the financial account.

## 11. Failure, Interruption and Business Recovery

### 11.1 Active Borrowing Cannot Be Established

The return is not closed as a normal circulation return until the authoritative borrowing relationship is established.

### 11.2 Copy Condition Cannot Be Determined

The copy remains under the appropriate business review/verification condition rather than being incorrectly returned to Available.

### 11.3 Fine Outcome Cannot Be Established

The circulation closure remains governed by the library return decision; the fine outcome follows the configured fine process. No unsupported financial rule is invented.

### 11.4 Historical Record Is Required

Historical circulation records remain immutable and are not deleted. fileciteturn170file0

No technical retry, queue, rollback or infrastructure behaviour is described.

## 12. Business Rules and Decision Constraints

Return processing may record:

- Return Date.
- Return Time.
- Physical Condition.
- Damage Observations.
- Lost Resource Information.
- Outstanding Fine.

Historical circulation records remain immutable. fileciteturn170file0

Fine categories may include Overdue Fine, Lost Resource, Damaged Resource, Replacement Charge and Administrative Penalty, with categories configurable. Finance remains owner of accounting/payment. fileciteturn170file0

The scenario does not invent fine amounts, grace periods, damage thresholds or replacement values.

## 13. State and Lifecycle Impact

The canonical Circulation lifecycle is:

`Issued → Active → Returned → Closed`

This scenario completes:

`Active → Returned → Closed`

The Library Copy lifecycle separately supports:

`Added → Available → Issued → Returned → Withdrawn`

The returned copy's operational status is established based on its condition and configured operational policy.

No invented state such as `Return Processing`, `Fine Pending`, or `Inspection Pending` is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

The applicable canonical events are:

- **`ResourceReturned`** — Resource returned.
- **`ResourceOverdue`** — Due date exceeded, where applicable.
- **`FineCalculated`** — Fine generated, where applicable.

These are canonical G05 events. fileciteturn167file0

Library Management is the canonical publisher.

### 14.2 Business Collaboration

- Finance receives the applicable financial outcome/reference without acquiring fine-management ownership.
- H consumes the return/fine events for communication.
- Library Management remains owner of circulation, copy status and fine calculation.

## 15. Business Authorization, Approval and Audit Obligations

The G05 authorization matrix identifies Librarian / Circulation Desk as typical authorization for returning resources. fileciteturn173file0

The return date/time, physical condition, damage/loss observations, fine outcome and circulation closure must remain auditable.

Historical circulation records are immutable.

No technical access control/IAM implementation is described.

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Active borrowing identified.
- Member/copy relationship verified.
- Return recorded.
- Physical condition recorded.
- Overdue/fine implications assessed.
- Borrowing closed.
- `ResourceReturned` published.
- Applicable copy status established.
- History preserved.
- Finance/H collaboration completed where applicable.

### Postconditions

- Circulation is Closed.
- The returned Library Copy has an authoritative operational status.
- A suitable copy may become Available for subsequent circulation/reservation workflows.
- Any fine remains governed by Library Management and Finance boundaries.
- Historical circulation remains immutable.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

No material ownership, lifecycle or publisher conflict identified.

### Assumptions

- Fine categories and calculation rules remain configurable.
- Copy availability after return depends on physical/operational condition.
- Notification requirements remain policy driven.

### Deferred Questions

None material to the frozen scenario outcome.
