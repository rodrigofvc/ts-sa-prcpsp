use rand::{Rng, SeedableRng,StdRng};

use crate::prcpsp::project::Project as Project;
use crate::prcpsp::activity::Activity as Activity;
use crate::prcpsp::rn_resource::RnResource as RnResource;
use crate::prcpsp::rn_resource_usage::RnResourceUsage as RnResourceUsage;

/**
* Create a random RCPSP instance.
* random_seed: seed for pseudo-random numbers.
* random_cost: makespan for project.
* random_activities: activities number for project.
* random_resources: resources number for project.
* random_resources_max_capacity:: total resources for project.
*/
pub fn get_random_rcpsp(random_seed: u64, random_cost: u32, random_activities: u32,
                        random_resources: u32, random_resources_max_capacity: u32) -> Project {

    let mut rng : StdRng = SeedableRng::seed_from_u64(random_seed);

    let (intervals_time, duration_intervals_time) = get_intervals_time(random_cost, random_activities);
    let columns = intervals_time.len() as u32;

    let (intervals_resource, duration_intervals_resource) = get_intervals_resource(columns, random_activities, random_resources_max_capacity);
    let rows = intervals_resource.len() as u32;

    let mut resources : Vec<RnResource> = vec![];
    let mut resource_capacity : u32;
    resource_capacity = random_resources_max_capacity / random_resources;
    for i in 1..=random_resources {
        if i == random_resources {
            resource_capacity += random_resources_max_capacity % random_resources;
        }
        let new_resource = RnResource::new(i, i.to_string(), resource_capacity);
        resources.push(new_resource);
    }

    let mut activities_id : Vec<u32> = (2..=random_activities+1).collect();
    let mut activities : Vec<Activity> = vec![];

    let mut resources_not_avaible : Vec<u32> =  vec![];
    let mut resources_remaining : Vec<u32> = vec![];
    for i in 0..resources.len() {
        resources_remaining.push(resources[i].capacity);
    }

    for row in 0..rows {
        let mut resources_used : Vec<RnResource> = vec![];
        let demand = duration_intervals_resource[row as usize];
        let mut availability = 0;
        for i in 0..resources.len() {
            if resources_not_avaible.contains(&resources[i].id) {
                continue;
            }
            let remainig = resources_remaining[i];
            availability += remainig;
            resources_used.push(resources[i].clone());
            if availability >= demand {
                break;
            }
        }
        for column in 0..columns {
            if activities.len() as u32 == random_activities {
                break;
            }
            let id = activities_id.remove(0);
            let mut supplies : Vec<RnResourceUsage> = vec![];
            let mut demand = duration_intervals_resource[row as usize];
            let mut duration = duration_intervals_time[column as usize];
            for i in 0..resources_used.len() {
                let using : u32;
                let remainig = resources_remaining[(resources_used[i].id-1) as usize];
                if remainig >= demand {
                    using = demand;
                    demand = 0;
                } else {
                    using = remainig;
                    demand -= remainig;
                }
                let supply = RnResourceUsage::new(resources_used[i].clone(), using);
                supplies.push(supply);
                if demand == 0 {
                    break;
                }
            }
            let start_time = intervals_time[column as usize] as i32 - duration as i32;
            if activities.len() as u32 == random_activities-1 {
                duration += random_cost - (start_time as u32 + duration);
            }
            let activity = Activity::new(id,-1, id.to_string(), vec![], vec![], supplies, duration, start_time);
            activities.push(activity);
        }

        let mut demand = duration_intervals_resource[row as usize];
        for i in 0..resources_used.len() {
            let mut remainig = resources_remaining[(resources_used[i].id-1) as usize];
            if demand >= remainig {
                resources_not_avaible.push(resources_used[i].id);
                resources_remaining[(resources_used[i].id-1) as usize] = 0;
                demand -= remainig;
                if demand == 0 {
                    break;
                }
            } else {
                remainig -= demand;
                resources_remaining[(resources_used[i].id-1) as usize] = remainig;
                break;
            }
        }
    }

    for i in 0..activities.len() {
        if i+1 == activities.len() {
            break;
        }
        let j = i+1;
        let mut current = activities[i].clone();
        let next = activities[j].clone();
        if next.start_time < current.start_time {
            continue;
        }
        let mut max_successors = (activities.len() as f32).sqrt().floor() as u32;
        let posible_successors = activities.clone().into_iter().filter(|x|x.start_time > current.start_time
                                                                            && current.start_time + current.duration as i32 <= x.start_time ).count() as u32;
        if posible_successors < max_successors {
            max_successors = posible_successors;
        }
        max_successors = rng.gen_range(1, max_successors + 1);
        let mut successors_count = 0;
        while successors_count < max_successors {
            let mut random_index = rng.gen_range(0, activities.len());
            let mut random_activity = activities[random_index].clone();
            let current_end_time = current.start_time + current.duration as i32;
            while random_index == i || random_activity.start_time <= current.start_time
            || current.successors.contains(&random_activity.id)
            || current_end_time > random_activity.start_time {
                random_index = rng.gen_range(0, activities.len());
                random_activity = activities[random_index].clone();
            }
            current.add_successor(random_activity.id);
            random_activity.add_anteccessor(current.id);
            activities[i] = current.clone();
            activities[random_index] = random_activity;
            successors_count += 1;
        }
    }

    let mut initial = Activity::new(1, -1, String::from("1"), vec![], vec![], vec![], 0, 0);
    let mut end = Activity::new(random_activities+2,-1, (random_activities+2).to_string(), vec![], vec![], vec![], 0, random_cost as i32);
    for activity in &mut activities {
        if activity.start_time == 0 || activity.predecessors.len() == 0 {
            initial.add_successor(activity.id);
            activity.add_anteccessor(initial.id);
        }
    }
    for i in 0..activities.len() {
        let mut activity = activities[i].clone();
        if i+1 != activities.len() {
            let other = &activities[i+1];
            if activity.start_time > other.start_time {
                end.add_anteccessor(activity.id);
                activity.add_successor(end.id);
                activities[i] = activity;
            }
        } else {
            end.add_anteccessor(activity.id);
            activity.add_successor(end.id);
            activities[i] = activity;
        }
    }
    activities.insert(0, initial);
    activities.push(end);
    let project = Project::new(activities,resources);
    return project;
}

/**
* Return the intervals for resources capacity,
* where each resource has capacity according to its interval.
* columns: number of intervals in start_time.
* random_activities: number of activities in project.
* random_resources_max_capacity: total resources for project
**/
fn get_intervals_resource(columns: u32, random_activities: u32, random_resources_max_capacity: u32) -> (Vec<u32>, Vec<u32>){
    let mut rows = 0;
    let mut possible_activities = 0;

    while possible_activities <= random_activities {
        possible_activities = rows * columns;
        rows+=1;
    }
    rows-=1;

    let mut intervals_resource : Vec<u32> = vec![];
    let mut duration_intervals_resource : Vec<u32> = vec![];
    let mut interval = random_resources_max_capacity / rows;
    let mut duration = random_resources_max_capacity / rows;
    let mut k = 1;
    while k <= rows {
        if k == rows {
            interval += random_resources_max_capacity % rows;
            duration += random_resources_max_capacity % rows
        }
        intervals_resource.push(interval);
        duration_intervals_resource.push(duration);
        interval += random_resources_max_capacity / rows;
        k+=1;
    }
    return (intervals_resource, duration_intervals_resource);
}

/**
* Return the intervals of start_time for activities in project,
* where the intervals are of duration 1,2,3,4,..,random_cost.
* random_cost: last interval.
* random_activities: number of activities in project.
**/
fn get_intervals_time(random_cost: u32, random_activities: u32) -> (Vec<u32>,Vec<u32>) {
    let mut intervals = vec![];
    let mut duration = vec![];
    let mut duration_interval = 1;
    let mut i : u32 = 1;
    while duration_interval <= random_cost {
        intervals.push(duration_interval);
        duration.push(i);
        i+=1;
        duration_interval += i;
    }

    if *intervals.last().unwrap() != random_cost {
        let diff = random_cost as i32 - *intervals.last().unwrap() as i32;
        duration.push(diff as u32);
        intervals.push(random_cost);
    }

    if intervals.len() > random_activities as usize {
        intervals.resize(random_activities as usize, 0);
        duration.resize(random_activities as usize, 0);
        let diff = random_cost as i32 - *intervals.last().unwrap() as i32;
        duration.pop();
        duration.push(diff as u32);
        intervals.pop();
        intervals.push(random_cost);
    }
    return (intervals, duration);
}
