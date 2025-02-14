use rand::{rng, Rng};
use std::cmp::Ordering;

const N: usize = 100; // Liczba zadań
const PROCESSORS: usize = 4;
const MAX_ITERATIONS: usize = 100_000;
const MAX_FAILED_MUTATIONS: usize = 1_000;

// Wagi procesorów
const PROCESSOR_SPEEDS: [f64; PROCESSORS] = [1.0, 1.25, 1.5, 1.75];

// Struktura rozwiązania
#[derive(Clone)]
struct Solution {
    assignment: [usize; N], // Przypisania zadań do procesorów
    times: [f64; PROCESSORS], // Czas zakończenia pracy procesorów
    max_time: f64, // TZ
}

impl Solution {
    fn new(task_times: &[f64]) -> Self {
        let mut rng = rng();
        let mut assignment = [0; N];
        let mut times = [0.0; PROCESSORS];

        for i in 0..N {
            let proc = rng.gen_range(0..PROCESSORS);
            assignment[i] = proc;
            times[proc] += task_times[i] / PROCESSOR_SPEEDS[proc];
        }

        let max_time = *times.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        Solution { assignment, times, max_time }
    }

    fn mutate(&mut self, task_times: &[f64]) {
        let mut rng = rng();
        let task = rng.gen_range(0..N);
        let new_proc = rng.gen_range(0..PROCESSORS);
        let old_proc = self.assignment[task];

        if old_proc != new_proc {
            self.times[old_proc] -= task_times[task] / PROCESSOR_SPEEDS[old_proc];
            self.times[new_proc] += task_times[task] / PROCESSOR_SPEEDS[new_proc];
            self.assignment[task] = new_proc;
            self.max_time = *self.times.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        }
    }
}

fn main() {
    let mut rng = rng();
    let task_times: Vec<f64> = (0..N).map(|_| rng.gen_range(10.0..90.0)).collect();

    let mut best_solution = Solution::new(&task_times);
    let mut failed_mutations = 0;

    for _ in 0..MAX_ITERATIONS {
        let mut candidate = best_solution.clone();
        candidate.mutate(&task_times);

        match candidate.max_time.partial_cmp(&best_solution.max_time) {
            Some(Ordering::Less) => {
                best_solution = candidate;
                failed_mutations = 0;
            }
            _ => {
                failed_mutations += 1;
                if failed_mutations >= MAX_FAILED_MUTATIONS {
                    break;
                }
            }
        }
    }

    println!("Najlepszy czas zakończenia: {:.2}", best_solution.max_time);
    println!("Przydział zadań: {:?}", best_solution.assignment);
}
