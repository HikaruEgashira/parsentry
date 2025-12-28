//! Cache management commands

use anyhow::Result;
use parsentry_cache::Cache;
use std::io::{self, Write};

use crate::cli::args::CacheAction;
use crate::config::ParsentryConfig;

/// Execute cache command
pub async fn handle_cache_command(action: &CacheAction, config: &ParsentryConfig) -> Result<()> {
    let cache_dir = &config.cache.directory;

    match action {
        CacheAction::Clean {
            all,
            max_age,
            max_idle,
            dry_run,
        } => {
            println!("🧹 Cache cleanup");
            println!("   Directory: {}", cache_dir.display());

            let mut policy = config.cache.to_cleanup_policy();

            // Override policy if specified
            if let Some(age) = max_age {
                policy.max_age_days = *age;
            }
            if let Some(idle) = max_idle {
                policy.max_idle_days = *idle;
            }

            let cache = Cache::with_cleanup_config(
                cache_dir,
                policy.clone(),
                config.cache.to_cleanup_trigger(),
            )?;

            if *dry_run {
                println!("   [Dry run - no files will be deleted]");
            }

            if *all {
                // Clear all
                println!("\n⚠️  Warning: This will delete ALL cache entries");

                if !dry_run {
                    print!("   Continue? [y/N]: ");
                    io::stdout().flush()?;

                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;

                    if !input.trim().eq_ignore_ascii_case("y") {
                        println!("   Aborted");
                        return Ok(());
                    }

                    let removed = cache.clear_all()?;
                    println!("✅ Removed {} entries", removed);
                } else {
                    let stats = cache.stats()?;
                    println!("   Would remove {} entries ({} MB)", stats.total_entries, stats.total_size_mb);
                }
            } else {
                // Clean stale only
                println!("\nPolicy:");
                println!("   Max age: {} days", policy.max_age_days);
                println!("   Max idle: {} days", policy.max_idle_days);
                println!("   Remove version mismatch: {}", policy.remove_version_mismatch);

                if !dry_run {
                    let stats = cache.cleanup_stale()?;
                    println!("\n✅ Cleanup complete");
                    println!("   Removed {} stale entries", stats.removed_count);
                    println!("   Freed {} MB", stats.freed_bytes / 1_048_576);
                } else {
                    // TODO: Implement dry-run mode that collects stats without deleting
                    println!("\n   (Dry run mode - would execute cleanup with above policy)");
                }
            }
        }

        CacheAction::Stats => {
            println!("📊 Cache statistics");
            println!("   Directory: {}", cache_dir.display());

            let cache = Cache::new(cache_dir)?;
            let stats = cache.stats()?;

            println!("\nSize:");
            println!("   Total entries: {}", stats.total_entries);
            println!("   Total size: {} MB ({} bytes)", stats.total_size_mb, stats.total_size_bytes);

            println!("\nConfiguration:");
            println!("   Max cache size: {} MB", config.cache.cleanup.size_limit_mb.unwrap_or(500));
            println!("   Max age: {} days", config.cache.cleanup.max_age_days);
            println!("   Max idle: {} days", config.cache.cleanup.max_idle_days);
            println!("   Cleanup trigger: {}", config.cache.cleanup.trigger);

            if let Some(periodic) = config.cache.cleanup.periodic_days {
                println!("   Periodic cleanup: every {} days", periodic);
            }

            // Show status
            if stats.total_entries == 0 {
                println!("\n💡 Cache is empty");
            } else {
                let limit_mb = config.cache.cleanup.size_limit_mb.unwrap_or(500);
                let usage_pct = (stats.total_size_mb as f64 / limit_mb as f64) * 100.0;
                println!("\n📈 Usage: {:.1}% of limit", usage_pct);

                if usage_pct > 90.0 {
                    println!("   ⚠️  Cache is nearly full - consider running cleanup");
                } else if usage_pct > 75.0 {
                    println!("   ⚠️  Cache usage is high");
                }
            }
        }

        CacheAction::Clear { yes } => {
            println!("🗑️  Clear all cache");
            println!("   Directory: {}", cache_dir.display());

            let cache = Cache::new(cache_dir)?;
            let stats = cache.stats()?;

            println!("\n⚠️  Warning: This will delete ALL {} cache entries ({} MB)",
                     stats.total_entries, stats.total_size_mb);

            if !yes {
                print!("   Continue? [y/N]: ");
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                if !input.trim().eq_ignore_ascii_case("y") {
                    println!("   Aborted");
                    return Ok(());
                }
            }

            let removed = cache.clear_all()?;
            println!("\n✅ Removed {} entries", removed);
        }
    }

    Ok(())
}
