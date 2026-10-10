import { useEffect } from "react";

export type ThemeRole = "administrator" | "faculty" | "student";

/** Role accents: administrator orange, faculty blue, student violet. */
export const ROLE_ACCENTS: Record<ThemeRole, { ac: string; acs: string; act: string }> = {
  administrator: { ac: "#E8743B", acs: "#FFE1CC", act: "#A84A18" },
  faculty: { ac: "#3B7BE8", acs: "#DCEBFF", act: "#2155B0" },
  student: { ac: "#7254E6", acs: "#E4DCFF", act: "#5434C4" },
};

/** Applies the role accent by setting `data-role` on <html>; `null` = neutral (sign-in). */
export function useRoleTheme(role: ThemeRole | null) {
  useEffect(() => {
    const el = document.documentElement;
    if (role) el.dataset.role = role;
    else delete el.dataset.role;
  }, [role]);
}

export function RoleTheme({ role }: { role: ThemeRole | null }) {
  useRoleTheme(role);
  return null;
}
