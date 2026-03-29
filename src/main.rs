//! # QueueSimulator — Simulador de Filas de Eventos Discretos (G/G/c/K)
//!
//! Implementa simulação baseada na notação de Kendall utilizando:
//! - Gerador Congruente Linear (LCG) como RNG
//! - Fila de prioridade (min-heap) como escalonador de eventos
//! - Acumulação de tempo por estado para cálculo da distribuição de probabilidade
//!
//! ## Uso
//! ```sh
//! cargo run                     # Usa config.json no diretório atual
//! cargo run -- caminho/para/config.json  # Especifica outro arquivo
//! ```

mod config;
mod event;
mod queue_state;
mod rng;
mod scheduler;
mod simulator;

use std::env;

use config::Config;
use simulator::Simulator;

fn main() {
    // Determina o caminho do arquivo de configuração:
    // - Usa o primeiro argumento da CLI, se fornecido.
    // - Caso contrário, usa "config.json" no diretório atual.
    let config_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "config.json".to_string());

    // Carrega e valida a configuração
    let config = Config::from_file(&config_path);
    config.validate();

    println!("╔══════════════════════════════════════════════════╗");
    println!("║   QueueSimulator — Simulação de Filas (DES)     ║");
    println!("║   Notação de Kendall: G/G/{}/{:<21} ║", config.servers, config.capacity);
    println!("╚══════════════════════════════════════════════════╝");
    println!();
    println!("Configuração carregada de: {}", config_path);
    println!("  Servidores (c):       {}", config.servers);
    println!("  Capacidade (K):       {}", config.capacity);
    println!("  Chegada:              [{:.2}, {:.2})", config.arrival_time_min, config.arrival_time_max);
    println!("  Atendimento:          [{:.2}, {:.2})", config.service_time_min, config.service_time_max);
    println!("  Limite de RNG:        {}", config.random_number_limit);
    println!("  Primeira chegada:     t = {:.2}", config.first_arrival_time);
    println!("  Seed do RNG:          {}", config.rng_seed);
    println!();

    // Cria e executa o simulador
    let mut simulator = Simulator::new(config);
    simulator.run();
}
