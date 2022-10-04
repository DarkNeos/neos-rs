use std::{net::SocketAddr, str::FromStr};
use warp::Filter;

const ADDR: &str = "127.0.0.1:3000";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::try_init()?;

    let hello = warp::path!("hello" / String).map(|name| format!("hello, {}", name));

    warp::serve(hello).run(SocketAddr::from_str(ADDR)?).await;

    Ok(())
}
