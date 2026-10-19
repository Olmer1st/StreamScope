//! Fabricated loopback-only SSE fixtures. No traffic or external data is replayed.
use axum::{
    body::Body,
    extract::State,
    http::{header, Response},
    routing::get,
    Router,
};
use bytes::Bytes;
use std::{convert::Infallible, time::Duration};
use tokio::{net::TcpListener, sync::watch};

/// Supported deterministic delay scripts; there is no wall-clock seeded randomness.
#[derive(Clone, Copy, Debug)]
pub enum Scenario {
    Normal,
    Gap,
    Burst,
    Truncated,
    BadUtf8,
}

/// Bind exclusively to IPv4 loopback; port zero asks the OS for an unused port.
pub async fn bind(port: u16) -> std::io::Result<TcpListener> {
    TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, port)).await
}

/// Serve local synthetic routes, draining active streams on cancellation.
pub async fn serve(
    listener: TcpListener,
    scenario: Scenario,
    mut stop: watch::Receiver<bool>,
) -> std::io::Result<()> {
    let app = Router::new()
        .route("/events", get(events))
        .route("/wrong-type", get(|| async { "not SSE" }))
        .route(
            "/unavailable",
            get(|| async {
                (
                    axum::http::StatusCode::SERVICE_UNAVAILABLE,
                    "synthetic failure",
                )
            }),
        )
        .with_state(scenario);
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            if !*stop.borrow() {
                let _ = stop.changed().await;
            }
        })
        .await
}

async fn events(State(scenario): State<Scenario>) -> Response<Body> {
    let stream = async_stream::stream! {
        for index in 0..4 {
            let delay = match scenario {
                Scenario::Burst => 0,
                Scenario::Gap if index == 2 => 4100,
                _ => 150,
            };
            tokio::time::sleep(Duration::from_millis(delay)).await;
            if matches!(scenario, Scenario::Truncated) && index == 3 { break; }
            if matches!(scenario, Scenario::BadUtf8) && index == 2 {
                yield Ok::<_, Infallible>(Bytes::from_static(b"data: \xff\n\n")); break;
            }
            let kind = if index == 3 { "done" } else { "delta" };
            let data = format!("id: {}\nevent: {kind}\ndata: synthetic {} 🦀\n\n", index + 1, index + 1);
            // Split every UTF-8 code point and line delimiter across body writes.
            for byte in data.into_bytes() { yield Ok::<_, Infallible>(Bytes::from(vec![byte])); }
        }
    };
    Response::builder()
        .header(header::CONTENT_TYPE, "text/event-stream; charset=utf-8")
        .header(header::CACHE_CONTROL, "no-cache")
        .body(Body::from_stream(stream))
        .unwrap()
}
