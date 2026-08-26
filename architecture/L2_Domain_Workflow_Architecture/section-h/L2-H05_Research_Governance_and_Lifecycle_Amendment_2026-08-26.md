# L2-H05 — Research Workflows — Governance and Lifecycle Amendment

| Attribute | Value |
|---|---|
| Target | L2-H05 — Research Workflows v1.0 |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-08-26 |
| Approved Through | ADR-H-RESEARCH-GOV-001 |
| Authority | Explicit Lead Architect remediation authorization recorded 2026-08-26 |

This amendment is canonical L2-H05 architecture. It completes missing governance semantics needed by frozen L3 outcomes without transferring ownership of Faculty, Student, Finance, Academic or document-management facts. Unaffected H05 content remains unchanged.

## 1. Business Authority

Existing H05 roles remain authoritative.

- **Research Director** is the canonical business authority for strategic approval, rejection, suspension, revocation, abandonment, closure and reopening decisions defined by this amendment where institutional approval is required.
- **Research Administrator** administers Research-domain processes and records governed decisions but does not replace the Research Director's approval authority.
- Existing investigators, reviewers, committees and other H05 actors retain their responsibilities where the base workflow already assigns them.

## 2. Research Proposal Withdrawal

A submitted Research Proposal may be withdrawn before project establishment when the Principal Investigator or other authorized proposal owner requests withdrawal and no immutable downstream award/project fact would be contradicted.

Canonical proposal branch:

`Submitted / Under Review / Approved → Withdrawal Requested → Withdrawn`

For an Approved proposal, withdrawal is allowed only before the approved proposal has established a Research Project or otherwise crossed a base-H05 irreversible downstream boundary. Once a project exists, the project lifecycle governs suspension/termination instead of proposal withdrawal.

Canonical events added:

- `ResearchProposalWithdrawalRequested`
- `ResearchProposalWithdrawn`

This supersedes any earlier wording that could be read as an absolute prohibition on withdrawal immediately after approval.

## 3. Grant Amendment and Funding Closure

Research owns the Grant lifecycle and research-side funding metadata; Finance remains authoritative for accounting, budgets, payments, postings and financial reconciliation.

A governed Grant may follow:

`Awarded → Active → Closure Pending → Closed`

An Active Grant may enter `Amendment Pending` and return to `Active` only after the applicable research approval and any required authoritative Finance confirmation have been received.

Grant closure requires:

1. research obligations completed or formally closed;
2. required sponsor/award evidence recorded;
3. authoritative Finance confirmation that the applicable financial reconciliation condition has been satisfied, where financial reconciliation is required.

H05 records only the Finance-confirmed reference/status; it shall not create or alter ledger, payment, budget or posting facts.

Canonical H05 events added:

- `ResearchGrantAmendmentRequested`
- `ResearchGrantAmendmentApproved`
- `ResearchGrantClosureInitiated`
- `ResearchGrantClosed`

## 4. Ethics Continuing Compliance

The Ethics Approval lifecycle is completed with the following governed outcomes:

`Submitted → Under Review → Approved`

Branches:

- `Under Review → Revision Required → Submitted`
- `Under Review → Rejected`
- `Approved → Suspended → Approved`
- `Approved / Suspended → Revoked`
- `Approved → Closed` when the governed research activity ends and no continuing ethics obligation remains.

`Revoked` is terminal for that approval record. A future approval requires a new governed ethics submission rather than mutation of the revoked record.

Canonical H05 events added:

- `EthicsRevisionRequired`
- `EthicsApprovalSuspended`
- `EthicsApprovalReinstated`
- `EthicsApprovalRevoked`
- `EthicsApprovalClosed`

## 5. Research Publication Governance

Research Publication metadata remains H05-owned; the authoritative content/document remains with the applicable source/document owner.

Canonical Publication lifecycle:

`Draft → Submitted → Under Review → Approved → Published → Archived`

Branches:

- `Under Review → Revision Required → Submitted`
- `Under Review → Rejected`
- a material correction after publication creates a successor publication version; the Published predecessor remains immutable and may become `Superseded` for current-use purposes.

Canonical H05 events added:

- `ResearchPublicationRevisionRequired`
- `ResearchPublicationRejected`
- `ResearchPublicationPublished`
- `ResearchPublicationSuperseded`
- `ResearchPublicationArchived`

## 6. Intellectual Property Lifecycle

H05 owns research IP disclosure and institutional protection-workflow metadata; it does not create external legal rights by itself.

Canonical Research IP lifecycle:

`Disclosed → Under Review → Protection Approved → Protection In Progress → Protected`

Governed alternative outcomes:

- `Under Review → Rejected`
- `Under Review / Protection Approved / Protection In Progress → Abandoned`
- `Protected → Commercialization Active → Commercialization Closed`
- `Protected → Closed` where no commercialization lifecycle is applicable.

`Protected` means authoritative evidence of the applicable protection outcome has been recorded in H05; H05 does not invent external legal status.

Canonical H05 events added:

- `ResearchIPDisclosed`
- `ResearchIPProtectionApproved`
- `ResearchIPProtectionRecorded`
- `ResearchIPAbandoned`
- `ResearchIPCommercializationActivated`
- `ResearchIPClosed`

## 7. Research Collaboration Lifecycle

Canonical Collaboration lifecycle:

`Requested → Under Review → Approved → Active → Closed`

Governed branches:

- `Under Review → Rejected`
- `Active → Amendment Pending → Active`
- `Active / Amendment Pending → Terminated`

A Closed or Terminated collaboration remains historical and immutable. External agreement/legal facts remain references to their authoritative evidence; H05 governs the Research collaboration relationship only.

Canonical events added:

- `ResearchCollaborationRejected`
- `ResearchCollaborationActivated`
- `ResearchCollaborationAmendmentApproved`
- `ResearchCollaborationTerminated`
- `ResearchCollaborationClosed`

## 8. Consultancy Delivery and Financial Reconciliation Boundary

H05 owns Consultancy operational lifecycle and delivery evidence. Finance exclusively owns accounting/payment/reconciliation truth.

Canonical Research-side Consultancy progression:

`Approved → Active → Delivery Submitted → Delivery Accepted → Financial Reconciliation Pending → Closed`

H05 may move from `Financial Reconciliation Pending` to `Closed` only after consuming an authoritative Finance confirmation/reference that the applicable reconciliation condition is satisfied.

H05 shall not publish a Finance-owned accounting or payment fact as if Research were its semantic owner.

Canonical H05 events added:

- `ResearchConsultancyDeliverySubmitted`
- `ResearchConsultancyDeliveryAccepted`
- `ResearchConsultancyFinancialReconciliationRequested`
- `ResearchConsultancyClosed`

## 9. Research Portfolio Governance Outcome

A Research Portfolio is a governed H05 projection/metadata aggregate derived from canonical Research records and referenced upstream facts. It shall not become an alternative owner of those source facts.

Canonical portfolio review progression:

`Compiled → Under Review → Approved → Published`

Branches:

- `Under Review → Revision Required → Compiled`
- `Published → Superseded` when a successor approved portfolio is published.

Canonical H05 events added:

- `ResearchPortfolioReviewStarted`
- `ResearchPortfolioApproved`
- `ResearchPortfolioRevisionRequired`
- `ResearchPortfolioPublished`
- `ResearchPortfolioSuperseded`

## 10. Cross-Domain Ownership Preservation

- Faculty capability, eligibility, availability, workload and profile remain owned by L2-E03.
- Student identity/lifecycle remains Section C-owned.
- Academic facts remain with their owning academic domains.
- Finance remains sole authority for ledger, budget accounting, payment, posting, taxation and financial reconciliation facts.
- H05 may consume authoritative external facts by contract/reference and publish only Research-owned lifecycle facts.

## 11. Downstream Effect

This amendment resolves the upstream gaps identified by L3-H-030 and L3-H-032 through L3-H-038, subject to scenario-level revalidation against the exact frozen outcome and source references.

## 12. Locked Decisions

The authority, lifecycles, events, Finance boundary and immutable-history rules in §§1–10 are Locked L2-H05 decisions under ADR-H-RESEARCH-GOV-001.
