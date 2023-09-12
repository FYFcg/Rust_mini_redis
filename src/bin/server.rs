#![feature(impl_trait_in_assoc_type)]

use std::{collections::HashMap, net::SocketAddr, sync::Mutex};

use mini_redis::{FilterLayer, LogLayer, S};

#[volo::main]
async fn main() {
    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);

    volo_gen::volo::example::ItemServiceServer::new(S {
        map: Mutex::new(HashMap::new()),
    })
    .layer_front(LogLayer)
    .layer_front(FilterLayer)
    .run(addr)
    .await
    .unwrap();
}
