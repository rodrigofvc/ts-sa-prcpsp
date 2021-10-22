use rand::{Rng, SeedableRng, StdRng};
use crate::prcpsp::project::Project as Project;
use crate::prcpsp::activity::Activity as Activity;
use crate::metaheuristics::state::State as State;

/**
* State representation for Simulated annealing and Tabu Search.
*/
#[derive(Clone, Debug)]
pub struct SaState {
    pub project: Project,
    rng: StdRng,
    pub planning: Vec<u32>,
    pub times: Vec<i32>
}

impl SaState {

    pub fn new(project: Project, seed: u64) -> SaState {
        SaState::get_initial_state(project, seed)
    }


    /**
    * Get an initial solution using BFS in the project network.
    * project: project to planning.
    * seed: seed for pseudo-random numbers.
    */
    fn get_initial_state(mut project: Project, seed: u64) -> SaState {
        let mut planning : Vec<u32> = vec![];
        let mut times : Vec<i32> = vec![];
        let mut queue : Vec<Activity> = vec![];
        let initial = project.activities[0].clone();
        queue.push(initial);
        let mut time : i32 = 0;
        while queue.len() != 0 {
            let current = queue.remove(0);
            let predecessors_planned = project.predecessors_planned(current.clone());
            if !predecessors_planned {
                queue.push(current);
                continue;
            }
            time = project.get_time_planning(current.clone(), time);
            while project.resource_conflict(current.clone(), time) {
                time += 1;
            }
            project.set_time(current.clone(), time);
            planning.push(current.id);
            times.push(time);
            let successors : Vec<Activity> = project.activities.clone()
                                                               .into_iter()
                                                               .filter(|x| current.successors.contains(&x.id) &&
                                                                           !queue.contains(&x)).collect();
            successors.iter().for_each(|x| queue.push(x.clone()));
        }
        return SaState{ project, rng: SeedableRng::seed_from_u64(seed), planning: planning, times: times};
    }

    fn get_planning(&mut self) {
        let mut time = 0;
        let other = self.project.clone();
        for (i,id) in self.planning.iter().enumerate() {
            let current = other.activities.iter().find(|x|x.id == *id).unwrap();
            time = self.project.get_time_planning(current.clone(), time);
            while self.project.resource_conflict(current.clone(), time) {
                time += 1;
            }
            self.project.set_time(current.clone(), time);
            self.times[i] = time;
        }
    }

    pub fn get_makespan(&self) -> u32 {
        let last = self.times.last().unwrap();
        return *last as u32;
    }

    fn get_neighbor(&mut self) -> (u32, usize) {
        loop {
            let i= self.rng.gen_range(2, self.project.activities.len()-2) as usize;
            let id_choosen_before = self.planning[i-1];
            let id_choosen = self.planning[i];
            let id_choosen_next = self.planning[i+1];
            let activity = self.project.activities.iter().find(|x| x.id == id_choosen).unwrap();
            let before_activity = self.project.activities.iter().find(|x| x.id == id_choosen_before).unwrap();
            let next_activity = self.project.activities.iter().find(|x| x.id == id_choosen_next).unwrap();
            if !activity.is_successor(next_activity.clone()) &&
               !activity.is_predecessor(before_activity.clone()) &&
               !before_activity.is_predecessor(next_activity.clone()) &&
               !next_activity.is_predecessor(before_activity.clone()) {
                   let mut neighbor = SaState { project: self.project.clone(), planning: self.planning.clone(), times: self.times.clone(), rng: self.rng.clone() };
                   neighbor.planning[i-1] = next_activity.id;
                   neighbor.planning[i] = before_activity.id;
                   neighbor.planning[i+1] = activity.id;
                   neighbor.get_planning();
                   return (neighbor.get_makespan(), i);
            }
        }
    }

    fn change_planning(&mut self, index: usize) {
        let before = self.planning[index-1];
        let current = self.planning[index];
        let next = self.planning[index+1];
        self.planning[index-1] = next;
        self.planning[index] = before;
        self.planning[index+1] = current;
        self.get_planning();
    }

    pub fn get_string(&self) -> String {
        let mut str = String::from("   [");
        for (i,p) in self.planning.iter().enumerate() {
            str.push_str(&p.to_string());
            if i != self.planning.len()-1 {
                str.push_str(&", ");
            }
        }
        str.push_str(&"]");
        str.push_str(&"\n");
        str.push_str(&"    [");
        for (i,p) in self.times.iter().enumerate() {
            str.push_str(&p.to_string());
            if i != self.planning.len()-1 {
                str.push_str(&", ");
            }
        }
        str.push_str(&"]");
        return str;
    }
}

impl State for SaState {

    fn get_neighbor(&mut self) -> (u32, usize) {
        return self.get_neighbor();
    }

    fn set_neighbor(&mut self, movement: usize) {
        self.change_planning(movement);
    }

    fn get_cost(&self) -> u32 {
        return self.get_makespan();
    }

    fn to_string(&self) -> String {
        return self.get_string();
    }
}
