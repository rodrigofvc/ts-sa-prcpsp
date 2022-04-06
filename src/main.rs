mod metaheuristics;
mod prcpsp;

use std::env;
use std::time::Instant;

use crate::prcpsp::project::Project as Project;
use crate::prcpsp::activity::Activity as Activity;
use crate::prcpsp::rn_resource::RnResource as RnResource;
use crate::prcpsp::rn_resource_usage::RnResourceUsage as RnResourceUsage;
use crate::prcpsp::sa_state::SaState as SaState;
use crate::prcpsp::random_rcpsp as random_rcpsp;
use crate::prcpsp::utils as utils;

use crate::metaheuristics::simulated_annealing::simulated_annealing as sa;
use crate::metaheuristics::tabu_search::tabu_search as ts;
use crate::metaheuristics::state::State as State;

fn main() {
    let args: Vec<String> = env::args().collect();
    let metaheuristic = args[1].as_str();
    let filename = args[2].as_str();

    let params = utils::read_random_prcpsp(filename.to_string());
    let random_seed = params[0].into();
    let random_cost = params[1];
    let random_activities = params[2];
    let random_resources = params[3];
    let random_resources_max_capacity = params[4];
    let random_optimum_project = random_rcpsp::get_random_rcpsp(random_seed, random_cost, random_activities,
        random_resources, random_resources_max_capacity);
    let optimum_state = SaState::from_project_planned(random_optimum_project.clone(), random_seed);

    println!(" Optimo {:?}", optimum_state.planning);
    println!("        {:?}", optimum_state.times);
    println!(">>>>> Costo {:?}", optimum_state.get_makespan());
    utils::write_svg(optimum_state.get_svg(), "optimum.svg");

    let mut random_project = random_optimum_project.clone();
    for a in &mut random_project.activities {
        a.start_time = -1;
    }

    let seed = args[3].parse::<u64>().unwrap();
    let m = args[4].parse::<u32>().unwrap();
    random_project = random_project.pre_emptive_project(m);

    let initial = SaState::new(random_project.clone(), seed);
    utils::write_svg(initial.get_svg(), "initial.svg");
    println!("{:?}", initial.planning);
    println!("{:?}", initial.times);
    println!(">>>>> Costo {:?}", initial.get_makespan());

    let start : Instant;
    let seconds : u64;
    let time : String;

    match metaheuristic {
        "SA" => {
            let iterations = args[5].parse::<u32>().unwrap();
            let temperature = args[6].parse::<f32>().unwrap();
            let decrement = args[7].parse::<f32>().unwrap();
            let epsilon = args[8].parse::<f32>().unwrap();

            start = Instant::now();
            let (best,log) = sa::simulated_annealing(initial.clone(), iterations, temperature, decrement, epsilon, seed);
            seconds = start.elapsed().as_secs();
            time = utils::get_time(seconds);
            utils::write_log_sa(best.to_string(), best.get_cost(), iterations, temperature, epsilon, decrement, seed, log, time.clone());

            println!("\n  <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< \n  Mejor solucion: \n {} ", best.to_string());
            println!(" Costo: {}", best.get_cost());
            println!(" Tiempo {:?}", time );
            println!(">>>>>>>>> Optimo {:?}", optimum_state.planning);
            println!(">>>>>>>>>        {:?}", optimum_state.times);

            utils::write_log_random(random_seed, random_cost, random_activities, random_resources, random_resources_max_capacity, initial.get_cost());
            utils::write_svg(best.to_file(), "best.svg");
        }
        "TS" => {
            let tabu_time = args[5].parse::<u32>().unwrap();
            let neighbors = args[6].parse::<u32>().unwrap();
            let iterations = args[7].parse::<u32>().unwrap();

            start = Instant::now();
            let (best,log) = ts::tabu_search(initial.clone(), tabu_time, neighbors, iterations);
            seconds = start.elapsed().as_secs();
            time = utils::get_time(seconds);
            utils::write_log_ts(best.to_string(), best.get_cost(), tabu_time, neighbors, iterations, seed, log, time.clone());

            println!("\n  <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< \n  Mejor solucion: \n {} ", best.to_string());
            println!(" Costo: {}", best.get_cost());
            println!(" Tiempo {:?}", time );
            println!(">>>>>>>>> Optimo {:?}", optimum_state.planning);
            println!(">>>>>>>>>        {:?}", optimum_state.times);

            utils::write_log_random(random_seed, random_cost, random_activities, random_resources, random_resources_max_capacity, initial.get_cost());
            utils::write_svg(best.to_file(), "best.svg");
        }
        _ => panic!("La metaheuristica no se encontro"),
    }

}

#[allow(dead_code)]
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

#[allow(dead_code)]
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
