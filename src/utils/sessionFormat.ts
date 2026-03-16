import type { SessionListItem } from "../types";

const fullDateTimeFormatter = new Intl.DateTimeFormat("zh-CN", {
  dateStyle: "medium",
  timeStyle: "short",
});

const countFormatter = new Intl.NumberFormat("en-US");
const compactCountFormatter = new Intl.NumberFormat("en-US", {
  notation: "compact",
});

export function getSessionTitle(
  session: Pick<SessionListItem, "id" | "title">,
): string {
  return session.title.trim() || session.id;
}

export function getSessionSummary(
  session: Pick<SessionListItem, "summary">,
): string {
  return session.summary.trim() || "No summary available.";
}

export function getCompactSessionSummary(
  session: Pick<SessionListItem, "summary">,
  maxLength = 120,
): string {
  const summary = getSessionSummary(session);
  if (summary.length <= maxLength) {
    return summary;
  }

  return `${summary.slice(0, maxLength).trimEnd()}…`;
}

export function formatFullTimestamp(timestamp: number): string {
  return fullDateTimeFormatter.format(timestamp * 1000);
}

export function formatCount(value: number, compact = false): string {
  if (compact && value > 999) {
    return compactCountFormatter.format(value);
  }

  return countFormatter.format(value);
}

export function compactPath(path: string): string {
  const normalized = path.replace(/\\/g, "/");
  const segments = normalized.split("/").filter(Boolean);

  if (segments.length <= 2) {
    return path;
  }

  return `…/${segments.slice(-2).join("/")}`;
}
