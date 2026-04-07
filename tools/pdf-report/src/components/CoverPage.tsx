import React from "react";
import { Page, View, Text, StyleSheet, Svg, Defs, Stop, Rect, LinearGradient, Circle } from "@react-pdf/renderer";
import { colors } from "../theme.js";
import type { ReportData } from "../types.js";

const s = StyleSheet.create({
  page: {
    backgroundColor: colors.slate950,
    position: "relative",
  },
  // Decorative grid dots overlay
  gridOverlay: {
    position: "absolute",
    top: 0,
    left: 0,
    width: "100%",
    height: "100%",
  },
  content: {
    flex: 1,
    justifyContent: "center",
    alignItems: "center",
    padding: 60,
  },
  badge: {
    backgroundColor: "rgba(255,255,255,0.08)",
    borderRadius: 20,
    paddingHorizontal: 16,
    paddingVertical: 6,
    marginBottom: 32,
  },
  badgeText: {
    fontSize: 11,
    color: colors.slate300,
    letterSpacing: 2,
  },
  badgeAccent: {
    color: colors.blue600,
    fontFamily: "Helvetica-Bold",
  },
  title: {
    fontSize: 52,
    fontFamily: "Helvetica-Bold",
    color: colors.white,
    marginBottom: 8,
    letterSpacing: -1,
  },
  subtitle: {
    fontSize: 28,
    fontFamily: "Helvetica-Bold",
    color: colors.blue600,
    marginBottom: 40,
  },
  divider: {
    width: 80,
    height: 3,
    marginBottom: 40,
  },
  metaRow: {
    flexDirection: "row",
    marginBottom: 8,
  },
  metaLabel: {
    fontSize: 10,
    color: colors.slate400,
    width: 100,
    textTransform: "uppercase",
    letterSpacing: 1.5,
  },
  metaValue: {
    fontSize: 10,
    color: colors.slate200,
    fontFamily: "Helvetica-Bold",
  },
  statsRow: {
    flexDirection: "row",
    gap: 16,
    marginTop: 48,
  },
  statBox: {
    alignItems: "center",
    paddingHorizontal: 20,
    paddingVertical: 14,
    borderRadius: 8,
    backgroundColor: "rgba(255,255,255,0.05)",
    minWidth: 80,
  },
  statNumber: {
    fontSize: 28,
    fontFamily: "Helvetica-Bold",
    color: colors.white,
  },
  statLabel: {
    fontSize: 8,
    color: colors.slate400,
    textTransform: "uppercase",
    letterSpacing: 1,
    marginTop: 4,
  },
  footer: {
    position: "absolute",
    bottom: 30,
    left: 0,
    right: 0,
    alignItems: "center",
  },
  footerText: {
    fontSize: 8,
    color: colors.slate500,
    letterSpacing: 1,
  },
});

export function CoverPage({ data }: { data: ReportData }) {
  const statItems = [
    { n: data.stats.total, label: "Findings", color: colors.white },
    { n: data.stats.critical, label: "Critical", color: colors.critical },
    { n: data.stats.high, label: "High", color: colors.high },
    { n: data.stats.medium, label: "Medium", color: colors.medium },
    { n: data.stats.low + data.stats.info, label: "Low/Info", color: colors.low },
  ];

  return (
    <Page size="A4" style={s.page}>
      {/* Gradient accent blob (top-right) */}
      <Svg
        style={{ position: "absolute", top: -80, right: -80, width: 400, height: 400, opacity: 0.15 }}
      >
        <Defs>
          <LinearGradient id="blob" x1="0" y1="0" x2="1" y2="1">
            <Stop offset="0%" stopColor={colors.blue600} />
            <Stop offset="100%" stopColor={colors.purple600} />
          </LinearGradient>
        </Defs>
        <Circle cx="200" cy="200" r="200" fill="url(#blob)" />
      </Svg>

      {/* Bottom-left accent */}
      <Svg
        style={{ position: "absolute", bottom: -60, left: -60, width: 300, height: 300, opacity: 0.1 }}
      >
        <Defs>
          <LinearGradient id="blob2" x1="0" y1="0" x2="1" y2="1">
            <Stop offset="0%" stopColor={colors.purple600} />
            <Stop offset="100%" stopColor={colors.blue600} />
          </LinearGradient>
        </Defs>
        <Circle cx="150" cy="150" r="150" fill="url(#blob2)" />
      </Svg>

      <View style={s.content}>
        {/* Badge */}
        <View style={s.badge}>
          <Text style={s.badgeText}>
            <Text style={s.badgeAccent}>AI-POWERED</Text>  SECURITY ANALYSIS
          </Text>
        </View>

        {/* Title */}
        <Text style={s.title}>Parsentry</Text>
        <Text style={s.subtitle}>Security Report</Text>

        {/* Gradient divider */}
        <Svg style={s.divider} viewBox="0 0 80 3">
          <Defs>
            <LinearGradient id="divGrad" x1="0" y1="0" x2="1" y2="0">
              <Stop offset="0%" stopColor={colors.blue600} />
              <Stop offset="100%" stopColor={colors.purple600} />
            </LinearGradient>
          </Defs>
          <Rect x="0" y="0" width="80" height="3" fill="url(#divGrad)" />
        </Svg>

        {/* Metadata */}
        <View style={s.metaRow}>
          <Text style={s.metaLabel}>Target</Text>
          <Text style={s.metaValue}>{data.title}</Text>
        </View>
        <View style={s.metaRow}>
          <Text style={s.metaLabel}>Generated</Text>
          <Text style={s.metaValue}>{data.generatedAt}</Text>
        </View>
        <View style={s.metaRow}>
          <Text style={s.metaLabel}>Scanner</Text>
          <Text style={s.metaValue}>
            {data.toolName} v{data.toolVersion}
          </Text>
        </View>

        {/* Stats */}
        <View style={s.statsRow}>
          {statItems.map((item) => (
            <View key={item.label} style={s.statBox}>
              <Text style={[s.statNumber, { color: item.color }]}>{item.n}</Text>
              <Text style={s.statLabel}>{item.label}</Text>
            </View>
          ))}
        </View>
      </View>

      <View style={s.footer}>
        <Text style={s.footerText}>CONFIDENTIAL - FOR AUTHORIZED RECIPIENTS ONLY</Text>
      </View>
    </Page>
  );
}
