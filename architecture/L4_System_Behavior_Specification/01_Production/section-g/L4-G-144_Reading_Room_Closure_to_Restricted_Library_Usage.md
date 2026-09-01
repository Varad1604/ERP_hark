# L4-G-144 — Reading Room Closure to Restricted Library Usage

## Specification Metadata
**Specification ID:** L4-G-144  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Library Management  
**Realizes:** L3-G-144 — Reading Room Closure to Restricted Library Usage  
**Primary Owner:** Library Management  
**Status:** Draft  
**Version:** 1.0.0

## Behavioral Objective & Boundary
Govern the transition of an operating Reading Room to **Operational Closure**, ending ordinary availability for the applicable operating period. “Restricted usage” is a business consequence and not a new lifecycle state. The behavior ends at Operational Closure and does not initiate a later opening.

## Required Context
- Reading Room is in an applicable operating condition.
- Operating period and closure condition are authoritative.
- Applicable Reading Room rules and Library Management authority are available.

## System Behavioral Obligations
1. Identify the authoritative Reading Room and operating period.
2. Establish the applicable closure condition and rules.
3. Complete relevant monitoring/supervision required by the business process.
4. Validate that closure conditions are satisfied.
5. Establish **Operational Closure**.
6. End ordinary Reading Room availability for the operating period.
7. Preserve closure history and decision evidence.

## Failure / Continuation
The system shall not assert closure when the closing condition, operating rules, or required authority cannot be established. Existing authoritative restrictions shall remain effective. No `Closed`, `Restricted`, `Closing`, or similar lifecycle state shall be invented.

## Cross-Domain & Events
The L3 does not expose a dedicated canonical Reading Room closure event; none is invented. G remains sole owner.

## Audit & Completion
Completion requires authoritative Operational Closure, ended ordinary availability for the period, and traceable closure history.

## Traceability
**Parent:** L3-G-144 — Reading Room Closure to Restricted Library Usage.

## L4 Boundary
No technical scheduling, authentication, access control, persistence, infrastructure, or deployment behavior is specified.