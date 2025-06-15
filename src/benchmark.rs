use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::fs;
use tokio::process::Command;
use tracing::{debug, error, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkMetadata {
    pub name: String,
    pub description: String,
    pub level: u8,
    pub win_condition: String,
    pub tags: Vec<String>,
    pub canaries: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub benchmark_id: String,
    pub detected: bool,
    pub correct_type: bool,
    pub confidence_score: f64,
    pub metadata: BenchmarkMetadata,
    pub vulnerabilities_found: Vec<VulnerabilityResult>,
    pub execution_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerabilityResult {
    pub vulnerability_type: String,
    pub confidence: f64,
    pub file_path: String,
    pub line_number: Option<u32>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverallScore {
    pub timestamp: String,
    pub total_benchmarks: usize,
    pub detected_count: usize,
    pub correct_type_count: usize,
    pub detection_rate: f64,
    pub accuracy_rate: f64,
    pub avg_confidence: f64,
    pub avg_execution_time_ms: f64,
    pub scores_by_level: HashMap<u8, LevelScore>,
    pub scores_by_tag: HashMap<String, TagScore>,
    pub detailed_results: Vec<BenchmarkResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelScore {
    pub total: usize,
    pub detected: usize,
    pub correct_type: usize,
    pub detection_rate: f64,
    pub accuracy_rate: f64,
    pub avg_confidence: f64,
    pub avg_execution_time_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagScore {
    pub total: usize,
    pub detected: usize,
    pub correct_type: usize,
    pub detection_rate: f64,
    pub accuracy_rate: f64,
    pub avg_confidence: f64,
}

pub struct BenchmarkRunner {
    benchmarks_dir: PathBuf,
    output_dir: PathBuf,
    vulnerability_mappings: HashMap<String, Vec<String>>,
}

impl BenchmarkRunner {
    pub fn new(benchmarks_dir: PathBuf, output_dir: PathBuf) -> Self {
        let vulnerability_mappings = Self::create_vulnerability_mappings();
        
        Self {
            benchmarks_dir,
            output_dir,
            vulnerability_mappings,
        }
    }

    fn create_vulnerability_mappings() -> HashMap<String, Vec<String>> {
        let mut mappings = HashMap::new();
        
        mappings.insert("xss".to_string(), vec!["XSS".to_string(), "CROSS_SITE_SCRIPTING".to_string()]);
        mappings.insert("sqli".to_string(), vec!["SQLI".to_string(), "SQL_INJECTION".to_string()]);
        mappings.insert("idor".to_string(), vec!["IDOR".to_string(), "INSECURE_DIRECT_OBJECT_REFERENCE".to_string()]);
        mappings.insert("lfi".to_string(), vec!["LFI".to_string(), "LOCAL_FILE_INCLUSION".to_string()]);
        mappings.insert("rfi".to_string(), vec!["RFI".to_string(), "REMOTE_FILE_INCLUSION".to_string()]);
        mappings.insert("ssrf".to_string(), vec!["SSRF".to_string(), "SERVER_SIDE_REQUEST_FORGERY".to_string()]);
        mappings.insert("csrf".to_string(), vec!["CSRF".to_string(), "CROSS_SITE_REQUEST_FORGERY".to_string()]);
        mappings.insert("command_injection".to_string(), vec!["COMMAND_INJECTION".to_string(), "CODE_INJECTION".to_string()]);
        mappings.insert("path_traversal".to_string(), vec!["PATH_TRAVERSAL".to_string(), "DIRECTORY_TRAVERSAL".to_string()]);
        mappings.insert("file_upload".to_string(), vec!["FILE_UPLOAD".to_string(), "UNRESTRICTED_FILE_UPLOAD".to_string()]);
        mappings.insert("authentication".to_string(), vec!["AUTH_BYPASS".to_string(), "WEAK_AUTHENTICATION".to_string()]);
        mappings.insert("authorization".to_string(), vec!["AUTHZ_BYPASS".to_string(), "BROKEN_AUTHORIZATION".to_string()]);
        mappings.insert("default_credentials".to_string(), vec!["DEFAULT_CREDENTIALS".to_string(), "WEAK_CREDENTIALS".to_string()]);
        mappings.insert("information_disclosure".to_string(), vec!["INFO_DISCLOSURE".to_string(), "SENSITIVE_DATA_EXPOSURE".to_string()]);
        mappings.insert("deserialization".to_string(), vec!["UNSAFE_DESERIALIZATION".to_string()]);
        mappings.insert("xxe".to_string(), vec!["XXE".to_string(), "XML_EXTERNAL_ENTITY".to_string()]);
        
        mappings
    }

    pub async fn discover_benchmarks(&self) -> Result<Vec<String>> {
        let mut benchmarks = Vec::new();
        
        if !self.benchmarks_dir.exists() {
            return Err(anyhow::anyhow!("Benchmarks directory does not exist: {:?}", self.benchmarks_dir));
        }

        let mut entries = fs::read_dir(&self.benchmarks_dir).await
            .context("Failed to read benchmarks directory")?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with("XBEN-") {
                        let benchmark_json = path.join("benchmark.json");
                        if benchmark_json.exists() {
                            benchmarks.push(name.to_string());
                        }
                    }
                }
            }
        }

        benchmarks.sort();
        info!("Discovered {} benchmarks", benchmarks.len());
        Ok(benchmarks)
    }

    pub async fn load_benchmark_metadata(&self, benchmark_id: &str) -> Result<BenchmarkMetadata> {
        let metadata_path = self.benchmarks_dir.join(benchmark_id).join("benchmark.json");
        let content = fs::read_to_string(&metadata_path).await
            .with_context(|| format!("Failed to read metadata for {}", benchmark_id))?;
        
        let data: serde_json::Value = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse metadata JSON for {}", benchmark_id))?;
        
        Ok(BenchmarkMetadata {
            name: data.get("name").and_then(|v| v.as_str()).unwrap_or(benchmark_id).to_string(),
            description: data.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            level: data.get("level").and_then(|v| v.as_u64()).unwrap_or(1) as u8,
            win_condition: data.get("win_condition").and_then(|v| v.as_str()).unwrap_or("flag").to_string(),
            tags: data.get("tags").and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|t| t.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default(),
            canaries: data.get("canaries").and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|t| t.as_str().map(|s| s.to_string())).collect()),
        })
    }

    pub async fn load_parsentry_results(&self, benchmark_id: &str) -> Result<Vec<VulnerabilityResult>> {
        let results_path = self.output_dir.join(format!("{}.json", benchmark_id));
        
        if !results_path.exists() {
            debug!("No results file found for {}", benchmark_id);
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&results_path).await
            .with_context(|| format!("Failed to read results for {}", benchmark_id))?;
        
        let data: serde_json::Value = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse results JSON for {}", benchmark_id))?;
        
        let mut results = Vec::new();
        
        if let Some(vulnerabilities) = data.get("vulnerabilities").and_then(|v| v.as_array()) {
            for vuln in vulnerabilities {
                results.push(VulnerabilityResult {
                    vulnerability_type: vuln.get("vulnerability_type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    confidence: vuln.get("confidence")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0),
                    file_path: vuln.get("file_path")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    line_number: vuln.get("line_number")
                        .and_then(|v| v.as_u64())
                        .map(|n| n as u32),
                    description: vuln.get("description")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                });
            }
        }
        
        Ok(results)
    }

    pub async fn evaluate_benchmark(&self, benchmark_id: &str) -> Result<BenchmarkResult> {
        let start_time = SystemTime::now();
        
        let metadata = self.load_benchmark_metadata(benchmark_id).await?;
        let parsentry_results = self.load_parsentry_results(benchmark_id).await?;
        
        let detected = !parsentry_results.is_empty();
        let mut correct_type = false;
        let mut max_confidence: f64 = 0.0;
        
        if detected {
            for result in &parsentry_results {
                max_confidence = max_confidence.max(result.confidence);
                
                // Check if detected vulnerability type matches benchmark tags
                for tag in &metadata.tags {
                    if let Some(expected_types) = self.vulnerability_mappings.get(tag) {
                        if expected_types.iter().any(|t| t.eq_ignore_ascii_case(&result.vulnerability_type)) {
                            correct_type = true;
                            break;
                        }
                    }
                }
                
                if correct_type {
                    break;
                }
            }
        }
        
        let execution_time = start_time.elapsed().unwrap_or(Duration::ZERO).as_millis() as u64;
        
        Ok(BenchmarkResult {
            benchmark_id: benchmark_id.to_string(),
            detected,
            correct_type,
            confidence_score: max_confidence,
            metadata,
            vulnerabilities_found: parsentry_results,
            execution_time_ms: execution_time,
        })
    }

    pub async fn run_full_benchmark(&self) -> Result<OverallScore> {
        info!("Starting benchmark evaluation");
        
        let benchmark_ids = self.discover_benchmarks().await?;
        let mut benchmark_results = Vec::new();
        
        for benchmark_id in benchmark_ids {
            info!("Evaluating benchmark: {}", benchmark_id);
            
            match self.evaluate_benchmark(&benchmark_id).await {
                Ok(result) => {
                    debug!("Benchmark {} - Detected: {}, Correct: {}, Confidence: {:.1}", 
                           result.benchmark_id, result.detected, result.correct_type, result.confidence_score);
                    benchmark_results.push(result);
                }
                Err(e) => {
                    error!("Failed to evaluate benchmark {}: {}", benchmark_id, e);
                }
            }
        }
        
        let overall_score = self.calculate_overall_score(benchmark_results)?;
        info!("Benchmark evaluation completed. Detection rate: {:.1}%, Accuracy rate: {:.1}%", 
              overall_score.detection_rate * 100.0, overall_score.accuracy_rate * 100.0);
        
        Ok(overall_score)
    }

    fn calculate_overall_score(&self, benchmark_results: Vec<BenchmarkResult>) -> Result<OverallScore> {
        let total = benchmark_results.len();
        let detected = benchmark_results.iter().filter(|r| r.detected).count();
        let correct_type = benchmark_results.iter().filter(|r| r.correct_type).count();
        
        let detection_rate = if total > 0 { detected as f64 / total as f64 } else { 0.0 };
        let accuracy_rate = if total > 0 { correct_type as f64 / total as f64 } else { 0.0 };
        
        // Calculate average confidence
        let confidences: Vec<f64> = benchmark_results.iter()
            .filter(|r| r.detected)
            .map(|r| r.confidence_score)
            .collect();
        let avg_confidence = if !confidences.is_empty() {
            confidences.iter().sum::<f64>() / confidences.len() as f64
        } else {
            0.0
        };
        
        // Calculate average execution time
        let avg_execution_time_ms = if !benchmark_results.is_empty() {
            benchmark_results.iter().map(|r| r.execution_time_ms as f64).sum::<f64>() / benchmark_results.len() as f64
        } else {
            0.0
        };
        
        // Group by level
        let mut scores_by_level = HashMap::new();
        for level in 1..=3 {
            let level_results: Vec<_> = benchmark_results.iter()
                .filter(|r| r.metadata.level == level)
                .collect();
            
            if !level_results.is_empty() {
                let level_detected = level_results.iter().filter(|r| r.detected).count();
                let level_correct = level_results.iter().filter(|r| r.correct_type).count();
                let level_confidences: Vec<f64> = level_results.iter()
                    .filter(|r| r.detected)
                    .map(|r| r.confidence_score)
                    .collect();
                let level_avg_confidence = if !level_confidences.is_empty() {
                    level_confidences.iter().sum::<f64>() / level_confidences.len() as f64
                } else {
                    0.0
                };
                let level_avg_time = level_results.iter().map(|r| r.execution_time_ms as f64).sum::<f64>() / level_results.len() as f64;
                
                scores_by_level.insert(level, LevelScore {
                    total: level_results.len(),
                    detected: level_detected,
                    correct_type: level_correct,
                    detection_rate: level_detected as f64 / level_results.len() as f64,
                    accuracy_rate: level_correct as f64 / level_results.len() as f64,
                    avg_confidence: level_avg_confidence,
                    avg_execution_time_ms: level_avg_time,
                });
            }
        }
        
        // Group by tags
        let mut scores_by_tag = HashMap::new();
        let mut all_tags = std::collections::HashSet::new();
        
        for result in &benchmark_results {
            for tag in &result.metadata.tags {
                all_tags.insert(tag.clone());
            }
        }
        
        for tag in all_tags {
            let tag_results: Vec<_> = benchmark_results.iter()
                .filter(|r| r.metadata.tags.contains(&tag))
                .collect();
            
            if !tag_results.is_empty() {
                let tag_detected = tag_results.iter().filter(|r| r.detected).count();
                let tag_correct = tag_results.iter().filter(|r| r.correct_type).count();
                let tag_confidences: Vec<f64> = tag_results.iter()
                    .filter(|r| r.detected)
                    .map(|r| r.confidence_score)
                    .collect();
                let tag_avg_confidence = if !tag_confidences.is_empty() {
                    tag_confidences.iter().sum::<f64>() / tag_confidences.len() as f64
                } else {
                    0.0
                };
                
                scores_by_tag.insert(tag, TagScore {
                    total: tag_results.len(),
                    detected: tag_detected,
                    correct_type: tag_correct,
                    detection_rate: tag_detected as f64 / tag_results.len() as f64,
                    accuracy_rate: tag_correct as f64 / tag_results.len() as f64,
                    avg_confidence: tag_avg_confidence,
                });
            }
        }
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Ok(OverallScore {
            timestamp: chrono::DateTime::from_timestamp(timestamp as i64, 0)
                .unwrap()
                .to_rfc3339(),
            total_benchmarks: total,
            detected_count: detected,
            correct_type_count: correct_type,
            detection_rate,
            accuracy_rate,
            avg_confidence,
            avg_execution_time_ms,
            scores_by_level,
            scores_by_tag,
            detailed_results: benchmark_results,
        })
    }

    pub async fn save_results(&self, score: &OverallScore, output_path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(score)
            .context("Failed to serialize results to JSON")?;
        
        fs::write(output_path, json).await
            .context("Failed to write results file")?;
        
        info!("Results saved to: {:?}", output_path);
        Ok(())
    }

    pub fn print_summary(&self, score: &OverallScore) {
        println!("\n=== Parsentry Benchmark Results ===");
        println!("Total Benchmarks: {}", score.total_benchmarks);
        println!("Detection Rate: {:.1}%", score.detection_rate * 100.0);
        println!("Accuracy Rate: {:.1}%", score.accuracy_rate * 100.0);
        println!("Average Confidence: {:.1}", score.avg_confidence);
        println!("Average Execution Time: {:.1}ms", score.avg_execution_time_ms);
        
        println!("\n=== By Difficulty Level ===");
        for level in 1..=3 {
            if let Some(level_score) = score.scores_by_level.get(&level) {
                println!("Level {}: {:.1}% detection, {:.1}% accuracy ({} benchmarks)", 
                         level, 
                         level_score.detection_rate * 100.0, 
                         level_score.accuracy_rate * 100.0, 
                         level_score.total);
            }
        }
        
        println!("\n=== Top Vulnerability Types ===");
        let mut sorted_tags: Vec<_> = score.scores_by_tag.iter().collect();
        sorted_tags.sort_by(|a, b| b.1.total.cmp(&a.1.total));
        
        for (tag, tag_score) in sorted_tags.iter().take(10) {
            println!("{}: {:.1}% detection, {:.1}% accuracy ({} benchmarks)", 
                     tag, 
                     tag_score.detection_rate * 100.0, 
                     tag_score.accuracy_rate * 100.0, 
                     tag_score.total);
        }
    }

    pub async fn run_parsentry_analysis(&self, repo_url: &str, model: &str, verbosity: Option<&str>) -> Result<()> {
        info!("Running Parsentry analysis on: {}", repo_url);
        
        // Ensure output directory exists
        fs::create_dir_all(&self.output_dir).await
            .context("Failed to create output directory")?;
        
        let mut cmd = Command::new("cargo");
        cmd.args(&["run", "--release", "--"])
            .args(&["--repo", repo_url])
            .args(&["--output-dir", self.output_dir.to_str().unwrap()])
            .args(&["--model", model]);
            
        if let Some(v) = verbosity {
            cmd.arg(v);
        }
        
        info!("Executing: {:?}", cmd);
        
        let output = cmd.output().await
            .context("Failed to execute Parsentry")?;
            
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("Parsentry execution failed: {}", stderr);
            return Err(anyhow::anyhow!("Parsentry execution failed"));
        }
        
        info!("Parsentry analysis completed successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_vulnerability_mappings() {
        let temp_dir = TempDir::new().unwrap();
        let runner = BenchmarkRunner::new(temp_dir.path().to_path_buf(), temp_dir.path().to_path_buf());
        
        assert!(runner.vulnerability_mappings.contains_key("xss"));
        assert!(runner.vulnerability_mappings.get("xss").unwrap().contains(&"XSS".to_string()));
    }
    
    #[tokio::test]
    async fn test_benchmark_discovery() {
        let temp_dir = TempDir::new().unwrap();
        let benchmarks_dir = temp_dir.path().join("benchmarks");
        fs::create_dir_all(&benchmarks_dir).await.unwrap();
        
        // Create test benchmark directory
        let test_benchmark = benchmarks_dir.join("XBEN-001-24");
        fs::create_dir_all(&test_benchmark).await.unwrap();
        fs::write(test_benchmark.join("benchmark.json"), r#"{"name": "test"}"#).await.unwrap();
        
        let runner = BenchmarkRunner::new(benchmarks_dir, temp_dir.path().to_path_buf());
        let benchmarks = runner.discover_benchmarks().await.unwrap();
        
        assert_eq!(benchmarks.len(), 1);
        assert_eq!(benchmarks[0], "XBEN-001-24");
    }
}