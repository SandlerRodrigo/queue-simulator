use crate::config::Config;
use crate::event::Event;
use crate::queue_state::QueueState;
use crate::rng::Rng;
use crate::scheduler::Scheduler;

/// Simulador principal de filas de eventos discretos.
///
/// Orquestra o ciclo completo da simulação:
/// 1. Inicializa a fila vazia com o primeiro evento de `Arrival`.
/// 2. Executa o loop principal, processando eventos em ordem cronológica.
/// 3. Para quando o RNG consome o número máximo de chamadas configurado.
/// 4. Imprime os resultados estatísticos.
pub struct Simulator {
    /// Configuração da simulação (parâmetros lidos do JSON)
    config: Config,
    /// Gerador de números aleatórios com estado encapsulado
    rng: Rng,
    /// Escalonador de eventos (min-heap)
    scheduler: Scheduler,
    /// Estado estatístico da fila
    state: QueueState,
    /// Número atual de clientes no sistema (em serviço + na fila de espera).
    /// Mantido separadamente para acesso rápido e clareza semântica.
    clients_in_system: usize,
}

impl Simulator {
    /// Cria uma nova instância do simulador a partir da configuração.
    ///
    /// Agenda automaticamente o primeiro evento de Arrival no tempo
    /// especificado pela configuração (`first_arrival_time`).
    pub fn new(config: Config) -> Self {
        let rng = Rng::new(config.rng_seed);
        let mut scheduler = Scheduler::new();
        let state = QueueState::new(config.servers, config.capacity);

        // Agenda a primeira chegada no tempo estrito configurado
        scheduler.schedule(Event::Arrival(config.first_arrival_time));

        Self {
            config,
            rng,
            scheduler,
            state,
            clients_in_system: 0,
        }
    }

    /// Executa o loop principal da simulação.
    ///
    /// O loop processa eventos em ordem cronológica até que o RNG
    /// tenha consumido exatamente o limite de chamadas definido na configuração.
    ///
    /// Para cada evento:
    /// 1. Acumula o tempo delta no estado atual (antes de modificar o estado).
    /// 2. Processa a lógica do evento (Arrival ou Departure).
    /// 3. Verifica se o limite do RNG foi atingido.
    pub fn run(&mut self) {
        println!("Iniciando simulação G/G/{}/{}...", self.config.servers, self.config.capacity);
        println!(
            "Primeira chegada em t={:.2}, limite de RNG: {}",
            self.config.first_arrival_time, self.config.random_number_limit
        );
        println!("─────────────────────────────────────────────");

        while let Some(event) = self.scheduler.next_event() {
            // ── Passo 1: Acumula tempo no estado atual ────────────────
            // Antes de alterar o número de clientes no sistema,
            // registramos quanto tempo o sistema permaneceu no estado
            // atual (com `clients_in_system` clientes).
            self.state
                .accumulate_time(self.clients_in_system, event.time());

            // ── Passo 2: Processa o evento ────────────────────────────
            match event {
                Event::Arrival(_) => self.process_arrival(),
                Event::Departure(_) => self.process_departure(),
            }

            // ── Passo 3: Verifica condição de parada ──────────────────
            // O loop para EXATAMENTE quando o RNG atingir o limite.
            // Isso significa que eventos já agendados ainda são processados,
            // mas novas chamadas ao RNG param de ser feitas quando o
            // limite é atingido (controlado nos métodos auxiliares).
            if self.rng.count() >= self.config.random_number_limit {
                break;
            }
        }

        // Imprime os resultados finais
        self.state.print_results();
    }

    /// Processa um evento de chegada (Arrival).
    ///
    /// Lógica:
    /// - Se o sistema está cheio (clientes == K): descarta o cliente (loss).
    /// - Caso contrário: adiciona o cliente ao sistema.
    ///   - Se há servidor livre: inicia atendimento e agenda Departure.
    ///   - Se não há servidor livre: o cliente entra na fila de espera.
    /// - Sempre agenda a próxima chegada (se o RNG ainda não atingiu o limite).
    fn process_arrival(&mut self) {
        // Tenta admitir o cliente no sistema
        if self.state.try_admit(self.clients_in_system) {
            // Cliente aceito: incrementa a contagem
            self.clients_in_system += 1;

            // Verifica se há servidor disponível para atendimento imediato
            if self.state.has_free_server() {
                self.state.occupy_server();
                self.schedule_departure();
            }
            // Se não há servidor livre, o cliente apenas espera na fila
        }
        // Se try_admit retornou false, o cliente foi descartado (loss)

        // Agenda a próxima chegada (consome 1 número aleatório)
        self.schedule_next_arrival();
    }

    /// Processa um evento de saída (Departure).
    ///
    /// Lógica:
    /// - Remove um cliente do sistema.
    /// - Libera o servidor que o atendeu.
    /// - Se ainda há clientes na fila de espera:
    ///   ocupa um servidor e agenda nova Departure para o próximo da fila.
    fn process_departure(&mut self) {
        self.clients_in_system -= 1;
        self.state.release_server();

        // Se há clientes esperando na fila (mais clientes que servidores),
        // o próximo da fila entra em atendimento imediatamente.
        if self.clients_in_system >= self.config.servers {
            // Há clientes aguardando (queue_length > 0),
            // ou seja, clients_in_system > busy_servers (que acabou de diminuir)
            self.state.occupy_server();
            self.schedule_departure();
        }
    }

    /// Agenda a próxima chegada usando o RNG.
    ///
    /// O intervalo entre chegadas segue distribuição uniforme
    /// no intervalo [arrival_time_min, arrival_time_max).
    /// Consome 1 chamada do RNG.
    fn schedule_next_arrival(&mut self) {
        // Verifica se ainda podemos consumir números aleatórios
        if self.rng.count() >= self.config.random_number_limit {
            return;
        }

        let interval = self.rng.next_in_range(
            self.config.arrival_time_min,
            self.config.arrival_time_max,
        );
        let next_time = self.state.global_time + interval;
        self.scheduler.schedule(Event::Arrival(next_time));
    }

    /// Agenda um evento de saída (Departure) para o cliente que acaba de
    /// iniciar o atendimento.
    ///
    /// O tempo de atendimento segue distribuição uniforme
    /// no intervalo [service_time_min, service_time_max).
    /// Consome 1 chamada do RNG.
    fn schedule_departure(&mut self) {
        // Verifica se ainda podemos consumir números aleatórios
        if self.rng.count() >= self.config.random_number_limit {
            return;
        }

        let service_time = self.rng.next_in_range(
            self.config.service_time_min,
            self.config.service_time_max,
        );
        let departure_time = self.state.global_time + service_time;
        self.scheduler.schedule(Event::Departure(departure_time));
    }
}
