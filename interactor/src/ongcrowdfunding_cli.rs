use multiversx_sc_snippets::imports::*;
use multiversx_sc_snippets::{
    multiversx_sc::types::{BigUint, ManagedAddress},
    multiversx_sc_scenario::rust_biguint,
    scenario_model::*,
};
use std::process::Command;

const GATEWAY: &str = sdk::blockchain::TESTNET_GATEWAY;

pub struct ContractInteract {
    pub world: ScenarioWorld,
    pub owner_address: AddressValue,
    pub contract_address: AddressValue,
}

impl ContractInteract {
    pub async fn new() -> Self {
        let mut blockchain = ScenarioWorld::new();
        blockchain.set_current_dir_from_workspace("ongcrowdfunding");
        blockchain.set_gateway(GATEWAY);

        // Lê a chave privada do arquivo .env
        let private_key = std::env::var("OWNER_PRIVATE_KEY")
            .expect("OWNER_PRIVATE_KEY must be set in .env file");
        
        // Cria a carteira do owner usando a chave privada
        let owner_address = blockchain.create_user_account_from_private_key(
            &private_key,
            &rust_biguint!(1_000_000)
        );
        
        println!("Owner address: {}", owner_address);
        
        let contract_address = blockchain.create_user_account(&rust_biguint!(0));

        Self {
            world: blockchain,
            owner_address,
            contract_address,
        }
    }

    pub async fn deploy(&mut self, target: u64, deadline: u64) {
        println!("Deploying contract with target: {} and deadline: {}", target, deadline);
        
        let tx_result = self.world
            .tx()
            .from(&self.owner_address)
            .to(&self.contract_address)
            .typed(ongcrowdfunding::CrowdfundingProxy)
            .init(target, deadline)
            .returns(ReturnsNewAddress)
            .run();

        println!("Contract deployed at address: {}", tx_result);
        
        // Salva o endereço do contrato em um arquivo state.toml
        let state_content = format!("contract_address = \"{}\"", tx_result);
        std::fs::write("state.toml", state_content).expect("Failed to write state.toml");
    }

    pub async fn fund(&mut self, amount: u64) {
        self.world
            .tx()
            .from(&self.owner_address)
            .to(&self.contract_address)
            .typed(ongcrowdfunding::CrowdfundingProxy)
            .fund()
            .egld(amount)
            .run();
    }

    pub async fn claim(&mut self) {
        self.world
            .tx()
            .from(&self.owner_address)
            .to(&self.contract_address)
            .typed(ongcrowdfunding::CrowdfundingProxy)
            .claim()
            .run();
    }

    pub async fn status(&mut self) {
        let status = self.world
            .query()
            .to(&self.contract_address)
            .typed(ongcrowdfunding::CrowdfundingProxy)
            .status()
            .returns(ExpectValue(ongcrowdfunding::Status::FundingPeriod))
            .run();

        println!("Status: {:?}", status);
    }

    pub async fn get_current_funds(&mut self) {
        let funds = self.world
            .query()
            .to(&self.contract_address)
            .typed(ongcrowdfunding::CrowdfundingProxy)
            .get_current_funds()
            .returns(ExpectValue(rust_biguint!(0)))
            .run();

        println!("Current funds: {}", funds);
    }

    pub async fn get_target(&mut self) {
        let target = self.world
            .query()
            .to(&self.contract_address)
            .typed(ongcrowdfunding::CrowdfundingProxy)
            .target()
            .returns(ExpectValue(0u64))
            .run();

        println!("Target: {}", target);
    }

    pub async fn get_deadline(&mut self) {
        let deadline = self.world
            .query()
            .to(&self.contract_address)
            .typed(ongcrowdfunding::CrowdfundingProxy)
            .deadline()
            .returns(ReturnsResultUnmanaged)
            .run();

        println!("Deadline: {}", deadline);
    }

    pub async fn get_deposit(&mut self, donor_address: String) {
        let deposit = self.world
            .query()
            .to(&self.contract_address)
            .typed(ongcrowdfunding::CrowdfundingProxy)
            .deposit(AddressValue::from(donor_address))
            .returns(ExpectValue(rust_biguint!(0)))
            .run();

        println!("Deposit: {}", deposit);
    }

    pub async fn deploy_with_mxpy(&self, keyfile_path: &str) {
        let output = Command::new("mxpy")
            .args(&[
                "--verbose",
                "contract",
                "deploy",
                "--recall-nonce",
                "--bytecode=./output/ongcrowdfunding.wasm",
                &format!("--keyfile={}", keyfile_path),
                "--gas-limit=100000000",
                "--proxy=https://devnet-gateway.multiversx.com",
                "--chain=D",
                "--send"
            ])
            .output()
            .expect("Falha ao executar o comando mxpy");

        println!("Status: {}", output.status);
        println!("Saída: {}", String::from_utf8_lossy(&output.stdout));
        println!("Erro: {}", String::from_utf8_lossy(&output.stderr));
    }
} 