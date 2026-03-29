/// Estado acumulado da fila ao longo da simulação.
///
/// Rastreia o tempo acumulado em cada estado (de 0 a K clientes no sistema),
/// o número de clientes descartados por lotação, o tempo global e a
/// quantidade de servidores ocupados no instante atual.
pub struct QueueState {
    /// Vetor de tamanho K+1 onde `times[i]` acumula o tempo total
    /// que o sistema passou com exatamente `i` clientes presentes.
    /// Usado ao final para calcular a distribuição de probabilidade.
    times: Vec<f64>,
    /// Número total de clientes que chegaram mas foram descartados
    /// porque o sistema estava em capacidade máxima (K).
    pub loss: u64,
    /// Relógio global da simulação (tempo do último evento processado).
    pub global_time: f64,
    /// Quantidade de servidores atualmente ocupados atendendo clientes.
    pub busy_servers: usize,
    /// Capacidade máxima do sistema (K).
    capacity: usize,
    /// Número total de servidores (c).
    servers: usize,
}

impl QueueState {
    /// Cria um novo estado com fila vazia (0 clientes, 0 servidores ocupados).
    ///
    /// O vetor de tempos tem tamanho `capacity + 1` (estados 0..=K).
    pub fn new(servers: usize, capacity: usize) -> Self {
        Self {
            times: vec![0.0; capacity + 1],
            loss: 0,
            global_time: 0.0,
            busy_servers: 0,
            servers,
            capacity,
        }
    }

    /// Acumula o tempo delta no estado atual da fila.
    ///
    /// # Argumento
    /// - `current_state`: número de clientes presentes no sistema neste instante.
    /// - `event_time`: tempo do evento que está sendo processado.
    ///
    /// Calcula o delta entre o tempo do evento e o tempo global atual,
    /// soma esse delta ao acumulador do estado correspondente,
    /// e avança o relógio global.
    pub fn accumulate_time(&mut self, current_state: usize, event_time: f64) {
        let delta = event_time - self.global_time;
        debug_assert!(
            delta >= 0.0,
            "Delta negativo detectado: evento em {}, global em {}",
            event_time,
            self.global_time
        );
        if current_state <= self.capacity {
            self.times[current_state] += delta;
        }
        self.global_time = event_time;
    }

    /// Tenta registrar a chegada de um novo cliente.
    ///
    /// Retorna `true` se o cliente foi aceito (havia espaço no sistema).
    /// Retorna `false` se o sistema está cheio (loss/drop).
    pub fn try_admit(&mut self, current_length: usize) -> bool {
        if current_length >= self.capacity {
            self.loss += 1;
            false
        } else {
            true
        }
    }

    /// Verifica se há um servidor disponível para iniciar atendimento.
    pub fn has_free_server(&self) -> bool {
        self.busy_servers < self.servers
    }

    /// Ocupa um servidor (marca como em uso).
    pub fn occupy_server(&mut self) {
        debug_assert!(
            self.busy_servers < self.servers,
            "Tentativa de ocupar servidor quando todos já estão ocupados"
        );
        self.busy_servers += 1;
    }

    /// Libera um servidor (marca como disponível).
    pub fn release_server(&mut self) {
        debug_assert!(
            self.busy_servers > 0,
            "Tentativa de liberar servidor quando nenhum está ocupado"
        );
        self.busy_servers -= 1;
    }

    /// Imprime a distribuição de probabilidade de cada estado e o total de perdas.
    ///
    /// A probabilidade de cada estado é calculada como:
    ///   P(estado i) = tempo_acumulado_no_estado_i / tempo_global_total
    pub fn print_results(&self) {
        println!("\n╔══════════════════════════════════════════════════╗");
        println!("║        RESULTADOS DA SIMULAÇÃO G/G/{}/{}         ║", self.servers, self.capacity);
        println!("╠══════════════════════════════════════════════════╣");
        println!("║  Estado │  Tempo Acumulado  │  Probabilidade     ║");
        println!("╠─────────┼───────────────────┼────────────────────╣");

        for (i, &time) in self.times.iter().enumerate() {
            let probability = if self.global_time > 0.0 {
                time / self.global_time
            } else {
                0.0
            };
            println!(
                "║  {:>5}  │  {:>15.4}  │  {:>16.6}  ║",
                i, time, probability
            );
        }

        println!("╠══════════════════════════════════════════════════╣");
        println!("║  Tempo global total: {:>27.4}  ║", self.global_time);
        println!("║  Clientes perdidos (loss): {:>21}  ║", self.loss);
        println!("╚══════════════════════════════════════════════════╝");
    }
}
