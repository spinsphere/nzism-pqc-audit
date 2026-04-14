pub mod cdn;
pub mod tls;

use crate::input::Target;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::sync::Semaphore;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub entity_name: String,
    pub sector: String,
    pub sub_sector: String,
    pub domain: String,
    pub host: String,
    pub port: u16,
    pub tls: Option<TlsResult>,
    pub cdn: Option<CdnResult>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsResult {
    pub connected: bool,
    pub tls_version: Option<String>,
    pub cipher_suite: Option<String>,
    pub key_exchange: Option<String>,
    pub pqc_supported: bool,
    pub pqc_algorithms: Vec<String>,
    pub offered_kex: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdnResult {
    pub detected: bool,
    pub provider: Option<String>,
    pub note: String,
}

pub async fn run_scan(
    targets: &[Target],
    timeout: Duration,
    concurrency: usize,
) -> Result<Vec<ScanResult>> {
    let semaphore = Arc::new(Semaphore::new(concurrency));
    let mut handles = Vec::new();

    for target in targets {
        let sem = semaphore.clone();
        let target = target.clone();

        let handle = tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            scan_target(&target, timeout).await
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        match handle.await {
            Ok(result) => results.push(result),
            Err(e) => eprintln!("Task join error: {}", e),
        }
    }

    Ok(results)
}

async fn scan_target(target: &Target, timeout: Duration) -> ScanResult {
    let host = &target.host;
    let port = target.port;

    eprintln!("Scanning {}:{} ({})", host, port, target.entity_name);

    let tls_result = match tls::scan_tls(host, port, timeout).await {
        Ok(r) => Some(r),
        Err(e) => {
            eprintln!("  TLS error for {}:{}: {}", host, port, e);
            return ScanResult {
                entity_name: target.entity_name.clone(),
                sector: target.sector.clone(),
                sub_sector: target.sub_sector.clone(),
                domain: target.domain.clone(),
                host: host.clone(),
                port,
                tls: None,
                cdn: None,
                error: Some(format!("TLS scan failed: {}", e)),
            };
        }
    };

    let cdn_result = match cdn::detect_cdn(host, timeout).await {
        Ok(r) => Some(r),
        Err(e) => {
            eprintln!("  CDN detection error for {}: {}", host, e);
            None
        }
    };

    ScanResult {
        entity_name: target.entity_name.clone(),
        sector: target.sector.clone(),
        sub_sector: target.sub_sector.clone(),
        domain: target.domain.clone(),
        host: host.clone(),
        port,
        tls: tls_result,
        cdn: cdn_result,
        error: None,
    }
}
