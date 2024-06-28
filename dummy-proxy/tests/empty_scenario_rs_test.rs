use multiversx_sc_scenario::ScenarioWorld;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "mxsc:output/dummy-proxy.mxsc.json",
        dummy_proxy::ContractBuilder,
    );
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/empty.scen.json");
}
