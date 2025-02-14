use rand::{rng, Rng};
const POP_SIZE: usize = 100;
const MAX_GENERATIONS: usize = 1000;
const MUTATION_RATE: f64 = 0.1;
const CAPACITY: u32 = 2500;
const ELITE_COUNT: usize = 5; // Number of top individuals to retain each generation
const TOURNAMENT_SIZE: usize = 5; // Number of individuals in tournament selection

#[derive(Clone)]
struct Individual {
    genes: Vec<bool>,
    fitness: u32,
    total_weight: u32,
}

impl Individual {
    fn new(size: usize) -> Self {
        let mut rng = rng();
        let genes = (0..size).map(|_| rng.gen_bool(0.5)).collect();
        Self { genes, fitness: 0, total_weight: 0 }
    }
    
    fn evaluate(&mut self, values: &[u32], weights: &[u32]) {
        let (total_value, total_weight): (u32, u32) = self.genes.iter().enumerate()
            .filter(|(_, &selected)| selected)
            .map(|(i, _)| (values[i], weights[i]))
            .fold((0, 0), |(v, w), (val, wt)| (v + val, w + wt));
        
        self.total_weight = total_weight;
        self.fitness = if total_weight <= CAPACITY { total_value } else { 0 };
    }
}

fn mutate(individual: &mut Individual) {
    let mut rng = rng();
    for gene in &mut individual.genes {
        if rng.gen::<f64>() < MUTATION_RATE {
            *gene = !*gene;
        }
    }
}

fn crossover(parent1: &Individual, parent2: &Individual) -> Individual {
    let mut rng = rng();
    let crossover_point = rng.gen_range(0..parent1.genes.len());
    let genes: Vec<bool> = parent1.genes[..crossover_point].iter()
        .chain(&parent2.genes[crossover_point..])
        .cloned()
        .collect();
    
    Individual { genes, fitness: 0, total_weight: 0 }
}

fn tournament_selection(population: &[Individual]) -> &Individual {
    let mut rng = rng();
    let mut best = &population[rng.gen_range(0..population.len())];
    for _ in 1..TOURNAMENT_SIZE {
        let contender = &population[rng.gen_range(0..population.len())];
        if contender.fitness > best.fitness {
            best = contender;
        }
    }
    best
}

fn evolve(values: &[u32], weights: &[u32]) -> Individual {
    let mut population: Vec<Individual> = (0..POP_SIZE).map(|_| Individual::new(values.len())).collect();
    
    for individual in &mut population {
        individual.evaluate(values, weights);
    }
    
    for _ in 0..MAX_GENERATIONS {
        population.sort_by(|a, b| b.fitness.cmp(&a.fitness));
        
        let mut new_population = population[..ELITE_COUNT].to_vec(); // Retain elites
        
        while new_population.len() < POP_SIZE {
            let parent1 = tournament_selection(&population);
            let parent2 = tournament_selection(&population);
            let mut child = crossover(parent1, parent2);
            mutate(&mut child);
            child.evaluate(values, weights);
            new_population.push(child);
        }
        
        population = new_population;
    }
    
    population.sort_by(|a, b| b.fitness.cmp(&a.fitness));
    population[0].clone()
}

fn main() {
    let values = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let weights = vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50];
    let best_solution = evolve(&values, &weights);
    println!("Best solution fitness: {}", best_solution.fitness);
    println!("Total weight stored in backpack: {}", best_solution.total_weight);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_evolution_basic() {
        let values = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        let weights = vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50];
        let best_solution = evolve(&values, &weights);
        assert!(best_solution.fitness > 0);
    }
    
    #[test]
    fn test_empty_knapsack() {
        let values = vec![10, 20, 30];
        let weights = vec![2501, 2600, 2700]; // All items exceed capacity
        let best_solution = evolve(&values, &weights);
        assert_eq!(best_solution.fitness, 0);
    }
    
    #[test]
    fn test_single_item_fit() {
        let values = vec![100];
        let weights = vec![2000]; // Single item within capacity
        let best_solution = evolve(&values, &weights);
        assert_eq!(best_solution.fitness, 100);
    }
}
