# L4-I-147 — Conflicting Completion Inputs to Valid Completion Basis

## 1. Specification Metadata
**Specification ID:** L4-I-147  
**Architecture Level:** L4 — System Behavior Specification  
**Realizes Scenario:** L3-I-147 — Conflicting Completion Inputs to Valid Completion Basis  
**Primary Business Owner:** Academic Completion & Completion Evidence  
**Participating Domains:** C — Student Lifecycle; D — Academic Delivery; I — Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0

## 2. Behavioral Objective & Boundary
Govern resolution of conflicting completion inputs so that a valid authoritative completion-evidence basis is established without altering source-owned academic facts.

**Start:** Conflicting completion inputs are identified.  
**End:** A valid authoritative completion basis is established, or the conflict remains explicitly unresolved.

## 3. Required Business Context
- Student/programme context is authoritative.
- Academic Delivery and Examination source outcomes are identifiable.
- Applicable completion-evidence rules are available.
- Canonical ownership of each source input is known.

## 4. Required System Behaviors
1. Identify the conflicting completion inputs and their source owners.
2. Validate provenance and authority of each input.
3. Apply the governing completion-evidence rules to determine whether a conflict can be resolved.
4. Obtain authoritative clarification where existing rules are insufficient.
5. Establish the resulting valid completion basis only when authority is sufficient.
6. Preserve conflicting inputs and resolution history.
7. Prevent non-authoritative inputs from being treated as current completion evidence.

## 5. Cross-Domain Behavior
Student Lifecycle, Academic Delivery, and Examination retain ownership of their respective source facts. Academic Completion & Completion Evidence owns the resulting completion-evidence basis only within its defined scope.

## 6. Failure / Recovery
Unresolved authority or incompatible source evidence prevents a conclusive completion basis. The system shall not locally choose a winner or fabricate missing academic facts.

## 7. Audit & Evidence
Preserve conflicting inputs, provenance, authority determination, governing rule, resolution decision, resulting basis, and historical evidence.

## 8. Observable Completion
A valid authoritative completion basis is established with traceable resolution evidence, or an explicit unresolved position remains.

## 9. Traceability
Parent: L3-I-147 — Conflicting Completion Inputs to Valid Completion Basis.

## 10. Deferred Design
Technical conflict detection, persistence, reconciliation tooling, and interfaces remain outside L4.