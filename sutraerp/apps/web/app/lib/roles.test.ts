import { describe, expect, it } from "vitest";
import { CATALOG, filterCatalog } from "./api";
import { ROLE_CONFIG, ROLES, routeFor, SCREEN_BATCH } from "./roles";

describe("role configuration", () => {
  it("has unique nav ids including home and workspaces", () => {
    for (const role of ROLES) {
      const ids = ROLE_CONFIG[role].nav.map((n) => n.id);
      expect(new Set(ids).size).toBe(ids.length);
      expect(ids).toContain("home");
      expect(ids).toContain("workspaces");
    }
  });

  it("maps every non-home screen to a delivery batch", () => {
    for (const role of ROLES) {
      for (const n of ROLE_CONFIG[role].nav) {
        if (n.id === "home" || n.id === "workspaces") continue;
        expect(SCREEN_BATCH[n.id], `${role}:${n.id}`).toBeDefined();
      }
    }
  });

  it("builds routes", () => {
    expect(routeFor("home")).toBe("/");
    expect(routeFor("fees")).toBe("/fees");
  });
});

describe("workspace catalog", () => {
  it("matches the prototype totals (99 workspaces / 595 workflows)", () => {
    expect(CATALOG).toHaveLength(99);
    expect(CATALOG.reduce((n, w) => n + w.workflows.length, 0)).toBe(595);
  });

  it("filters by role", () => {
    expect(filterCatalog("administrator").total).toBe(99);
    expect(filterCatalog("faculty").total).toBe(50);
    expect(filterCatalog("student").total).toBe(44);
    expect(filterCatalog("student").items.every((w) => w.workflows.every((f) => f.roles.includes("student")))).toBe(true);
  });
});
