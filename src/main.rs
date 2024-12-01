use salvo::prelude::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::with_path("/workspaces/MarketTrack/pkg").get(
        StaticDir::new([
            "/workspaces/MarketTrack/pkg/index.html"
        ])
        .defaults("index.html")
        .auto_list(true),
    );
    let acceptor = TcpListener::new("127.0.0.1:5000").bind().await;
    Server::new(acceptor).serve(router).await;
}