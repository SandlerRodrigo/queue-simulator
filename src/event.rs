use std::cmp::Ordering;

/// Representa os dois tipos de evento possíveis na simulação.
///
/// Cada variante carrega o instante de tempo (f64) em que o evento ocorre.
/// A ordenação é implementada de forma **reversa** para que a `BinaryHeap`
/// (que é um max-heap por padrão em Rust) funcione como um **min-heap**,
/// garantindo que o evento com menor timestamp tenha a maior prioridade.
#[derive(Debug, Clone)]
pub enum Event {
    /// Chegada de um novo cliente ao sistema no instante `t`.
    Arrival(f64),
    /// Saída (fim de atendimento) de um cliente no instante `t`.
    Departure(f64),
}

impl Event {
    /// Retorna o instante de tempo associado ao evento.
    pub fn time(&self) -> f64 {
        match self {
            Event::Arrival(t) | Event::Departure(t) => *t,
        }
    }
}

// ── Implementação de ordenação reversa para min-heap ──────────────────────
//
// A BinaryHeap do Rust é um max-heap. Para obter comportamento de min-heap
// (menor tempo = maior prioridade), invertemos a ordenação nos traits Ord/PartialOrd.
// Usamos `partial_cmp` de f64 com fallback para `Equal` em caso de NaN
// (que não deveria ocorrer, mas garante safety).

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.time() == other.time()
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        // Ordenação REVERSA: tempo menor → prioridade maior.
        // other.time() comparado com self.time() (invertido).
        other
            .time()
            .partial_cmp(&self.time())
            .unwrap_or(Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BinaryHeap;

    #[test]
    fn min_heap_behavior() {
        let mut heap = BinaryHeap::new();
        heap.push(Event::Arrival(5.0));
        heap.push(Event::Departure(1.0));
        heap.push(Event::Arrival(3.0));

        // O evento com menor tempo deve sair primeiro (min-heap)
        assert_eq!(heap.pop().unwrap().time(), 1.0);
        assert_eq!(heap.pop().unwrap().time(), 3.0);
        assert_eq!(heap.pop().unwrap().time(), 5.0);
    }
}
