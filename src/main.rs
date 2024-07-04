use std::net::SocketAddr;

use bytes::Bytes;
use clap::Parser;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::Incoming;
use hyper::header::HeaderValue;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::HeaderMap;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use serde_json::Value;
use std::collections::HashMap;
use std::time::SystemTime;
use tokio::net::TcpListener;
use tracing::metadata::LevelFilter;
use tracing_appender::non_blocking::{NonBlockingBuilder, WorkerGuard};
use tracing_appender::rolling;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;
#[macro_use]
extern crate tracing;
#[macro_use]
extern crate serde_json;
use tracing_subscriber::{fmt, layer::SubscriberExt};

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Cli {
    /// The http port,default port is 9345
    #[arg(
        default_value_t = 9345,
        short = 'P',
        long = "port",
        value_name = "Port"
    )]
    http_port: u32,
}

fn convert(headers: &HeaderMap<HeaderValue>) -> HashMap<String, String> {
    let mut header_hashmap = HashMap::new();
    for (k, v) in headers {
        let k = k.as_str().to_owned();
        let v = String::from_utf8_lossy(v.as_bytes()).into_owned();
        header_hashmap.entry(k).or_insert_with(|| v);
    }
    header_hashmap
}
#[instrument]
async fn echo(
    req: Request<hyper::body::Incoming>,
    remote_ip: String,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let (first, second) = req.into_parts();
    let body_bytes = second.collect().await?.to_bytes();
    let uri = first.clone().uri;
    let hash_map = convert(&first.clone().headers);
    let result_value = json!({
        "headers":  hash_map,
        "path": uri.to_string(),
        "origin":remote_ip,
        "method":first.clone().method.to_string(),
        "version":format!("{:?}",first.clone().version),
        "body":String::from_utf8_lossy(&body_bytes).to_string(),
    });

    info!("remote_ip:{},uri:{}", remote_ip, uri);

    Ok(Response::new(full(
        serde_json::to_string_pretty(&result_value).unwrap(),
    )))
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
fn setup_logger() -> Result<WorkerGuard, anyhow::Error> {
    let app_file = rolling::daily("./logs", "access.log");
    let console_layer = tracing_subscriber::fmt::Layer::new()
        .with_target(true)
        .with_filter(tracing_subscriber::filter::LevelFilter::INFO);
    let (non_blocking_appender, guard) = NonBlockingBuilder::default()
        .buffered_lines_limit(10)
        .finish(app_file);
    let file_layer = tracing_subscriber::fmt::Layer::new()
        .with_target(true)
        .with_ansi(false)
        .with_writer(non_blocking_appender)
        .with_filter(tracing_subscriber::filter::LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(file_layer)
        .with(tracing_subscriber::filter::LevelFilter::TRACE)
        .with(console_layer)
        .init();
    Ok(guard)
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _worker_guard = setup_logger()?;
    let cli: Cli = Cli::parse();
    let port = cli.http_port;
    let addr = format!(r#"0.0.0.0:{port}"#);

    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on http://{}", addr);
    loop {
        let (stream, addr) = listener.accept().await?;
        let addr_str = addr.to_string();
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            let addr_str_cloned = addr_str.clone();
            if let Err(err) = http1::Builder::new()
                .keep_alive(true)
                .serve_connection(
                    io,
                    service_fn(move |req: Request<Incoming>| echo(req, addr_str_cloned.clone())),
                )
                .await
            {
                info!("Error serving connection: {:?},addr is:{:}", err, addr_str);
            }
        });
    }
}
