use super::CdnResult;
use anyhow::Result;
use std::time::Duration;

pub async fn detect_cdn(host: &str, timeout: Duration) -> Result<CdnResult> {
    let url = format!("https://{}/", host);

    let client = reqwest::Client::builder()
        .timeout(timeout)
        .danger_accept_invalid_certs(true)
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let resp = match client.head(&url).send().await {
        Ok(r) => r,
        Err(_) => {
            // Try GET if HEAD fails
            match client.get(&url).send().await {
                Ok(r) => r,
                Err(e) => {
                    return Ok(CdnResult {
                        detected: false,
                        provider: None,
                        note: format!("HTTP request failed: {}", e),
                    });
                }
            }
        }
    };

    let headers = resp.headers();

    // Cloudflare
    if headers.contains_key("cf-ray") || headers.contains_key("cf-cache-status") {
        return Ok(CdnResult {
            detected: true,
            provider: Some("Cloudflare".to_string()),
            note: "PQC support may reflect Cloudflare's configuration, not the origin server".to_string(),
        });
    }

    if let Some(server) = headers.get("server") {
        if let Ok(s) = server.to_str() {
            if s.to_lowercase().contains("cloudflare") {
                return Ok(CdnResult {
                    detected: true,
                    provider: Some("Cloudflare".to_string()),
                    note: "PQC support may reflect Cloudflare's configuration, not the origin server".to_string(),
                });
            }
        }
    }

    // AWS CloudFront
    if headers.contains_key("x-amz-cf-id") || headers.contains_key("x-amz-cf-pop") {
        return Ok(CdnResult {
            detected: true,
            provider: Some("AWS CloudFront".to_string()),
            note: "PQC support may reflect AWS CloudFront configuration, not the origin server".to_string(),
        });
    }

    if let Some(via) = headers.get("via") {
        if let Ok(s) = via.to_str() {
            if s.to_lowercase().contains("cloudfront") {
                return Ok(CdnResult {
                    detected: true,
                    provider: Some("AWS CloudFront".to_string()),
                    note: "PQC support may reflect AWS CloudFront configuration, not the origin server".to_string(),
                });
            }
        }
    }

    // Akamai
    if headers.contains_key("x-akamai-transformed") {
        return Ok(CdnResult {
            detected: true,
            provider: Some("Akamai".to_string()),
            note: "PQC support may reflect Akamai's configuration, not the origin server".to_string(),
        });
    }

    // Fastly
    if let Some(served_by) = headers.get("x-served-by") {
        if let Ok(s) = served_by.to_str() {
            if s.to_lowercase().contains("cache-") {
                return Ok(CdnResult {
                    detected: true,
                    provider: Some("Fastly".to_string()),
                    note: "PQC support may reflect Fastly's configuration, not the origin server".to_string(),
                });
            }
        }
    }

    // Incapsula/Imperva
    if headers.contains_key("x-iinfo") || headers.contains_key("x-cdn") {
        let provider = headers
            .get("x-cdn")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("Imperva/Incapsula");
        return Ok(CdnResult {
            detected: true,
            provider: Some(provider.to_string()),
            note: "PQC support may reflect CDN configuration, not the origin server".to_string(),
        });
    }

    Ok(CdnResult {
        detected: false,
        provider: None,
        note: "No CDN detected from HTTP headers".to_string(),
    })
}
