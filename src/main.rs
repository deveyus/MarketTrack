use salvo::prelude::*;
use yew::ServerRenderer;

mod components {
    pub mod hello;
    pub mod root;
}

#[handler]
async fn hello_handler(res: &mut Response) {
    let renderer = ServerRenderer::<components::root::Root>::new();
    let rendered_html = renderer.render().await;
    res.headers_mut().insert("content-type", "text/html".parse().unwrap());
    res.render(Text::Html(rendered_html));
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new().get(hello_handler);
    let acceptor = TcpListener::new("127.0.0.1:5000").bind().await;
    Server::new(acceptor).serve(router).await;
}