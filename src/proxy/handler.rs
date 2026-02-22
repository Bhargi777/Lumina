use axum::{
    body::Body,
    extract::{Path, Request, State},
    response::Response,
};
use reqwest::Client;
use std::sync::Arc;
use crate::error::LuminaError;

#[derive(Clone)]
pub struct ProxyState {
    pub client: Client,
    pub upstreams: std::collections::HashMap<String, String>,
}

pub async fn handle_proxy(
    State(state): State<Arc<ProxyState>>,
    Path(route): Path<String>,
    mut req: Request<Body>,
) -> Result<Response, LuminaError> {
    // Determine the upstream URL
    let upstream_base = state.upstreams.get(&route)
        .ok_or_else(|| LuminaError::InvalidRoute(format!("No upstream configured for route '{}'", route)))?;

    // We can extract simply the rest of the path from the request URI
    let path_and_query = req
        .uri()
        .path_and_query()
        .map(|pq| pq.as_str())
        .unwrap_or("");

    // Strip the /route prefix if needed
    // In our simplified version, we just append
    let upstream_url = format!("{}{}", upstream_base, path_and_query);

    // Build the upstream request
    let method = req.method().clone();
    let mut upstream_req = state.client.request(method, &upstream_url);

    // Forward headers
    for (name, value) in req.headers() {
        if name != reqwest::header::HOST && name != "host" {
            upstream_req = upstream_req.header(name.clone(), value.clone());
        }
    }

    // Forward body if any
    let body_bytes = axum::body::to_bytes(req.into_body(), usize::MAX)
        .await
        .map_err(|e| LuminaError::Internal(anyhow::anyhow!("Failed to read body: {}", e)))?;
    
    let upstream_req = upstream_req.body(reqwest::Body::from(body_bytes));

    // Send the request
    let res = upstream_req.send().await?;

    // Build the axum response
    let mut builder = Response::builder().status(res.status());
    if let Some(headers) = builder.headers_mut() {
        for (name, value) in res.headers() {
            headers.insert(name.clone(), value.clone());
        }
    }

    let out_body = Body::from_stream(res.bytes_stream());
    let response = builder.body(out_body)
        .map_err(|e| LuminaError::Internal(anyhow::anyhow!("Failed to build response: {}", e)))?;

    Ok(response)
}
