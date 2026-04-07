import React from "react";
import { Page, View, Text, StyleSheet, Svg, Defs, Stop, Rect, LinearGradient } from "@react-pdf/renderer";
import { colors, severityColor, severityLabel } from "../theme.js";
import type { ReportData, SarifResult } from "../types.js";

const s = StyleSheet.create({
  page: {
    backgroundColor: colors.white,
    paddingTop: 48,
    paddingBottom: 60,
    paddingHorizontal: 48,
  },
  header: {
    marginBottom: 24,
  },
  headerTitle: {
    fontSize: 20,
    fontFamily: "Helvetica-Bold",
    color: colors.slate900,
    marginBottom: 4,
  },
  headerSub: {
    fontSize: 9,
    color: colors.slate400,
    textTransform: "uppercase",
    letterSpacing: 2,
  },
  gradientBar: {
    width: "100%",
    height: 3,
    marginBottom: 24,
  },
  // Severity distribution bar
  distBar: {
    flexDirection: "row",
    height: 12,
    borderRadius: 6,
    overflow: "hidden",
    marginBottom: 24,
    backgroundColor: colors.slate100,
  },
  distSegment: {
    height: "100%",
  },
  distLegend: {
    flexDirection: "row",
    gap: 16,
    marginBottom: 32,
  },
  legendItem: {
    flexDirection: "row",
    alignItems: "center",
    gap: 4,
  },
  legendDot: {
    width: 8,
    height: 8,
    borderRadius: 4,
  },
  legendText: {
    fontSize: 8,
    color: colors.slate600,
  },
  // Table
  table: {
    borderWidth: 1,
    borderColor: colors.slate200,
    borderRadius: 6,
    overflow: "hidden",
  },
  tableHead: {
    flexDirection: "row",
    backgroundColor: colors.slate50,
    borderBottomWidth: 1,
    borderBottomColor: colors.slate200,
    paddingVertical: 8,
    paddingHorizontal: 12,
  },
  tableRow: {
    flexDirection: "row",
    paddingVertical: 7,
    paddingHorizontal: 12,
    borderBottomWidth: 1,
    borderBottomColor: colors.slate100,
  },
  tableRowAlt: {
    backgroundColor: colors.slate50,
  },
  thText: {
    fontSize: 8,
    fontFamily: "Helvetica-Bold",
    color: colors.slate500,
    textTransform: "uppercase",
    letterSpacing: 1,
  },
  tdText: {
    fontSize: 8,
    color: colors.slate700,
  },
  colSeverity: { width: "15%" },
  colRule: { width: "20%" },
  colFile: { width: "30%" },
  colLine: { width: "10%" },
  colConfidence: { width: "12%" },
  colStatus: { width: "13%" },
  // Severity pill
  pill: {
    paddingHorizontal: 6,
    paddingVertical: 2,
    borderRadius: 4,
    alignSelf: "flex-start",
  },
  pillText: {
    fontSize: 7,
    fontFamily: "Helvetica-Bold",
    color: colors.white,
    textTransform: "uppercase",
  },
  // Footer
  footer: {
    position: "absolute",
    bottom: 24,
    left: 48,
    right: 48,
    flexDirection: "row",
    justifyContent: "space-between",
    alignItems: "center",
  },
  footerText: {
    fontSize: 7,
    color: colors.slate400,
  },
});

function location(r: SarifResult) {
  const loc = r.locations?.[0]?.physicalLocation;
  if (!loc) return { file: "-", line: "-" };
  return {
    file: loc.artifactLocation.uri,
    line: loc.region ? String(loc.region.startLine) : "-",
  };
}

function confidenceLabel(c?: number): string {
  if (c === undefined) return "-";
  const pct = c <= 1 ? Math.round(c * 100) : Math.round(c);
  return `${pct}%`;
}

export function SummaryPage({ data }: { data: ReportData }) {
  const segments = [
    { count: data.stats.critical, color: colors.critical },
    { count: data.stats.high, color: colors.high },
    { count: data.stats.medium, color: colors.medium },
    { count: data.stats.low, color: colors.low },
    { count: data.stats.info, color: colors.info },
  ].filter((seg) => seg.count > 0);

  const sorted = [...data.results].sort((a, b) => {
    const order: Record<string, number> = { error: 0, warning: 1, note: 2, none: 3 };
    return (order[a.level] ?? 4) - (order[b.level] ?? 4);
  });

  // Paginate: ~25 rows per page
  const perPage = 25;
  const pages: SarifResult[][] = [];
  for (let i = 0; i < sorted.length; i += perPage) {
    pages.push(sorted.slice(i, i + perPage));
  }

  return (
    <>
      {pages.map((pageResults, pageIdx) => (
        <Page key={pageIdx} size="A4" style={s.page}>
          {pageIdx === 0 && (
            <>
              <View style={s.header}>
                <Text style={s.headerSub}>Executive Summary</Text>
                <Text style={s.headerTitle}>Findings Overview</Text>
              </View>

              {/* Gradient bar */}
              <Svg style={s.gradientBar} viewBox="0 0 500 3">
                <Defs>
                  <LinearGradient id="hbar" x1="0" y1="0" x2="1" y2="0">
                    <Stop offset="0%" stopColor={colors.blue600} />
                    <Stop offset="100%" stopColor={colors.purple600} />
                  </LinearGradient>
                </Defs>
                <Rect x="0" y="0" width="500" height="3" fill="url(#hbar)" />
              </Svg>

              {/* Distribution bar */}
              <View style={s.distBar}>
                {segments.map((seg, i) => (
                  <View
                    key={i}
                    style={[
                      s.distSegment,
                      {
                        width: `${(seg.count / data.stats.total) * 100}%`,
                        backgroundColor: seg.color,
                      },
                    ]}
                  />
                ))}
              </View>

              {/* Legend */}
              <View style={s.distLegend}>
                {[
                  { label: `Critical (${data.stats.critical})`, color: colors.critical },
                  { label: `High (${data.stats.high})`, color: colors.high },
                  { label: `Medium (${data.stats.medium})`, color: colors.medium },
                  { label: `Low (${data.stats.low})`, color: colors.low },
                  { label: `Info (${data.stats.info})`, color: colors.info },
                ].map((item) => (
                  <View key={item.label} style={s.legendItem}>
                    <View style={[s.legendDot, { backgroundColor: item.color }]} />
                    <Text style={s.legendText}>{item.label}</Text>
                  </View>
                ))}
              </View>
            </>
          )}

          {/* Table */}
          <View style={s.table}>
            <View style={s.tableHead}>
              <Text style={[s.thText, s.colSeverity]}>Severity</Text>
              <Text style={[s.thText, s.colRule]}>Rule</Text>
              <Text style={[s.thText, s.colFile]}>File</Text>
              <Text style={[s.thText, s.colLine]}>Line</Text>
              <Text style={[s.thText, s.colConfidence]}>Confidence</Text>
              <Text style={[s.thText, s.colStatus]}>Status</Text>
            </View>
            {pageResults.map((r, i) => {
              const loc = location(r);
              return (
                <View
                  key={i}
                  style={[s.tableRow, i % 2 === 1 && s.tableRowAlt]}
                >
                  <View style={s.colSeverity}>
                    <View style={[s.pill, { backgroundColor: severityColor(r.level) }]}>
                      <Text style={s.pillText}>{severityLabel(r.level)}</Text>
                    </View>
                  </View>
                  <Text style={[s.tdText, s.colRule, { fontFamily: "Courier" }]}>
                    {r.ruleId}
                  </Text>
                  <Text style={[s.tdText, s.colFile]}>{loc.file}</Text>
                  <Text style={[s.tdText, s.colLine]}>{loc.line}</Text>
                  <Text style={[s.tdText, s.colConfidence]}>
                    {confidenceLabel(r.properties?.confidence)}
                  </Text>
                  <Text style={[s.tdText, s.colStatus]}>
                    {r.baselineState ?? "new"}
                  </Text>
                </View>
              );
            })}
          </View>

          {/* Footer */}
          <View style={s.footer} fixed>
            <Text style={s.footerText}>Parsentry Security Report</Text>
            <Text
              style={s.footerText}
              render={({ pageNumber, totalPages }) =>
                `${pageNumber} / ${totalPages}`
              }
            />
          </View>
        </Page>
      ))}
    </>
  );
}
