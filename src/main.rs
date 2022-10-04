use std::{net::SocketAddr, str::FromStr};
use warp::Filter;

mod deck;

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

    let deck = warp::path!("deck" / String).map(deck_service).with(cors);

    warp::serve(deck).run(SocketAddr::from_str(ADDR)?).await;

    Ok(())
}

fn deck_service(param: String) -> String {
    let deck = deck::Deck::from_path(format!("deck/{}", param)).unwrap_or_default();
    let json = serde_json::to_string(&deck).unwrap_or_default();
    json
}
