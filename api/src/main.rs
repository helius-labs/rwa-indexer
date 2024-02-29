use cadence_macros::statsd_time;
use hyper::Method;
use jsonrpsee::server::{
    logger::{Logger, TransportProtocol},
    middleware::proxy_get_request::ProxyGetRequestLayer,
    ServerBuilder,
};
use log::debug;
use std::time::Instant;
use tower_http::cors::{Any, CorsLayer};
use {
    rwa_api::api::RwaApi,
    rwa_api::builder::RpcApiBuilder,
    rwa_api::config::load_config,
    rwa_api::error::RwaApiError,
    rwa_api::metrics::{safe_metric, setup_metrics},
    std::env,
    std::net::SocketAddr,
};

// Using jemallocator because default allocator holds onto
// memory too easily. This causes OOM when large accounts (>100MB)
// are processed. jemallocator frees up memory much more aggressively.
// jemallocator used to be default but was removed because of stability issues in Windows and Mac OS.
// Since we are running in Linux we should be okay
// 1. https://lib.rs/crates/jemallocator
// 2. https://github.com/rust-lang/rfcs/blob/master/text/1974-global-allocators.md#jemalloc
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[derive(Clone)]
struct MetricMiddleware;

impl Logger for MetricMiddleware {
    type Instant = Instant;

    fn on_request(&self, _t: TransportProtocol) -> Self::Instant {
        Instant::now()
    }

    fn on_result(
        &self,
        name: &str,
        success: bool,
        started_at: Self::Instant,
        _t: TransportProtocol,
    ) {
        let stat = match success {
            true => "success",
            false => "failure",
        };
        debug!(
            "Call to '{}' {} took {:?}",
            name,
            stat,
            started_at.elapsed()
        );
        safe_metric(|| {
            let success = success.to_string();
            statsd_time!("api_call", started_at.elapsed(), "method" => name, "success" => &success);
        });
    }

    fn on_connect(
        &self,
        remote_addr: SocketAddr,
        _request: &jsonrpsee::server::logger::HttpRequest,
        _t: TransportProtocol,
    ) {
        debug!("Connecting from {}", remote_addr)
    }

    fn on_call(
        &self,
        method_name: &str,
        params: jsonrpsee::types::Params,
        _kind: jsonrpsee::server::logger::MethodKind,
        _transport: TransportProtocol,
    ) {
        debug!("Call: {} {:?}", method_name, params);
    }

    fn on_response(&self, result: &str, _started_at: Self::Instant, _transport: TransportProtocol) {
        debug!("Response: {}", result);
    }

    fn on_disconnect(&self, remote_addr: SocketAddr, _transport: TransportProtocol) {
        debug!("Disconnecting from {}", remote_addr);
    }
}

#[tokio::main]
async fn main() -> Result<(), RwaApiError> {
    env::set_var(
        env_logger::DEFAULT_FILTER_ENV,
        env::var_os(env_logger::DEFAULT_FILTER_ENV)
            .unwrap_or_else(|| "info,sqlx::query=warn,jsonrpsee_server::server=warn".into()),
    );
    env_logger::init();
    let config = load_config();
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    let cors = CorsLayer::new()
        .allow_methods([Method::POST, Method::GET])
        .allow_origin(Any)
        .allow_headers([hyper::header::CONTENT_TYPE]);
    setup_metrics(&config);
    let middleware = tower::ServiceBuilder::new()
        .layer(cors)
        .layer(ProxyGetRequestLayer::new("/health", "healthz")?)
        .layer(ProxyGetRequestLayer::new("/liveness", "liveness")?)
        .layer(ProxyGetRequestLayer::new("/readiness", "readiness")?);

    let server = ServerBuilder::default()
        .set_middleware(middleware)
        .set_logger(MetricMiddleware)
        .build(addr)
        .await?;

    let api = RwaApi::from_config(config).await?;
    let rpc = RpcApiBuilder::build(Box::new(api))?;
    println!("Server Started");
    let server_handle = server.start(rpc)?;

    match tokio::signal::ctrl_c().await {
        Ok(()) => {
            println!("Shutting down server");
            server_handle.stop()?;
        }

        Err(err) => {
            println!("Unable to listen for shutdown signal: {}", err);
        }
    }
    tokio::spawn(server_handle.stopped());
    println!("Server ended");
    Ok(())
}
