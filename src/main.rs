// Copyright 2023 Tricked <tricked@tricked.dev>

use axum::{
    extract::State,
    http::{Request, Response},
    routing::get,
    Router,
};
use clap::Parser;
use hyper::{client::HttpConnector, Body, Method, StatusCode};
use hyper_tls::HttpsConnector;
use once_cell::sync::Lazy;
use ratelimit::HySmartLimiter;
use redis::Commands;
use response::ResultResponse;
use std::time::{Duration, Instant};

type Client = hyper::client::Client<HttpsConnector<HttpConnector>, Body>;

mod ratelimit;
mod response;

#[derive(Clone)]
pub struct HyState {
    client: Client,
    redis: redis::Client,
    limiter: HySmartLimiter,
}

#[derive(Parser)]
pub struct Cli {
    #[clap(short, long, env)]
    api_key: String,
    #[clap(short, long, env, default_value = "60")]
    key_limit: u32,
    #[clap(short, long, env, default_value = "redis://127.0.0.1/")]
    redis_url: String,
    #[clap(short, long, env, default_value = "0.0.0.0")]
    bind_addr: String,
    #[clap(short = 'p', long, env, default_value = "4000")]
    bind_port: u16,
    #[clap(short = 'u', long, env, default_value = "https://api.hypixel.net")]
    hypixel_api: String,
}

static CLI: Lazy<Cli> = Lazy::new(Cli::parse);
static HYPIXEL_API_DOMAIN: Lazy<String> = Lazy::new(|| CLI.hypixel_api.clone());
static HYPIXEL_API_KEY: Lazy<String> = Lazy::new(|| CLI.api_key.clone());
static HYPIXEL_API_LIMIT: Lazy<u32> = Lazy::new(|| CLI.key_limit);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::SubscriberBuilder::default()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let cli = Cli::parse();

    let https = HttpsConnector::new();
    let client: Client = hyper::Client::builder().build::<_, hyper::Body>(https);
    let redis = redis::Client::open(cli.redis_url)?;

    let app = Router::new().route("/*all", get(handler)).with_state(HyState {
        client,
        redis,
        limiter: HySmartLimiter::new(60),
    });
    let addr = format!("{}:{}", cli.bind_addr, cli.bind_port).parse()?;
    println!("reverse proxy listening on {addr}",);
    axum::Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}

async fn handler(State(state): State<HyState>, req: Request<Body>) -> ResultResponse<Response<Body>> {
    if req.method() != Method::GET {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())?);
    }
    //  else if path == "/"
    //     || path.starts_with("/resources")
    //     || path.starts_with("/skyblock/auctions")
    //     || path.starts_with("/skyblock/bazaar")
    //     || path.starts_with("/skyblock/auctions_ended")
    //     || path.starts_with("/skyblock/news")
    // {
    //     return Ok(Response::builder().status(StatusCode::FORBIDDEN).body(Body::empty())?);
    // }

    let mut conn = state.redis.get_connection()?;

    let full_path = req
        .uri()
        .path_and_query()
        .map(|x| x.to_string())
        .unwrap_or_else(|| req.uri().path().to_owned());

    let recent: redis::RedisResult<u8> = conn.get(format!("recent:{full_path}"));
    if state.limiter.remaining.load(std::sync::atomic::Ordering::Relaxed) == 0 || recent.is_ok() {
        let res: redis::RedisResult<String> = conn.get(&full_path);
        tracing::debug!("recent: {}", recent.is_ok());
        if let Ok(res) = res {
            tracing::debug!("cache hit!");
            return Ok(Response::builder()
                .status(StatusCode::OK)
                .header("x-cache-hit", "true")
                .body(Body::from(res))?);
        } else {
            tracing::debug!("cache miss!");
            return Ok(Response::builder()
                .status(StatusCode::TOO_MANY_REQUESTS)
                .header("x-cache-miss", "true")
                .body(Body::empty())?);
        }
    }

    let url = HYPIXEL_API_DOMAIN.clone();

    let res = state
        .client
        .request(
            Request::builder()
                .uri(format!("{url}{full_path}"))
                .header("api-key", HYPIXEL_API_KEY.as_str())
                .body(Body::empty())?,
        )
        .await?;

    let headers = res.headers().clone();

    let get_header_u64 = |name: &str| {
        if let Some(val) = headers.get(name) {
            if let Ok(val) = val.to_str() {
                if let Ok(val) = val.parse::<u64>() {
                    return val;
                }
            }
        }
        1
    };

    let remaining = get_header_u64("ratelimit-remaining");
    let reset = get_header_u64("ratelimit-reset");

    state.limiter.set_remaining(remaining);

    let new_reset = Instant::now() + Duration::from_secs(reset);
    if *state.limiter.reset.lock() < new_reset {
        *state.limiter.reset.lock() = new_reset;
    }
    tracing::debug!("ratelimit remaining: {remaining}");
    if remaining == 0 {
        println!("ratelimit hit! spawning reset task...");
        tokio::spawn(async move {
            let now = Instant::now();
            let reset = *state.limiter.reset.lock();
            tracing::debug!("sleeping for {:?}...", reset - now);
            if now < reset {
                tokio::time::sleep(reset - now).await;
            }
            tracing::debug!("resetting ratelimit!");
            state
                .limiter
                .remaining
                .store(*HYPIXEL_API_LIMIT as u64, std::sync::atomic::Ordering::Relaxed);
        });
    }
    tracing::debug!("status: {}", res.status());
    if res.status() == 429 {
        return Ok(Response::builder()
            .status(StatusCode::TOO_MANY_REQUESTS)
            .body(Body::empty())?);
    }
    let body = hyper::body::to_bytes(res.into_body()).await?;
    let _: bool = conn.set_ex(&full_path, body.to_vec(), 21600)?;
    let _: bool = conn.set_ex(format!("recent:{full_path}"), true, 120)?;
    tracing::debug!("path: {full_path}");
    Ok(Response::new(Body::from(body)))
}
