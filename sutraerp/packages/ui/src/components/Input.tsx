import { Search } from "lucide-react";
import { useId, type InputHTMLAttributes, type SelectHTMLAttributes } from "react";
import { cx } from "./cx";

interface FieldProps {
  label: string;
  error?: string;
}

export function Input({ label, error, className, ...rest }: FieldProps & InputHTMLAttributes<HTMLInputElement>) {
  const id = useId();
  return (
    <label className="sb-field" htmlFor={id}>
      {label}
      <input id={id} className={cx("sb-input", className)} aria-invalid={error ? true : undefined} {...rest} />
      {error && <span className="sb-field-error">{error}</span>}
    </label>
  );
}

export interface SelectOption {
  value: string;
  label: string;
}

export function Select({ label, error, options, className, ...rest }: FieldProps & { options: SelectOption[] } & SelectHTMLAttributes<HTMLSelectElement>) {
  const id = useId();
  return (
    <label className="sb-field" htmlFor={id}>
      {label}
      <select id={id} className={cx("sb-select", className)} aria-invalid={error ? true : undefined} {...rest}>
        {options.map((o) => (
          <option key={o.value} value={o.value}>
            {o.label}
          </option>
        ))}
      </select>
      {error && <span className="sb-field-error">{error}</span>}
    </label>
  );
}

/** Pill search field used in page headers. */
export function SearchField({ label, ...rest }: { label: string } & InputHTMLAttributes<HTMLInputElement>) {
  return (
    <label className="sb-search">
      <Search strokeWidth={1.75} aria-hidden="true" />
      <input type="search" aria-label={label} {...rest} />
    </label>
  );
}
