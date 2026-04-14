mod input;
mod nzism;
mod report;
mod scanner;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "nzism-pqc-audit")]
#[command(about = "PQC readiness scanner for NZ critical infrastructure, mapped to NZISM Section 2.4")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan targets for PQC readiness
    Scan {
        /// Input file (CSV or text, one host:port per line)
        #[arg(short, long)]
        input: PathBuf,

        /// Input format
        #[arg(short, long, default_value = "csv")]
        format: String,

        /// Output JSON results file
        #[arg(long)]
        output_json: Option<PathBuf>,

        /// Output HTML report file
        #[arg(long)]
        output_html: Option<PathBuf>,

        /// Connection timeout in seconds
        #[arg(long, default_value = "10")]
        timeout: u64,

        /// Maximum concurrent scans
        #[arg(long, default_value = "20")]
        concurrency: usize,
    },
    /// Generate report from existing JSON results
    Report {
        /// Input JSON results file
        #[arg(short, long)]
        input: PathBuf,

        /// Output HTML report file
        #[arg(short, long)]
        output: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan {
            input,
            format,
            output_json,
            output_html,
            timeout,
            concurrency,
        } => {
            let targets = match format.as_str() {
                "csv" => input::csv::load_csv(&input)?,
                "text" => input::text::load_text(&input)?,
                _ => anyhow::bail!("Unsupported format: {}. Use 'csv' or 'text'.", format),
            };

            eprintln!("Loaded {} targets", targets.len());

            let results =
                scanner::run_scan(&targets, std::time::Duration::from_secs(timeout), concurrency)
                    .await?;

            eprintln!("Scan complete. {} results.", results.len());

            if let Some(json_path) = &output_json {
                let json = serde_json::to_string_pretty(&results)?;
                std::fs::write(json_path, json)?;
                eprintln!("JSON results written to {}", json_path.display());
            }

            if let Some(html_path) = &output_html {
                let scan_report = nzism::build_report(&results);
                report::html::write_html_report(&scan_report, html_path)?;
                eprintln!("HTML report written to {}", html_path.display());
            }

            let scan_report = nzism::build_report(&results);
            report::summary::print_summary(&scan_report);
        }
        Commands::Report { input, output } => {
            let json_data = std::fs::read_to_string(&input)?;
            let results: Vec<scanner::ScanResult> = serde_json::from_str(&json_data)?;
            let scan_report = nzism::build_report(&results);
            report::html::write_html_report(&scan_report, &output)?;
            eprintln!("HTML report written to {}", output.display());
        }
    }

    Ok(())
}
