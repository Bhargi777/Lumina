mod api;
mod config;
mod error;
mod metrics;
mod proxy;

use axum::{
    body::Body,
    extract::{Path, Request, State},
    response::IntoResponse,
    routing::{any, get},
    Router,
};
use clap::Parser;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::config::AppConfig;
use crate::proxy::handler::{handle_proxy, ProxyState};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "config.yaml")]
    config: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Setup observability
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // 2. Parse command arguments & load configuration
    let args = Args::parse();
    info!("Loading config from {}", args.config);
    let app_config = AppConfig::load(&args.config)
        .unwrap_or_else(|e| panic!("Failed to load configuration: {}", e));

    // 3. Setup Prometheus Metrics
    metrics::init_metrics();

    // 4. Initialize HTTP client for upstreams
    let client = reqwest::Client::builder()
        .build()
        .expect("Failed to build HTTP client");

    // 5. Build routing map
    let mut upstreams = HashMap::new();
    for route_config in app_config.routes {
        upstreams.insert(route_config.path.clone(), route_config.upstream.clone());
        info!("Mapped route /{} to {}", route_config.path, route_config.upstream);
    }

    let proxy_state = Arc::new(ProxyState { client, upstreams });

    // 6. Build Axum App Router
    // We add health and metrics API routers, then dynamic routing for upstreams
    let router_with_state = Router::new()
        .route("/api/:route/*path", any(handle_proxy_wrapper))
        .with_state(proxy_state);

    let app = Router::new()
        .merge(api::health::health_routes())
        .merge(metrics::metrics_routes())
        .merge(router_with_state)
        .layer(TraceLayer::new_for_http());

    // 7. Bind & Serve
    let addr = SocketAddr::from((
        app_config.server.host.parse::<std::net::IpAddr>().unwrap(),
        app_config.server.port,
    ));
    info!("Lumina API Gateway running at http://{}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn handle_proxy_wrapper(
    State(state): State<Arc<ProxyState>>,
    Path((route, path_remainder)): Path<(String, String)>,
    req: Request<Body>,
) -> impl IntoResponse {
    // We rebuild a path string because our handler signature expects it.
    // In a full production proxy, we might do more advanced re-writing here.
    let path = format!("/api/{}/{}", route, path_remainder);
    
    match crate::proxy::handler::handle_proxy(State(state), Path(route.clone()), req).await {
        Ok(resp) => resp,
        Err(e) => e.into_response(),
    }
}
