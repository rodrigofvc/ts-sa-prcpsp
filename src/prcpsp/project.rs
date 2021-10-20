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



 
