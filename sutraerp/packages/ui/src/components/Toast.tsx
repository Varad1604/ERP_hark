import { CircleCheck } from "lucide-react";
import { createContext, useCallback, useContext, useMemo, useState, type ReactNode } from "react";

interface ToastItem {
  id: number;
  message: string;
}

const ToastContext = createContext<((message: string) => void) | null>(null);

export function ToastProvider({ children, duration = 2600 }: { children: ReactNode; duration?: number }) {
  const [items, setItems] = useState<ToastItem[]>([]);
  const push = useCallback(
    (message: string) => {
      const id = Date.now() + Math.random();
      setItems((xs) => [...xs, { id, message }]);
      setTimeout(() => setItems((xs) => xs.filter((x) => x.id !== id)), duration);
    },
    [duration],
  );
  const value = useMemo(() => push, [push]);
  return (
    <ToastContext.Provider value={value}>
      {children}
      <div className="sb-toasts" aria-live="polite">
        {items.map((t) => (
          <div key={t.id} className="sb-toast" role="status">
            <CircleCheck strokeWidth={1.75} aria-hidden="true" />
            <span>{t.message}</span>
          </div>
        ))}
      </div>
    </ToastContext.Provider>
  );
}

/** Returns `toast(message)`. */
export function useToast(): (message: string) => void {
  const ctx = useContext(ToastContext);
  if (!ctx) throw new Error("useToast must be used inside <ToastProvider>");
  return ctx;
}
