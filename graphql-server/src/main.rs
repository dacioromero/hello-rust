extern crate openssl;
#[macro_use]
extern crate diesel;

mod graphql;
pub mod models;
pub mod schema;

use std::env;
use std::sync::Arc;
use std::net::SocketAddr;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Method, Response, Server, StatusCode,
};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let addr: SocketAddr = env::var("ADDR")
        .unwrap_or("127.0.0.1:3000".to_string())
        .parse()
        .expect("Unable to parse ADDR");

    let context = Arc::new(graphql::create_context(&database_url));
    let root_node = Arc::new(graphql::create_root_node());

    let new_service = make_service_fn(move |_| {
        let root_node = root_node.clone();
        let context = context.clone();

        async move {
            Ok::<_, hyper::Error>(service_fn(move |request| {
                let root_node = root_node.clone();
                let context = context.clone();
                async move {
                    match (request.method(), request.uri().path()) {
                        (&Method::GET, "/") => juniper_hyper::graphiql("/graphql", None).await,
                        (&Method::GET, "/graphql") | (&Method::POST, "/graphql") => {
                            juniper_hyper::graphql(root_node, context, request).await
                        }
                        _ => {
                            let mut response = Response::new(Body::empty());
                            *response.status_mut() = StatusCode::NOT_FOUND;
                            Ok(response)
                        }
                    }
                }
            }))
        }
    });

    let server = Server::bind(&addr).serve(new_service);
    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e)
    }
}
