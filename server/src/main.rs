use std::net::SocketAddr;

use http_body_util::combinators::UnsyncBoxBody;
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, Result, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use tower::ServiceExt;
use tower_http::services::fs::ServeDir;

static NOT_FOUND: &[u8] = b"Not Found";

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();

    let listener = TcpListener::bind(addr).await?;

    println!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(handle_request))
                .await
            {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}

async fn handle_request(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<UnsyncBoxBody<Bytes, std::io::Error>>> {
    let mut static_svc: ServeDir = ServeDir::new("static")
        .append_index_html_on_directories(true);

    let ready_service = match ServiceExt::<Request<Full<Bytes>>>::ready(&mut static_svc).await {
         Ok(ready_service) => ready_service,
         Err(infallible) => match infallible {},
    };

    match ready_service.try_call(req).await {
        Ok(response) => Ok(response.map(|body|  body.map_err(Into::into).boxed_unsync())),
        Err(_)  => Ok(not_found())
    }

}

fn not_found() -> Response<UnsyncBoxBody<Bytes, std::io::Error>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new(NOT_FOUND.into()).map_err(|e| match e {}).boxed_unsync())
        .unwrap()
}

