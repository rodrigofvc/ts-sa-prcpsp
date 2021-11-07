use rand::{Rng, SeedableRng, StdRng};
use crate::prcpsp::project::Project as Project;
use crate::prcpsp::activity::Activity as Activity;
use crate::metaheuristics::state::State as State;

/**
* State representation for Simulated annealing.
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

    pub fn from_project_planned(project: Project, seed: u64) -> SaState {
        SaState::get_state(project, seed)
    }

    /**
    * From a started project, create a new state.
    * project: project started.
    * seed: seed for state.
    */
    fn get_state(project: Project, seed: u64) -> SaState {
        let mut planning : Vec<u32> = vec![];
        let mut times : Vec<i32> = vec![];
        let mut activities = project.activities.clone();
        activities.sort_by_key(|x|x.start_time);
        if activities.first().unwrap() != project.activities.first().unwrap() ||
           activities.last().unwrap() != project.activities.last().unwrap()  {
            panic!("Dummy activities not found");
        }
        for activity in &activities {
            planning.push(activity.id);
            times.push(activity.start_time)
        }
        return SaState{ project, rng: SeedableRng::seed_from_u64(seed), planning: planning, times: times};
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

    /**
    * According to planning vector, for each activity,
    * get its start_time and save it in times vector.
    */
    pub fn get_planning(&mut self) {
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

    /**
    * Return finish time of project, which is latest activity.
    */
    pub fn get_makespan(&self) -> u32 {
        let last = self.times.last().unwrap();
        return *last as u32;
    }

    /**
    * Get a neighbor cost and index from current state.
    * Create a neighbor swapping three activities around a position randomly taked in planning vector.
    *
    * Return a pair (cost, index) where cost is the makespan of neighbor
    * and index is the position in planning vector where swapping create the neighbor ,
    */
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
                   let neighbor_cost = neighbor.get_makespan();
                   return (neighbor_cost, i);
            }
        }
    }

    /**
    * Update current planning swapping activities around index.
    * index: position where activities around it have to be swapped.
    */
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

    /*
    * Get an SVG file representating the state.
    **/
    pub fn get_svg(&self) -> String  {
        let mut str = String::new();
        let mut width = self.get_makespan() * 100 + 600;
        let total_resources = self.project.resources.iter().fold(0,|acc, x| acc + x.capacity );
        let height = total_resources * 100 + 600;

        let x1 = 500;
        let y1 = height-500;
        let x2 = 500 + self.get_makespan() * 100;
        let y2 = y1;

        let mut x_axis = String::from("  <line x1='");
        x_axis.push_str(&x1.to_string());
        x_axis.push_str(&"' x2='");
        x_axis.push_str(&x2.to_string());
        x_axis.push_str(&"' y1='");
        x_axis.push_str(&y1.to_string());
        x_axis.push_str(&"' y2='");
        x_axis.push_str(&y2.to_string());
        x_axis.push_str(&"' stroke='black' stroke-width='5'/>\n");

        let x1 = 500;
        let x2 = x1;
        let y2 = height-500;
        let y1 = y2 - total_resources*100;

        let mut y_axis = String::from("  <line x1='");
        y_axis.push_str(&x1.to_string());
        y_axis.push_str(&"' x2='");
        y_axis.push_str(&x2.to_string());
        y_axis.push_str(&"' y1='");
        y_axis.push_str(&y1.to_string());
        y_axis.push_str(&"' y2='");
        y_axis.push_str(&y2.to_string());
        y_axis.push_str(&"' stroke='black' stroke-width='5'/>\n");

        str.push_str(&x_axis);
        str.push_str(&y_axis);

        let mut x1 = 500;
        let y1 = height-500;
        let mut x2 = x1;
        let y2 = height-400;

        let mut tag = String::from("   <text x='");
        tag.push_str(&(width/2).to_string());
        tag.push_str(&"' y='");
        tag.push_str(&(height-200).to_string());
        tag.push_str(&"' font-size='100' text-anchor='middle'>");
        tag.push_str(&"Tiempo");
        tag.push_str(&"</text>\n");
        str.push_str(&tag);

        for x in 0..=self.get_makespan() {
            let mut limit = String::from("  <line x1='");
            limit.push_str(&x1.to_string());
            limit.push_str(&"' x2='");
            limit.push_str(&x2.to_string());
            limit.push_str(&"' y1='");
            limit.push_str(&y1.to_string());
            limit.push_str(&"' y2='");
            limit.push_str(&y2.to_string());
            limit.push_str(&"' stroke='black' stroke-width='5'/>\n");
            str.push_str(&limit);

            let mut tag = String::from("   <text x='");
            tag.push_str(&x1.to_string());
            tag.push_str(&"' y='");
            tag.push_str(&(y2 + 50).to_string());
            tag.push_str(&"' font-size='60' text-anchor='middle'>");
            tag.push_str(&x.to_string());
            tag.push_str(&"</text>\n");
            str.push_str(&tag);

            x1 += 100;
            x2 += 100;
        }

        let x1 = 400;
        let mut y1 = height-500;
        let x2 = 500;
        let mut y2 = y1;

        let mut tag = String::from("   <text x='");
        tag.push_str(&(-1*(height as i32)/2).to_string());
        tag.push_str(&"' y='");
        tag.push_str(&(200).to_string());
        tag.push_str(&"' transform='rotate(270)' font-size='100' text-anchor='middle'>");
        tag.push_str(&"Recursos");
        tag.push_str(&"</text>\n");
        str.push_str(&tag);

        for y in 0..=total_resources {
            let mut limit = String::from("  <line x1='");
            limit.push_str(&x1.to_string());
            limit.push_str(&"' x2='");
            limit.push_str(&x2.to_string());
            limit.push_str(&"' y1='");
            limit.push_str(&y1.to_string());
            limit.push_str(&"' y2='");
            limit.push_str(&y2.to_string());
            limit.push_str(&"' stroke='black' stroke-width='5'/>\n");
            str.push_str(&limit);

            let mut tag = String::from("   <text x='");
            tag.push_str(&(x1 - 50).to_string());
            tag.push_str(&"' y='");
            tag.push_str(&(y1 + 20).to_string());
            tag.push_str(&"' font-size='60' text-anchor='middle'>");
            tag.push_str(&y.to_string());
            tag.push_str(&"</text>\n");
            str.push_str(&tag);

            y1 -= 100;
            y2 -= 100;
        }

        let mut x_p = 500;
        let mut y_p = height-500;

        let mut x_done : Vec<u32> = vec![];
        let mut y_done : Vec<u32> = vec![];
        let mut width_done : Vec<u32> = vec![];
        let mut height_done : Vec<u32> = vec![];
        for i in 0..self.planning.len() {
            let id = self.planning[i];
            if id == *self.planning.first().unwrap() || id == *self.planning.last().unwrap() {
                continue;
            }

            let activity = self.project.activities.iter().find(|x|x.id == id).unwrap();

            let width_rectangle = activity.duration * 100;
            let height_rectangle = activity.get_demand() * 100;

            y_p -= height_rectangle;
            let mut check_again = true;

            while check_again {
                check_again = false;
                for i in 0..x_done.len() {
                    let x_i = x_done[i];
                    let y_i = y_done[i];
                    let width_i = width_done[i];
                    let height_i = height_done[i];
                    loop {
                        if x_i == x_p || (x_i < x_p && x_p < (x_i + width_i) ) {
                            if y_i == y_p
                            || (y_i < y_p && y_p + height_rectangle <= y_i + height_i )
                            || (y_p < y_i && y_p + height_rectangle >= y_i + height_i )
                            || (y_i < y_p && y_p + height_rectangle >= y_i + height_i && y_i + height_i > y_p  )
                                {
                                if height_rectangle > y_i {
                                    y_p = 0;
                                    break;
                                }
                                y_p = y_i - height_rectangle;
                                check_again = true;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }

            let mut rectangle = String::from("  <rect x='");
            rectangle.push_str(&(x_p).to_string());
            rectangle.push_str(&"' y='");
            rectangle.push_str(&(y_p).to_string());
            rectangle.push_str(&"' width='");
            rectangle.push_str(&width_rectangle.to_string());
            rectangle.push_str(&"' height='");
            rectangle.push_str(&height_rectangle.to_string());
            rectangle.push_str(&"' fill='rgb(100, 149, 237)'");
            rectangle.push_str(&" stroke='black' stroke-width='.7mm' />\n");


            let mut id_rectangle = String::from("   <text x='");
            id_rectangle.push_str(&(width_rectangle/2 + x_p).to_string());
            id_rectangle.push_str(&"' y='");
            id_rectangle.push_str(&(height_rectangle/2 + y_p).to_string());
            id_rectangle.push_str(&"' font-size='50' text-anchor='middle'>");
            if activity.parent != -1 {
                id_rectangle.push_str(&activity.parent.to_string());
            } else {
                id_rectangle.push_str(&activity.id.to_string());
            }
            id_rectangle.push_str(&"</text>\n");
            str.push_str(&rectangle);
            str.push_str(&id_rectangle);

            x_done.push(x_p);
            y_done.push(y_p);
            width_done.push(width_rectangle);
            height_done.push(height_rectangle);

            if i+1 != self.planning.len() {
                if self.times[i+1] != self.times[i] {
                    x_p = self.times[i+1] as u32 * 100 + 500;
                    y_p = height-500;
                }
            }
        }

        width = x_p + 100;
        let mut file = String::from("<svg version='1.1' width='");
        file.push_str(&width.to_string());
        file.push_str(&"' height='");
        file.push_str(&height.to_string());
        file.push_str(&"' xmlns='http://www.w3.org/2000/svg'>\n");
        file.push_str(&str);
        file.push_str("</svg>");

        return file;
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

    fn to_file(&self) -> String {
        return self.get_svg();
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
