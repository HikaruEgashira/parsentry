import React from "react";
import { Page, View, Text, StyleSheet, Svg, Defs, Stop, Rect, LinearGradient, Circle } from "@react-pdf/renderer";
import { colors } from "../theme.js";
import type { RemediationGroup } from "../markdown-parser.js";

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
  gradientBar: { width: "100%", height: 3, marginBottom: 28 },
  // Priority group
  group: {
    marginBottom: 24,
  },
  groupHeader: {
    flexDirection: "row",
    alignItems: "center",
    marginBottom: 12,
    gap: 10,
  },
  priorityBadge: {
    paddingHorizontal: 10,
    paddingVertical: 4,
    borderRadius: 6,
  },
  priorityText: {
    fontSize: 9,
    fontFamily: "Helvetica-Bold",
    color: colors.white,
    textTransform: "uppercase",
    letterSpacing: 0.5,
  },
  groupTitle: {
    fontSize: 13,
    fontFamily: "Helvetica-Bold",
    color: colors.slate800,
  },
  // Items
  item: {
    flexDirection: "row",
    alignItems: "flex-start",
    paddingVertical: 8,
    paddingHorizontal: 14,
    borderBottomWidth: 1,
    borderBottomColor: colors.slate100,
    gap: 10,
  },
  itemNumber: {
    width: 22,
    height: 22,
    borderRadius: 11,
    alignItems: "center",
    justifyContent: "center",
  },
  itemNumberText: {
    fontSize: 9,
    fontFamily: "Helvetica-Bold",
    color: colors.white,
  },
  itemText: {
    fontSize: 9,
    color: colors.slate700,
    lineHeight: 1.5,
    flex: 1,
  },
  itemBold: {
    fontFamily: "Helvetica-Bold",
    color: colors.slate800,
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

const priorityColors: Record<string, string> = {
  "Immediate (Critical Risk)": colors.critical,
  "Immediate": colors.critical,
  "High Priority": colors.high,
  "High": colors.high,
  "Medium Priority": colors.medium,
  "Medium": colors.medium,
  "Low Priority": colors.low,
  "Low": colors.low,
};

function priorityColor(priority: string): string {
  for (const [key, color] of Object.entries(priorityColors)) {
    if (priority.includes(key)) return color;
  }
  return colors.info;
}

function parseItemText(text: string): { bold: string; rest: string } {
  const m = text.match(/^\*\*(.+?)\*\*[:\s]*(.*)$/);
  if (m) return { bold: m[1], rest: m[2] };
  return { bold: "", rest: text };
}

export function RemediationPage({ groups }: { groups: RemediationGroup[] }) {
  if (groups.length === 0) return null;

  let globalIndex = 0;

  return (
    <Page size="A4" style={s.page} wrap>
      <View style={s.header} fixed>
        <Text style={s.headerSub}>Action Plan</Text>
        <Text style={s.headerTitle}>Remediation Priority</Text>
        <Svg style={s.gradientBar} viewBox="0 0 500 3">
          <Defs>
            <LinearGradient id="rembar" x1="0" y1="0" x2="1" y2="0">
              <Stop offset="0%" stopColor={colors.blue600} />
              <Stop offset="100%" stopColor={colors.purple600} />
            </LinearGradient>
          </Defs>
          <Rect x="0" y="0" width="500" height="3" fill="url(#rembar)" />
        </Svg>
      </View>

      {groups.map((group, gi) => {
        const color = priorityColor(group.priority);
        return (
          <View key={gi} style={s.group} wrap={false}>
            <View style={s.groupHeader}>
              <View style={[s.priorityBadge, { backgroundColor: color }]}>
                <Text style={s.priorityText}>
                  {group.priority.split("(")[0].trim()}
                </Text>
              </View>
              {group.priority.includes("(") && (
                <Text style={[s.groupTitle, { color }]}>
                  ({group.priority.split("(")[1]}
                </Text>
              )}
            </View>

            {group.items.map((item, ii) => {
              globalIndex++;
              const { bold, rest } = parseItemText(item);
              return (
                <View key={ii} style={s.item}>
                  <View style={[s.itemNumber, { backgroundColor: color }]}>
                    <Text style={s.itemNumberText}>{globalIndex}</Text>
                  </View>
                  <Text style={s.itemText}>
                    {bold && <Text style={s.itemBold}>{bold}: </Text>}
                    {rest}
                  </Text>
                </View>
              );
            })}
          </View>
        );
      })}

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
