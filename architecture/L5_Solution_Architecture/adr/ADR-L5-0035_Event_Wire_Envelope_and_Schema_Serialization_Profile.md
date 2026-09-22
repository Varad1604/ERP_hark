# ADR-L5-0035 — Event Wire, Envelope & Schema Serialization Profile

**ADR ID:** `ADR-L5-0035`  
**Title:** Event Wire, Envelope & Schema Serialization Profile  
**Scope:** Shared / Family  
**Decision Owner:** L5-C-02 — Event & Messaging Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-22  
**Affected L5 Specifications:** `L5-C-02-001`; `L5-C-02-003`  
**Related ADRs:** `ADR-L5-0007`; `ADR-L5-0010`; `ADR-L5-0011`; `ADR-L5-0022`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context

NATS JetStream is already the canonical broker and C02 already governs durable delivery/version compatibility. The repository still requires one portable event-wire representation and one schema authority so producers and consumers do not invent incompatible payload/envelope formats.

## 2. Scope & Ownership

This ADR is owned by C02. It defines transport representation and schema governance only. Canonical event names, publishers, business meaning and owner-domain state transitions remain upstream-owned.

## 3. Decision

SutraERP business-event transport shall use:

1. **Serialization:** **JSON** for the canonical business-event wire payload.
2. **Schema authority:** each released event contract has a governed **JSON Schema (2020-12 family)** definition/version in the canonical contract catalog. Producer and consumer implementations conform to that schema.
3. **JSON naming:** SutraERP-defined JSON members use **camelCase**.
4. **Envelope:** the wire envelope is **CloudEvents 1.0-compatible**. Standard CloudEvents attributes retain their standard names/semantics. SutraERP context is added without redefining CloudEvents semantics.
5. **SutraERP context:** the envelope/payload context preserves, where applicable, canonical event contract/version reference, Tenant/Institution scope, correlation/causation, publisher/source reference, occurrence/publication timestamps and governed principal/actor reference.
6. **Compatibility:** additive backward-compatible change is preferred; breaking change creates a distinct governed successor event-contract version under ADR-L5-0011.
7. **NATS separation:** JetStream subject/routing names remain technical routing identifiers and are not the canonical business event name or publisher authority.
8. **Optimization exception:** Protobuf or another binary serialization may be introduced only for a measured, bounded high-volume technical stream through a separate exception/ADR; it is not the default business-event representation.

## 4. CloudEvents Compatibility Rule

Where CloudEvents defines a standard attribute (for example `specversion`, `id`, `source`, `type`, `time`, `datacontenttype`), SutraERP uses the standard attribute as defined. SutraERP-specific JSON structures use camelCase. No custom field may silently override a standard CloudEvents meaning.

## 5. Conformance Rules

- Events without a valid released schema/version are rejected from normal publication.
- Consumers must tolerate compatible additive fields according to the active contract.
- Canonical business-event identity never derives from a NATS subject alone.
- Tenant/Institution context cannot be inferred from broker placement.
- JSON Schema is contract evidence, not business-rule ownership.
- Numeric retention, maximum payload size and throughput limits remain E02/operational profile decisions.

## 6. Consequences

The event platform gains one human-readable, toolable and provider-neutral wire standard while retaining future binary-serialization escape hatches for measured needs. Contract evolution becomes independently testable across publishers and consumers.

## 7. Migration / Exit Position

A future serialization/envelope replacement requires a C02 superseding ADR and coexistence/consumer migration strategy under ADR-L5-0011.

## 8. External Standards Evidence

- CloudEvents specification: https://cloudevents.io/
- JSON Schema Draft 2020-12: https://json-schema.org/draft/2020-12

## 9. Approval Record

**Decision selected:** JSON + governed JSON Schema; camelCase SutraERP members; CloudEvents-compatible envelope; binary formats by measured exception only.  
**Approval source:** Explicit Lead Architect approval in the SutraERP architecture workflow on 2026-09-22.  
**Formal Status:** `Locked`.
