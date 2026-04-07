export interface SarifReport {
  $schema: string;
  version: string;
  runs: SarifRun[];
}

export interface SarifRun {
  tool: { driver: SarifDriver };
  results: SarifResult[];
  invocation?: {
    executionSuccessful: boolean;
    startTimeUtc?: string;
    endTimeUtc?: string;
  };
}

export interface SarifDriver {
  name: string;
  version: string;
  information_uri?: string;
  rules?: SarifRule[];
}

export interface SarifRule {
  id: string;
  name?: string;
  short_description?: { text: string };
  help?: { text: string };
  properties?: {
    tags?: string[];
    security_severity?: string;
  };
}

export interface SarifResult {
  ruleId: string;
  ruleIndex?: number;
  level: "error" | "warning" | "note" | "none";
  message: { text: string };
  locations: SarifLocation[];
  fingerprints?: Record<string, string>;
  baselineState?: "new" | "unchanged" | "updated" | "absent";
  properties?: {
    confidence?: number;
    cwe?: string[];
    owasp?: string[];
    mitre_attack?: string[];
    principal?: string;
    action?: string;
    resource?: string;
    data_flow?: string;
  };
}

export interface SarifLocation {
  physicalLocation: {
    artifactLocation: { uri: string };
    region?: {
      startLine: number;
      startColumn?: number;
      endLine?: number;
      snippet?: { text: string };
    };
  };
}

export interface ReportData {
  title: string;
  generatedAt: string;
  toolName: string;
  toolVersion: string;
  results: SarifResult[];
  rules: SarifRule[];
  stats: {
    total: number;
    critical: number;
    high: number;
    medium: number;
    low: number;
    info: number;
  };
}
