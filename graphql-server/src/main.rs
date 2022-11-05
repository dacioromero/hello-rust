// https://github.com/graphql-rust/juniper/blob/47f7ffaa5b2c22c7ee3a3907cb4240bfd8056a70/juniper_hyper/examples/hyper_server.rs
#[cfg(not(debug_assertions))]
extern crate openssl;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate juniper;

// TODO: Determine if making models and schema public is correct.
mod graphql;
pub mod models;
pub mod schema;

use dotenv::dotenv;
use hyper::{
    service::{make_service_fn, service_fn},
    Method, Response, Server, StatusCode,
};
use std::{convert::Infallible, env, net::SocketAddr, sync::Arc};

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
        let ctx = context.clone();

        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                let root_node = root_node.clone();
                let ctx = ctx.clone();
                async move {
                    Ok::<_, Infallible>(match (req.method(), req.uri().path()) {
                        (&Method::GET, "/") => juniper_hyper::graphiql("/graphql", None).await,
                        (&Method::GET, "/graphql") | (&Method::POST, "/graphql") => {
                            juniper_hyper::graphql(root_node, ctx, req).await
                        }
                        _ => {
                            let mut response = Response::new(String::new());
                            *response.status_mut() = StatusCode::NOT_FOUND;
                            response
                        }
                    })
                }
            }))
        }
    });

    let server = Server::bind(&addr).serve(new_service);
    println!("Listening on http://{addr}");

    if let Err(e) = server.await {
        eprintln!("server error: {e}")
    }
}
