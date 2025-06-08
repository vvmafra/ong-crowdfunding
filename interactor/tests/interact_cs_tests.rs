use multiversx_sc_snippets::imports::*;
use rust_interact::{Config, ContractInteract};

// Simple deploy test that runs using the chain simulator configuration.
// In order for this test to work, make sure that the `config.toml` file contains the chain simulator config (or choose it manually)
// The chain simulator should already be installed and running before attempting to run this test.
// The chain-simulator-tests feature should be present in Cargo.toml.
// Can be run with `sc-meta test -c`.
#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn deploy_test_ongcrowdfunding_cs() {
    let mut interactor = ContractInteract::new(Config::chain_simulator_config()).await;

    interactor.deploy(500_000_000_000u64).await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn realize_test_ongcrowdfunding_cs() {
    let mut interactor = ContractInteract::new(Config::chain_simulator_config()).await;

    interactor.deploy(500_000_000_000u64).await;
    interactor.realize().await;
}
