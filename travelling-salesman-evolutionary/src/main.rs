use rand::{rng, Rng};
use rand::seq::SliceRandom;

const N: usize = 100;
const MIN_DIST: u32 = 10;
const MAX_DIST: u32 = 100;
const MAX_EPOCHS: usize = 10000;

// Tworzenie losowej macierzy odległości
fn generate_distance_matrix() -> [[u32; N]; N] {
    let mut rng = rng();
    let mut matrix = [[0; N]; N];
    for i in 0..N {
        for j in (i + 1)..N {
            let distance = rng.random_range(MIN_DIST..=MAX_DIST);
            matrix[i][j] = distance;
            matrix[j][i] = distance; // Brak warunku symetrii
        }
    }
    matrix
}

// Obliczanie długości trasy
fn route_length(route: &[usize], matrix: &[[u32; N]; N]) -> u32 {
    let mut length = 0;
    for i in 0..(N - 1) {
        length += matrix[route[i]][route[i + 1]];
    }
    length + matrix[route[N - 1]][route[0]] // Powrót do startowego miasta
}

// Operator inwersji
fn mutate(route: &mut Vec<usize>) {
    let mut rng = rng();
    let (i, j) = {
        let mut a = rng.random_range(0..N);
        let mut b = rng.random_range(0..N);
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        (a, b)
    };
    route[i..=j].reverse();
}

fn main() {
    let matrix = generate_distance_matrix();
    let mut rng = rng();

    // Inicjalizacja losowej trasy
    let mut parent: Vec<usize> = (0..N).collect();
    parent.shuffle(&mut rng);
    let mut parent_length = route_length(&parent, &matrix);
    
    let mut epochs = 0;
    while epochs < MAX_EPOCHS {
        let mut child = parent.clone();
        mutate(&mut child);
        let child_length = route_length(&child, &matrix);
        
        if child_length < parent_length {
            parent = child;
            parent_length = child_length;
        }
        epochs += 1;
    }
    
    println!("Najlepsza znaleziona trasa: {:?}", parent);
    println!("Długość trasy: {}", parent_length);
    println!("Liczba epok: {}", epochs);
}
