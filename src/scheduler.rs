use std::collections::BinaryHeap;

use crate::event::Event;

/// Escalonador de eventos baseado em fila de prioridade (min-heap).
///
/// Encapsula uma `BinaryHeap<Event>` que, graças à ordenação reversa
/// implementada no `Event`, sempre retorna o evento com o menor timestamp
/// primeiro — comportamento essencial para a simulação de eventos discretos.
pub struct Scheduler {
    heap: BinaryHeap<Event>,
}

impl Scheduler {
    /// Cria um escalonador vazio.
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    /// Agenda um novo evento na fila de prioridade.
    pub fn schedule(&mut self, event: Event) {
        self.heap.push(event);
    }

    /// Remove e retorna o próximo evento (menor timestamp).
    /// Retorna `None` se a fila estiver vazia.
    pub fn next_event(&mut self) -> Option<Event> {
        self.heap.pop()
    }

    /// Verifica se há eventos pendentes.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}
