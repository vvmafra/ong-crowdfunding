use multiversx_sc_snippets::imports::*;
use rust_interact::{Config, ContractInteract};

// Simple deploy test that runs on the real blockchain configuration.
// In order for this test to work, make sure that the `config.toml` file contains the real blockchain config (or choose it manually)
// Can be run with `sc-meta test`.
#[tokio::test]
#[ignore = "run on demand, relies on real blockchain state"]
async fn deploy_test_ongcrowdfunding() {
    let mut interactor = ContractInteract::new(Config::new()).await;

    interactor.deploy(500_000_000_000u64).await;
}
