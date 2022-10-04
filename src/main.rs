use hyper::{server as HyperServer, service as HyperService};
use tokio::net::TcpListener;

const ADDR: &str = "127.0.0.1:3000";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::try_init()?;

    let listener = TcpListener::bind(ADDR).await?;
    log::info!("Listening on https://{}", ADDR);

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            if let Err(e) = HyperServer::conn::Http::new()
                .serve_connection(stream, HyperService::service_fn(hello))
                .await
            {
                log::error!("Hyper serve_connection error={:?}", e);
            }
        });
    }
}

async fn hello(_: hyper::Request<hyper::Body>) -> anyhow::Result<hyper::Response<hyper::Body>> {
    Ok(hyper::Response::new("hello, Neos.".into()))
}
