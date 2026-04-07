import React from "react";
import { Document } from "@react-pdf/renderer";
import { CoverPage } from "./CoverPage.js";
import { SummaryPage } from "./SummaryPage.js";
import { CategoriesPage } from "./CategoriesPage.js";
import { FindingPages } from "./FindingPage.js";
import { RemediationPage } from "./RemediationPage.js";
import type { ReportData } from "../types.js";
import type { MarkdownReport } from "../markdown-parser.js";

export function Report({
  data,
  markdown,
}: {
  data: ReportData;
  markdown?: MarkdownReport;
}) {
  return (
    <Document
      title={`Parsentry Report - ${data.title}`}
      author="Parsentry"
      subject="Security Analysis Report"
      creator="@parsentry/pdf-report"
    >
      <CoverPage data={data} />
      {markdown && markdown.categories.length > 0 && (
        <CategoriesPage categories={markdown.categories} />
      )}
      <SummaryPage data={data} />
      <FindingPages
        results={data.results}
        rules={data.rules}
        mdFindings={markdown?.findings}
      />
      {markdown && markdown.remediationGroups.length > 0 && (
        <RemediationPage groups={markdown.remediationGroups} />
      )}
    </Document>
  );
}
