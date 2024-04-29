use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {

    let body = Body::from("Hello, World!");

    let response = Response::builder()
        .status(200)
        .header("Content-Type", "text/plain")
        .body(body)
        .unwrap();

    Ok(response)
}

async fn server() {

    let addr = ([127, 0, 0, 1], 8088).into();

    let server = Server::bind(&addr).serve(make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(handle_request))
    }));

    println!("Server running at http://127.0.0.1:3000/");

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

fn main() {
    println!("Hello, world!");
}
