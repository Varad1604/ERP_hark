# L2-A02 — User Account and Person Reconciliation Authority — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-A02 — Person & Identity Workflows |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-09-23 |
| Approved Through | Explicit Lead Architect decision in L5 Post-Validation Upstream Blocker Resolution programme |
| Scope | User Account provisioning authority; Person merge/split authority; Identity/User Account consequences; owner-preserving downstream reference reconciliation |

This amendment is authoritative only for the subjects listed above. All unaffected L2-A02 ownership, lifecycles, business rules and events remain unchanged.

## 1. User Account Provisioning Authority

A successful Person/Identity activation does **not** by itself authorize creation of a User Account.

A User Account may be created only when:

1. the Person and applicable Identity satisfy their authoritative activation prerequisites;
2. login capability is required by an authoritative business/policy basis; and
3. a governed **User Account Provisioning Authority** authorizes creation within the applicable Tenant scope.

`User Account Provisioning Authority` is an abstract business authority. Institutional or platform policy may map it to an eligible local role without changing this canonical responsibility.

A Person may remain Active without a User Account.

### 1.1 Canonical provisioning rule

```text
Active Person + eligible Identity
        +
authoritative login-capability requirement
        +
User Account Provisioning Authority
        ↓
User Account may be created
```

`IdentityActivated` alone is insufficient.

## 2. Canonical User Account Authentication Eligibility

The canonical User Account lifecycle remains owned by A02.

For authentication eligibility:

- `Enabled` means the User Account may proceed to A03 authentication subject to all A03 policy/credential/MFA checks.
- `Locked`, `Disabled`, and `Archived` are not authentication-eligible.
- legacy wording such as “Active User Account” shall be read as `Enabled User Account` for authentication eligibility.

This amendment creates no new User Account state.

## 3. Person Reconciliation Authority

**Person Reconciliation Authority** is the canonical abstract business authority for approval of Person merge and Person split decisions.

It may:

- approve or reject a proposed Person merge;
- approve or reject a proposed corrective Person split;
- require supporting duplicate/fraud/migration evidence before decision.

Institutional policy may map this authority to an eligible local office or governance role.

No universal separation-of-duties rule is introduced by this amendment.

## 4. Person Merge Semantics

A Person merge establishes exactly one surviving canonical Person for the reconciled real-world individual.

Rules:

1. only source-eligible Person records may enter the merge;
2. the surviving Person is explicitly selected under the approved reconciliation decision;
3. non-surviving Person records become historical through existing valid Person lifecycle transitions;
4. historical audit references are not rewritten;
5. `PersonMerged` remains the canonical merge fact published by Person & Identity.

### 4.1 Identity and User Account consequences

Associated Identities and User Accounts are **not blindly merged** merely because Persons are merged.

For each associated Identity/User Account relationship, Person & Identity shall establish one authoritative disposition:

- retain against the surviving Person;
- reassociate where source semantics and evidence permit;
- disable where continued use is not valid;
- archive where lifecycle conditions are satisfied;
- hold for manual/governed reconciliation where the correct disposition is unresolved.

No Credential, MFA factor or Authentication Session is directly reassigned by A02; A03 applies authentication-side consequences after receiving the authoritative A02 disposition.

### 4.2 Downstream-owned references

A02 shall not directly mutate business-domain-owned records.

After `PersonMerged`, downstream owners reconcile their own references through governed owner-preserving contracts/events using the surviving canonical Person reference.

The A005 end boundary is the authoritative Person/Identity reconciliation and publication of `PersonMerged`; downstream reference updates are subsequent consumer outcomes unless a domain-specific contract explicitly governs synchronous completion.

## 5. Person Split Semantics

A Person split is a corrective reconciliation that restores two or more canonical Person records where prior consolidation incorrectly represented multiple real individuals.

Rules:

1. Person Reconciliation Authority approves the split;
2. the preserved merge history is used as evidence;
3. each resulting Person follows the normal A02 Person/Identity establishment and activation semantics appropriate to its recovered evidence;
4. exact resulting cardinality is evidence-driven and is not limited to one-to-two;
5. `PersonSplit` remains the canonical split fact published by Person & Identity.

### 5.1 Identity and User Account consequences

Each associated Identity/User Account is explicitly assigned one governed disposition:

- retain with one resulting Person;
- reassociate to another resulting Person where evidence supports it;
- disable/archive where continuation is invalid;
- hold for governed reconciliation where evidence is insufficient.

No automatic Credential, MFA factor or session reassignment occurs in A02.

### 5.2 Downstream-owned references

After `PersonSplit`, downstream owners redistribute their own references using governed contracts/events. A02 publishes the canonical Person split fact but does not take ownership of downstream business data.

## 6. Audit Obligations

Provisioning, merge and split decisions shall preserve:

- Tenant;
- Person/Identity/User Account references;
- decision authority;
- login-capability basis;
- survivor/resulting Person references;
- per-Identity/User Account disposition;
- evidence and decision reason;
- event publication;
- downstream reconciliation correlation where applicable.

## 7. Locked Decisions

1. User Account creation requires an authoritative login-capability requirement plus User Account Provisioning Authority.
2. `IdentityActivated` alone never authorizes account creation.
3. `Enabled` is the canonical User Account state eligible to proceed to authentication.
4. Person Reconciliation Authority governs merge and split approval.
5. Merge selects one canonical surviving Person and preserves non-survivors historically.
6. Split restores the evidence-supported number of canonical Persons; it is not restricted to exactly two.
7. Identity/User Account consequences of merge/split are explicit per-association dispositions; no blind merge/split is permitted.
8. A02 does not directly reassign Credential, MFA or Authentication Session state.
9. Downstream business domains reconcile their own references after `PersonMerged` / `PersonSplit`.
10. No downstream ownership is transferred into A02.
