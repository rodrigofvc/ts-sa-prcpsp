use crate::prcpsp::activity::Activity as Activity;
use crate::prcpsp::rn_resource::RnResource as RnResource;
use crate::prcpsp::rn_resource_usage::RnResourceUsage as RnResourceUsage;


/** Represents the project network for scheduling problems **/
#[derive(Debug,Clone)]
pub struct Project {
    pub activities : Vec<Activity>,
    pub resources : Vec<RnResource>
}

/*
* Subactivities for an activity.
* (Original,[subactivity_1, subactivity_2,..])
*/
#[derive(Debug,Clone)]
struct ActivitySubs(Activity, Vec<Activity>);

impl Project {

    pub fn new(activities : Vec<Activity>, resources: Vec<RnResource>) -> Project {
        Project { activities, resources }
    }

    /**
    * From the current project, create a new project where
    * activities are split in subactivities with duration 1.
    */
    pub fn pre_emptive_project(&self) -> Project {
        let pair_activities : Vec<ActivitySubs> = self.split_activities();
        let subactivities : Vec<Activity> = self.set_neighbor_subactivities(pair_activities);
        let resources = self.resources.clone();
        Project { activities: subactivities, resources }
    }

    /**
    * From the current activities, split each one in subactivities of duration 1.
    * Return a list of tuples, containing the original activity and its subactivities in a vector.
    * [ActivitySubs_1,..,ActivitySubs_n]
    */
    fn split_activities(&self) -> Vec<ActivitySubs> {
        let mut pair_activities : Vec<ActivitySubs> = vec![];
        let mut subactivities_count = 2;
        let initial = self.activities[0].clone();
        let initial_subactivity = Activity::new(1,1,String::from("1"),vec![],vec![],vec![],0,-1);
        pair_activities.push(ActivitySubs(initial, vec![initial_subactivity]));

        for i in 1..self.activities.len()-1 {
            let activity = self.activities[i].clone();
            let mut subs : Vec<Activity> = vec![];
            for j in 1..=activity.duration {
                let mut subactivity = Activity::new(subactivities_count,activity.id as i32,subactivities_count.to_string(),vec![],vec![],vec![],1,-1);
                subactivity.supplies = activity.clone().supplies;
                subactivities_count+=1;
                if j > 1 {
                    let mut anteccessor = subs.pop().unwrap();
                    subactivity.add_anteccessor(anteccessor.id);
                    anteccessor.add_successor(subactivity.id);
                    subs.push(anteccessor);
                    subs.push(subactivity);
                } else {
                    subs.push(subactivity);
                }
            }
            pair_activities.push(ActivitySubs(activity, subs));
        }

        let last = self.activities[self.activities.len()-1].clone();
        let mut last_subactivity = Activity::new(subactivities_count,-1,subactivities_count.to_string(),vec![],vec![],vec![],0,-1);
        last_subactivity.parent = last.id as i32;
        pair_activities.push(ActivitySubs(last,vec![last_subactivity]));

        pair_activities
    }

    /**
    * For each ActivitySubs tuple, set predecessors and successors of first
    * and last subactivity respectively from current activity.
    * pair_activities: vector to set anteccessor and successor to subactivities.
    */
    fn set_neighbor_subactivities(&self, pair_activities: Vec<ActivitySubs>) -> Vec<Activity> {
        let mut subactivities : Vec<Activity> = vec![];
        for pair in &pair_activities {
            let activity = pair.0.clone();
            let mut subs = pair.1.clone();
            let pair_predecessors : Vec<ActivitySubs> = pair_activities.clone().into_iter().filter(|x| activity.predecessors.contains(&x.0.id)).collect();
            let mut first = subs.remove(0);
            for pair_a in &pair_predecessors {
                let last_anteccessor = pair_a.1.last().unwrap();
                first.add_anteccessor(last_anteccessor.id);
            }
            subs.insert(0,first);
            let mut last = subs.pop().unwrap();
            let pair_succesors : Vec<ActivitySubs> = pair_activities.clone().into_iter().filter(|x| activity.successors.contains(&x.0.id)).collect();
            for pair_s in &pair_succesors {
                let first_successor = pair_s.1.first().unwrap();
                last.add_successor(first_successor.id);
            }
            subs.push(last);
            subactivities.append(&mut subs);
        }
        subactivities
    }

    /**
    * Given an activity, set its start_time in project.
    * activity: activity to change start_time.
    * time: time to assign
    */
    pub fn set_time(&mut self, activity: Activity, time: i32) {
        let get_index = self.activities.iter().position(|x| x.clone() == activity);
        match get_index {
            Some(index) => { self.activities[index].start_time = time },
            _ => panic!(),
        }
    }

    /**
    * Get the time where activity can be planned.
    * Return the maximum between the latest end time of predecessors
    * in current activity and the current time.
    * activity: activity to check.
    * time: time where activity can be planned.
    */
    pub fn get_time_planning(&self, activity: Activity, time: i32) -> i32 {
        let mut max_time_predecessor : i32 = 0;
        for pre in activity.predecessors {
            let predecessor = self.activities.iter().find(|x|x.id == pre).unwrap();
            if predecessor.start_time == -1 {
                panic!("Predecessor should be planned");
            }
            let end_time = predecessor.start_time + predecessor.duration as i32;
            if end_time > max_time_predecessor {
                max_time_predecessor = end_time;
            }
        }
        if max_time_predecessor < time {
            max_time_predecessor = time;
        }
        return max_time_predecessor;
    }

    /**
    * Given an activity, check if all its predecessors have been planned.
    * An activity is planned if its start_time isn't -1.
    * activity: activity to check.
    */
    pub fn predecessors_planned(&self, activity: Activity) -> bool {
        for pred in activity.predecessors {
            let predecessor = self.activities.iter().find(|x| x.id == pred).unwrap();
            if predecessor.start_time == -1 {
                return false;
            }
        }
        return true;
    }

    /**
    * Check if a conflict occurs when try to planning
    * an activity in a specified time.
    * activity: activity to check.
    * time: time where activity can be planned.
    */
    pub fn resource_conflict(&self, activity: Activity, time: i32) -> bool {
        let activities_time : Vec<Activity> = self.activities.clone()
                                                             .into_iter()
                                                             .filter(|x| *x  != activity && (x.start_time == time ||
                                                                        (x.start_time < time && time < x.start_time + x.duration as i32 && x.start_time != -1)) ).collect();
        if activities_time.len() == 0 {
            return false;
        }

        let supplies_used : Vec<Vec<RnResourceUsage>> = activities_time.iter().map(|x| x.supplies.clone() ).collect();
        let resources_used : Vec<RnResource> = activity.supplies.iter().map(|x|x.resource.clone()).collect();
        for resource in resources_used {
            let mut demand = 0;
            for supplies in &supplies_used {
                for supply in supplies {
                    if supply.resource == resource {
                        demand += supply.usage;
                    }
                }
            }
            let supply = activity.supplies.iter().find(|&x|x.resource == resource).unwrap();
            let usage = supply.usage;
            if demand + usage  >  resource.capacity {
                return true;
            }
        }
        return false;
    }

}



#[cfg(test)]
 mod tests {
     use crate::prcpsp::project::Project as Project;
     use crate::prcpsp::activity::Activity as Activity;
     use crate::prcpsp::rn_resource::RnResource as RnResource;
     use crate::prcpsp::rn_resource_usage::RnResourceUsage as RnResourceUsage;

     // Example of project.
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
     fn test_split_activities() {
         let project = initial();
         let pairs = project.split_activities();
         for pair in pairs {
             let activity = pair.0;
             let subactivities = pair.1;
             if activity.id != 1 && activity.id != 9  {
                 assert_eq!(activity.duration, subactivities.len() as u32);
             }
         }
     }

     #[test]
     fn test_set_neighbor_subactivities_project () {
         let project = initial();
         let pre_emptive_project = project.pre_emptive_project();
         let subactivities = pre_emptive_project.activities;
         let subactivity = subactivities.iter().find(|x|x.id == 1).unwrap();
         assert!(subactivity.predecessors.is_empty());
         assert_subactivity(subactivity.clone(), 1, vec![], vec![2,3,5,9]);

         let subactivity = subactivities.iter().find(|x|x.id == 2).unwrap();
         assert_subactivity(subactivity.clone(), 2, vec![1], vec![12]);

         let subactivity = subactivities.iter().find(|x|x.id == 3).unwrap();
         assert_subactivity(subactivity.clone(), 3, vec![1], vec![4]);

         let subactivity = subactivities.iter().find(|x|x.id == 4).unwrap();
         assert_subactivity(subactivity.clone(), 3, vec![3], vec![21]);

         let subactivity = subactivities.iter().find(|x|x.id == 5).unwrap();
         assert_subactivity(subactivity.clone(), 4, vec![1], vec![6]);

         let subactivity = subactivities.iter().find(|x|x.id == 6).unwrap();
         assert_subactivity(subactivity.clone(), 4, vec![5], vec![7]);

         let subactivity = subactivities.iter().find(|x|x.id == 7).unwrap();
         assert_subactivity(subactivity.clone(), 4, vec![6], vec![8]);

         let subactivity = subactivities.iter().find(|x|x.id == 8).unwrap();
         assert_subactivity(subactivity.clone(), 4, vec![7], vec![21]);

         let subactivity = subactivities.iter().find(|x|x.id == 9).unwrap();
         assert_subactivity(subactivity.clone(), 5, vec![1], vec![10]);

         let subactivity = subactivities.iter().find(|x|x.id == 10).unwrap();
         assert_subactivity(subactivity.clone(), 5, vec![9], vec![11]);

         let subactivity = subactivities.iter().find(|x|x.id == 11).unwrap();
         assert_subactivity(subactivity.clone(), 5, vec![10], vec![18]);

         let subactivity = subactivities.iter().find(|x|x.id == 12).unwrap();
         assert_subactivity(subactivity.clone(), 6, vec![2], vec![13]);

         let subactivity = subactivities.iter().find(|x|x.id == 13).unwrap();
         assert_subactivity(subactivity.clone(), 7, vec![12], vec![14]);

         let subactivity = subactivities.iter().find(|x|x.id == 14).unwrap();
         assert_subactivity(subactivity.clone(), 7, vec![13], vec![15]);

         let subactivity = subactivities.iter().find(|x|x.id == 15).unwrap();
         assert_subactivity(subactivity.clone(), 7, vec![14], vec![16]);

         let subactivity = subactivities.iter().find(|x|x.id == 16).unwrap();
         assert_subactivity(subactivity.clone(), 7, vec![15], vec![17]);

         let subactivity = subactivities.iter().find(|x|x.id == 17).unwrap();
         assert_subactivity(subactivity.clone(), 7, vec![16], vec![21]);

         let subactivity = subactivities.iter().find(|x|x.id == 18).unwrap();
         assert_subactivity(subactivity.clone(), 8, vec![11], vec![19]);

         let subactivity = subactivities.iter().find(|x|x.id == 19).unwrap();
         assert_subactivity(subactivity.clone(), 8, vec![18], vec![20]);

         let subactivity = subactivities.iter().find(|x|x.id == 20).unwrap();
         assert_subactivity(subactivity.clone(), 8, vec![19], vec![21]);

         let subactivity = subactivities.iter().find(|x|x.id == 21).unwrap();
         assert_subactivity(subactivity.clone(), 9, vec![17,4,8,20], vec![]);
     }

     #[test]
     fn test_set_neighbor_subactivities_project_1 () {
         let project = initial_1();
         let pre_emptive_project = project.pre_emptive_project();
         let subactivities = pre_emptive_project.activities;
         let subactivity = subactivities.iter().find(|x|x.id == 1).unwrap();
         assert!(subactivity.predecessors.is_empty());
         assert_subactivity(subactivity.clone(), 1, vec![], vec![2,3,5,9]);

         let subactivity = subactivities.iter().find(|x|x.id == 2).unwrap();
         assert_subactivity(subactivity.clone(), 2, vec![1], vec![12,3]);

         let subactivity = subactivities.iter().find(|x|x.id == 3).unwrap();
         assert_subactivity(subactivity.clone(), 3, vec![1,2], vec![4]);

         let subactivity = subactivities.iter().find(|x|x.id == 4).unwrap();
         assert_subactivity(subactivity.clone(), 3, vec![3], vec![21,13]);

         let subactivity = subactivities.iter().find(|x|x.id == 5).unwrap();
         assert_subactivity(subactivity.clone(), 4, vec![1,11,20], vec![6]);

         let subactivity = subactivities.iter().find(|x|x.id == 6).unwrap();
         assert_subactivity(subactivity.clone(), 4, vec![5], vec![7]);

         let subactivity = subactivities.iter().find(|x|x.id == 7).unwrap();
         assert_subactivity(subactivity.clone(), 4, vec![6], vec![8]);

         let subactivity = subactivities.iter().find(|x|x.id == 8).unwrap();
         assert_subactivity(subactivity.clone(), 4, vec![7], vec![21]);

         let subactivity = subactivities.iter().find(|x|x.id == 9).unwrap();
         assert_subactivity(subactivity.clone(), 5, vec![1], vec![10]);

         let subactivity = subactivities.iter().find(|x|x.id == 10).unwrap();
         assert_subactivity(subactivity.clone(), 5, vec![9], vec![11]);

         let subactivity = subactivities.iter().find(|x|x.id == 11).unwrap();
         assert_subactivity(subactivity.clone(), 5, vec![10], vec![18,5]);

         let subactivity = subactivities.iter().find(|x|x.id == 12).unwrap();
         assert_subactivity(subactivity.clone(), 6, vec![2], vec![13]);

         let subactivity = subactivities.iter().find(|x|x.id == 13).unwrap();
         assert_subactivity(subactivity.clone(), 7, vec![12,4], vec![14]);

         let subactivity = subactivities.iter().find(|x|x.id == 14).unwrap();
         assert_subactivity(subactivity.clone(), 7, vec![13], vec![15]);

         let subactivity = subactivities.iter().find(|x|x.id == 15).unwrap();
         assert_subactivity(subactivity.clone(), 7, vec![14], vec![16]);

         let subactivity = subactivities.iter().find(|x|x.id == 16).unwrap();
         assert_subactivity(subactivity.clone(), 7, vec![15], vec![17]);

         let subactivity = subactivities.iter().find(|x|x.id == 17).unwrap();
         assert_subactivity(subactivity.clone(), 7, vec![16], vec![21]);

         let subactivity = subactivities.iter().find(|x|x.id == 18).unwrap();
         assert_subactivity(subactivity.clone(), 8, vec![11], vec![19]);

         let subactivity = subactivities.iter().find(|x|x.id == 19).unwrap();
         assert_subactivity(subactivity.clone(), 8, vec![18], vec![20]);

         let subactivity = subactivities.iter().find(|x|x.id == 20).unwrap();
         assert_subactivity(subactivity.clone(), 8, vec![19], vec![21,5]);

         let subactivity = subactivities.iter().find(|x|x.id == 21).unwrap();
         assert_subactivity(subactivity.clone(), 9, vec![17,4,8,20], vec![]);
     }

     /**
     * Assert if subactivity has a specified parent, predecessors, successors.
     */
     fn assert_subactivity(subactivity: Activity, parent: i32, predecessors: Vec<u32>, successors: Vec<u32>){
         assert_eq!(subactivity.parent, parent);
         assert_eq!(subactivity.predecessors.len(), predecessors.len());
         assert_eq!(subactivity.successors.len(), successors.len());
         if subactivity.id != 1 && subactivity.id != 21 {
             assert_eq!(subactivity.duration, 1);
         }
         assert_contains_antecessor(subactivity.clone(), predecessors);
         assert_contains_successor(subactivity, successors);
     }

     /**
     * Assert if activity has elements as successors.
     */
     fn assert_contains_successor (activity: Activity, elements: Vec<u32>) {
         for i in elements {
             assert!(activity.successors.contains(&i));
         }
     }

     /**
     * Assert if activity has elements as predecessors.
     */
     fn assert_contains_antecessor (activity: Activity, elements: Vec<u32>) {
         for i in elements {
             assert!(activity.predecessors.contains(&i));
         }
     }

     #[test]
     fn test_get_time_planning() {
         let mut project = initial();
         let planning = vec![1,2,3,4,5,6,8,7,9];
         let times = vec![0,0,0,0,2,4,5,5,10];
         for activity in &mut project.activities {
             let j = planning.iter().position(|x|*x == activity.id).unwrap();
             let time = times[j];
             activity.start_time = time;
         }

         let input = vec![0,0,0,0,2,4,3,4,4];
         for activity in &project.activities {
             let j = planning.iter().position(|x|*x == activity.id).unwrap();
             let time = times[j];
             let index = (activity.id-1) as usize;
             assert_eq!(project.get_time_planning(activity.clone(), input[index]) , time);
         }

         let mut p_project = project.pre_emptive_project();
         let planning = vec![1,2,3,5,9,12,4,6,10,13,7,11,14,8,18,15,19,16,20,17,21];
         let times = vec![0,0,0,0,1,1,2,2,3,3,3,4,4,4,5,5,6,6,7,7,8];

         for activity in &mut p_project.activities {
             let j = planning.iter().position(|x|*x == activity.id).unwrap();
             let time = times[j];
             activity.start_time = time;
         }

         let input = vec![0,0,0,2,0,2,2,2,1,3,2,1,3,1,1,1,1,2,1,1,1];
         for activity in &p_project.activities {
             let j = planning.iter().position(|x|*x == activity.id).unwrap();
             let time = times[j];
             let index = (activity.id-1) as usize;
             assert_eq!(p_project.get_time_planning(activity.clone(), input[index]) , time);
         }

         let mut project = initial_1();
         let planning = vec![1,2,3,5,6,7,8,4,9];
         let times = vec![0,0,1,1,3,4,4,7,11];

         for activity in &mut project.activities {
             let j = planning.iter().position(|x|*x == activity.id).unwrap();
             let time = times[j];
             activity.start_time = time;
         }

         let input = vec![0,0,0,1,1,3,2,3,1];
         for activity in &project.activities {
             let j = planning.iter().position(|x|*x == activity.id).unwrap();
             let time = times[j];
             let index = (activity.id-1) as usize;
             assert_eq!(project.get_time_planning(activity.clone(), input[index]) , time);
         }

         let mut p_project = project.pre_emptive_project();
         let planning = vec![1,2,3,9,12,4,10,13,11,14,18,15,19,16,20,17,5,6,7,8,21];
         let times = vec![0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7,8,9,10,11,12];

         for activity in &mut p_project.activities {
             let j = planning.iter().position(|x|*x == activity.id).unwrap();
             let time = times[j];
             activity.start_time = time;
         }

         let input = vec![0,0,0,0,8,8,8,8,1,3,2,2,2,2,3,4,4,4,5,5,1];
         for activity in &p_project.activities {
             let j = planning.iter().position(|x|*x == activity.id).unwrap();
             let time = times[j];
             let index = (activity.id-1) as usize;
             assert_eq!(p_project.get_time_planning(activity.clone(), input[index]) , time);
         }
     }

     #[test]
     fn test_resource_conflict() {
         let mut project = initial();
         let planning = vec![1,2,3,4,5,6,8,7,9];
         let times = vec![0,0,0,0,2,4,5,5,10];

         for activity in &mut project.activities {
             let j = planning.iter().position(|x|*x == activity.id).unwrap();
             let time = times[j];
             activity.start_time = time;
         }

         let other = project.clone();
         let input = vec![9,1,2,4,3,8,8,8,1];
         let is_conflict = vec![false,false,true,true,false,false,false,false,false];
         for activity in &mut project.activities {
             let index = (activity.id-1) as usize;
             let conflict = is_conflict[index as usize];
             assert_eq!(other.resource_conflict(activity.clone(), input[index]),conflict);
         }

         let mut p_project = project.pre_emptive_project();
         let planning = vec![1,2,3,5,9,12,4,6,10,13,7,11,14,8,18,15,19,16,20,17,21];
         let times = vec![0,0,0,0,1,1,2,2,3,3,3,4,4,4,5,5,6,6,7,7,8];

         for activity in &mut p_project.activities {
             let j = planning.iter().position(|x|*x == activity.id).unwrap();
             let time = times[j];
             activity.start_time = time;
         }

         let other = p_project.clone();
         let input = vec![3,1,5,1,5,7,0,2,0,8,1,0,1,2,6,8,0,9,9,8,0];
         let is_conflict = vec![false,false,false,true,false,false,true,true,true,false,
                                true,true,false,false,false,false,true,false,false,false,false];
         for activity in &mut p_project.activities {
             let index = (activity.id-1) as usize;
             let conflict = is_conflict[index as usize];
             assert_eq!(other.resource_conflict(activity.clone(), input[index]),conflict);
         }


         let mut project = initial_1();
         let planning = vec![1,2,3,5,6,7,8,4,9];
         let times = vec![0,0,1,1,3,4,4,7,11];

         for activity in &mut project.activities {
             let j = planning.iter().position(|x|*x == activity.id).unwrap();
             let time = times[j];
             activity.start_time = time;
         }

         let input = vec![10,1,5,1,8,0,1,1,1];
         let other = project.clone();
         let is_conflict = vec![false,false,false,true,false,false,false,true,false];
         for activity in &project.activities {
             let index = (activity.id-1) as usize;
             let conflict = is_conflict[index];
             assert_eq!(other.resource_conflict(activity.clone(), input[index]), conflict);
         }

         let mut p_project = project.pre_emptive_project();
         let planning = vec![1,2,3,9,12,4,10,13,11,14,18,15,19,16,20,17,5,6,7,8,21];
         let times = vec![0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7,8,9,10,11,12];

         for activity in &mut p_project.activities {
             let j = planning.iter().position(|x|*x == activity.id).unwrap();
             let time = times[j];
             activity.start_time = time;
         }

         let input = vec![0,1,6,1,2,8,3,2,0,4,5,0,1,1,2,11,9,2,1,1,3];
         let other = p_project.clone();
         let is_conflict = vec![false,false,false,true,true,false,false,true,false,false,false,false,
         false,false,false,false,false,true,true,true,false];

         for activity in &p_project.activities {
             let index = (activity.id-1) as usize;
             let conflict = is_conflict[index];
             assert_eq!(other.resource_conflict(activity.clone(), input[index]), conflict);
         }


     }
 }
