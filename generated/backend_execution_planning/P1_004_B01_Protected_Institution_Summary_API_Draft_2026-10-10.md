# P1-004 — B01 Protected Institution Summary API: REVIEW DRAFT

**Date:** 2026-10-10 | **Status: PROPOSED; NOT RELEASED; NO ENDPOINT EXISTS**

## Purpose
Specify a possible **read-only, web-and-future-mobile-compatible** wire agreement for the first real Institution display. The user's accepted product requirement is to permit **appropriately authorized administrators** to view Suspended and Archived Institution information while their **Tenant is Active**. Neither Administrator persona nor Archived status creates permission on its own.

## Evidence and authority
- L2-B01 Institution Workflow §§2,6,7,12,13: Institution owns its identity, Name, Code and lifecycle; each Institution is under one Tenant; Institution Code is unique within its Tenant.
- L4-B-002 and L4-B-003: Suspended/Archived preserve authorized historical/reference access, while new normal Institution operation is prohibited; Archived is terminal and read-only.
- L2-A01 context resolution §4.13: inactive Tenant fails normal business context resolution. This proposed API MUST NOT bypass that rule. An exceptional inactive-Tenant historical access path is an unresolved A01/D01/D02/A04/D03 governance question.
- L5-C-01-001/002/003, ADR-L5-0034: OpenAPI 3.2, HTTPS REST/JSON, major URL version, camelCase JSON, RFC 9457 Problem Details, released contract versioning.
- L5-D-01, D02, A04: verified session, authorization and Tenant-root context must be enforced server-side.
- L5-G-01-003/004: reject stale browser scope responses, preserve client-neutral contract for future mobile, without presuming identical web/mobile session carriers.

## Candidate operation (illustrative: NOT approved)
- `GET /api/v1/institutions/{institutionId}/summary` — **proposed route and parameter name only**. Final external contract owner may select a different route and selector.
- Request: validated D01 session, validated Tenant root and current scope (mechanism **not yet selected**), D02-approved resource-specific read capability. No browser-selected Tenant default and no authentication token in query strings.
- Success: exactly one authorized Institution's **canonical business identity, code and current lifecycle state**, from a B01-owned authoritative read; no writing, activation, profile revision or historical profile query. Institution Name is deferred until authoritative current-profile resolution is established.
- Candidate camelCase response body (illustrative wire names, **not approved**):

```json
{
  "institutionId": "<canonical B01 ID>",
  "institutionCode": "<code unique within owning Tenant>",
  "lifecycleStatus": "Suspended"
}
```

`institutionId`, `institutionCode`, `lifecycleStatus` and lifecycle JSON spelling are **candidate wire names**; canonical source labels are Institution Business Identity, Institution Code and Institution Lifecycle State. **Institution Name is an existing B01 master-information fact, not an unresolved business requirement**, but remains outside this initial three-field contract to avoid inventing current-profile selection. Exact field typing, optionality, metadata and information classification require B01/C01/D02 approval. The literal example is not executable, valid synthetic identity data or released test fixture.

## Proposed inclusion/exclusion
- **Include**, subject to owner disclosure approval: canonical Institution business identity, code and current lifecycle. Treat source-owned business UUID and technical PostgreSQL identity separately under ADR-L5-0036.
- **Exclude from initial response**: Institution Name until authoritative current-profile selection is proved; Institution administrators' identities, Contact/Address, registration details, policy and authorization data, activation decisions/reasons, past state history, tenant directory, downstream business records, internal SQL keys, sensitive audit or readiness claims.
- Only one selected Institution, **no directory/list API** in this first slice.
- A current record of an Archived Institution is NOT a historical timeline/report; historical reporting needs another governed contract and data retention definition.

## Authorization and failure rules
1. D01 session/principal must be valid; browser uses ADR-0038 server-owned opaque secure cookie. Future mobile carrier separately D01-approved; business API remains client-neutral.
2. A04 resolves one Tenant root and validates B01 Institution association. **Inactive Tenant = reject normal protected request.**
3. D02 evaluates explicit scoped permission for the exact Institution and its current resource state (including Archived/Suspended); UI role label is not authority.
4. Nonexistent/not entitled/wrong-Tenant resource must never be disclosed through body/headers/logs. Whether external status is indistinguishable requires C01/D02/D03 decision.
5. Out-of-date session, unsupported contract version, unavailable B01 source or client scope-change yields bounded technical failure/stale-state suppression, not mock fallback.
6. Authorized Suspended/Archived Institutions: read current approved fields only; no mutation exposed.

### Wire outcome matrix — no HTTP statuses decided
| Scenario | Contract behavior required | Approval still needed |
|---|---|---|
| Valid session + active Tenant + entitled Institution | Return approved current B01 summary | Exact wire body/status/freshness |
| Tenant Suspended or Archived | Reject normal protected read | Exceptional historical service is separate A01 governance |
| Institution Suspended/Archived, Tenant Active | Allow **only if D02 authorizes**; read-only summary | Permission scopes/visibility rules |
| Unauthorized, wrong Institution or cross-Tenant | Fail closed, no leakage | Status mapping/non-enumeration |
| Missing/expired login | Reject, no protected data | D01/C01 mapping |
| Source unavailable | Error, never fixture fallback | C01 technical error mapping |
| Changed Tenant/Institution during fetch | Discard obsolete response client-side | G01 cache/context-epoch tests |

## Contract approval and implementation gates
- [ ] B01 agrees exact identity semantics and disclosure whitelist.
- [ ] D02 specifies *who* can see which Institution and states; A04 validates Tenant association and exception exclusion.
- [ ] C01 approves exact route, OpenAPI 3.2 document, error details, contract version/digest and release lifecycle; review mobile future compatibility.
- [ ] D01 approves secure session/cookie configuration for real deployment and permissions evaluation handoff.
- [ ] Frontend owner verifies real live Administrator screen entry point; existing B01 action component is mock-only and not proven mounted.
- [ ] B02 approves persistence/version/freshness contract; synthetic governed test record plan.
- [ ] User confirms GitHub protection is enforced; no coding agent write or CI launch before review.
- [ ] Create real API and validation tests **only after these approvals and separate execution authorization**.

**Integrity:** This is a **proposal**, not generated OpenAPI 3.2, not an active registry entry, not a permission grant, no code or tests.
