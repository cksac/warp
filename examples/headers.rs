#![deny(warnings)]
extern crate pretty_env_logger;
extern crate warp;

use std::net::SocketAddr;
use warp::Filter;

/// Create a server that requires header conditions:
///
/// - `Host` is a `SocketAddr`
/// - `Accept` is exactly `*/*`
///
/// If these conditions don't match, a 404 is returned.
fn main() {
    pretty_env_logger::init();

    // For this example, we assume no DNS was used,
    // so the Host header should be an address.
    let host = warp::header::<SocketAddr>("host");

    // Match when we get `accept: */*` exactly.
    let accept_stars = warp::header::exact("accept", "*/*");

    let index = host.and(accept_stars)
        .map(|addr| {
            format!("accepting stars on {}", addr)
        });

    // Only allow GETs in this example
    let routes = warp::get(
        index
    );

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030));
}

