# Published Financial Obligation to Accounts Receivable Recognition

# 1. Scenario Metadata

**Scenario ID:** L3-F-039  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Accounts Receivable  
**Participating Domains:** C, F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-F00_Finance_Domain_Model_and_Foundation.md`; `L2-F07_Accounts_Receivable_Management.md`; frozen Phase 4 Stage 3 Section F canonical scenario register  
**Referenced By:** None yet  

---
# 2. Scenario Purpose

This scenario governs recognition of an institutional receivable after an authoritative Finance bounded context has established and published a valid financial obligation. It gives Accounts Receivable an authoritative, traceable representation of the amount owed while preserving the originating bounded context's ownership of the obligation.

---

# 3. Business Outcome

One valid published financial obligation is represented by one Accounts Receivable-owned `Receivable` aggregate with an initialized balance, aging profile, collection profile, immutable obligation reference, and completed recognition fact.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a Finance bounded context that owns a financial obligation publishes an authoritative completed business fact establishing that obligation.

## 4.2 End Boundary

The scenario ends when Accounts Receivable has validated the obligation fact, created the corresponding `Receivable` aggregate, initialized its receivable information, preserved complete financial lineage, and published `ReceivableCreated`.

## 4.3 In Scope

- Receipt and validation of an authoritative financial-obligation fact.
- Confirmation that the obligation may result in an institutional receivable.
- Creation of exactly one `Receivable` aggregate for the originating obligation.
- Establishment of immutable receivable identity and obligation reference.
- Initialization of receivable balance, aging profile, collection profile, classification, and status information required by the aggregate.
- Preservation of source ownership and financial traceability.
- Publication of the completed receivable-recognition fact.

## 4.4 Out of Scope

- Creation, approval, revision, cancellation, or ownership of the originating financial obligation.
- Direct user or external creation of a receivable without an authoritative obligation fact.
- Payment allocation, settlement, refund, adjustment, collection, dunning, write-off, reopening, or closure.
- Accounting posting, budgeting, treasury activity, reporting, or analytics.
- Technical event transport, interfaces, persistence, or duplicate-processing mechanisms.

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accounts Receivable / Section F | Owns validation and registration of the receivable representation, its aggregate consistency, balance and profile initialization, obligation reference, and final recognition outcome. |
| Authoritative Finance obligation-owning bounded context / Section F | Owns establishment and publication of the originating financial obligation and retains ownership of that obligation after recognition. |

Participation does not transfer ownership. Accounts Receivable owns only the receivable representation; the originating bounded context continues to own the financial obligation.

---

# 6. Business Actors, Roles and Responsibilities

`L2-F07` does not define canonical business actors or roles for receivable registration. The scenario therefore assigns behaviour to the owning business domains and does not invent a user, reviewer, or approver role. The missing actor definition is recorded in Section 17.

---

# 7. Preconditions and Entry Criteria

- An authoritative Finance bounded context has established a valid financial obligation.
- The obligation fact is complete and has been published by its canonical owner.
- The obligation identifies the financial amount and source reference required for receivable traceability.
- The obligation is eligible to result in an institutional receivable.
- No existing `Receivable` aggregate already represents the same originating financial obligation.
- Accounts Receivable can preserve exactly one immutable reference to the originating obligation.

---

# 8. Trigger

Accounts Receivable consumes an authoritative completed Finance business event establishing a financial obligation that results in an institutional receivable.

Illustrative upstream facts in `L2-F07` include Student Charge Published, Debit Note Issued, Vendor Recovery Established, Institutional Receivable Registered, and Sponsor Receivable Created; only canonical events defined by their owning bounded contexts may trigger recognition.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Publish a completed business fact establishing a valid financial obligation. | Authoritative Finance obligation-owning bounded context | The obligation becomes available for governed receivable recognition without transferring source ownership. |
| 2 | Receive the authoritative obligation fact and verify its source, identity, amount, eligibility, and financial traceability. | Accounts Receivable | A valid recognition basis is established. |
| 3 | Confirm that no `Receivable` aggregate already represents the same originating obligation. | Accounts Receivable | The one-obligation-to-one-receivable boundary is protected. |
| 4 | Establish a new `Receivable` aggregate with an immutable aggregate identity and exactly one reference to the originating financial obligation. | Accounts Receivable | Accounts Receivable owns a canonical representation of the obligation. |
| 5 | Initialize the receivable balance from the authoritative obligation and establish the initial aging, collection, classification, and status information required by the aggregate. | Accounts Receivable | The receivable is complete enough to enter its governed lifecycle. |
| 6 | Validate aggregate consistency, financial integrity, source lineage, and all applicable receivable invariants. | Accounts Receivable | The receivable recognition is complete and valid. |
| 7 | Publish `ReceivableCreated` as the immutable fact that the receivable has been established. | Accounts Receivable | Interested consumers may use the recognized receivable fact without modifying or re-owning the aggregate. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Different obligation source | The valid obligation originates from Student Charge, an approved adjustment, vendor recovery, sponsor activity, or another authoritative Finance source. | Apply the same validation, identity, lineage, and registration constraints while retaining the source owner's obligation reference. | One canonical receivable is recognized independently of the obligation source. | Accounts Receivable and source-owning Finance bounded context |
| Obligation is not receivable-producing | The authoritative fact does not establish an amount owed to the institution or otherwise fails receivable eligibility. | Do not create a `Receivable`; retain the source fact with its canonical owner. | No Accounts Receivable recognition occurs. | Accounts Receivable |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| The obligation fact is missing its authoritative source, identity, amount, or traceable reference. | Accounts Receivable cannot establish a valid and auditable receivable. | Withhold recognition and require the source owner to correct or republish its authoritative business fact under its own governance. | Source-owning Finance bounded context |
| A receivable already references the same originating obligation. | A second registration would violate canonical uniqueness and distort the outstanding position. | Do not create another aggregate; retain the existing receivable and investigate the repeated source fact. | Accounts Receivable |
| Aggregate validation, consistency, or financial integrity fails. | The receivable cannot enter a valid recognized position. | Do not publish `ReceivableCreated`; preserve the authoritative obligation with its source owner and resolve the invalid recognition basis before a new valid attempt. | Accounts Receivable and affected source owner |
| The source obligation is revised or cancelled after recognition. | The recognized receivable may no longer represent the current authoritative obligation. | Route the later authoritative fact to the separately governed receivable modification, adjustment, cancellation, or resolution scenario; do not rewrite the originating event or silently replace the receivable. | Accounts Receivable and source-owning Finance bounded context |
| Recognition is interrupted before aggregate validation completes. | No completed receivable business fact exists. | Resume or restart business validation from the authoritative obligation while preventing duplicate recognition; publish no completion event until consistency is established. | Accounts Receivable |

`L2-F07` defines no dedicated failed or rejected registration state or event. This scenario therefore introduces none.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Accounts Receivable never creates financial obligations; it represents obligations established by their authoritative owners. | Accounts Receivable and obligation-owning Finance context | Recognition requires an upstream authoritative obligation and cannot originate the debt. | `L2-F07`, Sections 2.2, 6.5 and 6.7 |
| Receivable creation may be triggered only by authoritative Finance events from bounded contexts that own financial obligations. | Accounts Receivable | A user request, direct external instruction, or non-authoritative fact cannot create the aggregate. | `L2-F07`, Section 6.7 |
| Every `Receivable` references exactly one originating financial obligation and cannot exist without financial traceability. | Accounts Receivable | Registration establishes one immutable source relationship and rejects untraceable recognition. | `L2-F07`, Section 6.7; `INV-AR-03`; `INV-AR-04` |
| Every receivable has exactly one immutable aggregate identity. | Accounts Receivable | Recognition cannot reuse or later replace the receivable identity. | `L2-F07`, `INV-AR-01` |
| Receivable balance represents the authoritative receivable amount maintained by the aggregate. | Accounts Receivable | Initial balance must be established consistently from the authoritative obligation fact. | `L2-F07`, `INV-AR-02` |
| External bounded contexts never directly modify Receivable entities or bypass aggregate invariants. | Accounts Receivable | All recognition behaviour remains inside the `Receivable` consistency boundary. | `L2-F07`, Sections 6.6, 8.5 and `INV-AR-08` |
| Aggregate consistency and financial integrity must be established before event publication. | Accounts Receivable | `ReceivableCreated` is published only after valid registration completes. | `L2-F07`, Sections 11.4 and 14.1 |
| Consumed and published events represent completed immutable business facts and do not transfer ownership. | Publishing domain | Source and receivable ownership remain distinct throughout recognition. | `L2-F07`, Sections 11.1-11.5 |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Originating Financial Obligation | Valid obligation established by its authoritative Finance owner | Its completed fact is referenced as the basis for receivable recognition; its state and ownership are not changed by Accounts Receivable. | Unchanged in the source owner's authoritative lifecycle | Source-owning Finance bounded context | `L2-F07`, Sections 6.5 and 6.7 |
| `Receivable` | Does not yet exist | A new aggregate is created, identified, linked to one obligation, and initialized with balance and relevant profiles. | `Created` is implied by `ReceivableCreated`; exact canonical initial state name is not separately defined | Accounts Receivable | `L2-F07`, Sections 6.5, 6.7, 10.3, 11.3 and States appendix |

The source describes receivable state concepts but does not define a canonical receivable state machine or exact initial status. This prevents silent normalization and is retained in Section 17.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| Authoritative financial-obligation event | Finance bounded context that owns the obligation | Accounts Receivable | Supplies the completed obligation fact that may trigger recognition. `ChargeEstablished` is the only exact consumed example defined in the source table; other obligation sources require their own canonical event definitions. | `L2-F07`, Sections 6.7 and 11.5 |
| `ReceivableCreated` | Accounts Receivable | Financial Reporting, Treasury, dashboards, Audit, Enterprise Analytics, and other governed consumers where applicable | Announces that a valid receivable representation has been established after aggregate validation. | `L2-F07`, Sections 10.3, 11.3-11.4 and 12.4 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Student Charge Management | Accounts Receivable | Receivable creation trigger carrying the authoritative financial-obligation fact | Enables recognition while Student Charge Management retains obligation ownership. | `L2-F07`, Sections 5.1, 9 and 12.3 |
| Other authoritative Finance obligation owner | Accounts Receivable | Published completed obligation fact with identity, amount, and source lineage | Enables equivalent recognition for non-student obligations without direct aggregate sharing. | `L2-F07`, Sections 6.7, 11.5 and 12.5 |
| Accounts Receivable | Governed downstream consumers | `ReceivableCreated` and read-only receivable facts | Provides receivable visibility without ownership transfer or direct modification. | `L2-F07`, Sections 5.2, 11.3 and 12.4 |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Establish and publish the originating obligation | No canonical actor or role defined in `L2-F07` | Authority remains with the Finance bounded context that owns the obligation and must be defined by that source architecture. | Source-owning Finance bounded context |
| Determine eligibility and register the receivable | No canonical actor or role defined in `L2-F07` | Recognition is permitted only from an authoritative completed obligation fact and only when aggregate invariants are satisfied. | Accounts Receivable |
| Confirm the completed recognition outcome | `Receivable` aggregate responsibility; no canonical human approval role defined | Completion is evidenced by successful aggregate validation and `ReceivableCreated`; this does not invent a human approval. | Accounts Receivable |

## 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Obligation ownership versus recognition ownership | Source-owning Finance bounded context and Accounts Receivable | The source owner establishes the obligation; Accounts Receivable creates only its representation. Neither directly modifies the other's aggregate. |
| Direct creation prohibition | Any requesting party and Accounts Receivable | A user, direct external instruction, or unsupported source cannot bypass the authoritative obligation event. |
| Later correction | Source owner and Accounts Receivable | Revision or cancellation remains a new authoritative fact handled through the appropriate governed follow-on scenario; the original fact and recognition evidence are not overwritten. |
| Recognition exception | Accounts Receivable and source owner | Invalid, incomplete, or duplicated recognition is withheld without creating a local override path. |

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Prove the obligation basis | Authoritative source, obligation identity, amount, originating event, and publication reference | Source-owning Finance bounded context |
| Prove unique receivable registration | Immutable receivable identity and exactly one originating obligation reference | Accounts Receivable |
| Prove initial financial position | Initial receivable balance and initialized aging, collection, classification, and status information | Accounts Receivable |
| Prove validation and completion | Aggregate validation, invariant satisfaction, financial-integrity evidence, and `ReceivableCreated` | Accounts Receivable |
| Preserve ownership lineage | Separate source-obligation and receivable records connected by immutable references | Source owner and Accounts Receivable |

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The originating obligation is an authoritative completed Finance fact and remains owned by its source bounded context.
- Exactly one new `Receivable` aggregate represents the obligation.
- The receivable has an immutable identity and exactly one originating financial-obligation reference.
- Receivable balance, aging, collection, classification, and status information required for registration are initialized consistently.
- Financial traceability, aggregate consistency, and applicable invariants are satisfied.
- `ReceivableCreated` is published only after recognition has completed successfully.

## 16.2 Postconditions

- Accounts Receivable is the authoritative owner of the recognized receivable representation.
- The source bounded context remains the authoritative owner of the originating financial obligation.
- The receivable is available for subsequent aging, collection, settlement, adjustment, write-off, reopening, closure, reporting, and audit scenarios under their own governance.
- Downstream consumers may use the completed receivable fact without directly modifying or re-owning the aggregate.
- The complete lineage from obligation fact to receivable recognition remains auditable.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| `L2-F07` has no canonical Actors section and assigns no business role to receivable registration, validation, exception review, or recognition authority. | Sections 6 and 15 cannot identify a human initiator, reviewer, approver, or exception authority without invention. | Accounts Receivable owner and Lead Architect must define whether this scenario has business actors and, if so, their canonical roles and authorities. |
| `L2-F07` lists illustrative creation sources using labels such as Student Charge Published and Debit Note Issued, while its consumed-event table defines `ChargeEstablished` but no exact event for every other obligation source. | Section 14 cannot name a complete canonical trigger-event set for all eligible financial obligations. | Each obligation-owning Finance bounded context and the Accounts Receivable owner must align the canonical event names and required recognition contract. |
| `L2-F07` requires exactly one originating Financial Obligation but does not define the cross-source uniqueness rule, repeated-event treatment, or business evidence used to prove that one receivable already exists for an obligation. | The scenario can prohibit duplicate recognition but cannot define the authoritative business decision method for repeated or conflicting facts. | Accounts Receivable owner must define the obligation-identity and duplicate-recognition business rule upstream. |
| `L2-F07` does not define a dedicated receivable state machine or exact initial state/status, although `ReceivableCreated` implies a Created position and separate aging and collection states are listed. | Section 13 cannot lock an exact initial lifecycle transition or distinguish aggregate status from aging and collection classifications. | Accounts Receivable owner and Lead Architect must define the canonical Receivable lifecycle and initial status. |
| The source requires initialization of Aging Profile and Collection Profile but does not define their initial values, classification decision rules, or effective-date basis. | The Draft can require initialization but cannot specify the exact initial business positions. | Accounts Receivable owner must define the initial aging, collection, classification, and timing rules upstream. |
| `L2-F07` does not define a dedicated rejected, failed, or deferred registration state/event or a business recovery authority. | Section 11 can withhold incomplete recognition but cannot express a canonical exception lifecycle or completion fact for rejected recognition. | Accounts Receivable owner must decide whether recognition exceptions require canonical states, events, and authority definitions. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**End of Scenario L3-F-039**
