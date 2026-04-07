export const colors = {
  // Primary gradient endpoints (from website)
  blue600: "#2563EB",
  purple600: "#9333EA",

  // Severity palette
  critical: "#DC2626",
  high: "#EA580C",
  medium: "#D97706",
  low: "#2563EB",
  info: "#6B7280",

  // Neutral
  slate50: "#F8FAFC",
  slate100: "#F1F5F9",
  slate200: "#E2E8F0",
  slate300: "#CBD5E1",
  slate400: "#94A3B8",
  slate500: "#64748B",
  slate600: "#475569",
  slate700: "#334155",
  slate800: "#1E293B",
  slate900: "#0F172A",
  slate950: "#020617",
  white: "#FFFFFF",
} as const;

export function severityColor(level: string): string {
  switch (level) {
    case "error":
      return colors.critical;
    case "warning":
      return colors.high;
    case "note":
      return colors.medium;
    default:
      return colors.info;
  }
}

export function severityLabel(level: string): string {
  switch (level) {
    case "error":
      return "CRITICAL / HIGH";
    case "warning":
      return "MEDIUM";
    case "note":
      return "LOW";
    default:
      return "INFO";
  }
}

export function confidenceToSeverity(confidence?: number): string {
  if (confidence === undefined) return "info";
  const score = confidence <= 1 ? confidence * 100 : confidence;
  if (score >= 90) return "critical";
  if (score >= 70) return "high";
  if (score >= 50) return "medium";
  if (score >= 30) return "low";
  return "info";
}

export function confidenceColor(confidence?: number): string {
  const severity = confidenceToSeverity(confidence);
  return colors[severity as keyof typeof colors] || colors.info;
}
