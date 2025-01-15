use multiversx_sc_scenario::*;
#[allow(deprecated)]

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("dummy-proxy");

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
