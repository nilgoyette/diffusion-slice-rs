use wgpu::{Device, Queue};

use client::Client;

mod client;

pub struct Context {
    #[allow(unused)] // NOTE unused for now
    client: Client,
}

impl Context {
    pub async fn new() -> Self {
        Self {
            client: Client::new().await,
        }
    }
}
