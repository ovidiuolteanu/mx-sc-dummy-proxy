use multiversx_sc_scenario::ScenarioWorld;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn empty_go() {
    world().run("scenarios/empty.scen.json");
}
