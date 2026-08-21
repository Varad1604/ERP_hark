# Manual Journal Request to Approved Posting or Rejection

# 1. Scenario Metadata

**Scenario ID:** L3-F-026  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Accounting & General Ledger  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-F00_Finance_Domain_Model_and_Foundation.md`; `L2-F05_Accounting_and_General_Ledger_Management.md`; frozen Phase 4 Stage 3 Section F canonical scenario register  
**Referenced By:** None yet  

---
# 2. Scenario Purpose

This scenario governs an exceptional manual accounting request, including year-end adjustments, audit adjustments, correction entries, opening balances, and closing entries, from preparation through authoritative disposition. It ensures that only an authorized, supported, balanced, validated, and independently reviewed Journal Entry can affect the General Ledger, while a rejected request produces no ledger consequence.

---

# 3. Business Outcome

The manual Journal request has received an authoritative accounting disposition: it is either approved and posted as a balanced Journal Entry with complete ledger and audit consequences, or rejected with the decision preserved and no change to the General Ledger.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when authorized finance personnel present an exceptional manual Journal request with its accounting purpose, proposed debit and credit treatment, and required supporting business evidence.

## 4.2 End Boundary

The scenario ends when the request has reached one of its governed terminal outcomes: the approved Journal Entry reaches `Posted` and the affected ledger balances are updated, or the request is rejected, `JournalRejected` records the decision, and no ledger change occurs.

## 4.3 In Scope

- Preparation of a manual Journal request and its supporting authorization and evidence.
- Establishment and validation of the Journal Entry and balanced debit and credit lines.
- Independent review and the approval-or-rejection decision.
- Posting of an approved Journal Entry and update of the affected Ledger Accounts and General Ledger.
- Preservation of the decision, posting, ledger, and audit evidence.
- Publication of canonical completed Journal and ledger events.

## 4.4 Out of Scope

- Journals derived from operational financial transactions, which belong to `L3-F-025`.
- Posting Batch processing, which belongs to `L3-F-027`.
- Reversal of a posted Journal Entry, which belongs to `L3-F-028`.
- Reopening or unlocking an Accounting Period.
- Definition of accounting rules, the Chart of Accounts, Ledger Accounts, or institution role configuration.

## 4.5 Related Scenarios - Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | `L3-F-025` Financial Transaction to Balanced Ledger Posting | Governs journals derived from operational financial transactions rather than exceptional manual requests. |
| Related | `L3-F-027` Approved Accounting Batch to Posted or Partially Failed Batch | Governs posting at Accounting Batch level. |
| Followed by | `L3-F-028` Posted Journal to Accounting Reversal | Governs later correction of a posted manual Journal through a compensating Journal. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accounting & General Ledger / Section F | Owns the Manual Journal workflow, Journal Voucher authorization and supporting evidence, Journal Entry lifecycle, accounting validation, approval or rejection decision, posting, ledger balances, accounting events, and final disposition. |

The frozen Stage 3 row lists only Section F as participating. All collaboration in this scenario therefore remains within the Finance domain and does not transfer Accounting-owned records or decisions to another domain.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Accountant | Prepares the exceptional manual Journal request, proposed Journal Entry, accounting references, and supporting evidence. | Initiates and performs journal preparation under the source-defined typical Accounting role. |
| Senior Accountant | Reviews the validated manual Journal and makes the approval or rejection decision. | Reviews and decides under the source-defined typical Accounting role and maker-checker separation. |

`L2-F05` defines these as typical, institution-configurable roles rather than immutable enterprise role names. It requires only authorized finance personnel to create Manual Journals and does not assign posting to one canonical business role. These authority gaps are recorded in Section 17.

---

# 7. Preconditions and Entry Criteria

- The request concerns an exceptional accounting purpose supported by the Manual Journal workflow.
- The initiator is authorized under the institution's finance-role configuration to create a Manual Journal.
- The proposed accounting treatment identifies the applicable institution, Accounting Period, Fiscal Year, currency, Ledger Accounts, amounts, and business reference.
- The supporting business authorization and evidence required for the manual entry are available through the Journal Voucher relationship.
- For progression to posting, the Accounting Period is `Open`, the Fiscal Year is active, and the affected Ledger Accounts are active and posting-eligible.

---

# 8. Trigger

Authorized finance personnel submit an exceptional manual Journal request for Accounting review and disposition.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Present the exceptional accounting purpose, proposed debit and credit treatment, business reference, and supporting evidence for the Manual Journal. | Accountant / Accounting & General Ledger | A traceable manual Journal request is available for accounting preparation. |
| 2 | Establish the Journal Voucher authorization context and a uniquely numbered `Draft` Journal Entry linked to the request and its supporting evidence. | Accountant / Accounting & General Ledger | The request is represented by governed Accounting records without yet affecting ledger balances. |
| 3 | Verify the required debit and credit lines, valid Ledger Accounts, Accounting Period, Fiscal Year, currency, business reference, and double-entry balance. | Accountant / Accounting & General Ledger | The Journal satisfies the conditions required for formal validation. |
| 4 | Validate the Journal against the applicable accounting, posting, account, period, chronology, and audit rules. | Accounting & General Ledger | The Journal reaches `Validated`; any failed validation prevents approval and posting. |
| 5 | Independently review the validated Journal, its exceptional accounting purpose, and its supporting evidence. | Senior Accountant / Accounting & General Ledger | A maker-checker decision can be made on the validated Journal. |
| 6 | Approve the validated Journal for posting. | Senior Accountant / Accounting & General Ledger | The Journal reaches `Approved` and becomes eligible for posting. |
| 7 | Post the approved Journal as one complete balanced accounting outcome and update every affected Ledger Account and the General Ledger. | Accounting & General Ledger | The Journal reaches `Posted`, becomes immutable, and the balanced ledger consequence is recognized. |
| 8 | Preserve the approval, posting, ledger, and audit evidence and publish the completed accounting facts. | Accounting & General Ledger | `JournalApproved`, `JournalPosted`, and `LedgerUpdated` communicate the completed approval and posting consequences. |

---

# 10. Alternate Business Flows

No scenario-specific alternate business flow toward the approved-and-posted outcome is defined by the authoritative L2 architecture. Rejection and validation interruption are governed in Section 11 because they terminate or prevent the normal posting path.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| The Senior Accountant rejects the validated Manual Journal request. | Approval and posting do not occur, and ledger balances remain unchanged. | Preserve the rejection decision and publish `JournalRejected`. The resulting Journal lifecycle state and any permitted resubmission path are not defined upstream and remain open issues. | Accounting & General Ledger |
| Mandatory accounting context, business reference, authorization, or supporting evidence is missing or invalid. | The Journal cannot complete validation or proceed to a decision. | Correct the deficient request or evidence before the Journal is validated; no posting or ledger update occurs. | Accounting & General Ledger |
| Debit and credit totals do not balance, a required side is absent, or a referenced Ledger Account is ineligible. | The Journal fails accounting-integrity validation. | Correct the proposed Journal while it remains unposted and submit it to validation again. | Accounting & General Ledger |
| The Accounting Period is closed or locked, or the Fiscal Year is not active. | Posting is prohibited for the selected period. | The Journal is not posted. Any reopening or fiscal-governance decision must occur through its separately governed workflow. | Accounting & General Ledger |
| Approval is absent or the creator attempts to approve the same Journal. | The Journal cannot become posting-eligible. | Obtain a valid independent decision under maker-checker separation; no posting occurs until that condition is satisfied. | Accounting & General Ledger |
| The complete balanced posting cannot be achieved. | A partial or inconsistent ledger consequence would result. | Do not recognize the posting as completed; ledger balances remain unchanged and no completed posting event is published. | Accounting & General Ledger |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| `BR-001`: Every Journal Entry shall maintain double-entry accounting. | Accounting & General Ledger | The Manual Journal must represent balanced debit and credit consequences. | `L2-F05`, Section 36.1 |
| `BR-002`: Total Debit shall always equal Total Credit. | Accounting & General Ledger | An unbalanced Manual Journal cannot progress to posting. | `L2-F05`, Section 36.1 |
| `BR-003` and `BR-004`: Every Journal Entry shall contain at least one debit line and at least one credit line. | Accounting & General Ledger | Both sides of the entry are mandatory. | `L2-F05`, Section 36.1 |
| `BR-005`: Journal Numbers shall be unique. | Accounting & General Ledger | The Manual Journal must possess a unique canonical identity. | `L2-F05`, Section 36.1 |
| `BR-006`: Every Journal Entry shall reference a valid business transaction. | Accounting & General Ledger | The Manual Journal must preserve its valid business reference and accounting purpose. | `L2-F05`, Section 36.1 |
| `BR-007`, `BR-008`, and `BR-010`: Posted Journals are immutable, Journals are never physically deleted, and every Journal requires an audit record. | Accounting & General Ledger | Approval, rejection, and posting history must remain reconstructable without altering a posted entry. | `L2-F05`, Section 36.1 |
| `BR-014`, `BR-015`, and `BR-026`: Closed, suspended, or inactive Ledger Accounts reject postings. | Accounting & General Ledger | Every Manual Journal line must target a posting-eligible Ledger Account. | `L2-F05`, Sections 36.2 and 36.3 |
| `BR-029`, `BR-030`, and `BR-031`: Posting is permitted only in an Open Accounting Period; closed periods reject posting and locked periods reject modifications. | Accounting & General Ledger | Period eligibility constrains validation, modification, and posting. | `L2-F05`, Section 36.4 |
| `BR-041` and `BR-042`: Only approved Journal Entries may be posted, and posting requires successful validation. | Accounting & General Ledger | Independent validation and approval must precede Manual Journal posting. | `L2-F05`, Section 36.6 |
| `BR-043` and `BR-044`: Posting updates all affected Ledger Accounts and the General Ledger. | Accounting & General Ledger | Successful posting must produce one complete ledger consequence. | `L2-F05`, Section 36.6 |
| `BR-045`: Posting shall preserve transaction chronology. | Accounting & General Ledger | The Manual Journal posting must remain ordered and reconstructable. | `L2-F05`, Section 36.6 |
| `BR-047` and `BR-048`: Posting publishes domain events and generates audit logs. | Accounting & General Ledger | Completed posting must produce canonical facts and audit evidence. | `L2-F05`, Section 36.6 |
| Only authorized finance personnel may create Manual Journals. | Accounting & General Ledger | Unauthorized initiation cannot establish a valid Manual Journal request. | `L2-F05`, Section 35.2 |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Journal Entry | New manual accounting request | A uniquely numbered Journal Entry is established with its business reference and Journal Voucher evidence. | `Draft` | Accounting & General Ledger | `L2-F05`, Sections 28.5, 28.6, 29 and 35.2 |
| Journal Entry | `Draft` | Accounting, balance, account, period, fiscal, currency, reference, and evidence conditions are validated. | `Validated` | Accounting & General Ledger | `L2-F05`, Sections 28.4-28.6, 34.2 and 35.2 |
| Journal Entry | `Validated` | Independent review produces a positive approval decision. | `Approved` | Accounting & General Ledger | `L2-F05`, Sections 28.6 and 35.2 |
| Journal Entry | `Approved` | The complete balanced Manual Journal is posted. | `Posted` and immutable | Accounting & General Ledger | `L2-F05`, Sections 28.5, 28.6, 35.1 and 35.2 |
| Journal Entry | `Validated` | Independent review produces a rejection decision and `JournalRejected`. | No authoritative resulting Journal state is defined | Accounting & General Ledger | `L2-F05`, Sections 28.6-28.8 and 31.1 |
| Accounting Ledger and affected Ledger Accounts | Active and posting-eligible | Successful Manual Journal posting updates balances and posting history; rejection produces no ledger change. | Active ledger with updated balances only on the posting path | Accounting & General Ledger | `L2-F05`, Sections 23.1-23.5, 25.4-25.6 and 35.2 |

The rejection row preserves the defined command and event without inventing a `Rejected` Journal Entry state. The missing lifecycle definition materially prevents this Draft from becoming Locked.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `JournalCreated` | Accounting & General Ledger | Accounting & General Ledger | Records establishment of the Manual Journal. | `L2-F05`, Sections 28.8 and 37.2 |
| `JournalValidated` | Accounting & General Ledger | Accounting & General Ledger | Records successful accounting validation before decision. | `L2-F05`, Sections 28.8 and 37.2 |
| `JournalApproved` | Accounting & General Ledger | Accounting & General Ledger | Records the positive approval decision that permits progression to posting. | `L2-F05`, Sections 28.8 and 37.2 |
| `JournalRejected` | Accounting & General Ledger | Accounting & General Ledger | Records the governed rejection outcome without a ledger consequence. | `L2-F05`, Sections 28.8 and 37.2 |
| `JournalPosted` | Accounting & General Ledger | Procurement Finance, Financial Analytics, Audit & Compliance | Communicates successful posting of the approved balanced Manual Journal. | `L2-F05`, Sections 28.8, 37.2 and 37.8 |
| `LedgerUpdated` | Accounting & General Ledger | Budget Management, Treasury Management, Financial Analytics, Audit & Compliance | Communicates the General Ledger consequence of successful posting. | `L2-F05`, Sections 25.8, 37.3 and 37.8 |

Only the events applicable to the actual disposition are published: rejection does not publish completed posting or ledger-update facts.

## 14.2 Business Contracts and Cross-Domain Hand-offs

No cross-domain synchronous contract or hand-off materially participates. The Manual Journal request, Journal Voucher authorization and evidence, Journal Entry, decision, posting, and ledger outcome are all governed within Accounting & General Ledger in Section F.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Create and prepare the Manual Journal | Accountant | May initiate and prepare the entry only where institution-configured finance authority permits Manual Journal creation. | Accounting & General Ledger |
| Approve or reject the validated Manual Journal | Senior Accountant | May independently review and decide under institution-configured Accounting authority and maker-checker separation. | Accounting & General Ledger |
| Post the approved Manual Journal | Not assigned to one canonical business role in `L2-F05` | Posting requires explicit authorization, successful validation, approval, eligible accounts, and an Open Accounting Period. | Accounting & General Ledger |

## 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Maker-checker separation | Journal Creator and Journal Approver | The Journal Creator shall not approve the same Journal. |
| Posting authorization | Journal Approver and authorized posting authority | A Journal may be posted only after approval and by permitted authority. |
| Period protection | Accounting authority and period-governance authority | Closed or locked periods prohibit posting or modification; reopening requires separately governed approval. |
| Audit independence | Internal Auditor | Audit users shall not modify accounting records. |

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve the exceptional accounting basis | Manual Journal purpose, valid business reference, and Journal Voucher supporting authorization and documents | Accounting & General Ledger |
| Preserve Journal integrity | Unique Journal number, debit and credit lines, account references, period, Fiscal Year, currency, and validation outcome | Accounting & General Ledger |
| Preserve independent disposition | Creator, reviewer, approval or rejection outcome, and maker-checker separation | Accounting & General Ledger |
| Preserve posting consequence | Posting state, posting date, affected Ledger Accounts, General Ledger consequence, and posting reference, when posted | Accounting & General Ledger |
| Preserve completed facts and history | Applicable immutable Journal and ledger events and complete audit lineage | Accounting & General Ledger |

Section A retains ownership of authorization and audit infrastructure; Accounting owns the Manual Journal decisions, evidence obligations, and financial records.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A traceable Manual Journal request and Journal Entry exist with their exceptional accounting purpose, business reference, and supporting evidence.
- The Journal has completed applicable validation and independent review.
- Exactly one governed disposition is recorded: the balanced Journal is approved and reaches `Posted`, or the request is rejected with `JournalRejected` and no ledger change.
- On the posting path, every affected Ledger Account and the General Ledger reflect the complete balanced consequence and `JournalPosted` and `LedgerUpdated` are published.
- On either path, the applicable decision and audit history are preserved and reconstructable.

## 16.2 Postconditions

- A posted Manual Journal is immutable and may be corrected only through the governed accounting reversal workflow.
- A rejected request has no effect on Ledger Account or General Ledger balances.
- The accounting disposition and its supporting evidence remain available for audit and institutional oversight.
- Any permitted correction or resubmission following rejection remains undefined pending resolution of the lifecycle gap in Section 17.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| `L2-F05` defines `Reject Journal` and `JournalRejected`, but the Journal Entry lifecycle contains only `Draft`, `Validated`, `Approved`, `Posted`, and `Reversed`; no rejection transition or resulting state is defined. | The frozen scenario's rejection outcome cannot be mapped to an authoritative Journal lifecycle position. This material final-outcome and lifecycle gap prevents Architecture Approval and locking. | Accounting & General Ledger owner and Lead Architect must define the rejection transition and resulting lifecycle position in authoritative L2 architecture. |
| The Manual Journal workflow in `L2-F05` describes `Create -> Validate -> Approve -> Post -> Ledger -> Audit` but does not define where rejection occurs or its business consequences beyond the separate reject command and event. | Rejection can be preserved as a named outcome, but its exact decision point and relationship to the documented Manual Journal workflow remain ambiguous. | Accounting & General Ledger owner must align the Manual Journal workflow with the existing reject command and event. |
| `L2-F05` states that only authorized finance personnel may create Manual Journals and defines typical configurable roles, but it does not canonically assign creation, approval or rejection, and posting authorities for every institution; posting is not assigned to a role. | The Draft can use the source-defined typical Accountant and Senior Accountant roles, but complete business authority cannot be locked. | Accounting & General Ledger owner must define or confirm the configuration-governed authority mapping, including the posting authority. |
| `L2-F05` does not define whether or how a rejected Manual Journal request may be corrected and resubmitted. | Section 11 cannot prescribe a recovery transition after rejection without inventing lifecycle behaviour. | Accounting & General Ledger owner must define whether rejection is terminal for the request and, if not, the governed correction and resubmission path. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

## Draft Validation Classification

**BLOCKED** - The document is complete as a Template v2.0 Draft, but the authoritative architecture does not define the Journal Entry state reached on rejection, the aligned rejection/resubmission lifecycle, or complete business authority for posting. Because these gaps materially affect a frozen final outcome, lifecycle correctness, and authorization, the scenario cannot progress to Architecture Approved or Locked.

---

**End of Scenario L3-F-026**
