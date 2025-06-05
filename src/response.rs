use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum VulnType {
    LFI,
    RCE,
    SSRF,
    AFO,
    SQLI,
    XSS,
    IDOR,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextCode {
    pub name: String,
    pub reason: String,
    pub code_line: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub scratchpad: String,
    pub analysis: String,
    pub poc: String,
    pub confidence_score: i32,
    pub vulnerability_types: Vec<VulnType>,
    pub context_code: Vec<ContextCode>,
}

pub fn response_json_schema() -> serde_json::Value {
    json!({
        "type": "object",
        "properties": {
            "scratchpad": { "type": "string" },
            "analysis": { "type": "string" },
            "poc": { "type": "string" },
            "confidence_score": { "type": "integer" },
            "vulnerability_types": {
                "type": "array",
                "items": {
                    "type": "string",
                    "enum": ["LFI", "RCE", "SSRF", "AFO", "SQLI", "XSS", "IDOR"]
                }
            },
            "context_code": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "name": { "type": "string" },
                        "reason": { "type": "string" },
                        "code_line": { "type": "string" },
                        "path": { "type": "string" }
                    },
                    "required": ["name", "reason", "code_line", "path"]
                }
            }
        },
        "required": ["scratchpad", "analysis", "poc", "confidence_score", "vulnerability_types", "context_code"]
    })
}

impl Response {
    pub fn normalize_confidence_score(score: i32) -> i32 {
        if score > 0 && score <= 10 {
            score * 10
        } else {
            score
        }
    }

    pub fn print_readable(&self) {
        println!("\n📝 解析レポート");
        println!("{}", "=".repeat(80));

        let confidence_icon = match self.confidence_score {
            90..=100 => "🔴 高",
            70..=89 => "🟠 中高",
            50..=69 => "🟡 中",
            30..=49 => "🟢 中低",
            _ => "🔵 低",
        };
        println!(
            "\n🎯 信頼度スコア: {} ({})",
            self.confidence_score, confidence_icon
        );

        if !self.vulnerability_types.is_empty() {
            println!("\n⚠ 検出された脆弱性タイプ:");
            for vuln_type in &self.vulnerability_types {
                println!("  - {:?}", vuln_type);
            }
        }

        println!("\n🔍 解析結果:");
        println!("{}", "-".repeat(80));
        println!("{}", self.analysis);

        if !self.poc.is_empty() {
            println!("\n🔨 PoC(概念実証コード):");
            println!("{}", "-".repeat(80));
            println!("{}", self.poc);
        }

        if !self.context_code.is_empty() {
            println!("\n📄 関連コードコンテキスト:");
            println!("{}", "-".repeat(80));
            for context in &self.context_code {
                println!("関数名: {}", context.name);
                println!("理由: {}", context.reason);
                println!("コード: {}", context.code_line);
                println!("パス: {}", context.path);
                println!();
            }
        }

        if !self.scratchpad.is_empty() {
            println!("\n📓 解析ノート:");
            println!("{}", "-".repeat(80));
            println!("{}", self.scratchpad);
        }

        println!();
    }

    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str("# 解析レポート\n\n");

        let confidence_badge = match self.confidence_score {
            90..=100 => "![高信頼度](https://img.shields.io/badge/信頼度-高-red)",
            70..=89 => "![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange)",
            50..=69 => "![中信頼度](https://img.shields.io/badge/信頼度-中-yellow)",
            30..=49 => "![中低信頼度](https://img.shields.io/badge/信頼度-中低-green)",
            _ => "![低信頼度](https://img.shields.io/badge/信頼度-低-blue)",
        };
        md.push_str(&format!(
            "{} **信頼度スコア: {}**\n\n",
            confidence_badge, self.confidence_score
        ));

        if !self.vulnerability_types.is_empty() {
            md.push_str("## 脆弱性タイプ\n\n");
            for vuln_type in &self.vulnerability_types {
                md.push_str(&format!("- `{:?}`\n", vuln_type));
            }
            md.push('\n');
        }

        md.push_str("## 解析結果\n\n");
        md.push_str(&self.analysis);
        md.push_str("\n\n");

        if !self.poc.is_empty() {
            md.push_str("## PoC（概念実証コード）\n\n");
            md.push_str("```text\n");
            md.push_str(&self.poc);
            md.push_str("\n```\n\n");
        }

        if !self.context_code.is_empty() {
            md.push_str("## 関連コードコンテキスト\n\n");
            for context in &self.context_code {
                md.push_str(&format!("### 関数名: {}\n", context.name));
                md.push_str(&format!("- 理由: {}\n", context.reason));
                md.push_str(&format!("- パス: {}\n", context.path));
                md.push_str("```rust\n");
                md.push_str(&context.code_line);
                md.push_str("\n```\n\n");
            }
        }

        if !self.scratchpad.is_empty() {
            md.push_str("## 解析ノート\n\n");
            md.push_str(&self.scratchpad);
            md.push_str("\n\n");
        }

        md
    }
}

#[derive(Debug, Clone)]
pub struct FileAnalysisResult {
    pub file_path: PathBuf,
    pub response: Response,
}

#[derive(Debug, Clone, Default)]
pub struct AnalysisSummary {
    pub results: Vec<FileAnalysisResult>,
}

impl AnalysisSummary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_result(&mut self, file_path: PathBuf, response: Response) {
        self.results.push(FileAnalysisResult {
            file_path,
            response,
        });
    }

    pub fn sort_by_confidence(&mut self) {
        self.results.sort_by(|a, b| {
            b.response
                .confidence_score
                .cmp(&a.response.confidence_score)
        });
    }

    pub fn filter_by_min_confidence(&self, min_score: i32) -> Self {
        Self {
            results: self
                .results
                .iter()
                .filter(|r| r.response.confidence_score >= min_score)
                .cloned()
                .collect(),
        }
    }

    pub fn filter_by_vuln_types(&self, vuln_types: &[VulnType]) -> Self {
        Self {
            results: self
                .results
                .iter()
                .filter(|r| {
                    r.response
                        .vulnerability_types
                        .iter()
                        .any(|vt| vuln_types.contains(vt))
                })
                .cloned()
                .collect(),
        }
    }

    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str("# 脆弱性解析サマリーレポート\n\n");

        md.push_str("## 概要\n\n");
        md.push_str("| ファイル | 脆弱性タイプ | 信頼度 | 重要度 |\n");
        md.push_str("|---------|------------|--------|--------|\n");

        for result in &self.results {
            if result.response.confidence_score > 0 {
                let confidence_level = match result.response.confidence_score {
                    90..=100 => "🔴 高",
                    70..=89 => "🟠 中高",
                    50..=69 => "🟡 中",
                    30..=49 => "🟢 中低",
                    _ => "🔵 低",
                };

                let vuln_types = result
                    .response
                    .vulnerability_types
                    .iter()
                    .map(|vt| format!("{:?}", vt))
                    .collect::<Vec<_>>()
                    .join(", ");

                md.push_str(&format!(
                    "| [{}]({}.md) | {} | {} | {} |\n",
                    result
                        .file_path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy(),
                    result
                        .file_path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy(),
                    vuln_types,
                    result.response.confidence_score,
                    confidence_level
                ));
            }
        }

        md.push_str("\n## 脆弱性タイプ別集計\n\n");

        let mut type_count: HashMap<&VulnType, i32> = HashMap::new();
        for result in &self.results {
            for vuln_type in &result.response.vulnerability_types {
                *type_count.entry(vuln_type).or_insert(0) += 1;
            }
        }

        md.push_str("| 脆弱性タイプ | 件数 |\n");
        md.push_str("|------------|------|\n");

        for (vuln_type, count) in type_count.iter() {
            md.push_str(&format!("| {:?} | {} |\n", vuln_type, count));
        }

        md
    }
}
