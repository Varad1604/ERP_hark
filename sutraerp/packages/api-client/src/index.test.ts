import { describe, expect, it } from "vitest";
import { isProblem, newIdempotencyKey } from "./index";

describe("api-client helpers", () => {
  it("detects problem documents", () => {
    expect(isProblem({ type: "urn:sutraerp:problem:x", title: "x", status: 400, errorCode: "x" })).toBe(true);
    expect(isProblem({ items: [] })).toBe(false);
  });
  it("creates unique idempotency keys", () => {
    expect(newIdempotencyKey()).not.toEqual(newIdempotencyKey());
  });
});
