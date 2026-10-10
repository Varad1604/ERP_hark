import type { ButtonHTMLAttributes, ReactNode } from "react";
import { cx } from "./cx";

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "primary" | "ghost" | "accent";
  size?: "md" | "sm";
  icon?: ReactNode;
  iconPosition?: "start" | "end";
}

export function Button({ variant = "primary", size = "md", icon, iconPosition = "end", className, children, type = "button", ...rest }: ButtonProps) {
  return (
    <button type={type} className={cx("sb-btn", variant !== "primary" && `sb-btn--${variant}`, size === "sm" && "sb-btn--sm", className)} {...rest}>
      {iconPosition === "start" && icon}
      {children}
      {iconPosition === "end" && icon}
    </button>
  );
}
