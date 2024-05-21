use std::future::Future;
use tokio::runtime::Runtime;

pub fn sync() -> Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
}