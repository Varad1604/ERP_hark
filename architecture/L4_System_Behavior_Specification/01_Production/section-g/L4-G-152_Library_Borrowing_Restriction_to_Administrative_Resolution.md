# L4-G-152 — Library Borrowing Restriction to Administrative Resolution

## Specification Metadata
**Specification ID:** L4-G-152  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Library Management  
**Realizes:** L3-G-152 — Library Borrowing Restriction to Administrative Resolution  
**Primary Owner:** Library Management  
**Status:** Draft  
**Version:** 1.0.0

## Behavioral Objective & Boundary
Govern an identified borrowing restriction through administrative review to an authoritative resolution, without inventing a new restriction lifecycle state.

## Required Context
- An authoritative library borrowing/member context exists.
- The restriction basis is identifiable.
- Applicable administrative authority and library rules are available.

## System Behavioral Obligations
1. Register the borrowing restriction against the authoritative context.
2. Preserve the basis and evidence for the restriction.
3. Validate the restriction against applicable library policy.
4. Route the matter through the governed administrative review path.
5. Record the authorised administrative resolution.
6. Preserve restriction and resolution history.

## Failure / Continuation
Incomplete evidence shall retain the matter under administrative review. An invalid restriction shall not be treated as authoritative. No unsupported fine, suspension, financial liability, or disciplinary consequence is invented.

## Cross-Domain & Events
G owns the library borrowing restriction and administrative resolution. Any external consequence follows its owning domain and established contract. No unsupported event name is invented.

## Audit & Completion
Completion requires an authoritative administrative resolution and traceable evidence of the restriction and decision.

## Traceability
**Parent:** L3-G-152 — Library Borrowing Restriction to Administrative Resolution.

## L4 Boundary
No technical access-control, authentication, database, API, or notification implementation is specified.