use axum::{routing::get, Router};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use once_cell::sync::Lazy;

static PROMETHEUS_HANDLE: Lazy<PrometheusHandle> = Lazy::new(|| {
    let builder = PrometheusBuilder::new();
    builder.install_recorder().expect("failed to install Prometheus recorder")
});

pub fn init_metrics() {
    // Just force the initialization of the lazy static
    let _ = PROMETHEUS_HANDLE.render();
}

pub fn metrics_routes() -> Router {
    Router::new().route("/metrics", get(metrics_handler))
}

async fn metrics_handler() -> String {
    PROMETHEUS_HANDLE.render()
}
