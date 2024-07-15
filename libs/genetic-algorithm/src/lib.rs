use rand::RngCore;

pub struct GeneticAlgorithm;

impl GeneticAlgorithm {
    pub fn evolve<I>(&self, population: &[I]) -> Vec<I> {
        assert!(!population.is_empty());
        (0..population.len())
            .map(|_| {
                // TODO отбор
                // TODO скрещивание
                // TODO мутация
                todo!()
            })
            .collect()
    }
}

pub trait SelectionMethod {
    fn select<'a, R, I>(&self, rng: &mut R, population: &'a [I]) -> &'a I
    where
        R: RngCore,
        I: Individual;
}
