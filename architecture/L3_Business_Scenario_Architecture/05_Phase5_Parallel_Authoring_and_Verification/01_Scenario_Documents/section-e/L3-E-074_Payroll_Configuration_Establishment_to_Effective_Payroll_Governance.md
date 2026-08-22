# Payroll Configuration Establishment to Effective Payroll Governance

**Scenario ID:** L3-E-074  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Payroll — People & Workforce  
**Participating Domains:** E — People & Workforce  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-E06 — Payroll Workflows; L2-E01 — Workforce & Engagement; applicable Compensation and Compliance configuration architecture; L3 Phase 5 Template v2.0  
**Referenced By:** None yet  

## 2. Scenario Purpose

Establish a governed Payroll Configuration that can be used as the authoritative versioned configuration for payroll execution.

The scenario ensures that payroll runs resolve against an approved and active configuration while preserving configuration history and preventing an unapproved or superseded configuration from becoming effective.

## 3. Business Outcome

A Payroll Configuration progresses through its governed lifecycle to **Active**, becoming the effective configuration against which applicable payroll runs may resolve.

The resulting active configuration remains versioned and traceable, while prior configurations are preserved through the lifecycle defined by L2-E06.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a Payroll Configuration requires establishment or a new configuration version is proposed for payroll governance.

### 4.2 End Boundary

The scenario ends when the approved configuration is Active and therefore available as the effective configuration for applicable payroll processing.

### 4.3 In Scope

- Establishing a Payroll Configuration.
- Business review.
- Approval of the configuration.
- Activation of the approved configuration.
- Effective-date governance.
- Version continuity and supersession.
- Historical preservation through the configuration lifecycle.
- Making the active configuration available to payroll-run validation.

### 4.4 Out of Scope

- Defining compensation policy owned by the Compensation domain.
- Defining statutory/compliance rules owned by their authoritative domain.
- Payroll calculation execution.
- Payroll payment execution or accounting.
- Employee master or Workforce Engagement lifecycle.
- Technical configuration storage, APIs, database structures, or implementation mechanics.

## 5. Ownership and Participating Domains

Payroll is the sole canonical owner of the Payroll Configuration Aggregate and its lifecycle.

Payroll does not become the owner of originating business policies merely because those policies are applied through payroll configuration. L2-E06 explicitly states that source domains own business facts and policies while Payroll executes approved payroll processing rules.

No additional participating domain is required for the scenario's canonical business outcome.

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| Payroll Administrator | Establishes and administers the Payroll Configuration within the payroll business process. |
| Authorized Payroll Approver | Provides the governed approval required before the configuration can become Active where the payroll governance process requires approval. |
| Source policy owners | Remain owners of originating compensation or compliance rules used by Payroll; they do not become owners of Payroll Configuration. |

The L2-E06 architecture explicitly identifies Payroll Administrator as the actor initiating payroll processing and an Authorized Payroll Approver as the decision authority for payroll approval. No technical permission model is introduced.

## 7. Preconditions and Entry Criteria

- Payroll Configuration is required for the relevant payroll processing scope.
- The proposed configuration can be reviewed under applicable payroll governance.
- Any originating compensation or compliance rules required by the configuration are available from their authoritative owners.
- The configuration's effective period is defined.
- The configuration does not bypass the governed lifecycle.

## 8. Trigger

A new Payroll Configuration or configuration version is required for effective payroll governance.

## 9. Main Business Flow

| Step | Business Behaviour | Result |
|---|---|---|
| 1 | Establish the proposed Payroll Configuration. | Configuration enters `Draft`. |
| 2 | Review the proposed configuration against applicable payroll governance and its intended effective period. | Configuration is ready for approval or is returned for correction. |
| 3 | Obtain the required business approval. | Configuration enters `Approved` only when the approval condition is satisfied. |
| 4 | Activate the approved configuration when it is eligible to become effective. | Configuration enters `Active`. |
| 5 | Preserve the configuration's version and effective history. | The active configuration is traceable and prior configuration history remains governed. |
| 6 | Make the Active configuration available for payroll-run validation. | Applicable payroll runs can resolve against an active configuration. |

### 9.1 Configuration Lifecycle

The authoritative L2-E06 lifecycle is:

`Draft → Review → Approved → Active → Superseded → Archived`

No additional configuration state is introduced.

## 10. Alternate Business Flows

### A1 — Review Returns Configuration for Correction

A configuration that is not ready for approval remains within the governed pre-active lifecycle until the business issues are corrected and it can proceed through review again.

### A2 — Configuration Version Supersedes an Active Configuration

When a successor configuration is approved and becomes effective, the prior Active configuration moves to `Superseded`. The prior configuration remains part of the governed historical lineage.

### A3 — Configuration Reaches Archival Eligibility

A superseded configuration progresses to `Archived` according to the authoritative lifecycle and retention governance.

### A4 — Required Source Policy Is Not Available

If required compensation or compliance input is unavailable from its owning domain, the Payroll Configuration cannot become Active until the required authoritative business input is available.

## 11. Failure, Interruption and Business Recovery

| Business Situation | Governed Resolution |
|---|---|
| Configuration fails business review | Keep it out of Active status and return it for correction. |
| Required approval is not obtained | Configuration does not progress to Active. |
| Effective period is invalid or conflicting | Resolve the business-date/configuration issue before activation. |
| Required originating policy is unavailable | Do not activate a configuration that depends on unavailable authoritative policy. |
| Proposed configuration would bypass lifecycle governance | Reject the progression and preserve the existing valid active configuration. |
| Existing active configuration must remain effective | Preserve it until a valid successor configuration becomes Active. |

Technical failure handling is outside L3.

## 12. Business Rules and Decision Constraints

The scenario applies the authoritative L2-E06 rules and ownership principles:

- A payroll run cannot be created unless an applicable configuration exists.
- Payroll uses approved configuration and rules; it does not own originating compensation or compliance policies.
- Every payroll calculation must execute against a specific rule/configuration version.
- Configuration history must remain versioned and traceable.
- Payroll Configuration is a primary aggregate of the Payroll domain.
- Payroll owns the transformation from governed business facts into payroll financial outcomes.
- Finalized payroll is immutable and is not directly modified as part of configuration governance.

No new payroll configuration rule is created in this L3.

## 13. State and Lifecycle Impact

### 13.1 Payroll Configuration

| State | Business Meaning in This Scenario |
|---|---|
| Draft | Proposed configuration is being established. |
| Review | Configuration is undergoing governed business review. |
| Approved | Required approval has been granted. |
| Active | Configuration is effective for applicable payroll processing. |
| Superseded | A successor configuration has replaced it as the effective configuration. |
| Archived | Configuration is retained as historical configuration after supersession. |

### 13.2 Payroll Run

An applicable Payroll Run must resolve against an Active configuration. This scenario does not create or advance a Payroll Run itself.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

L2-E06 defines canonical payroll-run and payroll-result events, including `PayrollRunCreated`, `PayrollCalculationStarted`, `PayrollCalculationCompleted`, `PayrollValidationCompleted`, `PayrollApproved`, `PayrollFinalized`, `PayslipGenerated`, `PayrollAdjustmentCreated`, and `PayrollClosed`.

L2-E06 does **not** define a separate named Payroll Configuration lifecycle event in the authoritative event catalogue available for this scenario. Therefore this L3 does not invent a configuration event name or publisher.

The business outcome is instead expressed through the authoritative Payroll Configuration lifecycle and its availability to Payroll Run validation.

### 14.2 Cross-Domain Collaboration

Where configuration references Compensation or Compliance policy, Payroll consumes the authoritative policy/version from the owning domain. Payroll applies those approved rules but does not acquire ownership of them.

The collaboration is a business dependency, not a transfer of ownership.

## 15. Business Authorization, Approval and Audit Obligations

The configuration must pass the governed review and approval path before becoming Active.

Payroll audit history must preserve configuration usage as part of the payroll audit model. L2-E06 states that payroll audit history maintains records for configuration usage, calculations, approvals, adjustments and finalization.

Approval evidence must remain traceable to the configuration decision. Section A remains the owner of technical Authorization and Audit infrastructure.

## 16. Completion Criteria and Postconditions

- The Payroll Configuration has progressed through the required governed lifecycle.
- The configuration is `Active`.
- Its effective version is identifiable for applicable payroll processing.
- Prior configuration history remains preserved through `Superseded` and `Archived` lifecycle treatment where applicable.
- Payroll Run validation can require and resolve an Active configuration.
- Originating Compensation and Compliance ownership remains unchanged.
- No finalized payroll record is directly modified by this scenario.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

The locked L2-E06 architecture defines the Payroll Configuration Aggregate and its lifecycle, but it does not provide a configuration-specific event name in the canonical event catalogue. This scenario therefore deliberately does not invent one.

The exact business actor mapping for configuration review/approval is less explicit than the payroll-run approval mapping. The scenario uses the repository-supported Payroll Administrator and Authorized Payroll Approver concepts without adding a new role.

### 17.2 Assumptions

No assumption overrides locked architecture.

### 17.3 Deferred Questions

Any future requirement for a dedicated Payroll Configuration event or a configuration-specific approval role must be resolved through upstream architecture governance before being introduced into L3.
