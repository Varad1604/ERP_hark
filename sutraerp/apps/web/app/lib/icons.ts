import {
  Award,
  BookOpen,
  Building2,
  CalendarCheck,
  CalendarDays,
  ChartBar,
  ClipboardCheck,
  Contact,
  CreditCard,
  FilePlus,
  FileText,
  GraduationCap,
  House,
  Inbox,
  Landmark,
  LayoutGrid,
  Library,
  LifeBuoy,
  Megaphone,
  PencilLine,
  Plane,
  Presentation,
  Server,
  Table2,
  Upload,
  UserCheck,
  UserPlus,
  Users,
  Wallet,
  type LucideIcon,
} from "lucide-react";
import type { Tone } from "@sutra/ui";

export const Icons = {
  Award, BookOpen, Building2, CalendarCheck, CalendarDays, ChartBar, ClipboardCheck, Contact, CreditCard,
  FilePlus, FileText, GraduationCap, House, Inbox, Landmark, LayoutGrid, Library, LifeBuoy, Megaphone,
  PencilLine, Plane, Presentation, Server, Table2, Upload, UserCheck, UserPlus, Users, Wallet,
} satisfies Record<string, LucideIcon>;

/** Catalog navigation groups → icon, tint, label (from the prototype). */
export const GROUPS: Record<string, { icon: LucideIcon; tone: Tone; label: string }> = {
  platform: { icon: Server, tone: "grey", label: "Platform" },
  institution: { icon: Landmark, tone: "amber", label: "Institution" },
  academics: { icon: BookOpen, tone: "blue", label: "Academics" },
  students: { icon: GraduationCap, tone: "orange", label: "Students" },
  examination: { icon: ClipboardCheck, tone: "violet", label: "Examination" },
  finance: { icon: Wallet, tone: "green", label: "Finance" },
  people: { icon: Users, tone: "rose", label: "People" },
  campus: { icon: Building2, tone: "teal", label: "Campus" },
  resources: { icon: Library, tone: "amber", label: "Resources" },
  services: { icon: LifeBuoy, tone: "blue", label: "Services" },
  intelligence: { icon: ChartBar, tone: "violet", label: "Intelligence" },
};

export function groupMeta(group: string) {
  return GROUPS[group] ?? { icon: LayoutGrid, tone: "grey" as Tone, label: group };
}
