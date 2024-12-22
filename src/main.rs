use atuin_server_sqlite_unofficial::Sqlite;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let formatting_layer = tracing_tree::HierarchicalLayer::default()
        .with_writer(tracing_subscriber::fmt::TestWriter::new())
        .with_indent_lines(true)
        .with_ansi(true)
        .with_targets(true)
        .with_indent_amount(2);

    tracing_subscriber::registry()
        .with(formatting_layer)
        .with(EnvFilter::from_default_env())
        .init();

    let settings = atuin_server::Settings::new().unwrap();
    let host = settings.host.clone(); // Host is a String
    let port = settings.port; // Port is a u16

    // Combine host and port into a SocketAddr
    let addr: SocketAddr = format!("{host}:{port}")
        .parse()
        .expect("Invalid host or port");

    atuin_server::launch::<Sqlite>(settings, addr)
        .await
        .unwrap();
}
