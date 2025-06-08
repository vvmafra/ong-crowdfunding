use multiversx_sc_snippets::imports::*;
use rust_interact::ongcrowdfunding_cli;
mod interact;

#[tokio::main]
async fn main() {
    env_logger::init();

    ongcrowdfunding_cli().await;
}

