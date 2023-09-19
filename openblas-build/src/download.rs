use anyhow::Result;
use std::path::{Path, PathBuf};

const OPENBLAS_VERSION: &str = "0.3.21";

pub fn openblas_source_url() -> String {
    format!(
        "https://github.com/xianyi/OpenBLAS/releases/download/v{}/OpenBLAS-{}.tar.gz",
        OPENBLAS_VERSION, OPENBLAS_VERSION
    )
}

pub fn download(out_dir: &Path) -> Result<PathBuf> {
    let dest = out_dir.join(format!("OpenBLAS-{}", OPENBLAS_VERSION));
    if !dest.exists() {
        let buf = get_agent()
            .get(&openblas_source_url())
            .call()?
            .into_reader();
        let gz_stream = flate2::read::GzDecoder::new(buf);
        let mut ar = tar::Archive::new(gz_stream);
        ar.unpack(out_dir)?;
        assert!(dest.exists());
    }
    Ok(dest)
}

fn try_proxy_from_env() -> Option<ureq::Proxy> {
    macro_rules! try_env {
            ($($env:literal),+) => {
                $(
                    if let Ok(env) = std::env::var($env) {
                        if let Ok(proxy) = ureq::Proxy::new(env) {
                            return Some(proxy);
                        }
                    }
                )+
            };
        }

    try_env!(
        "ALL_PROXY",
        "all_proxy",
        "HTTPS_PROXY",
        "https_proxy",
        "HTTP_PROXY",
        "http_proxy"
    );
    None
}

fn get_agent() -> ureq::Agent {
    let mut agent_builder = ureq::AgentBuilder::new().tls_connector(std::sync::Arc::new(
        native_tls::TlsConnector::new().expect("failed to create TLS connector"),
    ));
    if let Some(proxy) = try_proxy_from_env() {
        agent_builder = agent_builder.proxy(proxy);
    }
    agent_builder.build()
}
