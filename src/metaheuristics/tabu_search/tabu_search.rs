use crate::metaheuristics::state::State as State;
use crate::metaheuristics::tabu_search::tabu_mv::TabuMv as TabuMv;

/**
* Tabu search metaheuristic.
* initial_state: initial state.
* tabu_time: tabu ternure.
* neighbors: admissible neighbors to search each time.
* iterations: total iterations.
* diversification_factor: use for penalty and get a diversification.
*/
pub fn tabu_search(initial_state: impl State + Clone, tabu_time: u32, neighbors: u32 , iterations: u32, diversification_factor : u32 ) -> (impl State, Vec<String>) {
    let mut log : Vec<String> = vec![];
    let mut tabu_list : Vec<TabuMv> = vec![];
    let mut current_state = initial_state;
    let mut optimum = current_state.clone();
    let mut limit = 0;
    let no_changes = 3;
    let mut i = 0;
    let p = 10;
    let mut penalty = 0;
    while limit != iterations {
        let (neighbor_cost, movement, activities) = best_admissible_neighbors(&mut current_state, neighbors, &mut tabu_list, penalty);
        if neighbor_cost < current_state.get_cost() + penalty && movement != 0 {
            current_state.set_neighbor(movement);
            let new_tabu_movement = TabuMv::new(activities, tabu_time + 1);
            tabu_list.push(new_tabu_movement);
            i = 0;
            penalty = 0;
        } else {
            if i == no_changes {
                penalty = diversification_factor * p;
                i = 0;
            } else {
                i += 1;
            }
        }
        if current_state.get_cost() < optimum.get_cost() {
            optimum = current_state.clone();
        }
        log.push(current_state.get_cost().to_string());
        update_tabu_time(&mut tabu_list);
        println!("\n  >>>>>>>>>>> \n ");
        println!("  Ejemplar: \n {}",current_state.to_string());
        println!("  Costo: {}", current_state.get_cost());
        println!("  Iteracion: {}/{}", limit, iterations);
        println!("  Tabu list {:?}", tabu_list);
        println!("  Penalty {:?}",penalty);
        println!("  Opt {} Cur {:?}", optimum.get_cost(), current_state.get_cost());
        limit += 1;
    }
    return (optimum, log);
}

/**
* Check only admissible neighbors (non-tabu or allowed by aspiration criteria).
* current_state: current state.
* neighbors: admissible neighbors to search.
* tabu_list: tabu struct.
* penalty: penalty for diversification.
*/
fn best_admissible_neighbors(current_state: &mut impl State, neighbors: u32, tabu_list : &mut Vec<TabuMv>, penalty: u32) -> (u32, usize, Vec<u32>) {
    let mut best_movement = 0;
    let mut best_neighbor_cost = current_state.get_cost() + penalty;
    let mut best_activities = vec![];
    let mut admissible_neighbors = neighbors;
    let mut checked : Vec<usize> = vec![];
    let mut attemps = neighbors * 2;
    while admissible_neighbors != 0 && attemps != 0 {
        attemps -= 1;
        let (neighbor_cost, movement, activities) = current_state.get_neighbor();

        let movement_checked = checked.iter().find(|x| **x == movement);
        match movement_checked {
            Some(_) => continue,
            None => checked.push(movement)
        }

        let is_tabu = tabu_list.iter().any(|x| x.is_tabu(activities.clone()));
        if is_tabu {
            if aspiration_criteria(neighbor_cost, current_state) {
                best_neighbor_cost = neighbor_cost;
                best_movement = movement;
                best_activities = activities.clone();
            } else {
                continue;
            }
        } else {
            if neighbor_cost < best_neighbor_cost && neighbor_cost != current_state.get_cost() {
                best_neighbor_cost = neighbor_cost;
                best_movement = movement;
                best_activities = activities.clone();
            }
        }
        admissible_neighbors -= 1;
    }
    return (best_neighbor_cost, best_movement, best_activities);
}

/**
* If tabu movement gets a solution better than
* any other seen before, then can be accepted.
*/
fn aspiration_criteria(neighbor_cost: u32, current_state: &impl State) -> bool {
    if neighbor_cost < current_state.get_cost() {
        return true;
    }
    return false;
}


fn update_tabu_time(tabu_list : &mut Vec<TabuMv>) {
    let mut new_tabu_list : Vec<TabuMv> = tabu_list.clone().into_iter().filter(|x|x.tabu_time > 1).collect();
    for tabu in &mut new_tabu_list {
        tabu.tabu_time -= 1;
    }
    *tabu_list = new_tabu_list;
}
