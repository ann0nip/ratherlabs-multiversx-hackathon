use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    // Configuración del entorno del blockchain simulado
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace(""); // Directorio base
    blockchain.register_contract(
        "mxsc:output/gass-pass-hack.mxsc.json", // Ruta del archivo JSON del contrato
        gass_pass_hack::ContractBuilder,       // Constructor del contrato
    );
    blockchain
}

#[test]
fn gaspass_test() {
    println!("Iniciando prueba de gas...");

    // Inicializa el entorno de prueba
    let mut world = world();

    // Carga y ejecuta el escenario
    world.run("scenarios/gass_pass_scenario.json");

    println!("Prueba de gas completada.");
}
