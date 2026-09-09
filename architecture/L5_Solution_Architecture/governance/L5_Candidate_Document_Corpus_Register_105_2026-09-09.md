# SutraERP L5 — Candidate Document Corpus Register — 105 Specifications

**Version:** 1.1.0  
**Status:** Architecture Approved  
**Owner:** Lead Architect  
**Architectural Level:** L5 — Solution Architecture Governance  
**Domain:** Cross-domain  
**Last Updated:** 2026-09-09  
**Depends On:** L5 Major Architecture Areas Baseline; L5 Architecture Family Register; L5 Naming Specialization Approval Decision; L5 L4-to-L5 Coverage Audit Decision 2026-09-09  
**Supersedes For Active Candidate-Corpus Planning:** `L5_Candidate_Document_Corpus_Register_99_2026-09-09.md`  

---

# 1. Decision and Purpose

This register defines the approved revised SutraERP L5 Solution Architecture candidate corpus after the repository-wide L4-to-L5 coverage, duplicate-ownership, completeness and granularity audit.

The revised candidate corpus contains **105 reusable L5 Solution Architecture specifications**.

The earlier 99-specification register remains repository history and shall not be deleted. It is superseded only for active candidate-corpus planning.

This register approves the 105-specification candidate population and identifiers for controlled L5 methodology/governance preparation. It does **not** authorize mass L5 specification authoring, technology selection, implementation, L6 authoring or final L5 corpus lock.

L5 remains many-to-many with L4:

```text
Many L4 behavioral obligations
        ↓
Reusable L5 architecture specifications
        ↑
One L4 behavior may consume multiple L5 architectures
```

L5 shall not redefine upstream business ownership, business rules, lifecycle states, canonical events, publishers, institutional authority or L3/L4 outcomes.

---

# 2. Approved Candidate Corpus Summary

| Family | Candidate Documents |
|---|---:|
| L5-A — Core Platform & Application | 20 |
| L5-B — Data & Information | 21 |
| L5-C — Contracts, Events & Integration | 18 |
| L5-D — Identity, Security & Trust | 19 |
| L5-E — Reliability, Deployment & Operations | 16 |
| L5-F — Engineering Delivery & Quality | 7 |
| L5-G — Client Platform | 4 |
| **Total** | **105** |

Change from the prior 99-register:

| Coverage refinement | Net change |
|---|---:|
| Document Platform refinement: B04 2 → 5 | +3 |
| Consent / Purpose-Limitation technical enforcement | +1 |
| Governed Bulk Data Import / Export | +1 |
| Device / Edge / Physical-System Integration | +1 |
| **Net** | **+6** |

---

# 3. Candidate Document Register

## L5-A — Core Platform & Application Architecture

| ID | Candidate Specification | Primary Responsibility / Boundary |
|---|---|---|
| L5-A-01-001 | Application & Platform Architecture | Overall solution/application structure and platform responsibility boundaries without redefining business domains. |
| L5-A-02-001 | Technology Platform & Standards Architecture | Governed technology-selection criteria and approved standards after ADR decisions; no silent framework/product lock. |
| L5-A-03-001 | Module, Dependency & Technical Boundary Architecture | Technical module boundaries aligned to canonical architecture ownership. |
| L5-A-03-002 | Module Composition & Dependency Enforcement Architecture | Enforceable technical dependency/composition rules and prevention of unauthorized coupling. |
| L5-A-04-001 | Tenant & Institutional Isolation Architecture | Technical Tenant/Institution isolation while preserving the locked business model. |
| L5-A-04-002 | Tenant and Institution Context Propagation & Data Isolation Architecture | Propagation/enforcement of Tenant, Institution and lower-scope context across execution and persistence. |
| L5-A-05-001 | Workflow & Orchestration Architecture Baseline | Reusable workflow/orchestration capability; business domains retain workflow rules and decisions. |
| L5-A-05-002 | Workflow Definition, Versioning & Policy Resolution Architecture | Technical workflow definition/versioning, scope resolution and effective policy selection. |
| L5-A-05-003 | Workflow Runtime, Task, Timer, Escalation & Compensation Architecture | Runtime task/timer/escalation/technical compensation mechanics without inventing business recovery policy. |
| L5-A-06-001 | Configuration & Institutional Extensibility Architecture Baseline | Governed configuration and extension mechanisms while upstream domains own business policy. |
| L5-A-06-002 | Configuration Scope, Inheritance & Effective-Dating Architecture | Technical scope hierarchy, inheritance, overrides and effective dating. |
| L5-A-06-003 | Feature Activation & Controlled Extension Architecture | Safe capability activation/deactivation and controlled extension boundaries. |
| L5-A-07-001 | Secure Computational Engine Architecture Baseline | Common trust, determinism, rule-package isolation and evidence requirements; engines do not own business rules. |
| L5-A-07-002 | Finance Computation Engine Architecture | Executes approved/versioned financial calculations; cannot autonomously approve/post canonical financial transactions. |
| L5-A-07-003 | Examination Result Computation Engine Architecture | Executes approved result formulas/regulations; cannot approve or publish official results autonomously. |
| L5-A-07-004 | Payroll Computation Engine Architecture | Executes approved payroll calculations while Payroll retains business ownership and Finance owns downstream monetary consequences. |
| L5-A-07-005 | General Rules & Formula Engine Architecture | Constrained declarative formula/rule execution; no arbitrary code, SQL, filesystem or unrestricted network execution. |
| L5-A-07-006 | Scheduling & Optimization Engine Architecture | Reusable scheduling/conflict/optimization computation without owning timetable/examination/resource business decisions. |
| L5-A-08-001 | Background Processing & Job Execution Architecture | Trusted background/asynchronous execution, job identity and execution governance. |
| L5-A-08-002 | Bulk, Scheduled & Retryable Work Execution Architecture | Long-running, scheduled and technically retryable execution without owning business retry/recovery policy. |

**L5-A Total: 20**

---

## L5-B — Data & Information Architecture

| ID | Candidate Specification | Primary Responsibility / Boundary |
|---|---|---|
| L5-B-01-001 | Enterprise Search & Indexing Architecture | Enterprise technical search/indexing over authorized canonical sources; search never owns indexed facts. |
| L5-B-01-002 | Search Index Lifecycle, Access Filtering & Synchronization Architecture | Index lifecycle/freshness/synchronization and authorization-aware filtering. |
| L5-B-02-001 | Database & Persistence Architecture Baseline | Physical persistence architecture and principles without changing domain ownership. |
| L5-B-02-002 | Domain Persistence Ownership & Isolation Architecture | Enforces technical persistence ownership and prevents unauthorized cross-domain writes. |
| L5-B-02-003 | Transaction, Consistency & Concurrency Architecture | Transaction boundaries, concurrency controls and consistency models preserving domain/event boundaries. |
| L5-B-02-004 | Schema Evolution & Data Migration Architecture | Controlled persistence-schema evolution/internal migration; excludes operational governed business import/export. |
| L5-B-03-001 | Master & Reference Data Technical Architecture | Technical representation/distribution of governed Master/Reference Data while L2-A09 retains business governance. |
| L5-B-03-002 | Master Data Versioning, Validation, Publication & Distribution Architecture | Technical versioning, validation execution, publication and distribution of approved datasets. |
| L5-B-04-001 | Document, Media & Object Platform Architecture Baseline | Common technical document/media/object platform spanning registry, ingestion, storage, generation, verification and credential integrity; no business acceptance/issuance ownership transfer. |
| L5-B-04-002 | Document Registry, Versioning, Storage & Retention Architecture | Technical registry, object identity, versions, storage lifecycle, metadata and retention execution. |
| L5-B-04-003 | Document Ingestion, Validation & Verification Architecture | Technical ingestion/file validation/evidence/verification mechanisms; owning domains retain semantic acceptance authority. |
| L5-B-04-004 | Document Generation, Rendering, Issuance & Template Architecture | Reusable document generation/rendering/template/issuance infrastructure while owning domains retain eligibility/approval decisions. |
| L5-B-04-005 | Digital Signing, Seal, QR & Credential Integrity Architecture | Application of signing/seal/QR/barcode credential-integrity mechanisms; cryptographic primitives and keys remain L5-D-03 owned. |
| L5-B-05-001 | Data Classification, Retention & Privacy Architecture Baseline | Technical privacy/data-classification control model derived from upstream policy and ownership. |
| L5-B-05-002 | Sensitive Data Handling & Masking Architecture | Technical masking, controlled display/export and handling of classified sensitive information. |
| L5-B-05-003 | Retention, Archival, Deletion & Legal Hold Architecture | Technical enforcement of retention/archival/governed deletion/legal holds without redefining business policy. |
| L5-B-05-004 | Aadhaar Data Protection Architecture | Specialized Aadhaar storage/access/masking/audit protection; no invented AUA/KUA status. |
| L5-B-05-005 | Consent, Purpose-Limitation & Privacy-Control Architecture | Technical enforcement of authoritative consent state/withdrawal, approved purpose, minimization and disclosure constraints; never canonical consent owner. |
| L5-B-06-001 | Reporting, Analytics & Data Projection Architecture Baseline | Governed reporting/analytics/projection architecture; source domains remain owners of operational facts. |
| L5-B-06-002 | Operational Read Model & Projection Architecture | Operational read models/projections and freshness/ownership boundaries. |
| L5-B-06-003 | Analytical, Dashboard & Statutory Reporting Data Architecture | Analytical/statutory datasets and dashboard projections without transferring KPI/fact ownership. |

**L5-B Total: 21**

---

## L5-C — Contracts, Events & Integration Architecture

| ID | Candidate Specification | Primary Responsibility / Boundary |
|---|---|---|
| L5-C-01-001 | API & Contract Architecture Baseline | Technical API/contract principles and ownership-preserving synchronous boundaries. |
| L5-C-01-002 | Synchronous Service Contract Architecture | Authoritative synchronous interaction, request/response and service-consumption architecture. |
| L5-C-01-003 | Contract Versioning, Compatibility, Error & Idempotency Architecture | Contract evolution, compatibility, technical errors and idempotent request handling without inventing business recovery. |
| L5-C-02-001 | Event & Messaging Architecture Baseline | Event/messaging infrastructure while canonical business-event ownership remains upstream. |
| L5-C-02-002 | Durable Event Delivery, Ordering & Failure Handling Architecture | Durable delivery, technical retries, ordering, failure/dead-letter treatment and transactional publication. |
| L5-C-02-003 | Event Schema, Versioning & Consumer Compatibility Architecture | Technical event schema evolution/compatibility while event names/publishers remain upstream-owned. |
| L5-C-03-001 | External Integration & Connector Architecture Baseline | Common external integration/connector boundary and ownership-preserving principles. |
| L5-C-03-002 | Connector Lifecycle & Failure Isolation Architecture | Connector configuration/lifecycle, credentials, health and failure isolation. |
| L5-C-03-003 | Payment Gateway Integration Architecture | Multi-gateway payment integration while Finance retains canonical financial/payment state. |
| L5-C-03-004 | APAAR, DigiLocker & Academic Credit Exchange Integration Architecture | Technical education identity/credential/credit exchange integration without owning institutional academic facts. |
| L5-C-03-005 | Government & Statutory Portal Integration Architecture | Governed exchange with government/statutory portals without owning source-domain facts. |
| L5-C-03-006 | External Education & University Integration Architecture | LMS/university/external-education interoperability and credential/academic handoffs. |
| L5-C-03-007 | Banking, Treasury & Financial Network Integration Architecture | Banking/treasury/network integration while Finance retains business ownership. |
| L5-C-03-008 | Bulk Data Import, Export & File Exchange Architecture | Governed bulk/file exchange: controlled upload, structural validation, staging, batches, export packaging, large transfer and delivery integrity; no imported/exported business-data ownership. |
| L5-C-03-009 | Device, Edge, Telemetry & Physical-System Integration Architecture | Reusable secure integration for scanners/readers, RFID/NFC, biometrics, ANPR, GPS, telemetry, IoT, CCTV/evidence feeds and controlled actuators; devices do not own business decisions. |
| L5-C-04-001 | Notification & Communication Architecture Baseline | Enterprise notification/communication capability; triggering business facts remain source-domain owned. |
| L5-C-04-002 | Template, Preference & Communication Policy Execution Architecture | Technical template/preference resolution and approved communication-policy execution. |
| L5-C-04-003 | Multi-Channel Delivery, Provider Abstraction & Delivery Evidence Architecture | Multi-channel provider abstraction, controlled failover and delivery evidence. |

**L5-C Total: 18**

---

## L5-D — Identity, Security & Trust Architecture

| ID | Candidate Specification | Primary Responsibility / Boundary |
|---|---|---|
| L5-D-01-001 | IAM Architecture Baseline | Technical identity/authentication/account architecture; canonical Person/business relationships remain upstream-owned. |
| L5-D-01-002 | Identity Account & Service Identity Architecture | User-account, service/workload/job/integration identity technical models without duplicating Person. |
| L5-D-01-003 | Authentication, Session & Credential Architecture | Authentication/session/credential mechanisms separate from authorization policy. |
| L5-D-01-004 | Identity Provisioning, Recovery & Lifecycle Architecture | Account provisioning/recovery/disablement synchronized with authoritative identity facts. |
| L5-D-02-001 | Authorization Architecture Baseline | Authorization architecture separate from IAM, including policy decision/enforcement and least privilege. |
| L5-D-02-002 | Permission & Capability Model Architecture | Technical permission/capability representation without encoding business ownership into generic roles. |
| L5-D-02-003 | Hierarchical Scope & Resource Context Architecture | Tenant, Institution, organisational-unit and resource-context authorization scope. |
| L5-D-02-004 | Policy Decision & Enforcement Architecture | Technical policy evaluation/enforcement; business/institutional policy ownership remains upstream. |
| L5-D-02-005 | Delegation & Temporary Authority Architecture | Technical delegated/time-bounded authority derived from approved business delegation. |
| L5-D-02-006 | Privileged & Administrative Access Architecture | High-risk privileged/administrative access safeguards. |
| L5-D-02-007 | Separation-of-Duties, Maker-Checker & Emergency Override Architecture | Technical enforcement of upstream-defined SoD/maker-checker/emergency override; no invented approver. |
| L5-D-03-001 | Security Architecture & Trust Boundary Baseline | Cross-solution threat/trust-boundary architecture and security-control principles. |
| L5-D-03-002 | Cryptography & Data Protection Architecture | Cryptographic primitives, encryption and approved data-protection patterns. |
| L5-D-03-003 | Key Management Architecture | Key generation, custody, rotation, scope, recovery and separation requirements. |
| L5-D-03-004 | Secrets, Certificates & Machine Credential Architecture | Application/integration secrets, certificates and machine credentials. |
| L5-D-03-005 | Application, Service & Network Trust Segmentation Architecture | Logical trust isolation among services/engines and restricted connectivity without requiring physical server separation. |
| L5-D-04-001 | Audit & Evidence Architecture Baseline | Shared audit/evidence capability while source domains retain business-evidence meaning. |
| L5-D-04-002 | Audit Event, Provenance & Evidence Model Architecture | Technical provenance/evidence model, actor attribution, causality and correlation. |
| L5-D-04-003 | Tamper-Evident Audit Storage, Access & Export Architecture | Integrity-protected audit storage/access/export without becoming owner of business records. |

**L5-D Total: 19**

---

## L5-E — Reliability, Deployment & Operations Architecture

| ID | Candidate Specification | Primary Responsibility / Boundary |
|---|---|---|
| L5-E-01-001 | Availability & Resilience Architecture | Availability, fault-tolerance and resilience principles independent of product choice. |
| L5-E-01-002 | Failure Isolation, Graceful Degradation & Service Recovery Architecture | Technical fault containment/degradation/service recovery without changing business recovery semantics. |
| L5-E-02-001 | Performance & Capacity Architecture | Performance quality attributes, capacity dimensions and architecture principles. |
| L5-E-02-002 | Caching, Throughput & Capacity Management Architecture | Caching/throughput/capacity mechanisms without making caches authoritative business truth. |
| L5-E-03-001 | Backup, Restore & Disaster Recovery Architecture Baseline | Recovery objectives and common backup/restore/DR architecture for supported deployment models. |
| L5-E-03-002 | Backup, Restore & Point-in-Time Recovery Architecture | Backup sets, restoration and point-in-time recovery across governed data stores. |
| L5-E-03-003 | Disaster Recovery, Failover & Recovery Validation Architecture | DR/failover models and proof of recoverability without assuming a cloud provider. |
| L5-E-04-001 | Runtime, Packaging & Deployment Architecture Baseline | Technology-neutral runtime/deployment supporting on-premise primary and cloud-capable deployment. |
| L5-E-04-002 | Application Packaging & Runtime Isolation Architecture | Package/runtime isolation and service boundary realization without mandating Kubernetes. |
| L5-E-04-003 | On-Premises Deployment & Installation Architecture | Deployable on-prem installation/configuration/operations model. |
| L5-E-04-004 | Cloud-Capable Deployment & Environment Architecture | Provider-neutral cloud-capable environment model without hyperscale assumptions. |
| L5-E-04-005 | Release Installation, Upgrade & Rollback Architecture | Controlled installation, upgrades and technical rollback across supported deployment models. |
| L5-E-05-001 | Observability & Operations Architecture Baseline | Operational observability/service operations without treating telemetry as business audit truth. |
| L5-E-05-002 | Logging, Metrics & Distributed Tracing Architecture | Technical logs/metrics/traces with privacy and sensitive-data boundaries. |
| L5-E-05-003 | Health, Alerting, Operational Diagnostics & Service Management Architecture | Health signals, alerting, diagnostics and service-operations integration. |
| L5-E-05-004 | Operational Readiness Technical Evidence & Service Acceptance Architecture | Technical readiness/service acceptance evidence; does not replace L1 dynamic business readiness or L2-A10 governance certification. |

**L5-E Total: 16**

---

## L5-F — Engineering Delivery & Quality Architecture

| ID | Candidate Specification | Primary Responsibility / Boundary |
|---|---|---|
| L5-F-01-001 | CI/CD & Software Supply Chain Security Architecture Baseline | Engineering delivery/pipeline trust/artifact-promotion principles; branch workflow governance remains separate. |
| L5-F-01-002 | Build, Artifact, Promotion & Release Pipeline Architecture | Controlled build, artifact creation, environment promotion and release gates. |
| L5-F-01-003 | Dependency Provenance, SBOM, Signing & Supply-Chain Control Architecture | Dependency provenance, SBOM, artifact signing and software-supply-chain assurance. |
| L5-F-02-001 | Verification & Testing Architecture Baseline | Solution-wide verification strategy and quality gates traceable to L4 behavior. |
| L5-F-02-002 | Component, Unit & Domain Verification Architecture | Lower-scope verification of technical components/domain implementations against architecture contracts. |
| L5-F-02-003 | Integration, Contract & End-to-End Verification Architecture | Contract/event/connector/end-to-end verification across technical boundaries. |
| L5-F-02-004 | Security, Performance, Resilience & Recovery Testing Architecture | Non-functional verification for security, capacity, failure isolation and disaster recovery. |

**L5-F Total: 7**

---

## L5-G — Client Platform Architecture

| ID | Candidate Specification | Primary Responsibility / Boundary |
|---|---|---|
| L5-G-01-001 | Client Platform Architecture Baseline | Shared technical client-platform principles; excludes L6 screens/journeys/navigation/layout. |
| L5-G-01-002 | Web Application Architecture | Web-client technical architecture for the immediate product direction without UI/UX design. |
| L5-G-01-003 | Client Security, Session, State & Contract Consumption Architecture | Client security/session/state/safe contract consumption without becoming authorization authority. |
| L5-G-01-004 | Future Mobile Application Architecture | Future mobile technical architecture aligned to shared backend contracts; no offline-first assumption or L6 design. |

**L5-G Total: 4**

---

# 4. Confirmed Coverage Remediation

## 4.1 Document Platform

The prior B04 pair was insufficient because upstream architecture requires reusable technical capability for document registry/versioning/storage, ingestion/validation/verification, generation/rendering/issuance and credential integrity.

Business domains retain business acceptance, eligibility, approval and issuance authority. L5-B-04 owns only reusable technical document-platform capability. L5-D-03 retains ownership of cryptographic primitives, keys, certificates and security controls.

## 4.2 Governed Bulk Import / Export

`L5-C-03-008` owns the reusable technical exchange mechanism for controlled operational import/export and file exchange. Source/destination business domains retain ownership of business data and semantic validation policy.

## 4.3 Device / Edge / Physical-System Integration

`L5-C-03-009` owns reusable device/edge integration architecture. Devices provide observations or execute controlled technical commands; they do not own Identity, asset lifecycle, access authorization, trip state, examination decision or other business facts.

## 4.4 Consent / Purpose Limitation

`L5-B-05-005` technically enforces authoritative consent/purpose facts. It does not create a new canonical Consent business owner.

---

# 5. Corpus Integrity Rules

1. The corpus is not one L5 document per L4 document.
2. One reusable L5 specification may realize many L4 behavioral obligations.
3. One L4 behavior may depend on multiple L5 specifications.
4. Every technical concern has one primary L5 owner.
5. Cross-family dependencies do not create duplicate ownership.
6. Business domains retain all upstream business ownership, business rules, lifecycle semantics, canonical event ownership and institutional authority.
7. L5 does not create UI/UX design; that belongs to L6.
8. L5 technology/product choices require the approved technology-selection/ADR process.
9. On-premises support is mandatory and primary/preferred; cloud architecture remains provider-neutral.
10. Kubernetes is not a baseline assumption.
11. The historical implementation repository/code has no authority over greenfield technology decisions.

---

# 6. Items Explicitly Not Added as Separate Candidate Specifications

The coverage audit does not currently justify standalone L5 specifications for:

- LMS as a separate major architecture concern;
- Webhooks;
- Localization;
- ETL separate from Reporting/Analytics/Data Projection;
- Identifier/Sequence Generation as a centralized business owner;
- Enterprise Calendar as a new technical business owner;
- Recruitment-specific integration;
- individual device/vendor/product types;
- individual payment gateways;
- Aadhaar as a peer family;
- AI-agent Git/branch/PR permissions.

These concerns remain covered by existing architecture responsibilities or belong to separate engineering/repository governance unless later authoritative evidence establishes a distinct responsibility.

---

# 7. Current Governance Position

**Candidate L5 specifications:** **105**  
**Candidate IDs:** **Approved for the revised candidate corpus**  
**Mass L5 authoring:** **NOT AUTHORIZED**  
**Final L5 corpus lock:** **NOT AUTHORIZED**  
**Technology stack selection:** **NOT AUTHORIZED**  
**L6 authoring:** **NOT AUTHORIZED**

The recommended next phase is to complete/lock the L5 Governance & Architecture Standard, L5 Solution Architecture Specification Template, L5 Validation Standard, technology-selection/ADR methodology, and controlled representative pilot before any broad L5 authoring allocation.
