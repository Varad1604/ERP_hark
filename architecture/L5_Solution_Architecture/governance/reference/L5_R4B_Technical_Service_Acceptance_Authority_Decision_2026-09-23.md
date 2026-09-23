# SutraERP L5 R4B — Technical Service Acceptance Authority Decision — 2026-09-23

**Version:** 1.0.0  
**Status:** Architecture Approved  
**Owner:** Lead Architect / L5-E-05  
**Architecture Level:** L5 — Technical Governance  
**Decision Type:** Post-validation blocker remediation / technical acceptance authority  
**Scope:** L5-E-05-004 and E05 peer acceptance/evidence consumers  
**ADR Required:** No — this is a technical-governance responsibility decision, not a technology/product selection  
**Final L5 Corpus Lock:** NOT AUTHORIZED

---

## 1. Decision

SutraERP establishes **Technical Service Acceptance Authority** as the canonical L5 technical-governance authority for E05 technical service-acceptance decisions.

The authority decides one of exactly three E05 technical outcomes for one explicitly identified Acceptance Subject:

- `Ready`;
- `Ready with conditions`;
- `Not ready`.

This authority is intentionally separate from L2-A10 `Governance Authority`.

---

## 2. Canonical Boundary

### Technical Service Acceptance Authority owns

- final technical acceptance decision for one Acceptance Subject;
- acceptance of bounded residual technical risk for `Ready with conditions`;
- attribution of the decision to an accountable technical/governance actor;
- decision reference, evidence package reference and revalidation conditions;
- refusal of technical acceptance where mandatory technical evidence or blocking conditions are unresolved.

### Technical Service Acceptance Authority does not own

- Operational Readiness Assessment;
- Readiness Requirement, Finding, Improvement Plan or Certificate;
- institutional/business readiness certification;
- L1 dynamic capability-readiness `READY / NOT READY`;
- deployment evidence truth;
- resilience/performance/recovery evidence truth;
- security/authorization/audit evidence truth;
- build/release evidence truth;
- telemetry/diagnostic evidence truth;
- underlying remediation execution;
- business lifecycle authority.

Underlying evidence remains owned by E01/E02/E03/E04/E05/D/F01 and other applicable technical owners.

L2-A10 remains the canonical business/governance readiness and certification owner.

---

## 3. Acceptance Workflow

```text
Identify Acceptance Subject
→ assemble Technical Readiness Evidence Package
→ validate evidence provenance/applicability/freshness
→ evaluate mandatory technical gates
→ resolve blocking technical conditions
→ Technical Service Acceptance Authority decision
   ├─ Ready
   ├─ Ready with conditions
   └─ Not ready
→ record attributable Acceptance Decision Record
→ trigger revalidation on material change
```

Automation may prepare evidence, perform deterministic validation and recommend an outcome.

Automation shall not fabricate the accountable acceptance authorization.

---

## 4. Ready

`Ready` may be recorded only when:

1. the exact Acceptance Subject is identified;
2. every mandatory applicable technical evidence class is present and valid;
3. no unresolved blocking technical condition remains;
4. applicable peer evidence is current for the subject/version/configuration;
5. no known material technical risk is being silently ignored;
6. Technical Service Acceptance Authority authorizes the outcome.

`Ready` is a technical acceptance outcome only.

It is not an Operational Readiness Certificate and does not authorize every business operation.

---

## 5. Ready with conditions

`Ready with conditions` requires all of:

- identified residual technical risk;
- explicit accountable owner;
- compensating control where applicable;
- bounded review or expiry condition;
- objective resolution/exit criteria;
- evidence that the condition does not violate an unwaivable mandatory technical control;
- explicit Technical Service Acceptance Authority acceptance.

A condition shall not be used to bypass:

- unresolved critical security controls;
- invalid Tenant/isolation guarantees;
- unrecoverable data-integrity risk;
- mandatory evidence that the owning architecture declares non-waivable;
- unresolved canonical business/governance authority.

---

## 6. Not ready

`Not ready` is required where any mandatory technical acceptance condition fails, including:

- evidence missing, stale, invalid or not bound to the Acceptance Subject;
- critical technical verification failure;
- unresolved blocking architecture decision;
- unacceptable residual technical risk;
- acceptance authority absent;
- required revalidation incomplete after material change.

No production-acceptance automation may convert `Not ready` into `Ready` without a new valid decision.

---

## 7. Relationship to L2-A10 Governance Authority

Technical Service Acceptance Authority and L2-A10 Governance Authority are distinct.

```text
Technical architecture evidence
        ↓
E05 Technical Service Acceptance
        ↓
Technical Acceptance Decision Record
        ↓
may be consumed as governed evidence
        ↓
L2-A10 Operational Readiness Governance & Certification
        ↓
Governance Authority decision / certification
```

Rules:

1. L2-A10 may consume the technical acceptance record as evidence.
2. E05 shall not issue or simulate an Operational Readiness Certificate.
3. L2-A10 Governance Authority does not automatically become the Technical Service Acceptance Authority.
4. Institutional policy may map both abstract authorities to the same organizational person/body where permitted, but the architectural responsibilities remain separate.
5. A Technical `Ready` result does not imply institutional/business readiness.
6. An L2-A10 certificate does not replace current technical acceptance evidence where a technical gate requires it.

---

## 8. Separation of evidence owner and acceptance decision

Evidence-producing technical owners remain responsible for the truth of their evidence.

The Technical Service Acceptance Authority decides whether the assembled evidence is sufficient for technical acceptance.

It shall not rewrite or override the evidence owner's technical result merely to reach a desired acceptance outcome.

Where an evidence owner reports failure or an unresolved mandatory blocker, the acceptance gate shall respect the owning architecture's rule.

---

## 9. Revalidation

A prior technical acceptance decision shall be revalidated where a material change invalidates applicability, including:

- candidate artifact/version change;
- material configuration/runtime-profile change;
- target environment-class change;
- security/control change;
- dependency topology or required-service change;
- deployment/recovery mechanism change;
- expiry/review condition reached;
- evidence owner declares prior evidence stale/invalid.

Targeted revalidation may preserve unaffected evidence.

---

## 10. Audit obligations

The Acceptance Decision Record shall preserve at minimum:

- Acceptance Subject;
- candidate artifact/version;
- environment/profile;
- evidence package/version;
- applicable mandatory gates;
- failed/waived/conditional items;
- residual risks;
- accountable owners;
- compensating controls;
- review/expiry conditions;
- decision outcome;
- Technical Service Acceptance Authority identity/reference;
- decision time and correlation;
- prior decision lineage where revalidated.

Canonical audit storage remains D04-owned where applicable.

---

## 11. Locked/Approved Decisions

1. Technical Service Acceptance Authority is the canonical L5 technical acceptance authority.
2. It is separate from L2-A10 Governance Authority.
3. E05 technical outcomes are exactly `Ready`, `Ready with conditions`, and `Not ready`.
4. Automation may evaluate but shall not fabricate acceptance authority.
5. `Ready with conditions` requires explicit risk acceptance and bounded conditions.
6. Technical acceptance never creates business/institutional readiness certification.
7. Evidence truth remains with the owning technical architecture.
8. Material change triggers governed revalidation.
9. No new ADR is required for this authority decision.
10. Organizational mapping of the abstract authority remains policy/configuration, not a new L5 technology decision.

---

## 12. Final Decision

```text
Technical Service Acceptance Authority    ESTABLISHED
L2-A10 Governance Authority               UNCHANGED
E05 acceptance outcomes                   Ready / Ready with conditions / Not ready
Automation may fabricate authorization    NO
New ADR                                   NO
Final L5 Corpus Lock                      NOT AUTHORIZED
```

**Final Decision:** `R4B AUTHORITY APPROVED — TECHNICAL SERVICE ACCEPTANCE IS NOW GOVERNED WITHOUT REDEFINING L2-A10 BUSINESS/GOVERNANCE READINESS.`
