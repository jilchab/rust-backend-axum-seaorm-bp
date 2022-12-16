mod api;
mod context;
mod response;
mod routes;

#[tokio::main(flavor = "current_thread")]
pub async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    api::start_server().await
}
