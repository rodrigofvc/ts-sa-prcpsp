use crate::metaheuristics::state::State as State;
use crate::metaheuristics::tabu_search::tabu_mv::TabuMv as TabuMv;

/**
* Tabu search metaheuristic.
* initial_state: initial state.
* tabu_time: tabu ternure.
* neighbors: admissible neighbors to search each time.
* iterations: total iterations.
*/
pub fn tabu_search(initial_state: impl State + Clone, tabu_time: u32, neighbors: u32 , iterations: u32) -> (impl State, Vec<String>) {
    let mut log : Vec<String> = vec![];
    let mut tabu_list : Vec<TabuMv> = vec![];
    let mut current_state = initial_state;
    let mut optimum = current_state.clone();
    let mut limit = 0;
    //let mut i = 0;
    while limit != iterations {
        let (neighbor_cost, movement, activities) = best_admissible_neighbors(&mut current_state, neighbors, &mut tabu_list, &optimum);

        current_state.set_neighbor(movement);

        if current_state.get_cost() < optimum.get_cost() {
            optimum = current_state.clone();
        }

        log.push(current_state.get_cost().to_string());
        update_tabu_time(&mut tabu_list);

        let new_tabu_movement = TabuMv::new(activities, tabu_time);
        tabu_list.push(new_tabu_movement);

        println!("\n  >>>>>>>>>>> \n ");
        println!("  Ejemplar: \n {}",current_state.to_string());
        println!("  Costo: {}", current_state.get_cost());
        println!("  Iteracion: {}/{}", limit, iterations);
        println!("  Lista tabu: {:?}", tabu_list);
        println!("  Optimo {} Actual {}", optimum.get_cost(), current_state.get_cost());
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
fn best_admissible_neighbors(current_state: &mut impl State, neighbors: u32, tabu_list : &mut Vec<TabuMv>, optimum: &impl State) -> (u32, usize, Vec<u32>) {
    let (mut best_neighbor_cost, mut best_movement, mut best_activities) = current_state.get_neighbor();
    let mut admissible_neighbors = neighbors;
    let mut checked : Vec<usize> = vec![];
    let mut attemps = neighbors + neighbors/2;
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
            if aspiration_criteria(neighbor_cost, optimum) {
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

    while best_neighbor_cost == current_state.get_cost() {
        let (neighbor_cost, movement, activities) = current_state.get_neighbor();
        best_neighbor_cost = neighbor_cost;
        best_movement = movement;
        best_activities = activities.clone();
    }

    if aspiration_criteria(best_neighbor_cost, optimum) {
        let is_tabu = tabu_list.iter().any(|x| x.is_tabu(best_activities.clone()));
        if is_tabu {
            let i = tabu_list.iter().position(|x| x.movements.contains(&best_activities[0]) &&
                                                  x.movements.contains(&best_activities[1]) &&
                                                  x.movements.contains(&best_activities[2]) ).unwrap();
            tabu_list.remove(i);
        }
    }
    return (best_neighbor_cost, best_movement, best_activities);
}

/**
* If tabu movement gets a solution better than
* any other seen before, then can be accepted.
*/
fn aspiration_criteria(neighbor_cost: u32, optimum: &impl State) -> bool {
    if neighbor_cost < optimum.get_cost() {
        return true;
    }
    return false;
}

/**
* Decrease tabu time in one unit time.
*/
fn update_tabu_time(tabu_list : &mut Vec<TabuMv>) {
    let mut new_tabu_list : Vec<TabuMv> = tabu_list.clone().into_iter().filter(|x|x.tabu_time > 1).collect();
    for tabu in &mut new_tabu_list {
        tabu.tabu_time -= 1;
    }
    *tabu_list = new_tabu_list;
}
