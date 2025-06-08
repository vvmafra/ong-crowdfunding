use multiversx_sc_scenario::{
    imports::*
};
use ongcrowdfunding::crowdfunding_proxy::{self, Status};

const OWNER: TestAddress = TestAddress::new("owner");
const DONOR: TestAddress = TestAddress::new("donor");
const CROWDFUNDING_ADDRESS: TestSCAddress = TestSCAddress::new("crowdfunding");
const CODE_PATH: MxscPath = MxscPath::new("output/ongcrowdfunding.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("ongcrowdfunding");
    blockchain.register_contract(CODE_PATH, ongcrowdfunding::ContractBuilder);
    blockchain
}

fn crowdfunding_deploy() -> ScenarioWorld {
    let mut world = world();

    world.account(OWNER).nonce(0).balance(1000000);

    let crowdfunding_address = world
        .tx()
        .from(OWNER)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .init(500_000_000_000u64)
        .code(CODE_PATH)
        .new_address(CROWDFUNDING_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(crowdfunding_address, CROWDFUNDING_ADDRESS.to_address());

    world
}

#[test]
fn crowdfunding_deploy_test() {
    let mut world = crowdfunding_deploy();
    world.check_account(OWNER).balance(1_000_000);

    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .target()
        .returns(ExpectValue(500_000_000_000u64))
        .run();
}

fn crowdfunding_fund() -> ScenarioWorld {
    let mut world = crowdfunding_deploy();

    world.account(DONOR).nonce(0).balance(400_000_000_000u64);

    world
        .tx()
        .from(DONOR)
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .fund()
        .egld(250_000_000_000u64)
        .run();

    world
}

#[test]
fn crowdfunding_fund_test() {
    let mut world = crowdfunding_fund();

    world.check_account(OWNER).nonce(1).balance(1_000_000u64);
    world
        .check_account(DONOR)
        .nonce(1)
        .balance(150_000_000_000u64);
    world
        .check_account(CROWDFUNDING_ADDRESS)
        .nonce(0)
        .balance(250_000_000_000u64);

    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .target()
        .returns(ExpectValue(500_000_000_000u64))
        .run();
    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .deposit(DONOR)
        .returns(ExpectValue(250_000_000_000u64))
        .run();
}

#[test]
fn crowdfunding_status_test() {
    let mut world = crowdfunding_fund();

    // Verifica status durante o período de funding
    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .status()
        .returns(ExpectValue(Status::FundingPeriod))
        .run();

    // Owner realiza o funding
    world
        .tx()
        .from(OWNER)
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .realize()
        .run();

    // Verifica status após a realização (falhou pois não atingiu a meta)
    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .status()
        .returns(ExpectValue(Status::Failed))
        .run();
}

#[test]
fn crowdfunding_claim_failed_test() {
    let mut world = crowdfunding_fund();

    // Owner realiza o funding
    world
        .tx()
        .from(OWNER)
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .realize()
        .run();

    // Tenta reivindicar o reembolso
    world
        .tx()
        .from(DONOR)
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .claim()
        .run();

    // Verifica se o doador recebeu o reembolso
    world
        .check_account(DONOR)
        .balance(400_000_000_000u64);
    
    // Verifica se o contrato está vazio
    world
        .check_account(CROWDFUNDING_ADDRESS)
        .balance(0u64);
}

#[test]
fn crowdfunding_claim_successful_test() {
    let mut world = crowdfunding_deploy();

    // Configura um doador com saldo suficiente
    world.account(DONOR).nonce(0).balance(600_000_000_000u64);

    // Faz uma doação que atinge a meta
    world
        .tx()
        .from(DONOR)
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .fund()
        .egld(500_000_000_000u64)
        .run();

    // Owner realiza o funding
    world
        .tx()
        .from(OWNER)
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .realize()
        .run();

    // Verifica status (deve ser Successful)
    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .status()
        .returns(ExpectValue(Status::Successful))
        .run();

    // Owner reivindica os fundos
    world
        .tx()
        .from(OWNER)
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .claim()
        .run();

    // Verifica se o owner recebeu os fundos
    world
        .check_account(OWNER)
        .balance(500_001_000_000u64);
    
    // Verifica se o contrato está vazio
    world
        .check_account(CROWDFUNDING_ADDRESS)
        .balance(0u64);
}

#[test]
fn crowdfunding_claim_before_realize_test() {
    let mut world = crowdfunding_fund();

    // Tenta reivindicar antes da realização
    world
        .tx()
        .from(DONOR)
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .claim()
        .with_result(ExpectError(4, "funding not realized yet"))
        .run();
}