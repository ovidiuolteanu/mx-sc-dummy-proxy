#![allow(non_snake_case)]

mod proxy;

use dummy_proxy::dummy_proxy::CallType;
use multiversx_sc_snippets::imports::*;
use multiversx_sc_snippets::sdk;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};

const GATEWAY: &str = sdk::blockchain::DEVNET_GATEWAY;
const STATE_FILE: &str = "state.toml";

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");
    let mut interact = ContractInteract::new().await;
    match cmd.as_str() {
        "deploy" => interact.deploy().await,
        "callEndpoint" => interact.call_endpoint().await,
        "callInternalTransferEndpoint" => interact.call_int_transfer_endpoint().await,
        "callTransferEndpoint" => interact.call_transfer_endpoint().await,
        _ => panic!("unknown command: {}", &cmd),
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct State {
    contract_address: Option<Bech32Address>,
}

impl State {
    // Deserializes state from file
    pub fn load_state() -> Self {
        if Path::new(STATE_FILE).exists() {
            let mut file = std::fs::File::open(STATE_FILE).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            toml::from_str(&content).unwrap()
        } else {
            Self::default()
        }
    }

    /// Sets the contract address
    pub fn set_address(&mut self, address: Bech32Address) {
        self.contract_address = Some(address);
    }

    /// Returns the contract address
    pub fn current_address(&self) -> &Bech32Address {
        self.contract_address
            .as_ref()
            .expect("no known contract, deploy first")
    }
}

impl Drop for State {
    // Serializes state to file
    fn drop(&mut self) {
        let mut file = std::fs::File::create(STATE_FILE).unwrap();
        file.write_all(toml::to_string(self).unwrap().as_bytes())
            .unwrap();
    }
}

struct ContractInteract {
    interactor: Interactor,
    wallet_address: Address,
    contract_code: BytesValue,
    state: State,
}

impl ContractInteract {
    async fn new() -> Self {
        let mut interactor = Interactor::new(GATEWAY).await;
        let wallet_address = interactor.register_wallet(test_wallets::alice());

        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/dummy-proxy.mxsc.json",
            &InterpreterContext::default(),
        );

        ContractInteract {
            interactor,
            wallet_address,
            contract_code,
            state: State::load_state(),
        }
    }

    async fn deploy(&mut self) {
        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(50_000_000u64)
            .typed(proxy::DummyProxyContractProxy)
            .init()
            .code(&self.contract_code)
            .returns(ReturnsNewAddress)
            .prepare_async()
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state.set_address(Bech32Address::from_bech32_string(
            new_address_bech32.clone(),
        ));

        println!("new address: {new_address_bech32}");
    }

    async fn call_endpoint(&mut self) {
        let call_type: CallType = CallType::Sync;
        let contract_address = bech32::decode("");
        let function_name = ManagedBuffer::new_from_bytes(&b""[..]);
        let args = MultiValueVec::from(vec![ManagedBuffer::new_from_bytes(&b""[..])]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(20_000_000u64)
            .typed(proxy::DummyProxyContractProxy)
            .call_endpoint(call_type, contract_address, function_name, args)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn call_int_transfer_endpoint(&mut self) {
        let call_type: CallType = CallType::Sync;
        let token_id = TokenIdentifier::from_esdt_bytes(&b""[..]);
        let nonce = 0u64;
        let amount = BigUint::<StaticApi>::from(0u128);
        let contract_address = bech32::decode("");
        let function_name = ManagedBuffer::new_from_bytes(&b""[..]);
        let args = MultiValueVec::from(vec![ManagedBuffer::new_from_bytes(&b""[..])]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(20_000_000u64)
            .typed(proxy::DummyProxyContractProxy)
            .call_int_transfer_endpoint(
                call_type,
                token_id,
                nonce,
                amount,
                contract_address,
                function_name,
                args,
            )
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn call_transfer_endpoint(&mut self) {
        let call_type: CallType = CallType::Sync;
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let contract_address = bech32::decode("");
        let function_name = ManagedBuffer::new_from_bytes(&b""[..]);
        let args = MultiValueVec::from(vec![ManagedBuffer::new_from_bytes(&b""[..])]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(20_000_000u64)
            .typed(proxy::DummyProxyContractProxy)
            .call_transfer_endpoint(call_type, contract_address, function_name, args)
            .payment((
                TokenIdentifier::from(token_id.as_str()),
                token_nonce,
                token_amount,
            ))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }
}

#[tokio::test]
async fn integration_test() {
    let mut interact = ContractInteract::new().await;

    interact.deploy().await;
}
