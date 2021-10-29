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

#[cfg(test)]
 mod tests {
     use crate::prcpsp::project::Project as Project;
     use crate::prcpsp::activity::Activity as Activity;
     use crate::prcpsp::rn_resource::RnResource as RnResource;
     use crate::prcpsp::rn_resource_usage::RnResourceUsage as RnResourceUsage;
     use crate::prcpsp::sa_state::SaState as SaState;

     fn initial() -> Project {
         let mut initial = Activity::new(1,-1,String::from("1"),vec![],vec![],vec![],0,-1);
         let mut last = Activity::new(9,-1,String::from("9"),vec![],vec![],vec![],0,-1);

         let resource = RnResource::new(1, String::from("1"), 5);

         let a_r = RnResourceUsage::new(resource.clone(),1);
         let mut a = Activity::new(2,-1,String::from("2"),vec![],vec![],vec![a_r],1,-1);
         a.add_anteccessor(initial.id);
         initial.add_successor(a.id);

         let b_r = RnResourceUsage::new(resource.clone(),2);
         let mut b = Activity::new(3,-1,String::from("3"),vec![],vec![],vec![b_r],2,-1);
         b.add_anteccessor(initial.id);
         initial.add_successor(b.id);
         last.add_anteccessor(b.id);
         b.add_successor(last.id);

         let c_r = RnResourceUsage::new(resource.clone(),2);
         let mut c = Activity::new(4,-1,String::from("4"),vec![],vec![],vec![c_r],4,-1);
         c.add_anteccessor(initial.id);
         initial.add_successor(c.id);

         last.add_anteccessor(c.id);
         c.add_successor(last.id);

         let d_r = RnResourceUsage::new(resource.clone(),2);
         let mut d = Activity::new(5,-1,String::from("5"),vec![],vec![],vec![d_r],3,-1);
         d.add_anteccessor(initial.id);
         initial.add_successor(d.id);

         let e_r = RnResourceUsage::new(resource.clone(),2);
         let mut e = Activity::new(6,-1,String::from("6"),vec![],vec![],vec![e_r],1,-1);
         e.add_anteccessor(a.id);
         a.add_successor(e.id);

         let f_r = RnResourceUsage::new(resource.clone(),1);
         let mut f = Activity::new(7,-1,String::from("7"),vec![],vec![],vec![f_r],5,-1);
         f.add_anteccessor(e.id);
         e.add_successor(f.id);
         last.add_anteccessor(f.id);
         f.add_successor(last.id);

         let g_r = RnResourceUsage::new(resource.clone(),2);
         let mut g = Activity::new(8,-1,String::from("8"),vec![],vec![],vec![g_r],3,-1);
         g.add_anteccessor(d.id);
         d.add_successor(g.id);
         last.add_anteccessor(g.id);
         g.add_successor(last.id);

         let activities = vec![initial,a,b,c,d,e,f,g,last];
         let resources = vec![resource];
         let project = Project::new(activities, resources);
         project
     }

     // Another example of project.
     fn initial_1() -> Project {
         let mut initial = Activity::new(1,-1,String::from("1"),vec![],vec![],vec![],0,-1);
         let mut last = Activity::new(9,-1,String::from("9"),vec![],vec![],vec![],0,-1);

         let resource = RnResource::new(1, String::from("1"), 5);

         let a_r = RnResourceUsage::new(resource.clone(),1);
         let mut a = Activity::new(2,-1,String::from("2"),vec![],vec![],vec![a_r],1,-1);
         a.add_anteccessor(initial.id);
         initial.add_successor(a.id);

         let b_r = RnResourceUsage::new(resource.clone(),2);
         let mut b = Activity::new(3,-1,String::from("3"),vec![],vec![],vec![b_r],2,-1);
         b.add_anteccessor(initial.id);
         initial.add_successor(b.id);
         last.add_anteccessor(b.id);
         b.add_successor(last.id);

         b.add_anteccessor(a.id);
         a.add_successor(b.id);

         let c_r = RnResourceUsage::new(resource.clone(),2);
         let mut c = Activity::new(4,-1,String::from("4"),vec![],vec![],vec![c_r],4,-1);
         c.add_anteccessor(initial.id);
         initial.add_successor(c.id);

         last.add_anteccessor(c.id);
         c.add_successor(last.id);

         let d_r = RnResourceUsage::new(resource.clone(),2);
         let mut d = Activity::new(5,-1,String::from("5"),vec![],vec![],vec![d_r],3,-1);
         d.add_anteccessor(initial.id);
         initial.add_successor(d.id);

         d.add_successor(c.id);
         c.add_anteccessor(d.id);

         let e_r = RnResourceUsage::new(resource.clone(),2);
         let mut e = Activity::new(6,-1,String::from("6"),vec![],vec![],vec![e_r],1,-1);
         e.add_anteccessor(a.id);
         a.add_successor(e.id);

         let f_r = RnResourceUsage::new(resource.clone(),1);
         let mut f = Activity::new(7,-1,String::from("7"),vec![],vec![],vec![f_r],5,-1);
         f.add_anteccessor(e.id);
         e.add_successor(f.id);
         last.add_anteccessor(f.id);
         f.add_successor(last.id);
         f.add_anteccessor(b.id);
         b.add_successor(f.id);

         let g_r = RnResourceUsage::new(resource.clone(),2);
         let mut g = Activity::new(8,-1,String::from("8"),vec![],vec![],vec![g_r],3,-1);
         g.add_anteccessor(d.id);
         d.add_successor(g.id);
         last.add_anteccessor(g.id);
         g.add_successor(last.id);

         c.add_anteccessor(g.id);
         g.add_successor(c.id);

         let activities = vec![initial,a,b,c,d,e,f,g,last];
         let resources = vec![resource];
         let project = Project::new(activities, resources);
         project
     }

     #[test]
     fn test_initial_planning() {
         let project = initial();
         let initial = SaState::new(project.clone(), 11);
         let planning = vec![1,2,3,4,5,6,8,7,9];
         let times = vec![0,0,0,0,2,4,5,5,10];
         assert_eq!(planning, initial.planning);
         assert_eq!(times, initial.times);

         let p_project = project.pre_emptive_project();
         let initial = SaState::new(p_project, 11);
         let planning = vec![1,2,3,5,9,12,4,6,10,13,7,11,14,8,18,15,19,16,20,17,21];
         let times = vec![0,0,0,0,1,1,2,2,3,3,3,4,4,4,5,5,6,6,7,7,8];
         assert_eq!(planning, initial.planning);
         assert_eq!(times, initial.times);

         let project = initial_1();
         let initial = SaState::new(project.clone(), 11);
         let planning = vec![1,2,3,5,6,7,8,4,9];
         let times = vec![0,0,1,1,3,4,4,7,11];
         assert_eq!(planning, initial.planning);
         assert_eq!(times, initial.times);

         let p_project = project.pre_emptive_project();
         let initial = SaState::new(p_project, 11);
         let planning = vec![1,2,3,9,12,4,10,13,11,14,18,15,19,16,20,17,5,6,7,8,21];
         let times = vec![0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7,8,9,10,11,12];
         assert_eq!(planning, initial.planning);
         assert_eq!(times, initial.times);
     }

     /**
     * Check if every activity in project has the correct
     * start_time according to times vector in SaState
     * state: state to check.
     */
     #[test]
     fn test_state() {
         let project = initial();
         let state_1 = SaState::new(project.clone(), 11);

         let project_1 = initial_1();
         let state_2 = SaState::new(project_1.clone(), 11);

         let states = vec![state_1, state_2];

         for state in states {
             assert_eq!(state.times.len(), state.planning.len());
             for (i,id) in state.planning.iter().enumerate() {
                 let activity = state.project.activities.iter().find(|x|x.id == *id ).unwrap();
                 let start_time_planning = state.times[i];
                 assert_eq!(activity.start_time, start_time_planning);
                 for pred in &activity.predecessors {
                     let predecessor = state.project.activities.iter().find(|x| x.id == *pred ).unwrap();
                     assert_ne!(predecessor.start_time, -1);
                     let end_time = predecessor.start_time + predecessor.duration as i32;
                     assert!(end_time <= start_time_planning);
                 }
             }
         }
     }

     #[test]
     fn test_get_neighbor() {
        let project = initial();
        let mut state = SaState::new(project.clone(), 11);
        let (cost_neighbor, i) = state.get_neighbor();
        let before = state.planning[i-1];
        let current = state.planning[i];
        let next = state.planning[i+1];
        state.planning[i-1] = next;
        state.planning[i] = before;
        state.planning[i+1] = current;
        state.get_planning();
        let expected = state.times.last().unwrap();
        assert_eq!(cost_neighbor, *expected as u32);
        let activity = state.project.activities.iter().find(|x| x.id == current).unwrap();
        let before_activity = state.project.activities.iter().find(|x| x.id == before).unwrap();
        let next_activity = state.project.activities.iter().find(|x| x.id == next).unwrap();
        assert!(!activity.is_successor(before_activity.clone()));
        assert!(!activity.is_predecessor(before_activity.clone()));
        assert!(!activity.is_successor(next_activity.clone()));
        assert!(!activity.is_predecessor(next_activity.clone()));
        assert!(!before_activity.is_predecessor(next_activity.clone()));
     }

     #[test]
     fn test_planning() {
        let project = initial();
        let mut state = SaState::new(project.clone(), 11);
        state.get_planning();
        let expected = vec![0,0,0,0,2,4,5,5,10];
        assert_eq!(expected, state.times);

        let project = project.pre_emptive_project();
        let mut state = SaState::new(project.clone(), 11);
        state.get_planning();
        let expected = vec![0,0,0,0,1,1,2,2,3,3,3,4,4,4,5,5,6,6,7,7,8];
        assert_eq!(expected, state.times);

        let project = initial_1();
        let mut state = SaState::new(project.clone(), 11);
        state.get_planning();
        let expected = vec![0,0,1,1,3,4,4,7,11];
        assert_eq!(expected, state.times);

        let project = project.pre_emptive_project();
        let mut state = SaState::new(project.clone(), 11);
        state.get_planning();
        let expected = vec![0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7,8,9,10,11,12];
        assert_eq!(expected, state.times);
     }

    #[test]
    fn test_set_movement() {
        let project = initial();
        let mut state = SaState::new(project.clone(), 11);
        state.change_planning(4);
        assert_eq!(state.planning, vec![1,2,3,6,4,5,8,7,9]);

        let project = project.pre_emptive_project();
        let mut state = SaState::new(project.clone(), 11);
        state.change_planning(7);
        assert_eq!(state.planning, vec![1,2,3,5,9,12,10,4,6,13,7,11,14,8,18,15,19,16,20,17,21]);

        let project = initial_1();
        let mut state = SaState::new(project.clone(), 11);
        state.change_planning(3);
        assert_eq!(state.planning, vec![1,2,6,3,5,7,8,4,9]);

        let project = project.pre_emptive_project();
        let mut state = SaState::new(project.clone(), 11);
        state.change_planning(5);
        assert_eq!(state.planning, vec![1,2,3,9,10,12,4,13,11,14,18,15,19,16,20,17,5,6,7,8,21]);
    }
 }
