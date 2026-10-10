import { ArrowUpRight } from "lucide-react";
import { Logo, ROLE_ACCENTS, RoleTheme, useToast } from "@sutra/ui";
import type { CSSProperties } from "react";
import { useState } from "react";
import { Navigate, useNavigate } from "react-router";
import { AUTH_MODE } from "../lib/api";
import { useAuth } from "../lib/auth";
import { ROLE_CONFIG, ROLES, type Role } from "../lib/roles";

export function meta() {
  return [{ title: "Sign in · SutraERP" }];
}

export default function SignIn() {
  const { ready, session, signIn } = useAuth();
  const navigate = useNavigate();
  const toast = useToast();
  const [busy, setBusy] = useState<Role | null>(null);
  if (ready && session) return <Navigate to="/" replace />;

  const choose = async (role: Role) => {
    setBusy(role);
    try {
      await signIn(role);
      toast(`Signed in as ${ROLE_CONFIG[role].name}`);
      navigate("/", { replace: true });
    } catch (e) {
      toast(e instanceof Error ? e.message : "Sign-in failed");
    } finally {
      setBusy(null);
    }
  };

  return (
    <div className="sb-signin">
      <RoleTheme role={null} />
      <Logo />
      <h1>Sign in to SutraERP</h1>
      <p>Sutra Institute of Technology · development sign-in — choose a role</p>
      <div className="sb-role-cards">
        {ROLES.map((role) => {
          const r = ROLE_CONFIG[role];
          return (
            <button
              key={role}
              type="button"
              className="sb-tile sb-role-card"
              data-role-card={role}
              disabled={busy !== null}
              onClick={() => choose(role)}
              style={{ "--ac": ROLE_ACCENTS[role].ac, "--acs": ROLE_ACCENTS[role].acs, "--act": ROLE_ACCENTS[role].act } as CSSProperties}
            >
              <span className="sb-avatar">{r.initials}</span>
              <small>{r.title}</small>
              <h3>{r.name}</h3>
              <small>{r.blurb}</small>
              <span className="sb-btn">
                {busy === role ? "Signing in…" : `Continue as ${r.title.toLowerCase()}`} <ArrowUpRight strokeWidth={1.75} />
              </span>
            </button>
          );
        })}
      </div>
      <div className="sb-env-tag">
        <i />
        {AUTH_MODE === "api" ? "Dev authenticator · API" : "Dev personas · mock data"}
      </div>
    </div>
  );
}
