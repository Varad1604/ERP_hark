# Overdue Receivable to Governed Dunning Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-F-040  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Accounts Receivable  
**Participating Domains:** C, F, H  
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

This scenario governs the institutional response when an outstanding receivable becomes overdue. It ensures that receivable aging, collection priority, reminder intervals, collection status, escalation, suspension, review, and completion are applied consistently under approved institutional policy while Accounts Receivable preserves the authoritative outstanding position and collection visibility.

---

# 3. Business Outcome

The overdue receivable has a governed, traceable dunning outcome recorded in its collection and aging information: active collection, review, suspension, escalation, or completion, without changing the originating obligation, imposing an unauthorized penalty, or transferring ownership of the `Receivable` aggregate.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an outstanding `Receivable` reaches an institution-configured overdue condition or collection-review point while retaining a positive unsettled balance.

## 4.2 End Boundary

The scenario ends when the applicable collection action has been governed under institutional policy, its evidence is retained, the receivable's aging and collection visibility are current, and the resulting collection outcome has been recorded and communicated through an applicable canonical receivable event.

## 4.3 In Scope

- Identification of an overdue receivable from its outstanding amount and aging position.
- Application of configured aging buckets, reminder intervals, collection priorities, collection categories, escalation levels, and tolerance thresholds.
- Confirmation of the current obligation, settlement, adjustment, dispute, and collection information relevant to dunning.
- Determination of the permitted dunning path under approved institutional policy.
- Recording of active collection, under-review, suspended, escalated, or completed collection visibility.
- Preservation of collection actions, decisions, exceptions, and financial lineage.
- Publication of an applicable completed receivable-change fact.

## 4.4 Out of Scope

- Creation or modification of the originating financial obligation.
- Payment execution, settlement allocation, refund processing, or financial adjustment approval.
- Application of late fees or penalties by Accounts Receivable.
- Write-off approval or reopening of a resolved receivable.
- Legal recovery, external collection-agency procedure, or institution-specific communication design not defined upstream.
- Technical scheduling, message delivery, interface, persistence, or retry design.

## 4.5 Related Scenarios - Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | `L3-F-039` Published Financial Obligation to Accounts Receivable Recognition | Establishes the receivable whose outstanding balance and aging may become overdue. |
| Followed by | `L3-F-041` Receipt or Adjustment to Receivable Settlement | A payment or approved adjustment may reduce or settle the overdue receivable. |
| Related | `L3-F-042` Uncollectible Receivable to Write-Off Decision | A governed collection history may contribute evidence to a separate write-off decision. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accounts Receivable / Section F | Owns the `Receivable` aggregate, outstanding balance representation, aging profile, collection visibility, traceability, and recorded dunning outcome. |
| Institutional administration / policy owner | Owns configuration values such as aging buckets, reminder intervals, collection priorities, collection categories, escalation levels, and outstanding tolerance thresholds. |
| Originating obligation owner / Section F | Retains ownership of the financial obligation referenced by the receivable and supplies any authoritative obligation change affecting collection eligibility. |
| Payment, Refund, Financial Adjustment, and Accounting capabilities / Section F | Retain ownership of completed financial facts referenced when determining the current outstanding and collection position. |

The collection process does not authorize Accounts Receivable to alter another bounded context's aggregate or independently change the underlying obligation.

---

# 6. Business Actors, Roles and Responsibilities

`L2-F07` defines no canonical actors for collection initiation, review, escalation, suspension, completion, or exception approval. It assigns configuration ownership to institutional administration but does not define a canonical administrative or collection role. Responsibilities therefore remain assigned to the owning domains pending the actor decision recorded in Section 17.

---

# 7. Preconditions and Entry Criteria

- A valid Accounts Receivable-owned `Receivable` aggregate exists and retains complete financial traceability.
- The receivable has a positive outstanding balance after all authoritative settlement and adjustment facts already received.
- Its aging profile identifies an overdue condition or configured collection-review point.
- Applicable aging buckets, reminder intervals, collection priorities, collection categories, escalation levels, and tolerance thresholds are available under institutional configuration.
- The current collection status and prior collection evidence are available.
- No authoritative fact already makes the receivable fully settled, closed, written off, or otherwise ineligible for dunning.

---

# 8. Trigger

The receivable reaches an institution-configured overdue aging boundary, reminder interval, collection-review point, or escalation threshold while an outstanding amount remains due.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the receivable as overdue using its authoritative outstanding balance, relevant date basis, aging profile, and configured aging boundary. | Accounts Receivable | A traceable overdue collection case is established. |
| 2 | Reconcile the current collection basis with completed payment, refund, adjustment, obligation-change, and Accounting references already incorporated into the receivable. | Accounts Receivable | Dunning proceeds against the current authoritative receivable position. |
| 3 | Determine the applicable collection category, priority, reminder interval, escalation level, and tolerance threshold under institutional configuration. | Accounts Receivable using institution-owned configuration | The permitted collection path and decision point are identified. |
| 4 | Check for a governed reason requiring collection review or suspension before further action. | Accounts Receivable and policy owner | The receivable is either eligible for active collection or directed to review or suspension. |
| 5 | Perform the institutionally authorized dunning action appropriate to the current stage without changing the underlying obligation or adding an Accounts Receivable-owned penalty. | Institutionally authorized collection responsibility | The overdue party is subjected to the permitted collection action and the business evidence is retained. |
| 6 | Evaluate the result against the applicable collection rule and determine whether collection remains active, requires review, is suspended, is escalated, or is complete. | Accounts Receivable and policy owner | A governed dunning outcome is reached. |
| 7 | Update the receivable's aging and collection visibility while preserving its authoritative outstanding balance and complete action and decision lineage. | Accounts Receivable | The `Receivable` reflects the current collection position consistently. |
| 8 | Publish the applicable completed receivable-change fact after aggregate validation and financial integrity checks succeed. | Accounts Receivable | Governed consumers receive the updated collection fact without ownership transfer. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Collection under review | A dispute, evidence issue, policy exception, or uncertain outstanding position requires examination. | Record `Under Review`, preserve the reason and evidence, and withhold further dunning until a governed decision is available. | The receivable remains outstanding but active escalation is paused pending review. | Accounts Receivable and policy owner |
| Collection suspended | A governed policy condition temporarily prohibits collection activity. | Record `Collection Suspended`, the authority, reason, effective boundary, and later review requirement. | Collection activity is suspended without settling, closing, or deleting the receivable. | Accounts Receivable and policy owner |
| Collection escalated | The configured reminder or collection stage completes without resolution and escalation criteria are satisfied. | Record `Collection Escalated` and hand off the evidence to the institutionally authorized escalation responsibility. | The receivable remains outstanding under a higher governed collection level. | Accounts Receivable and policy owner |
| Collection completed by financial resolution | An authoritative payment or approved adjustment fully resolves the outstanding amount during dunning. | Stop further collection action and route the completed fact through the receivable-settlement scenario. | Collection becomes complete through authoritative financial resolution. | Accounts Receivable and relevant Finance owner |
| Below tolerance | The outstanding amount falls within a configured tolerance threshold. | Apply the institutionally approved tolerance treatment without inventing settlement, write-off, or penalty authority. | Collection is handled according to configured policy; any financial extinguishment requires its separately governed decision. | Accounts Receivable and policy owner |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Aging basis, due boundary, or outstanding amount is incomplete or inconsistent. | The institution cannot determine whether dunning is justified or which stage applies. | Place the collection case under review, preserve the discrepancy, and reconcile authoritative receivable and source facts before further action. | Accounts Receivable and affected source owner |
| Required collection configuration is missing, conflicting, or not effective for the receivable. | A collection action or escalation level cannot be selected consistently. | Withhold the affected action and refer the configuration gap to institutional administration; do not create local thresholds. | Institutional administration and Accounts Receivable |
| A completed settlement or adjustment fact arrives during collection activity. | Continued dunning may use an obsolete outstanding amount. | Re-evaluate the balance and collection eligibility from the authoritative fact, then settle, reduce, continue, or complete collection as governed. | Accounts Receivable and publishing Finance owner |
| A dunning action is attempted while collection is under review or suspended. | The action would violate the current governed collection position. | Reject or stop the action and retain evidence of the attempted exception for review. | Accounts Receivable and policy owner |
| Unauthorized escalation, penalty, or obligation change is attempted. | Financial and collection governance would be bypassed. | Reject the action; Accounts Receivable shall not apply penalties, create obligations, or override institutional policy. | Accounts Receivable |
| Collection evidence is incomplete or the outcome cannot be verified. | The dunning result is not auditable. | Retain the case in its last valid collection position and obtain or reconstruct the missing business evidence before recording a later outcome. | Accounts Receivable and policy owner |

`L2-F07` does not define canonical failed-dunning or rejected-dunning states or events. No such lifecycle positions are introduced here.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Receivable aging is represented through configurable aging dimensions. | Accounts Receivable and institutional administration | The overdue condition and stage must use approved configuration rather than locally invented durations. | `L2-F07`, Sections 10.6 and 13.2-13.4 |
| Collection visibility may include Active Collection, Under Review, Collection Suspended, Collection Escalated, and Collection Completed. | Accounts Receivable | The recorded dunning outcome must remain within supported collection concepts pending lifecycle normalization. | `L2-F07`, Section 10.7 and States appendix |
| Reminder intervals, collection priorities, categories, escalation levels, and outstanding tolerance thresholds are configurable. | Institutional administration | Institution-specific dunning variation is governed by configuration without changing aggregate ownership or architecture. | `L2-F07`, Sections 13.1-13.4 |
| Receivable balance is authoritative within the aggregate, but balance calculations remain governed by upstream financial events. | Accounts Receivable and publishing Finance owners | Dunning uses the current incorporated financial facts and cannot independently recalculate or change the debt. | `L2-F07`, Sections 10.4 and 11.5; `INV-AR-02` |
| Accounts Receivable shall not apply late fees or penalties, generate obligations, manage institutional policies, or perform payment, refund, accounting, or treasury processing. | Relevant Finance owners and institutional policy owner | Dunning cannot create a new financial charge or execute another domain's outcome. | `L2-F07`, Section 14.2 |
| External bounded contexts shall not directly modify Receivable entities. | Accounts Receivable | Collection decisions and source facts affect the receivable only through its aggregate boundary. | `L2-F07`, `INV-AR-08` |
| Every published event corresponds to a completed valid aggregate transition and is published only after consistency is preserved. | Accounts Receivable | No collection-update fact is published for a partial or invalid outcome. | `L2-F07`, Sections 11.4 and 14.1 |
| Every receivable remains traceable to its originating financial obligation throughout collection. | Accounts Receivable | Dunning history cannot detach, replace, or obscure the source obligation. | `L2-F07`, `INV-AR-03` and `INV-AR-04` |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| `Receivable` aging profile | `Current`, `Short Outstanding`, `Medium Outstanding`, `Long Outstanding`, or `Critical Outstanding`, as configured | Re-evaluates the receivable against the applicable configured aging boundary while the amount remains outstanding. | Applicable configured aging position | Accounts Receivable | `L2-F07`, Sections 10.6 and 13.2 |
| `Receivable` collection visibility | Existing valid collection position | Records the governed dunning path and outcome without changing source ownership. | `Active Collection`, `Under Review`, `Collection Suspended`, `Collection Escalated`, or `Collection Completed`, as applicable | Accounts Receivable | `L2-F07`, Section 10.7 and States appendix |
| Originating financial obligation | Existing source-owned lifecycle position | Remains referenced and unchanged by Accounts Receivable dunning. | Unchanged unless its owner publishes a separate authoritative change | Source-owning Finance bounded context | `L2-F07`, Sections 2.2 and 6.7 |

`L2-F07` describes aging dimensions and collection states as examples rather than a canonical state machine. Their exact transitions, compatibility, and terminal meaning remain open in Section 17.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `PaymentAllocated` or `PaymentReversed` | Payment Management | Accounts Receivable | Updates the authoritative settlement basis used before or during dunning. | `L2-F07`, Section 11.5 |
| `RefundCompleted` | Refund Management | Accounts Receivable | Supplies a completed refund fact that may affect the current outstanding position. | `L2-F07`, Section 11.5 |
| `FinancialAdjustmentApproved` | Financial Adjustment Management | Accounts Receivable | Supplies an approved adjustment fact that may change collection eligibility or amount. | `L2-F07`, Section 11.5 |
| `ReceivableUpdated` | Accounts Receivable | Financial Reporting, Treasury, dashboards, Audit, Enterprise Analytics, and other governed consumers where applicable | Communicates a completed receivable information change after the dunning outcome is recorded. | `L2-F07`, Sections 11.3-11.4 and 12.4 |

No dunning-specific event is defined in `L2-F07` for reminder, review, suspension, escalation, or collection completion. This remains an open issue.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Institutional administration / policy owner | Accounts Receivable | Effective aging and collection configuration | Governs institution-specific overdue, reminder, priority, escalation, category, and tolerance behaviour. | `L2-F07`, Section 13 |
| Payment, Refund, Financial Adjustment, Accounting, or obligation owner | Accounts Receivable | Completed authoritative financial fact | Keeps the receivable and dunning basis aligned without direct aggregate modification. | `L2-F07`, Sections 5.1, 11.5 and 12.3 |
| Accounts Receivable | Institutionally authorized collection responsibility | Current outstanding, aging, collection status, prior-action, and source-traceability evidence | Enables the permitted dunning action while Accounts Receivable retains aggregate ownership. | `L2-F07`, Sections 10.4, 10.6, 10.7 and 10.9 |
| Accounts Receivable | Governed downstream consumers | Updated receivable and collection visibility | Supports collection monitoring, treasury visibility, reporting, analytics, dashboards, and audit. | `L2-F07`, Sections 5.2 and 12.4 |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Maintain institutional aging and collection configuration | No canonical actor defined; configuration ownership is assigned to institutional administration | Values may vary by institution but cannot redefine aggregate ownership, boundaries, responsibilities, or event philosophy. | Institutional administration |
| Initiate or perform a dunning action | No canonical collection actor defined in `L2-F07` | Action must follow effective institutional policy and configuration and cannot create obligations or penalties. | Institutional policy owner; Accounts Receivable retains collection visibility |
| Place collection under review or suspension | No canonical decision role defined in `L2-F07` | The reason, authority, effective boundary, and evidence must be retained; exact approval authority remains open. | Institutional policy owner and Accounts Receivable |
| Escalate or complete collection | No canonical decision role defined in `L2-F07` | Outcome must satisfy the applicable configured rule and preserve the receivable's financial integrity and lineage. | Institutional policy owner and Accounts Receivable |

## 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Policy ownership versus aggregate ownership | Institutional administration and Accounts Receivable | Administration governs configuration; Accounts Receivable owns the receivable record and collection visibility. |
| Dunning versus financial modification | Collection responsibility and source Finance owners | Collection cannot settle, adjust, write off, refund, penalize, or change the originating obligation without an authoritative fact from the relevant owner. |
| Review or suspension exception | Collection decision authority and Accounts Receivable | Exceptions require recorded reason, authority, evidence, and later review; no informal override is permitted. |
| Escalation control | Current collection responsibility and escalation authority | Escalation must follow the configured level and preserve the prior action and decision history. |

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Prove overdue eligibility | Receivable identity, obligation reference, outstanding balance, aging basis, applicable bucket, and effective configuration | Accounts Receivable and institutional administration |
| Prove collection action | Collection stage, permitted action, date or business boundary, responsible authority where defined, and evidence of completion | Institutional policy owner and Accounts Receivable |
| Prove review, suspension, or escalation | Reason, authority, prior collection history, applicable policy/configuration, evidence, and resulting collection position | Institutional policy owner and Accounts Receivable |
| Prove financial consistency | Payment, refund, adjustment, Accounting, and obligation-change references incorporated before the outcome | Accounts Receivable and source Finance owners |
| Prove completed receivable update | Aggregate validation, resulting aging and collection position, audit history, and applicable `ReceivableUpdated` event | Accounts Receivable |

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The receivable's overdue status is supported by its authoritative outstanding amount, aging basis, and effective institutional configuration.
- Relevant completed settlement, refund, adjustment, Accounting, and obligation-change facts have been considered.
- The permitted dunning action or exception path has been determined without applying an Accounts Receivable-owned penalty or modifying another aggregate.
- The collection result is recorded as active, under review, suspended, escalated, or completed where applicable.
- The aging and collection information remains consistent with the receivable balance and source lineage.
- Required action, decision, authority, exception, and audit evidence is preserved to the extent defined upstream.
- An applicable completed receivable-change fact is published only after aggregate validation.

## 16.2 Postconditions

- Accounts Receivable retains authoritative ownership of the `Receivable` and its collection visibility.
- The originating Finance owner retains ownership of the underlying obligation.
- The receivable remains available for later collection review, reminder, escalation, settlement, adjustment, write-off, reopening, or closure under their respective governance.
- Downstream reporting, treasury, dashboard, analytics, and audit consumers may use the updated collection position without directly modifying it.
- No unauthorized fee, penalty, settlement, write-off, obligation change, or cross-domain ownership transfer has occurred.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| `L2-F07` explicitly excludes implementation of collection workflows and institutional policies and has no canonical Actors section. | Sections 6, 9, and 15 cannot identify who initiates, performs, reviews, suspends, escalates, completes, or approves dunning actions. | Accounts Receivable, institutional administration, and Lead Architect must define the owning collection-policy capability and canonical business roles and authorities. |
| Aging buckets, reminder intervals, collection priorities, categories, escalation levels, and tolerance thresholds are configurable, but no default values, effective-date rules, hierarchy, or conflict-resolution rules are defined. | The Draft cannot lock when a receivable becomes overdue or which collection stage applies. | Institutional policy owner and Accounts Receivable owner must define the governed configuration model upstream. |
| `L2-F07` lists aging and collection positions as examples and provides no canonical state machine, transition guards, compatibility rules, or terminal semantics. | Section 13 cannot lock the lifecycle from overdue identification through review, suspension, escalation, and completion. | Accounts Receivable owner and Lead Architect must define canonical aging and collection lifecycle rules. |
| No dunning-specific canonical events exist for overdue identification, reminder completion, review, suspension, escalation, resumption, or collection completion. | Section 14 can only use generic `ReceivableUpdated`, which may be insufficient for downstream governance and audit. | Accounts Receivable owner must define the required collection events or explicitly approve generic receivable-update publication. |
| `L2-F07` does not define dunning communication channels, content authority, delivery evidence, recipient/contact ownership, consent, language, accessibility, or failed-delivery treatment. | The scenario cannot specify a complete reminder or notice procedure or prove that the intended party received it. | Institutional policy, communications, privacy, and Accounts Receivable owners must define the business communication contract upstream. |
| The source does not define dispute, hardship, deceased-party, legal-stay, sponsorship, government-receivable, scholarship-receivable, or vulnerable-party collection rules. | Sections 10 and 11 cannot determine all mandatory review or suspension conditions. | Institutional policy, legal/compliance, and Accounts Receivable owners must define exception categories and decision authority. |
| Accounts Receivable is prohibited from applying late fees or penalties, but no canonical hand-off is defined when institutional policy requires a separate financial charge. | The dunning scenario cannot identify the authoritative charge owner, approval, event, or receivable linkage for such a consequence. | Finance architecture owners must define the separate obligation-creation and recognition path for authorized late charges, if permitted. |
| No exact business event or rule defines when `Collection Completed` applies without full settlement, including tolerance treatment or administrative closure. | Completion could be confused with settlement, write-off, suspension, or closure of the receivable. | Accounts Receivable owner must define collection-completion semantics and its relationship to receivable lifecycle outcomes. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**End of Scenario L3-F-040**
