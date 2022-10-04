use std::{net::SocketAddr, str::FromStr};
use warp::Filter;

const ADDR: &str = "127.0.0.1:3030";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::try_init()?;

    // todo: use CORS correctly
    let cors = warp::cors()
        .allow_any_origin()
        .allow_credentials(true)
        .allow_header("content-type")
        .allow_method("GET");

    let hello = warp::path!("hello" / String)
        .map(|name| format!("hello, {}", name))
        .with(cors);

    warp::serve(hello).run(SocketAddr::from_str(ADDR)?).await;

    Ok(())
}
