# SutraERP L6 — Interaction, State and Feedback Standard — Draft

**Version:** 0.1.0  
**Status:** Draft  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture  
**Last Updated:** 2026-09-16  
**Depends On:** L6 Governance and Architecture Standard (Draft); L6 Design System Foundation (Draft); `L5-G-01-001`; `L5-G-01-002`; `L5-G-01-003`; applicable L4 behavior.  

---

## 1. Purpose

Define consistent interaction and feedback semantics for SutraERP so users can reliably distinguish business outcomes from client-side, technical and transitional states.

## 2. State Classes

L6 shall distinguish at minimum:

| State Class | Meaning | UI obligation |
|---|---|---|
| Authoritative success | Owning capability confirmed completion | Show confirmed outcome and refresh affected projections |
| Authoritative rejection | Owning capability rejected the requested operation | Show source-supported rejection and supported next action |
| Pending | Client is awaiting authoritative resolution | Show in-progress state; prevent unsafe duplicate action where required |
| Indeterminate | Client cannot establish whether mutation completed | Do not show success; reconcile before unsafe repeat |
| Technical failure | Execution failed for technical reasons without business rejection | Explain unavailability/retry only where safe |
| Stale / invalidated | Projection is no longer valid for current context | Remove/quarantine stale content and re-establish current context |
| Legitimate empty | Valid context contains no applicable records | Explain absence and supported next action if any |
| Partial composition | Some independently consumable projections are available and others are unavailable | Identify unavailable portion without inventing missing facts |

## 3. Mutation Interaction Model

For protected mutations:

```text
User initiates action
  ↓
Client validates presentation/input constraints
  ↓
Pending
  ↓
Authoritative result
  ├─ Success → confirmed outcome
  ├─ Rejection → rejection presentation
  └─ Technical ambiguity → indeterminate
```

Client-side validation is assistance only and does not replace authoritative validation.

## 4. Preventing Duplicate Actions

While a mutation is pending or indeterminate, the UI shall prevent or clearly constrain a duplicate attempt when repeating could create a second business operation.

The UI must not invent idempotency guarantees. Safe repeat behavior is inherited from L5/C01 semantics.

## 5. Success Feedback

Success feedback shall:

- represent only authoritative completion;
- state the resulting business position using upstream terminology where material;
- avoid celebratory visual treatment for routine high-volume ERP work;
- refresh or invalidate affected projections as defined by client architecture;
- preserve navigation continuity unless the workflow meaningfully completes and moves context.

## 6. Rejection Feedback

Authoritative rejection shall:

- remain distinct from technical failure;
- use the owning domain's supported reason/context;
- identify fields or conditions requiring correction when available;
- expose only supported continuation/correction/resubmission paths;
- never imply the user can override a decision without upstream authority.

## 7. Indeterminate Outcome

When a protected mutation outcome is technically ambiguous:

- do not show a success confirmation;
- do not automatically repeat an unsafe mutation;
- communicate that confirmation is pending/reconciliation is required;
- prevent users from unknowingly initiating a duplicate operation where material;
- reconcile through the authoritative owner before the UI transitions to success/rejection.

This is mandatory under the approved L5-G-01-003 boundary.

## 8. Loading

Loading states should preserve user orientation.

Use the least disruptive pattern appropriate to the scope:

- inline indicator for local content;
- skeleton for predictable content layout;
- section loading for independent workspace panels;
- full-workspace loading only when the workspace cannot responsibly render without required context.

Loading state must not erase known safe context unnecessarily.

## 9. Empty States

A legitimate empty state shall answer:

1. What is absent?
2. Is this expected/valid?
3. What can the user do next, if anything?

Do not use an error pattern for valid absence.

## 10. Unavailable and Partial Failure

Optional or independently consumable projections should fail locally where upstream architecture permits.

Example:

```text
Student core workspace → available
Finance projection → temporarily unavailable
Examination projection → available
```

The UI may continue presenting unaffected authoritative projections while clearly identifying the unavailable section. It shall not synthesize or cache-cross-contaminate missing data.

## 11. Scope Change

When Tenant/Institution or other protected scope materially changes:

- clearly communicate the new context;
- invalidate/quarantine prior protected projections according to L5-G-01-003;
- cancel or isolate stale pending reads where possible;
- prevent late results from prior context appearing as current;
- restore navigation to a valid location for the new scope.

## 12. Authorization Presentation

UI presentation may use capability/permission projections for usability.

Possible behaviors include:

- hiding irrelevant actions;
- disabling actions with explanatory context;
- showing read-only presentation;
- allowing the operation to reach authoritative enforcement where required.

No one pattern is universally mandated. Server-side authorization remains authoritative.

An authoritative deny shall be rendered as deny/rejection, never converted to technical error or success.

## 13. Destructive / High-Consequence Actions

Confirmation is required only when it materially reduces foreseeable error.

Use stronger confirmation for actions that are:

- destructive;
- difficult or impossible to reverse;
- high impact across many records;
- legally/audit significant;
- likely to be confused with a routine action.

Avoid confirmation fatigue for ordinary safe actions.

## 14. Forms and Validation

Forms shall support:

- immediate client-side format guidance where useful;
- authoritative validation after submission;
- preservation of user input after correctable rejection;
- clear error summary for multi-field failures where needed;
- focus movement/announcement to errors where accessibility requires;
- explicit pending state;
- distinction between save draft and submit/commit actions where upstream architecture defines such distinction.

L6 shall not invent Draft/Submitted states for a business lifecycle that lacks them upstream.

## 15. Notifications and Toasts

Transient toasts are appropriate for low-complexity confirmations or non-blocking information.

Do not rely on transient toast alone for:

- critical rejection;
- high-consequence outcome;
- unresolved/indeterminate mutation;
- information the user must act on later;
- persistent business state.

Persistent workspace/status presentation is required where the outcome remains materially relevant.

## 16. Long-Running Operations

Where upstream behavior allows long-running processing:

- distinguish accepted/queued/in-progress from completed only when those semantics exist upstream;
- provide current known state without inventing progress percentages;
- allow safe navigation away where implementation semantics support it;
- expose completion/failure through an appropriate persistent or notification surface.

## 17. Bulk Operations

Bulk interactions shall:

- make selection scope explicit;
- show affected-record count;
- identify partial eligibility before commit when authoritative information is available;
- distinguish full success, partial authoritative success, rejection and technical uncertainty;
- avoid implying atomicity unless upstream architecture provides it.

## 18. Accessibility of Feedback

Feedback shall not depend on color or animation alone.

Status changes and errors shall be available to assistive technology through appropriate semantics/live announcements where needed.

Focus shall move only when doing so improves recovery or task continuity; arbitrary focus theft is prohibited.

## 19. UX Acceptance Checks

A conforming interaction shall allow a user to answer:

- What context am I acting in?
- What action did I request?
- Is it pending, confirmed, rejected, unavailable or indeterminate?
- What business state/result is currently authoritative?
- What can I safely do next?

## 20. Open Decisions

- exact toast persistence durations;
- exact loading indicator timings;
- optimistic presentation policy by operation class;
- standard pattern for indeterminate reconciliation messaging;
- partial-failure visual treatment;
- bulk-operation progress pattern;
- global vs local error boundary presentation.

---

**Draft verdict:** suitable for interaction-standard review and frontend prototype validation; not yet canonical.
