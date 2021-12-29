use crate::metaheuristics::state::State as State;
use rand::{Rng, SeedableRng,StdRng};

/**
* Simulated annealing metaheuristic.
*
* initial_state: initial state.
* iterations: iterations per temperature level.
* temperature: temperature.
* decrement: temperature decrement after each temperature level.
* epsilon: minimum reached temperature to finish.
* seed: seed for pseudo-random numbers.
*/
pub fn simulated_annealing(initial_state: impl State + Clone, iterations: u32, mut temperature : f32, decrement: f32, epsilon: f32, seed: u64) -> (impl State, Vec<String>) {
    let mut log = vec![];
    let mut current_state = initial_state.clone();
    let mut optimum = initial_state.clone();
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
            let (neighbor_cost, movement, _) = current_state.get_neighbor();
            if movement == 0 {
                // Cannot find another solution
                break;
            }
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
        println!("  Optimo {} Actual {}", optimum.get_cost(), current_state.get_cost());
        temperature *= decrement;
        iteration += 1;
        log.push(current_state.get_cost().to_string());
        if current_state.get_cost() < optimum.get_cost() {
            optimum = current_state.clone();
        }
    }

    return (optimum, log);
}
