use salvo::prelude::*;

#[handler]
async fn hello()-> &'static str {
    "Hello, Sera"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new().get(hello);
    let acceptor = Tcplistener::new("127.0.0.1:5000").bind().await;
    Server::new(acceptor).serve(router).await;
}