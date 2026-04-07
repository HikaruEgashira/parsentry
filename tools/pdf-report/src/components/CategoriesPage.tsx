import React from "react";
import { Page, View, Text, StyleSheet, Svg, Defs, Stop, Rect, LinearGradient } from "@react-pdf/renderer";
import { colors } from "../theme.js";
import type { VulnCategory } from "../markdown-parser.js";

const s = StyleSheet.create({
  page: {
    backgroundColor: colors.white,
    paddingTop: 48,
    paddingBottom: 60,
    paddingHorizontal: 48,
  },
  header: { marginBottom: 24 },
  headerSub: {
    fontSize: 9,
    color: colors.slate400,
    textTransform: "uppercase",
    letterSpacing: 2,
  },
  headerTitle: {
    fontSize: 20,
    fontFamily: "Helvetica-Bold",
    color: colors.slate900,
    marginBottom: 4,
  },
  gradientBar: { width: "100%", height: 3, marginBottom: 24 },
  // Category cards
  grid: {
    flexDirection: "row",
    flexWrap: "wrap",
    gap: 12,
  },
  card: {
    width: "48%",
    borderWidth: 1,
    borderColor: colors.slate200,
    borderRadius: 8,
    padding: 14,
    backgroundColor: colors.slate50,
  },
  cardHeader: {
    flexDirection: "row",
    justifyContent: "space-between",
    alignItems: "center",
    marginBottom: 8,
  },
  cardCategory: {
    fontSize: 10,
    fontFamily: "Helvetica-Bold",
    color: colors.slate800,
    maxWidth: "75%",
  },
  countBadge: {
    paddingHorizontal: 8,
    paddingVertical: 3,
    borderRadius: 10,
    minWidth: 28,
    alignItems: "center",
  },
  countText: {
    fontSize: 9,
    fontFamily: "Helvetica-Bold",
    color: colors.white,
  },
  impactText: {
    fontSize: 8,
    color: colors.slate500,
    lineHeight: 1.4,
  },
  // Bar chart
  barRow: {
    flexDirection: "row",
    alignItems: "center",
    marginBottom: 6,
    gap: 8,
  },
  barLabel: {
    fontSize: 7,
    color: colors.slate600,
    width: 160,
    textAlign: "right",
  },
  barTrack: {
    flex: 1,
    height: 14,
    backgroundColor: colors.slate100,
    borderRadius: 4,
    overflow: "hidden",
  },
  barFill: {
    height: "100%",
    borderRadius: 4,
  },
  barCount: {
    fontSize: 8,
    fontFamily: "Helvetica-Bold",
    color: colors.white,
    position: "absolute",
    right: 6,
    top: 1,
  },
  // Footer
  footer: {
    position: "absolute",
    bottom: 24,
    left: 48,
    right: 48,
    flexDirection: "row",
    justifyContent: "space-between",
  },
  footerText: { fontSize: 7, color: colors.slate400 },
});

const barColors = [
  "#DC2626", "#EA580C", "#D97706", "#CA8A04", "#65A30D",
  "#0D9488", "#2563EB", "#7C3AED", "#DB2777", "#6B7280",
  "#DC2626", "#EA580C", "#D97706", "#CA8A04", "#65A30D",
  "#0D9488",
];

export function CategoriesPage({ categories }: { categories: VulnCategory[] }) {
  if (categories.length === 0) return null;

  const maxCount = Math.max(...categories.map((c) => c.count), 1);

  return (
    <Page size="A4" style={s.page}>
      <View style={s.header}>
        <Text style={s.headerSub}>Threat Landscape</Text>
        <Text style={s.headerTitle}>Vulnerability Categories</Text>
      </View>

      <Svg style={s.gradientBar} viewBox="0 0 500 3">
        <Defs>
          <LinearGradient id="catbar" x1="0" y1="0" x2="1" y2="0">
            <Stop offset="0%" stopColor={colors.blue600} />
            <Stop offset="100%" stopColor={colors.purple600} />
          </LinearGradient>
        </Defs>
        <Rect x="0" y="0" width="500" height="3" fill="url(#catbar)" />
      </Svg>

      {/* Horizontal bar chart */}
      {categories.map((cat, i) => (
        <View key={i} style={s.barRow}>
          <Text style={s.barLabel}>{cat.category}</Text>
          <View style={s.barTrack}>
            <View
              style={[
                s.barFill,
                {
                  width: `${(cat.count / maxCount) * 100}%`,
                  backgroundColor: barColors[i % barColors.length],
                  position: "relative",
                },
              ]}
            >
              <Text style={s.barCount}>{cat.count}</Text>
            </View>
          </View>
        </View>
      ))}

      {/* Impact cards grid */}
      <View style={[s.grid, { marginTop: 24 }]}>
        {categories.map((cat, i) => (
          <View key={i} style={s.card} wrap={false}>
            <View style={s.cardHeader}>
              <Text style={s.cardCategory}>{cat.category}</Text>
              <View
                style={[
                  s.countBadge,
                  { backgroundColor: barColors[i % barColors.length] },
                ]}
              >
                <Text style={s.countText}>{cat.count}</Text>
              </View>
            </View>
            <Text style={s.impactText}>{cat.impact}</Text>
          </View>
        ))}
      </View>

      <View style={s.footer} fixed>
        <Text style={s.footerText}>Parsentry Security Report</Text>
        <Text
          style={s.footerText}
          render={({ pageNumber, totalPages }) => `${pageNumber} / ${totalPages}`}
        />
      </View>
    </Page>
  );
}
