/// Gerador de Números Pseudoaleatórios via Método Congruente Linear (LCG).
///
/// Implementa a fórmula: `X(n+1) = (a * X(n) + c) mod m`
///
/// Os parâmetros utilizados são os mesmos do gerador "Numerical Recipes":
/// - a = 1664525
/// - c = 1013904223
/// - m = 2^32
///
/// A struct encapsula completamente seu estado interno (seed),
/// eliminando qualquer necessidade de variáveis globais.
pub struct Rng {
    /// Estado atual do gerador (a "seed" evolui a cada chamada)
    state: u64,
    /// Contador de quantos números aleatórios já foram gerados.
    /// Usado pelo simulador para saber quando parar.
    count: u64,
}

impl Rng {
    /// Cria um novo gerador com a seed inicial fornecida.
    pub fn new(seed: u64) -> Self {
        Self {
            state: seed,
            count: 0,
        }
    }

    /// Gera o próximo número pseudoaleatório no intervalo [0, 1).
    ///
    /// Cada chamada avança o estado interno e incrementa o contador.
    /// O valor retornado é normalizado dividindo pelo módulo (2^32).
    pub fn next_random(&mut self) -> f64 {
        // Parâmetros do LCG (Numerical Recipes)
        const A: u64 = 1664525;
        const C: u64 = 1013904223;
        const M: u64 = 1 << 32; // 2^32 = 4294967296

        self.state = (A.wrapping_mul(self.state).wrapping_add(C)) % M;
        self.count += 1;

        // Normaliza para [0, 1)
        self.state as f64 / M as f64
    }

    /// Retorna quantas vezes `next_random()` foi chamado.
    pub fn count(&self) -> u64 {
        self.count
    }

    /// Gera um valor uniformemente distribuído no intervalo [min, max).
    ///
    /// Internamente consome uma chamada ao RNG, portanto incrementa o contador.
    pub fn next_in_range(&mut self, min: f64, max: f64) -> f64 {
        let r = self.next_random();
        min + (max - min) * r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rng_produces_values_in_unit_interval() {
        let mut rng = Rng::new(42);
        for _ in 0..1000 {
            let val = rng.next_random();
            assert!((0.0..1.0).contains(&val), "Valor fora de [0,1): {}", val);
        }
    }

    #[test]
    fn rng_count_tracks_calls() {
        let mut rng = Rng::new(1);
        assert_eq!(rng.count(), 0);
        rng.next_random();
        rng.next_random();
        rng.next_random();
        assert_eq!(rng.count(), 3);
    }

    #[test]
    fn rng_range_respects_bounds() {
        let mut rng = Rng::new(99);
        for _ in 0..1000 {
            let val = rng.next_in_range(2.0, 5.0);
            assert!(val >= 2.0 && val < 5.0, "Valor fora do intervalo: {}", val);
        }
    }

    #[test]
    fn rng_is_deterministic() {
        let mut rng1 = Rng::new(123);
        let mut rng2 = Rng::new(123);
        for _ in 0..100 {
            assert_eq!(rng1.next_random(), rng2.next_random());
        }
    }
}
