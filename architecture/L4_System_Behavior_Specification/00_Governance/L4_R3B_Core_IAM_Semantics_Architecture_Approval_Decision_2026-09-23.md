# SutraERP L4 R3B — Core IAM Semantics — Architecture Approval Decision — 2026-09-23

**Version:** 1.0.0  
**Status:** Architecture Approved  
**Owner:** Lead Architect  
**Architecture Level:** Cross-level L2/L3/L4 governance  
**Decision Type:** Post-L5-validation upstream blocker remediation  
**Scope:** A02 User Account / Person reconciliation; A03 authentication/recovery/MFA; L3-A-004…009; L4-A-004…009  

## 1. Decision

The core human IAM semantic blockers identified during L5 production validation have been resolved at their canonical upstream owners and propagated downward.

### A02 decisions

- User Account creation requires both an authoritative login-capability requirement and User Account Provisioning Authority.
- `IdentityActivated` alone does not authorize User Account creation.
- `Enabled` is the canonical User Account state eligible to proceed to authentication.
- Person Reconciliation Authority governs Person merge/split approval.
- Merge selects one surviving canonical Person and assigns explicit Identity/User Account dispositions.
- Split restores the evidence-supported number of Persons and assigns explicit Identity/User Account dispositions.
- Downstream-owned references are reconciled by their owners after `PersonMerged` / `PersonSplit`.

### A03 decisions

- Authentication Attempt owns generic authentication execution events.
- Authentication Session follows distinct `Created → Active` steps.
- Credential recovery is generic where method/policy supports it.
- replacement Credential uses `Pending Activation → Active` and `CredentialActivated`;
- previous covered Credentials are revoked through `RevokeCredential` / `CredentialRevoked`;
- Authentication Recovery Authority governs exceptional administrative recovery;
- policy-required session consequences are completion barriers;
- new MFA enrollment uses `Pending Verification → Verified → Active`;
- `ActivateMFAFactor` is the activation operation and `MFAActivated` remains the activation event.

## 2. Canonical Artifacts

Added Locked amendments:

- `L2-A02_User_Account_and_Person_Reconciliation_Authority_Amendment_2026-09-23.md`
- `L2-A03_Authentication_Recovery_Session_and_MFA_Clarification_Amendment_2026-09-23.md`

Updated Locked L3 scenarios to v1.1:

- `L3-A-004`
- `L3-A-005`
- `L3-A-006`
- `L3-A-007`
- `L3-A-008`
- `L3-A-009`

Updated L4 specifications to v1.0.1 / Architecture Approved:

- `L4-A-004`
- `L4-A-005`
- `L4-A-006`
- `L4-A-007`
- `L4-A-008`
- `L4-A-009`

## 3. Hard-Fail Validation

The six remediated L4 specifications pass the scoped L4 hard-fail gate.

Verified:

- no L4 invention of business authority;
- no duplicate Person/Identity/User Account ownership;
- no blind merge/split of authentication state;
- no downstream-domain data ownership transfer;
- no new unsupported account state;
- no ambiguous “Active User Account” remains authoritative for authentication eligibility;
- generic authentication event publisher is explicit;
- session creation/activation sequence is explicit;
- recovery Credential lifecycle and prior-Credential invalidation use existing A03 lifecycle/events;
- MFA activation uses an explicit operation and existing event;
- no API/database/service/UI/infrastructure design is introduced.

## 4. Remaining IAM Semantic Item

R3B does **not** define the lifecycle of non-human technical principals.

That concern belongs to L5 D01 technical identity architecture and shall be resolved separately without fabricating Person/User Account business semantics.

Existing D02 authorization binding decisions remain authoritative and are not reopened.

## 5. Downstream Effect

R3B removes the upstream human-IAM semantic and Draft-L4 blockers associated with A004–A009.

D01 L5 specifications must still undergo:

1. exact L4 BH→L5 traceability validation;
2. targeted D01 revalidation;
3. separate non-human-principal lifecycle resolution where material;
4. 105-row register reconciliation;
5. lifecycle promotion only under separate authority.

## 6. Final Decision

```text
L2 amendments added                    2 Locked
L3 scenarios reconciled                6 / 6 Locked v1.1
L4 specifications approved             6 / 6
Human IAM upstream semantic blockers   RESOLVED
Non-human principal lifecycle          SEPARATE L5-OWNED ITEM
L5 dispositions changed                0
Final L5 Corpus Lock                   NOT AUTHORIZED
```

**Final Decision:** `R3B PASS — CORE HUMAN IAM SEMANTICS RESOLVED AT L2/L3/L4; PROCEED TO L5-OWNED NON-HUMAN PRINCIPAL LIFECYCLE AND CONTROLLED D01 TRACEABILITY`.
