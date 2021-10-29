use crate::metaheuristics::state::State as State;
use rand::{Rng, SeedableRng,StdRng};

pub fn simulated_annealing(initial_state: impl State, iterations: u32, mut temperature : f32, decrement: f32, epsilon: f32, seed: u64) -> (impl State, Vec<String>) {
    let mut log = vec![];
    let mut current_state = initial_state;
    let mut t = temperature;
    let mut total = 0;
    let mut iteration = 1;
    while t > epsilon {
        t *= decrement;
        total+=1;
    }
    let mut rng : StdRng = SeedableRng::seed_from_u64(seed);
    while temperature > epsilon {
        let mut n = 0;
        while n < iterations {
            let (neighbor_cost, movement) = current_state.get_neighbor();
            let delta = neighbor_cost as i32 - current_state.get_cost() as i32;
            if delta <= 0 {
                current_state.set_neighbor(movement);
            } else {
                let diff = (-1.0 * (delta as f32)) / temperature;
                let random : f32 = rng.gen::<f32>();
                if random < diff.exp() {
                    current_state.set_neighbor(movement);
                }
            }
            n += 1;
        }
        println!("\n  >>>>>>>>>>> \n  Temperatura actual: {} ", temperature );
        println!("  Ejemplar: \n {}",current_state.to_string());
        println!("  Costo: {}", current_state.get_cost());
        println!("  Iteracion: {}/{}", iteration, total);
        temperature *= decrement;
        iteration += 1;
        log.push(current_state.get_cost().to_string());
    }

    return (current_state, log);
}
