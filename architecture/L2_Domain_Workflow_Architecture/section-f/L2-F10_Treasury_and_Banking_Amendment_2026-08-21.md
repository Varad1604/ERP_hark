# L2-F10 — Treasury & Banking — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-F10 — Treasury & Banking v1.0 |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-08-21 |
| Approved Through | ADR-FIN-TREAS-001 |
| Authority | Lead Architect approval recorded 2026-08-21 |

This amendment is part of the canonical L2-F10 architecture. Where it conflicts with L2-F10 v1.0, this amendment is authoritative. Unaffected source content remains unchanged.

## 1. Treasury Account Ownership

Treasury & Banking remains the sole canonical owner of the Treasury Account business concept and its lifecycle. A Treasury Account is an internal Treasury business account and is not the external bank account itself. Bank infrastructure and bank-originated records remain external.

## 2. Canonical Treasury Account Lifecycle

The minimum canonical lifecycle required for institutional treasury-account governance is:

`Registered → Active → Closed`

Rules:

1. `Registered` means the Treasury Account has been formally established inside the Treasury Aggregate but is not yet available for ordinary treasury use.
2. `Registered → Active` requires successful verification and governed approval.
3. `Active` means the Treasury Account is available for permitted treasury operations subject to Treasury restrictions and enterprise authorization.
4. `Active → Closed` requires governed closure approval and completion of applicable business closure checks.
5. `Closed` is terminal for operational use. Historical identity, references, activity and audit evidence remain permanently preserved.
6. Closure does not delete, repurpose or transfer the Treasury Account.
7. Treasury-wide `Restricted` or `Suspended` states continue to govern the Treasury Aggregate and may restrict use of an otherwise Active Treasury Account; they are not duplicated as Treasury Account lifecycle states by this amendment.

## 3. Canonical Business Actors and Authorities

| Actor / authority | Responsibility | Authority boundary |
|---|---|---|
| Treasury Account Requestor | Submits the governed business requirement and supporting account context. | Cannot self-verify, self-approve and activate solely by initiating the request. |
| Treasury Account Verifier | Verifies institutional, treasury, currency and banking-reference completeness. | Verification does not itself activate the account. |
| Treasury Account Approval Authority | Approves or rejects establishment/activation according to enterprise treasury governance. | Approval policy execution remains governed by enterprise authorization. |
| Treasury Account Administrator | Registers the approved Treasury Account and, after required approvals, activates it. | Cannot bypass verification or approval. |
| Treasury Account Closure Authority | Confirms closure eligibility and authorizes the `Active → Closed` transition. | Cannot remove historical evidence or alter external bank records. |

Institutional role mapping may vary, but the responsibilities and separation boundaries above are canonical.

## 4. Canonical Events

Treasury & Banking is the sole publisher of these Treasury Account lifecycle events:

| Event | Meaning |
|---|---|
| `TreasuryAccountRegistered` | Treasury Account establishment has completed in `Registered` state. Existing event retained. |
| `TreasuryAccountActivated` | Governed activation has completed and the Treasury Account is `Active`. |
| `TreasuryAccountClosed` | Governed closure has completed and the Treasury Account is `Closed`. |

These events represent completed Treasury-owned business facts and do not transfer account ownership.

## 5. Closure Preconditions

A Treasury Account may close only when Treasury governance confirms that:

- no prohibited pending treasury activity requires the account to remain operational;
- settlement and reconciliation responsibilities affecting closure have been addressed or explicitly governed;
- historical banking and treasury references will remain traceable;
- closure authorization is recorded.

Exact institution-specific monetary thresholds or closure checklists remain configurable policy and are not fixed by this L2 amendment.

## 6. Separation of Duties

Request, verification, approval and closure must be independently governable responsibilities. Institutional policy may combine operational execution responsibilities only where enterprise authorization permits, but initiation alone shall never constitute approval.

## 7. Locked Decisions

The lifecycle, authorities, events and closure semantics in this amendment are Locked under ADR-FIN-TREAS-001. L3-F-056 through L3-F-061 shall consume them and shall not introduce implementation recovery mechanics such as aggregate-version retries, persistence rollback or publication recovery.
