use multiversx_sc_scenario::imports::*;
mod sc_proxy;

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("dummy-proxy");
const CODE_PATH: MxscPath = MxscPath::new("output/dummy-proxy.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(CODE_PATH, dummy_proxy::ContractBuilder);
    blockchain
}

#[test]
fn blackbox() {
    let mut world = world();

    world.start_trace();

    world.account(OWNER_ADDRESS).nonce(1);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(sc_proxy::DummyProxyContractProxy)
        .init()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    world.write_scenario_trace("scenarios/trace1-blackbox.scen.json");
}
