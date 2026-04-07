import React from "react";
import { Page, View, Text, StyleSheet, Svg, Defs, Stop, Rect, LinearGradient } from "@react-pdf/renderer";
import { colors, severityColor, severityLabel, confidenceColor } from "../theme.js";
import type { SarifResult, SarifRule } from "../types.js";
import type { FindingDetail } from "../markdown-parser.js";

const s = StyleSheet.create({
  page: {
    backgroundColor: colors.white,
    paddingTop: 48,
    paddingBottom: 60,
    paddingHorizontal: 48,
  },
  // Finding card
  card: {
    borderWidth: 1,
    borderColor: colors.slate200,
    borderRadius: 8,
    marginBottom: 16,
    overflow: "hidden",
  },
  cardHeader: {
    flexDirection: "row",
    alignItems: "center",
    paddingVertical: 10,
    paddingHorizontal: 14,
    gap: 10,
  },
  severityStripe: {
    width: 4,
    height: "100%",
    position: "absolute",
    left: 0,
    top: 0,
    bottom: 0,
  },
  pill: {
    paddingHorizontal: 8,
    paddingVertical: 3,
    borderRadius: 4,
  },
  pillText: {
    fontSize: 7,
    fontFamily: "Helvetica-Bold",
    color: colors.white,
    textTransform: "uppercase",
    letterSpacing: 0.5,
  },
  ruleId: {
    fontSize: 11,
    fontFamily: "Courier-Bold",
    color: colors.slate800,
  },
  ruleName: {
    fontSize: 9,
    color: colors.slate500,
    marginLeft: "auto",
  },
  cardBody: {
    paddingHorizontal: 14,
    paddingVertical: 10,
    backgroundColor: colors.slate50,
    borderTopWidth: 1,
    borderTopColor: colors.slate100,
  },
  messageText: {
    fontSize: 9,
    color: colors.slate700,
    lineHeight: 1.5,
  },
  // Metadata chips
  metaRow: {
    flexDirection: "row",
    flexWrap: "wrap",
    gap: 6,
    paddingHorizontal: 14,
    paddingVertical: 8,
    borderTopWidth: 1,
    borderTopColor: colors.slate100,
  },
  chip: {
    flexDirection: "row",
    alignItems: "center",
    backgroundColor: colors.slate100,
    paddingHorizontal: 8,
    paddingVertical: 3,
    borderRadius: 10,
    gap: 4,
  },
  chipLabel: {
    fontSize: 7,
    color: colors.slate500,
    textTransform: "uppercase",
    letterSpacing: 0.5,
  },
  chipValue: {
    fontSize: 7,
    fontFamily: "Helvetica-Bold",
    color: colors.slate700,
  },
  // Location
  locationRow: {
    flexDirection: "row",
    alignItems: "center",
    gap: 6,
    paddingHorizontal: 14,
    paddingVertical: 6,
    borderTopWidth: 1,
    borderTopColor: colors.slate100,
  },
  locationIcon: {
    fontSize: 8,
    color: colors.slate400,
  },
  locationText: {
    fontSize: 8,
    fontFamily: "Courier",
    color: colors.slate600,
  },
  // Snippet
  snippetBox: {
    marginHorizontal: 14,
    marginVertical: 6,
    backgroundColor: colors.slate900,
    borderRadius: 6,
    padding: 10,
  },
  snippetText: {
    fontSize: 7,
    fontFamily: "Courier",
    color: colors.slate200,
    lineHeight: 1.6,
  },
  // Confidence gauge
  gaugeContainer: {
    flexDirection: "row",
    alignItems: "center",
    gap: 6,
  },
  gaugeBar: {
    width: 40,
    height: 4,
    backgroundColor: colors.slate200,
    borderRadius: 2,
    overflow: "hidden",
  },
  gaugeFill: {
    height: "100%",
    borderRadius: 2,
  },
  // Page section header
  sectionHeader: {
    marginBottom: 16,
  },
  sectionSub: {
    fontSize: 9,
    color: colors.slate400,
    textTransform: "uppercase",
    letterSpacing: 2,
  },
  sectionTitle: {
    fontSize: 18,
    fontFamily: "Helvetica-Bold",
    color: colors.slate900,
    marginBottom: 4,
  },
  gradientBar: {
    width: "100%",
    height: 2,
    marginBottom: 16,
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
  footerText: {
    fontSize: 7,
    color: colors.slate400,
  },
});

function FindingCard({
  result,
  rule,
  index,
  mdFinding,
}: {
  result: SarifResult;
  rule?: SarifRule;
  index: number;
  mdFinding?: FindingDetail;
}) {
  const loc = result.locations?.[0]?.physicalLocation;
  const file = loc?.artifactLocation.uri ?? "-";
  const line = loc?.region?.startLine;
  const snippet = loc?.region?.snippet?.text;
  const conf = result.properties?.confidence;
  const confPct = conf !== undefined ? (conf <= 1 ? conf * 100 : conf) : undefined;

  return (
    <View style={s.card} wrap={false}>
      {/* Severity stripe */}
      <View style={[s.severityStripe, { backgroundColor: severityColor(result.level) }]} />

      {/* Header */}
      <View style={s.cardHeader}>
        <View style={[s.pill, { backgroundColor: severityColor(result.level) }]}>
          <Text style={s.pillText}>{severityLabel(result.level)}</Text>
        </View>
        <Text style={s.ruleId}>
          #{index + 1} {result.ruleId}
        </Text>
        {rule?.name && <Text style={s.ruleName}>{rule.name}</Text>}
      </View>

      {/* Location */}
      <View style={s.locationRow}>
        <Text style={s.locationText}>
          {file}
          {line ? `:${line}` : ""}
        </Text>
        {confPct !== undefined && (
          <View style={s.gaugeContainer}>
            <View style={s.gaugeBar}>
              <View
                style={[
                  s.gaugeFill,
                  {
                    width: `${confPct}%`,
                    backgroundColor: confidenceColor(conf),
                  },
                ]}
              />
            </View>
            <Text style={[s.chipValue, { color: confidenceColor(conf) }]}>
              {Math.round(confPct)}%
            </Text>
          </View>
        )}
      </View>

      {/* Message */}
      <View style={s.cardBody}>
        <Text style={s.messageText}>{result.message.text}</Text>
      </View>

      {/* Snippet */}
      {snippet && (
        <View style={s.snippetBox}>
          <Text style={s.snippetText}>{snippet}</Text>
        </View>
      )}

      {/* Metadata chips */}
      {result.properties && (
        <View style={s.metaRow}>
          {result.properties.cwe?.map((c) => (
            <View key={c} style={s.chip}>
              <Text style={s.chipLabel}>CWE</Text>
              <Text style={s.chipValue}>{c}</Text>
            </View>
          ))}
          {result.properties.owasp?.map((o) => (
            <View key={o} style={s.chip}>
              <Text style={s.chipLabel}>OWASP</Text>
              <Text style={s.chipValue}>{o}</Text>
            </View>
          ))}
          {result.properties.mitre_attack?.map((m) => (
            <View key={m} style={s.chip}>
              <Text style={s.chipLabel}>MITRE</Text>
              <Text style={s.chipValue}>{m}</Text>
            </View>
          ))}
          {result.properties.principal && (
            <View style={s.chip}>
              <Text style={s.chipLabel}>Principal</Text>
              <Text style={s.chipValue}>{result.properties.principal}</Text>
            </View>
          )}
          {result.properties.action && (
            <View style={s.chip}>
              <Text style={s.chipLabel}>Action</Text>
              <Text style={s.chipValue}>{result.properties.action}</Text>
            </View>
          )}
          {result.properties.resource && (
            <View style={s.chip}>
              <Text style={s.chipLabel}>Resource</Text>
              <Text style={s.chipValue}>{result.properties.resource}</Text>
            </View>
          )}
        </View>
      )}

      {/* Remediation */}
      {(mdFinding?.remediation || rule?.help?.text) && (
        <View style={[s.cardBody, { backgroundColor: "#EFF6FF" }]}>
          <Text style={[s.chipLabel, { marginBottom: 4, color: colors.blue600 }]}>
            REMEDIATION
          </Text>
          <Text style={[s.messageText, { color: colors.slate600 }]}>
            {mdFinding?.remediation || rule?.help?.text}
          </Text>
        </View>
      )}
    </View>
  );
}

export function FindingPages({
  results,
  rules,
  mdFindings,
}: {
  results: SarifResult[];
  rules: SarifRule[];
  mdFindings?: FindingDetail[];
}) {
  const ruleMap = new Map(rules.map((r) => [r.id, r]));
  const sorted = [...results].sort((a, b) => {
    const order: Record<string, number> = { error: 0, warning: 1, note: 2, none: 3 };
    return (order[a.level] ?? 4) - (order[b.level] ?? 4);
  });

  // Build lookup: match mdFindings to SARIF results by ruleId + file
  const mdMap = new Map<string, FindingDetail>();
  if (mdFindings) {
    for (const f of mdFindings) {
      // Key: ruleId (normalized)
      const key = f.ruleId.toUpperCase();
      if (!mdMap.has(key)) mdMap.set(key, f);
    }
  }

  function findMdFinding(r: SarifResult): FindingDetail | undefined {
    return mdMap.get(r.ruleId.toUpperCase());
  }

  return (
    <Page size="A4" style={s.page} wrap>
      <View style={s.sectionHeader} fixed>
        <Text style={s.sectionSub}>Detailed Analysis</Text>
        <Text style={s.sectionTitle}>Vulnerability Findings</Text>
        <Svg style={s.gradientBar} viewBox="0 0 500 2">
          <Defs>
            <LinearGradient id="fbar" x1="0" y1="0" x2="1" y2="0">
              <Stop offset="0%" stopColor={colors.blue600} />
              <Stop offset="100%" stopColor={colors.purple600} />
            </LinearGradient>
          </Defs>
          <Rect x="0" y="0" width="500" height="2" fill="url(#fbar)" />
        </Svg>
      </View>

      {sorted.map((r, i) => (
        <FindingCard
          key={i}
          result={r}
          rule={ruleMap.get(r.ruleId)}
          index={i}
          mdFinding={findMdFinding(r)}
        />
      ))}

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
