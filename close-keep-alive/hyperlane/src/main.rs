use hyperlane::*;

async fn request_middleware(ctx: Context) {
    let _ = ctx
        .set_response_header(CONNECTION, CONNECTION_CLOSE)
        .await
        .send_response_once(200, "Hello")
        .await;
}

#[tokio::main]
async fn main() {
    let server: Server = Server::new();
    server.host("0.0.0.0").await;
    server.port(60000).await;
    server.log_dir("./logs").await;
    server.disable_log().await;
    server.disable_inner_log().await;
    server.disable_inner_print().await;
    server.http_line_buffer_size(512).await;
    server.websocket_buffer_size(512).await;
    server.request_middleware(request_middleware).await;
    server.listen().await.unwrap();
}
