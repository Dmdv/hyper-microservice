use std::net::SocketAddr;
use std::convert::Infallible;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
// use futures::{future, Future};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // We used an implementation of the impl<I: Into<IpAddr>> From<(I, u16)> for SocketAddr trait here,
    // which, in turn, uses impl From<[u8; 4]> for IpAddr.
    // This lets us use the .into() method call to construct a socket address from the tuple.
    // Similarly, we can create new SocketAddr instances with a constructor.
    // In production applications, we will parse the socket addresses from external strings (command-line parameters or environment variables),
    // and if no variants are set, we'll create SocketAddr from a tuple with default values.
    let addr:SocketAddr = ([127, 0, 0, 1], 3000).into();

    // let builder = Server::bind(&addr);
    // let server = builder.serve(|| {
    //     service_fn(|_| {
    //         Response::new(Body::from("Rust Microservice"))
    //     })
    // });
    //
    // let server = server.map_err(drop);
    // hyper::rt::run(server);

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);
    println!("Listening on http://{}", addr);

    // And now add a graceful shutdown signal...
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    // Run this server for... forever!
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("Processing request");
    Ok(Response::new("Hello, World".into()))
}

// fn microservice_handler(req: Request<Body>) -> impl Future<Item=Response<Body>, Error=Error> {
//     unimplemented!();
// }
