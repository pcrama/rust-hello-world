use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let response = Response::new(Body::from("Hello, World!"));
    Ok(response)
}

#[tokio::main]
async fn main() {
    let make_svc = make_service_fn(|_conn| {
        let service = service_fn(handle_request);
        async { Ok::<_, hyper::Error>(service) }
    });

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
