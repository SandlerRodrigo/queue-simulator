use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Configuração completa da simulação, lida a partir de um arquivo JSON externo.
///
/// Mapeia diretamente a notação de Kendall G/G/c/K:
/// - `servers` → c (número de servidores paralelos)
/// - `capacity` → K (capacidade máxima do sistema, incluindo quem está em serviço)
///
/// Os intervalos de tempo de chegada e atendimento definem distribuições uniformes
/// que alimentam o gerador de números aleatórios.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Número de servidores (c na notação de Kendall)
    pub servers: usize,
    /// Capacidade máxima do sistema (K na notação de Kendall).
    /// Inclui tanto clientes na fila quanto em atendimento.
    pub capacity: usize,
    /// Limite inferior do intervalo de tempo entre chegadas
    pub arrival_time_min: f64,
    /// Limite superior do intervalo de tempo entre chegadas
    pub arrival_time_max: f64,
    /// Limite inferior do tempo de atendimento
    pub service_time_min: f64,
    /// Limite superior do tempo de atendimento
    pub service_time_max: f64,
    /// Quantidade máxima de números aleatórios a consumir.
    /// O loop principal da simulação para exatamente ao atingir este limite.
    pub random_number_limit: u64,
    /// Tempo do primeiro evento de chegada (ponto de partida da simulação)
    pub first_arrival_time: f64,
    /// Semente inicial para o gerador congruente linear
    pub rng_seed: u64,
}

impl Config {
    /// Carrega e deserializa a configuração a partir de um arquivo JSON.
    ///
    /// # Panics
    /// Encerra o programa com mensagem descritiva se o arquivo não existir
    /// ou se o JSON estiver malformado.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let content = fs::read_to_string(path.as_ref()).unwrap_or_else(|e| {
            panic!(
                "Erro ao ler o arquivo de configuração '{}': {}",
                path.as_ref().display(),
                e
            )
        });

        serde_json::from_str(&content).unwrap_or_else(|e| {
            panic!("Erro ao parsear o JSON de configuração: {}", e)
        })
    }

    /// Valida os invariantes da configuração após a leitura.
    /// Garante que os intervalos são válidos e os parâmetros fazem sentido.
    pub fn validate(&self) {
        assert!(
            self.servers > 0,
            "O número de servidores deve ser >= 1, encontrado: {}",
            self.servers
        );
        assert!(
            self.capacity >= self.servers,
            "A capacidade (K={}) deve ser >= número de servidores (c={})",
            self.capacity,
            self.servers
        );
        assert!(
            self.arrival_time_min <= self.arrival_time_max,
            "arrival_time_min ({}) deve ser <= arrival_time_max ({})",
            self.arrival_time_min,
            self.arrival_time_max
        );
        assert!(
            self.service_time_min <= self.service_time_max,
            "service_time_min ({}) deve ser <= service_time_max ({})",
            self.service_time_min,
            self.service_time_max
        );
        assert!(
            self.random_number_limit > 0,
            "O limite de números aleatórios deve ser > 0"
        );
        assert!(
            self.first_arrival_time >= 0.0,
            "O tempo da primeira chegada deve ser >= 0.0"
        );
    }
}
