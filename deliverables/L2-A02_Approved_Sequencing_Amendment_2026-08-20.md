# L2-A02 Approved Sequencing Amendment — 2026-08-20

**Target repository file:** `architecture/L2_Domain_Workflow_Architecture/section-a/L2-A02_Person_and_Identity_Workflows.md`  
**Decision authority:** Lead Architect approval provided 2026-08-20  
**Purpose:** Resolve the Person–Identity–User Account sequencing conflict without changing canonical ownership, lifecycle-state names, event names, or event publishers.

---

## Change 1 — Document Information

Replace:

```markdown
| Last Updated | 2026-08-13 — INT-301 ownership amendment |
```

with:

```markdown
| Last Updated | 2026-08-20 — Person–Identity establishment and activation sequencing decision |
```

---

## Change 2 — Insert the Approved Sequencing Decision

Insert the following block immediately after the existing `Canonical Ownership Amendment — INT-301` block and before `# Document Information`:

```markdown
## Canonical Person–Identity Establishment and Activation Sequencing Decision — 2026-08-20

> **Approved by the Lead Architect on 2026-08-20.** This decision resolves the conflict between the legacy Activation Workflow and the User Account Service Contracts. It is authoritative over later legacy wording that places User Account creation, initial-context assignment, or authentication enablement before successful Person and Identity activation.

The canonical establishment and activation order is:

1. create the canonical Person and publish `PersonCreated`;
2. create the associated Identity in `Pending` and publish `IdentityCreated`;
3. apply the applicable Tenant verification policy and validate the required evidence;
4. transition the Person from `Created` to `Verified` and publish `PersonVerified`;
5. transition the Identity from `Pending` to `Verified` and publish `IdentityVerified`;
6. transition the Person from `Verified` to `Active` and publish `PersonActivated`;
7. transition the Identity from `Verified` to `Active` and publish `IdentityActivated`; and
8. only after successful Person and Identity activation, create a User Account where login capability is required.

Person and Identity activation form one governed institutional outcome. The outcome is incomplete unless both aggregates reach their authoritative `Active` state.

User Account creation is a post-activation A02 lifecycle action. It is not a prerequisite for Person or Identity activation, and a Person may remain active without a User Account.

Credential creation, authentication-factor or MFA enrollment, Authentication Provider lifecycle, Authentication Session lifecycle, and authentication enablement remain owned by L2-A03. Role, Role Assignment, Delegation, authorization-policy lifecycle, and initial authorization-context assignment remain owned by L2-A04.

This decision does not rename or re-own any lifecycle state, event, aggregate, or contract.

---
```

---

## Change 3 — Replace the Conflicting Verification and Activation Workflows

Find the existing block beginning with `# Verification Workflow` and ending with `A Person becomes operational only after successful activation.` Replace that complete block with:

```markdown
# Canonical Person–Identity Establishment and Activation Workflow

```text
Created Person

↓

Create Associated Identity

↓

Publish IdentityCreated

↓

Identity Pending

↓

Determine Tenant Verification Policy

↓

Validate Required Evidence

↓

Approval (if required)

↓

Person: Created → Verified

↓

Publish PersonVerified

↓

Identity: Pending → Verified

↓

Publish IdentityVerified

↓

Person: Verified → Active

↓

Publish PersonActivated

↓

Identity: Verified → Active

↓

Publish IdentityActivated

↓

Active Person and Active Identity
```

Verification policies remain configurable per Tenant.

The Person and its associated Identity must each reach `Verified` before activation. The establishment outcome completes only when both aggregates reach `Active` and their respective canonical activation events have been published.

User Account creation, where required, occurs only after successful Person and Identity activation. It is not part of the Person and Identity establishment outcome and does not enable authentication by itself.

Authentication capability, credentials, factors, providers, and sessions remain governed by L2-A03. Roles, role assignments, delegations, authorization policies, and authorization-context assignment remain governed by L2-A04.
```

---

## Change 4 — Correct the User Account Service Contract Constraint

Under `# User Account Service Contracts`, replace:

```markdown
User Account creation shall occur only after successful Person activation.
```

with:

```markdown
User Account creation shall occur only after successful Person and associated Identity activation. It is a post-activation A02 lifecycle action, is not a prerequisite for Person or Identity activation, and does not transfer credential, authentication-factor, Authentication Provider, Authentication Session, role, delegation, or authorization-policy ownership into A02.
```

---

## Validation Result

- Canonical Person states remain unchanged: `Draft → Created → Verified → Active`.
- Canonical Identity states remain unchanged: `Pending → Verified → Active`.
- Canonical event names and publishers remain unchanged.
- User Account remains owned by A02 but is moved after successful Person and Identity activation.
- Authentication ownership remains with A03.
- Authorization ownership remains with A04.
- The frozen L3-A-004 outcome remains unchanged.
- No API, database, service-implementation, UI, infrastructure, queue, or technical retry design is introduced.

## Remaining Upstream Issue Not Resolved by This Amendment

Locked L1 §3.4 states that one Person exists “inside an Institute,” while locked L2-A01, locked L2-A02, and the frozen Phase 2/Stage 3 semantics establish Tenant-scoped Person and Identity ownership. This separate L1/L2 terminology and uniqueness-scope conflict requires Lead Architect alignment before L3-A-004 can become `Architecture Approved` or `Locked`.
