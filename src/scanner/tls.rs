use super::TlsResult;
use anyhow::Result;
use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::{ClientConfig, DigitallySignedStruct, SignatureScheme};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;

/// A certificate verifier that accepts all certificates.
/// We are scanning for PQC support, not validating trust chains.
#[derive(Debug)]
struct NoVerifier;

impl ServerCertVerifier for NoVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> std::result::Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> std::result::Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> std::result::Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        vec![
            SignatureScheme::RSA_PKCS1_SHA256,
            SignatureScheme::RSA_PKCS1_SHA384,
            SignatureScheme::RSA_PKCS1_SHA512,
            SignatureScheme::ECDSA_NISTP256_SHA256,
            SignatureScheme::ECDSA_NISTP384_SHA384,
            SignatureScheme::ECDSA_NISTP521_SHA512,
            SignatureScheme::RSA_PSS_SHA256,
            SignatureScheme::RSA_PSS_SHA384,
            SignatureScheme::RSA_PSS_SHA512,
            SignatureScheme::ED25519,
            SignatureScheme::ED448,
        ]
    }
}

pub async fn scan_tls(host: &str, port: u16, timeout: Duration) -> Result<TlsResult> {
    let provider = rustls::crypto::aws_lc_rs::default_provider();

    let config = ClientConfig::builder_with_provider(Arc::new(provider))
        .with_safe_default_protocol_versions()?
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(NoVerifier))
        .with_no_client_auth();

    let connector = TlsConnector::from(Arc::new(config));
    let server_name = ServerName::try_from(host.to_string())?;

    let addr = format!("{}:{}", host, port);
    let tcp_stream = tokio::time::timeout(timeout, TcpStream::connect(&addr)).await??;

    let tls_stream = tokio::time::timeout(timeout, connector.connect(server_name, tcp_stream)).await??;

    let (_, conn) = tls_stream.get_ref();

    let tls_version = conn.protocol_version().map(|v| format!("{:?}", v));

    let cipher_suite = conn
        .negotiated_cipher_suite()
        .map(|cs| format!("{:?}", cs.suite()));

    let key_exchange = conn
        .negotiated_key_exchange_group()
        .map(|kex| format!("{:?}", kex.name()));

    let pqc_supported = key_exchange
        .as_ref()
        .map(|kex| {
            let kex_lower = kex.to_lowercase();
            kex_lower.contains("mlkem")
                || kex_lower.contains("kyber")
                || kex_lower.contains("x25519mlkem")
                || kex_lower.contains("secp256r1mlkem")
        })
        .unwrap_or(false);

    let pqc_algorithms = if pqc_supported {
        vec![key_exchange.clone().unwrap_or_default()]
    } else {
        vec![]
    };

    Ok(TlsResult {
        connected: true,
        tls_version,
        cipher_suite,
        key_exchange,
        pqc_supported,
        pqc_algorithms,
        offered_kex: vec![],
    })
}
