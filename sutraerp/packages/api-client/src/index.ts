/**
 * Typed SutraERP API client.
 *
 * `schema.d.ts` is generated from `openapi/openapi.json` (which is itself generated from the Rust
 * code by utoipa) — run `pnpm gen:api` after changing any API. CI fails if either drifts.
 */
import createClient, { type Middleware } from "openapi-fetch";
import type { components, paths } from "./schema";

export type { components, paths } from "./schema";

type Schemas = components["schemas"];
export type Principal = Schemas["Principal"];
export type Role = Schemas["Role"];
export type Workspace = Schemas["Workspace"];
export type WorkflowEntry = Schemas["WorkflowEntry"];
export type WorkspaceList = Schemas["WorkspaceList"];
export type LoginRequest = Schemas["LoginRequest"];
export type LoginResponse = Schemas["LoginResponse"];
export type SessionInfo = Schemas["SessionInfo"];
export type ModuleDescriptor = Schemas["ModuleDescriptor"];
export type Problem = Schemas["Problem"];

export interface ClientOptions {
  /** Origin of the API; empty string = same origin (default, via dev proxy / ingress). */
  baseUrl?: string;
  /** Tenant/Institution scope headers (dev header mode, ADR-L5-0004). */
  scope?: () => { tenantId?: string; institutionId?: string } | undefined;
}

/** Error thrown for RFC 9457 problem responses. */
export class ApiProblem extends Error {
  constructor(public readonly problem: Problem) {
    super(problem.detail ?? problem.title);
    this.name = "ApiProblem";
  }
}

export function isProblem(value: unknown): value is Problem {
  return typeof value === "object" && value !== null && "errorCode" in value && "status" in value;
}

/** New UUIDv4 for `Idempotency-Key` headers on retry-sensitive commands. */
export function newIdempotencyKey(): string {
  return crypto.randomUUID();
}

export function createSutraClient(options: ClientOptions = {}) {
  const client = createClient<paths>({ baseUrl: options.baseUrl ?? "", credentials: "include" });
  const scopeMiddleware: Middleware = {
    onRequest({ request }) {
      const scope = options.scope?.();
      if (scope?.tenantId) request.headers.set("X-Sutra-Tenant-Id", scope.tenantId);
      if (scope?.institutionId) request.headers.set("X-Sutra-Institution-Id", scope.institutionId);
      return request;
    },
  };
  client.use(scopeMiddleware);
  return client;
}

export type SutraClient = ReturnType<typeof createSutraClient>;
