# Bank Account Request to Active or Closed Treasury Account

# 1. Scenario Metadata

**Scenario ID:** L3-F-056  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Treasury & Banking  
**Participating Domains:** B, F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-A01_Platform_Tenant_Foundation_Architecture.md`; `L2-A04_Authorization_and_Policy_Workflows.md`; `L2-B01_Institution_Workflow.md`; `L2-B02_Organisation_Workflows.md`; `L2-F00_Finance_Domain_Model_and_Foundation.md`; `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F09_Financial_Period_and_Closing.md`; `L2-F10_Treasury_and_Banking.md`; frozen Phase 4 Stage 3 Section F canonical scenario register  
**Referenced By:** None yet  

---
# 2. Scenario Purpose

This scenario governs an institutional request to establish a Treasury Account associated with an authoritative external Bank Account Reference, make that internal account available for permitted treasury operations under approved policy, maintain its operational status and banking relationship, and eventually reach a governed non-operational closure outcome. It preserves the distinction between the bank-owned physical account and the Treasury-owned internal business account.

The canonical register names the target outcomes `Active` and `Closed`. `L2-F10` requires Treasury Account lifecycle/status management and configurable activation policy, but does not define those exact states, transitions, closure behaviour, or related events. This draft therefore captures the intended business scenario without inventing missing lifecycle semantics.

---

# 3. Business Outcome

An eligible, approved bank-account request results in a traceable Bank Account Reference and an internally owned Treasury Account associated with one Institution, currency, and governed banking relationship. The Treasury Account becomes operationally usable only after applicable authorization, policy, relationship, configuration, and aggregate invariants succeed. When institutional use must end, the account reaches a governed non-operational outcome while its identity, relationship, activity, decisions, versions, and audit history remain preserved. Exact `Active` and `Closed` state definitions remain unresolved upstream.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorized institutional need is raised to register or associate a bank account for Treasury use, supported by identifiable Institution, Organisation context where applicable, bank, branch, account identifier, currency, intended account classification, and business purpose.

## 4.2 End Boundary

The scenario ends when the request is rejected without account creation; or when an approved Bank Account Reference and Treasury Account are registered and reach the configured operational outcome intended by the canonical `Active` label; or when a previously usable Treasury Account is governed into the non-operational outcome intended by `Closed`, with history retained. Because L2-F10 does not define these account states, final acceptance remains dependent on Section 17 resolutions.

## 4.3 In Scope

- Capture and governance of an institutional Bank Account Request.
- Verification of Institution, Organisation context, external bank/branch/account identity, currency, purpose, classification, and intended use.
- Evaluation of approved banking partner, banking relationship status, availability, operational limitations, and currency restrictions.
- Authorization, policy, governance approval, and segregation evidence.
- Registration of the Treasury-owned Bank Account Reference.
- Registration of the internally owned Treasury Account and association to the Bank Account Reference.
- Maintenance of Treasury Account operational status and lifecycle under institution-configured policy.
- Controlled activation/operational availability represented by the canonical scenario title.
- Governed termination of operational use represented by the canonical `Closed` title outcome.
- Immutable version, event, decision, relationship, and audit history.

## 4.4 Out of Scope

- Opening or closing the physical account at the external bank.
- Ownership or modification of the bank's infrastructure, customer records, KYC records, source transaction ledger, or contractual banking process.
- Payment scheduling, payment execution, batch execution, settlement, cash-position update, liquidity calculation, or bank reconciliation.
- Creation of Journal Entries or modification of General Ledger balances.
- Lower-level API integration, credential management, storage, encryption, messaging, or workflow implementation.
- Inventing Treasury Account lifecycle states, activation/closure guards, or events absent from locked L2 architecture.

## 4.5 Related Scenarios - Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Precedes | `L3-F-057` and later Treasury scenarios | An operational Treasury Account and Bank Account Reference support downstream treasury activity. |
| Related | `L3-F-045` Approved Payable to Financial Settlement | Settlement may use Treasury-owned accounts but cannot establish them. |
| Related | `L3-F-052` Open Financial Period to Locked Close | Financial Period governance constrains accounting activity but does not own Treasury Account lifecycle. |
| Related | `L3-F-025` Financial Transaction to Balanced Ledger Posting | General Ledger consumes treasury facts without owning Treasury accounts. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Treasury & Banking / Section F | Solely owns Treasury Aggregate, Treasury Account, Bank Account Reference business representation, account operational status, account lifecycle, aggregate version, and Treasury events. |
| External Banking Institution | Remains authoritative for the physical bank account, banking infrastructure, source banking information, and original bank transactions. |
| Enterprise Governance | Owns institutional approval hierarchy, delegated authority, exception decisions, and immutable approval outcomes. |
| Authorization & Policy / Section A | Owns authorization and policy evaluation; Treasury consumes outcomes only. |
| Institution and Organisation Management | Own Institution and Organisation identities, lifecycle, and structural context referenced by the Treasury Account. |
| Accounting & General Ledger / Section F | Consumes completed treasury facts for accounting; does not own or alter Treasury Account status. |
| Financial Reporting | Produces read-only account and treasury views from completed facts. |
| Audit | Reviews immutable request, approval, registration, relationship, lifecycle, use, and closure evidence. |
| Compliance | Supplies applicable banking, institutional, due-diligence, preservation, or restriction outcomes where canonically defined. |

Treasury owns the institutional business relationship and internal operational account. The bank owns the physical bank account. Neither ownership crosses the bounded-context boundary.

---

# 6. Business Actors, Roles and Responsibilities

`L2-F10` contains no canonical Actors section and defines no authoritative requester, verifier, approver, account administrator, activation authority, closure authority, banking signatory, reviewer, or custodian. Illustrative governance configuration cannot be converted into canonical roles. Required actors, responsibilities, and segregation remain open in Section 17.

---

# 7. Preconditions and Entry Criteria

- An authoritative active Institution exists; Organisation context is available where required.
- The request identifies its institutional business purpose, intended treasury use, account classification, and currency.
- The external bank, branch, account identifier, currency, and relationship evidence are available from an authoritative source.
- Banking partner eligibility, relationship status, banking availability, currency restrictions, and operational limitations can be evaluated.
- Treasury Account configuration, including activation policy, operational usage, and lifecycle defaults, is available.
- Authorization and governance outcomes can be obtained and recorded.
- Duplicate Bank Account References and Treasury Accounts can be detected within the relevant Institution and currency context.
- Creation will preserve tenant isolation, aggregate identity, relationship traceability, and external ownership.
- No request assumes Treasury can open, modify, or close the bank-owned physical account.

---

# 8. Trigger

An Institution identifies an approved operational need for a bank account to be represented and used within Treasury, or identifies a governed need to terminate an existing Treasury Account's operational use.

No canonical request, activation, suspension, or closure event and no mandatory trigger evidence are defined for Treasury Accounts in `L2-F10`.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Capture the Bank Account Request with Institution, Organisation context, purpose, classification, intended usage, bank, branch, account identifier, currency, and supporting evidence. | Requesting institutional function / Treasury & Banking | A traceable request context exists. |
| 2 | Validate Institution and Organisation references, tenant isolation, requested currency, business purpose, and completeness. | Treasury & Banking / context owners | Invalid or cross-institution requests are stopped. |
| 3 | Validate the external banking relationship, approved partner status, relationship status, availability, currency restrictions, and operational limitations. | Treasury & Banking / External Bank / Compliance | Only an eligible banking relationship proceeds. |
| 4 | Detect duplicate or conflicting Bank Account References and Treasury Accounts using authoritative institutional and banking identifiers. | Treasury & Banking | Duplicate internal representations are prevented or routed to review. |
| 5 | Obtain authorization, governance approval, and any externally owned compliance outcome required by institutional policy. | Enterprise Governance / Authorization & Policy / Compliance | An immutable approval or rejection is available. |
| 6 | Register the Bank Account Reference as Treasury's business representation of the external banking relationship, preserving external bank ownership. | Treasury & Banking | A governed reference exists inside the Treasury aggregate. |
| 7 | Publish `BankAccountReferenceRegistered` only after aggregate consistency and successful registration. | Treasury & Banking | The banking relationship association becomes an immutable completed fact. |
| 8 | Register the internal Treasury Account with immutable identifier, account name, operational status, currency, Institution association, and Bank Account Reference. | Treasury & Banking | The institution has an internally owned Treasury Account business concept. |
| 9 | Publish `TreasuryAccountRegistered` only after successful aggregate modification and version advancement. | Treasury & Banking | Consumers receive the completed registration fact. |
| 10 | Evaluate configured account activation policy, operational account usage, account lifecycle defaults, banking availability, restrictions, authorization, and aggregate invariants. | Treasury & Banking / Enterprise Governance | Eligibility for operational use is determined. |
| 11 | If eligible, establish the operational status intended by the canonical `Active` outcome and preserve the decision, effective sequence, version, and relationship history. | Treasury & Banking | The account becomes available only for permitted Treasury behaviour. |
| 12 | During its usable lifecycle, maintain account status, currency association, banking relationship, restrictions, and immutable activity traceability through Treasury-owned behaviour. | Treasury & Banking | Operational use remains governed and auditable. |
| 13 | When operational use must end, validate closure request, authority, absence or governed disposition of dependent activity, banking relationship, policy, and historical-preservation requirements. | Treasury & Banking / Enterprise Governance | Eligibility for a non-operational closure outcome is evaluated. |
| 14 | On valid approval, establish the non-operational status intended by the canonical `Closed` outcome, preserve history, advance version, and communicate the completed fact using only canonically available events. | Treasury & Banking | The account is no longer available for new Treasury use and its history remains retained. |

Steps 11 and 14 express the outcomes required by the canonical scenario register. Their exact states, transition guards, and events cannot be finalized until the L2 gaps in Section 17 are resolved.

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Request rejected | Authorization, policy, governance, or evidence fails. | Preserve rejection and create no new Treasury Account. | No operational account is established. | Governance / Treasury & Banking |
| Existing Bank Account Reference | An authoritative in-scope reference already exists. | Reuse the existing reference only if ownership, Institution, currency, and relationship rules permit; do not duplicate it. | Treasury Account may associate to the governed existing reference. | Treasury & Banking |
| Duplicate Treasury Account | Requested account conflicts with an existing internal account. | Reject or route governed review; preserve both request and detected conflict. | No duplicate active representation. | Treasury & Banking |
| Banking relationship unavailable | Bank, branch, account, or relationship is inactive/restricted/unavailable. | Keep the request pending/rejected until an authoritative valid relationship exists. | Treasury & Banking / External Bank |
| Currency not permitted | Requested currency violates configuration or policy. | Reject or amend through a new governed request. | Treasury & Banking / Governance |
| Activation conditions incomplete | Registration succeeds but operational eligibility does not. | Preserve the registered account in its valid source-defined status; do not make it operational. | Treasury & Banking |
| Temporary restriction | Institutional policy requires use to stop without permanent closure. | Apply Treasury-owned restriction/suspension behaviour where it applies; do not infer account closure. | Treasury & Banking |
| Closure blocked | Dependent activity, unresolved settlement, reconciliation, hold, or governance condition prevents termination. | Keep the current valid account status and route resolution to the canonical owner. | Treasury & Banking / affected owner |
| Physical bank account closed externally | Bank reports the external account is closed. | Update the governed relationship representation and evaluate internal account status through Treasury policy; do not treat the external event as a direct lifecycle command. | External Bank / Treasury & Banking |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Institution, bank, branch, account identifier, or currency is inconsistent. | Wrong banking relationship may be used. | Reject and obtain authoritative corrected evidence before retry. | Treasury & Banking / source owner |
| External banking evidence is stale or unverifiable. | Registration could represent a nonexistent or unavailable account. | Keep request pending/rejected and refresh from the authoritative banking source. | External Bank / Treasury & Banking |
| Approval is missing, expired, rejected, or untraceable. | Account registration or status change lacks authority. | Preserve current state and obtain a valid immutable outcome. | Enterprise Governance |
| Concurrent aggregate change advances version. | Registration, activation, or closure attempt is stale. | Reload Treasury aggregate, re-evaluate guards, and submit a new attempt. | Treasury & Banking |
| Registration commits but event publication is interrupted. | Consumers may lack the completed account fact. | Recover publication from committed state without registering again. | Treasury & Banking |
| Partial Bank Reference or Treasury Account creation occurs. | Aggregate consistency and relationship traceability are broken. | Roll back the failed modification, preserve prior version, and publish no false event. | Treasury & Banking |
| Unauthorized Treasury use occurs before activation eligibility. | Financial integrity and segregation are breached. | Contain activity, preserve evidence, apply restriction, and route governance review. | Treasury & Banking / Governance |
| Closure is attempted with unresolved dependent activity. | Payments, settlements, reconciliation, or reporting may lose a valid account reference. | Block closure until each canonical owner supplies a governed disposition. | Treasury & Banking / dependent owners |
| Bank status changes after internal approval. | Internal operational decision may become stale. | Re-evaluate relationship status and account availability before use or closure. | Treasury & Banking |
| Historical evidence becomes inaccessible after closure. | Auditability and traceability are impaired. | Restore authorized historical accessibility without rewriting account history. | Treasury & Banking / Audit / Reporting |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Treasury & Banking owns Treasury Accounts, Bank Account References, operational status, lifecycle, and events. | Treasury & Banking | No external domain may directly create or change internal account state. | `L2-F10`, Scope, Bounded Context, and Ownership Rules |
| Treasury Account is an internal business concept, not a physical bank account. | Treasury & Banking | Internal registration or closure does not open or close the bank-owned account. | `L2-F10`, Treasury Account Entity Specification |
| The external bank remains authoritative for banking infrastructure and original banking information. | External Banking Institution | Treasury stores a governed business reference only. | `L2-F10`, Bank Account Reference and Banking Events |
| Treasury Account maintains identifier, name, operational status, currency, and associated Bank Account Reference. | Treasury & Banking | These are mandatory internal business attributes. | `L2-F10`, Treasury Account Entity Specification |
| Bank Account Reference maintains bank, branch, account identifier, currency, and relationship status. | Treasury & Banking / external source | Registration must preserve banking relationship traceability. | `L2-F10`, Bank Account Reference Specification |
| Treasury Account behaviour is governed by classification, activation policy, operational usage, and lifecycle defaults. | Treasury & Banking | Operational use cannot bypass configuration and policy. | `L2-F10`, Treasury Account Configuration |
| Bank Account usage is governed by default operational/settlement/collection accounts, availability, and priority. | Treasury & Banking | Account selection and use remain configuration-driven. | `L2-F10`, Bank Account Configuration |
| Authorization is evaluated before aggregate modification and remains externally owned. | Authorization & Policy | Treasury consumes authorization outcomes only. | `L2-F10`, Security Constraints |
| Aggregate identity and relationships are immutable/traceable; ownership does not cross contexts. | Treasury & Banking | External banking identity cannot replace Treasury aggregate identity. | `L2-F10`, Aggregate and Ownership Constraints |
| Successful modifications advance version; failures preserve version and history. | Treasury & Banking | Account lifecycle changes are protected from conflicting simultaneous decisions. | `L2-F10`, Aggregate Versioning and Failure Constraints |
| Events represent completed immutable facts and publish only after consistency. | Treasury & Banking | Requests and failed operations cannot masquerade as registration success. | `L2-F10`, Domain Event Constraints |
| Treasury never creates Journal Entries or determines payment approval. | Accounting & GL / approval owners | Account lifecycle cannot absorb external accounting or approval responsibilities. | `L2-F10`, Boundary and Integration Constraints |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Bank Account Request | No canonical state model | Capture, validate, approve, reject, or defer the institutional request. | Approved/rejected/pending as business outcomes only | Request/governance owner unresolved | Canonical register; no L2 model |
| Bank Account Reference | No canonical lifecycle defined | Register authoritative banking relationship representation and maintain relationship status. | Registered relationship reference | Treasury & Banking | `L2-F10`, Bank Account Management and event catalogue |
| Treasury Account | No canonical state enumeration | Register internal account and initialize operational status under lifecycle defaults. | Registered Treasury Account | Treasury & Banking | `L2-F10`, Treasury Account Management |
| Treasury Account operational use | Operational status exists but values are undefined | Apply activation policy and permitted-use controls. | Canonical title calls this `Active`; L2 state undefined | Treasury & Banking | Canonical register and `L2-F10` configuration only |
| Treasury Account termination | Closure behaviour absent | End new operational use after governed eligibility and preserve history. | Canonical title calls this `Closed`; L2 state undefined | Treasury & Banking | Canonical register only; architectural gap |
| Treasury Aggregate | Illustrative aggregate lifecycle state | Encloses account modifications and advances version while preserving aggregate invariants. | Existing aggregate state unless its own lifecycle is separately changed | Treasury & Banking | `L2-F10`, Aggregate Lifecycle |

The Treasury Aggregate lifecycle (`Initialized -> Operational -> Restricted -> Suspended -> Archived`, with recovery paths) must not be substituted for the missing Treasury Account lifecycle. They are distinct business concepts.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `BankAccountReferenceRegistered` | Treasury & Banking | Reporting, Audit, Treasury consumers | Announces completed association of an institutional banking relationship. | `L2-F10`, Banking Events |
| `TreasuryAccountRegistered` | Treasury & Banking | Financial Reporting and other configured consumers | Announces completed internal Treasury Account registration. | `L2-F10`, Banking Events and Reporting Integration |
| `BankRelationshipUpdated` | Treasury & Banking | Treasury consumers, Reporting, Audit | Communicates a completed change in the represented banking relationship. | `L2-F10`, Banking Events |
| Institution/Organisation lifecycle facts | Institution and Organisation Management | Treasury & Banking | Provide authoritative institutional context and lifecycle. | `L2-F10`, Institution and Organisation Integration |
| Financial Period lifecycle facts | Financial Period & Closing | Treasury & Banking | Constrain Treasury activity without modifying Treasury Account ownership. | `L2-F10`, Financial Period Integration |

No canonical BankAccountRequested, TreasuryAccountApproved/Rejected, TreasuryAccountActivated, TreasuryAccountStatusChanged, TreasuryAccountClosureRequested, TreasuryAccountClosed, or BankAccountReferenceClosed event exists.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Requesting institutional function | Treasury & Banking / Governance | Institution, purpose, classification, bank/branch/account identity, currency, intended use, and evidence | Establishes the bank-account request. | Canonical scenario; exact L2 contract absent |
| External Bank / authoritative banking source | Treasury & Banking | Bank, branch, account identifier, currency, relationship status, availability, and source evidence | Grounds the internal Bank Account Reference. | `L2-F10`, Bank Account Reference; exact integration contract incomplete |
| Enterprise Governance / Authorization & Policy | Treasury & Banking | Approval/rejection, authority, policy result, restrictions, effective scope, and evidence | Permits or denies aggregate modification and operational use. | `L2-F10`, Authorization Integration; exact account contract incomplete |
| Treasury & Banking | Reporting / Audit / GL | Completed registration, relationship, status, and historical facts | Supports independent reporting, audit, and accounting reactions. | `L2-F10`, Event-driven collaboration; status/closure contracts incomplete |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Submit Bank Account Request | No canonical actor defined | Must provide complete institutional purpose and authoritative banking evidence. | Requesting function unresolved |
| Verify banking relationship | No canonical actor defined | Must use authoritative external facts without assuming bank ownership. | Treasury & Banking / external source |
| Approve or reject registration | No canonical role/hierarchy defined | Requires institutional governance and policy. | Enterprise Governance |
| Register Bank Account Reference | No canonical actor defined | Treasury-only aggregate behaviour after invariants succeed. | Treasury & Banking |
| Register Treasury Account | No canonical actor defined | Must associate Institution, currency, operational status, and governed Bank Account Reference. | Treasury & Banking |
| Authorize operational activation | No canonical actor or guard defined | Must follow account activation policy and external authorization. | Enterprise Governance / Treasury & Banking |
| Approve closure/non-operational outcome | No canonical actor or guard defined | Must preserve dependent activity and historical evidence. | Enterprise Governance / Treasury & Banking |

## 15.2 Segregation of Duties

- Request, banking verification, approval, registration, activation, operational use, closure approval, and audit review should be independently governed; mandatory combinations/prohibitions are not defined.
- The owner of payment execution must not gain authority to approve the underlying account merely through operational use.
- Treasury records authorization outcomes but does not determine institutional authority.
- The bank's physical-account authority must remain separate from SutraERP's internal Treasury Account authority.
- Audit and Reporting remain read-only consumers.

## 15.3 Mandatory Audit Evidence

- Request identity, Institution, Organisation context, purpose, classification, intended use, currency, and supporting documents.
- External bank, branch, account identifier, relationship status, availability, verification source, and verification sequence.
- Duplicate/conflict checks and resolution.
- Authorization, policy version, approval/rejection, conditions, restrictions, and delegated authority evidence.
- Bank Account Reference identifier, registration version, association, and `BankAccountReferenceRegistered`.
- Treasury Account identifier, name, currency, initial operational status, lifecycle configuration, association, version, and `TreasuryAccountRegistered`.
- Every subsequent status/relationship change, prior/resulting value, reason, authority, version, and event where available.
- Operational activation and closure decisions, dependent-activity checks, exceptions, failures, and recovery.
- Evidence that the external bank account remained externally owned and that no history was destroyed.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

| Measure | Business Meaning | Expected Direction / Acceptance Intent |
|---|---|---|
| Approved account-registration completeness | Requests with verified identity, currency, purpose, policy, authority, and evidence | 100% |
| Duplicate active representation count | Conflicting Treasury Accounts for the same governed banking relationship | Zero |
| Unauthorized operational-use count | Accounts used before valid activation eligibility | Zero |
| External ownership violation count | Treasury behaviour directly modifying bank-owned infrastructure/facts | Zero |
| Traceable relationship rate | Treasury Accounts linked to authoritative Bank Account References and Institutions | 100% |
| Closure with unresolved dependency count | Accounts made non-operational without governed dependent-activity disposition | Zero |
| Historical reconstruction rate | Request-to-registration-to-use-to-closure histories reconstructable | 100% |

No canonical approval SLA, verification validity period, activation threshold, closure waiting period, duplicate-match rule, inactivity threshold, or quantitative acceptance tolerance is defined upstream.

## 16.2 Postconditions

- Filename and Scenario ID match the frozen canonical register.
- Bank Account Reference and Treasury Account are treated as distinct Treasury-owned concepts, while the physical bank account remains externally owned.
- Registration occurs only after Institution, banking relationship, currency, policy, authorization, and invariants are validated.
- `BankAccountReferenceRegistered` and `TreasuryAccountRegistered` publish only after successful aggregate changes.
- No duplicate or cross-institution account representation is silently created.
- Operational use follows configured activation policy and restrictions.
- The intended closure outcome preserves identity, relationship, events, versions, dependent traceability, and audit history.
- Failed operations preserve prior aggregate version and publish no false completed fact.
- No payment, settlement, accounting, or bank-owned behaviour is absorbed into this account scenario.
- Missing Active/Closed lifecycle semantics are explicitly held for architecture resolution rather than invented.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| ID | Open Issue / Gap | Architectural Impact | Required Resolution Owner |
|---|---|---|---|
| OI-F056-001 | The canonical title requires `Active` and `Closed` Treasury Account outcomes, but `L2-F10` defines no Treasury Account state enumeration. | Lifecycle entry, transition, and acceptance cannot be finalized. | Treasury & Banking / Finance Architecture Governance |
| OI-F056-002 | `L2-F10` defines the Treasury Aggregate lifecycle, but not the child Treasury Account lifecycle; substituting one for the other would violate semantics. | Implementations may incorrectly close/suspend the entire Treasury aggregate. | Treasury & Banking |
| OI-F056-003 | No activation or closure transition guards, mandatory dependencies, authority, or historical-preservation rules exist for Treasury Accounts. | Active/Closed outcomes cannot be deterministically governed. | Treasury & Banking / Enterprise Governance |
| OI-F056-004 | No canonical Bank Account Request aggregate/entity, state model, owner, or event exists. | Request initiation, approval, rejection, expiry, amendment, and audit behaviour are incomplete. | Treasury & Banking / Enterprise Governance |
| OI-F056-005 | No `TreasuryAccountActivated`, `TreasuryAccountClosed`, status-change, activation-rejected, or closure-rejected event exists. | Consumers cannot reliably coordinate operational availability. | Treasury & Banking |
| OI-F056-006 | Relationship/cardinality between Treasury Account and Bank Account Reference is not defined. | One-to-one, one-to-many, and reuse rules remain ambiguous. | Treasury & Banking |
| OI-F056-007 | Duplicate detection rules for bank/branch/account/currency, masked identifiers, institution, and shared accounts are absent. | Duplicate or conflicting representations may be created. | Treasury & Banking / Data Governance |
| OI-F056-008 | Physical bank-account opening, verification, KYC, mandate, signatory, and closure hand-offs are outside L2-F10 and have no canonical integration contract. | External banking readiness cannot be completely verified. | Banking Integration / Compliance Governance |
| OI-F056-009 | Account activation policy and lifecycle defaults are configurable but their allowed values and invariants are undefined. | Institution configurations may produce incompatible lifecycle behaviour. | Treasury & Banking / Configuration Governance |
| OI-F056-010 | No rule defines required disposition of scheduled payments, batches, settlements, unreconciled transactions, balances, mandates, or forecasts before account closure. | Closure may break dependent Treasury processes and audit trails. | Treasury & Banking / affected domain owners |
| OI-F056-011 | No distinction is defined among account restriction, suspension, deactivation, external bank closure, and internal Treasury Account closure. | Operational state and relationship state may diverge or be conflated. | Treasury & Banking |
| OI-F056-012 | Actors, approval hierarchy, banking verifier, signatory authority, activation authority, closure authority, and segregation rules are absent. | Authorization and accountability cannot be fully accepted. | Enterprise Governance / Authorization & Policy |
| OI-F056-013 | Exact request, banking verification, authorization, registration, activation, status, closure, and consumer event contracts are incomplete. | Cross-domain hand-offs cannot be fully validated. | Relevant contract owners |
| OI-F056-014 | Repeat-request handling, simultaneous requests, late bank-status changes, version conflicts, partial registration, and published-event outcome confirmation lack complete business rules. | Duplicate or inconsistent account facts may occur. | Treasury & Banking |
| OI-F056-015 | Retention, archival, privacy, masking, authorized historical access, and post-closure correction rules are not defined for Treasury Accounts. | Closure may impair compliance and audit reconstruction. | Treasury & Banking / Compliance / Data Governance |
| OI-F056-016 | No canonical SLA, verification validity period, approval threshold, inactivity period, closure timing, or acceptance tolerance exists. | Measures remain directional rather than fully testable. | Enterprise Governance / Treasury & Banking |

This scenario remains **Draft** until the Treasury Account lifecycle, request ownership, activation and closure authority, dependency-disposition rules, banking hand-offs, events, contracts, and measurable acceptance criteria are defined. The draft does not convert the Treasury Aggregate lifecycle into an account lifecycle or create authority over the physical bank account.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**End of Scenario L3-F-056**
