use std::fs;
use std::io::Write;

const LOG_PATH : &str = "log/log.dat";
const HISTORY_PATH : &str = "log/log1.dat";
const IMAGE_PATH : &str = "output/";

pub fn get_time(seconds: u64) -> String {
        let mut time = String::new();
        let minutes = seconds/60;
        let hours =  minutes/60;
        let minutes = minutes % 60;
        let seconds = seconds % 60;
        time.push_str(&hours.to_string());
        time.push(':');
        time.push_str(&minutes.to_string());
        time.push(':');
        time.push_str(&seconds.to_string());
        time.push_str(" (hh:mm:ss)");
        return time;
}

pub fn write_log_random(random_seed: u64, random_cost: u32, random_activities: u32, random_resources: u32, random_resources_max_capacity: u32, initial_cost: u32){
    let mut content  = String::new();
    content.push_str("\n Datos del ejemplar: \n");
    content.push_str("  Semilla: ");
    content.push_str(&random_seed.to_string());
    content.push_str(&", ");
    content.push_str("Costo optimo: ");
    content.push_str(&random_cost.to_string());
    content.push_str(&", ");
    content.push_str("Actividades: ");
    content.push_str(&random_activities.to_string());
    content.push_str(&", ");
    content.push_str("Recursos: ");
    content.push_str(&random_resources.to_string());
    content.push_str(&", ");
    content.push_str("Unidades de recursos: ");
    content.push_str(&random_resources_max_capacity.to_string());
    content.push_str(&", ");
    content.push_str("Costo inicial: ");
    content.push_str(&initial_cost.to_string());
    if !std::path::Path::new(LOG_PATH).is_file() {
        fs::File::create(LOG_PATH).expect("No se pudo crear un archivo");
        fs::write(LOG_PATH, content.as_bytes()).expect("No se pudó escribir un archivo");
    } else {
        let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(LOG_PATH)
        .unwrap();
        write!(file, "{}", content).expect("No se pudo escribir un archivo");
    }
}

pub fn write_log_sa(state: String, cost: u32, iterations: u32, temperature: f32, epsilon: f32, decrement: f32, seed: u64, log: Vec<String>, time: String, m: u32){
    let mut content  = String::new();
    content.push_str("\n >>>>>>>>>>> Ejemplar: \n");
    content.push_str(&state);
    content.push('\n');
    content.push(' ');
    content.push_str("Metaheuristica: ");
    content.push_str("Recocido Simulado");
    content.push_str(&", ");
    content.push_str("Costo: ");
    content.push_str(&cost.to_string());
    content.push_str(&", ");
    content.push_str("Semilla: ");
    content.push_str(&seed.to_string());
    content.push_str(&", ");
    content.push_str("Interrupciones: ");
    content.push_str(&m.to_string());
    content.push_str(&", ");
    content.push_str("Tiempo: ");
    content.push_str(&time);
    content.push_str(&", ");
    content.push_str("Iteraciones: ");
    content.push_str(&iterations.to_string());
    content.push_str(&", ");
    content.push_str("Temperatura: ");
    content.push_str(&temperature.to_string());
    content.push_str(&", ");
    content.push_str("Epsilon: ");
    content.push_str(&epsilon.to_string());
    content.push_str(&", ");
    content.push_str("Decremento: ");
    content.push_str(&decrement.to_string());
    get_log(log.clone());
    if !std::path::Path::new(LOG_PATH).is_file() {
        fs::File::create(LOG_PATH).expect("No se pudo crear un archivo");
        fs::write(LOG_PATH, content.as_bytes()).expect("No se pudó escribir un archivo");
    } else {
        let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(LOG_PATH)
        .unwrap();
        write!(file, "{}", content).expect("No se pudo escribir un archivo");
    }
}

pub fn write_log_ts(state: String, cost: u32, tabu_time: u32, neighbors: u32, iterations: u32, seed: u64, log: Vec<String>, time: String, m: u32){
    let mut content  = String::new();
    content.push_str("\n >>>>>>>>>>> Ejemplar: \n");
    content.push_str(&state);
    content.push('\n');
    content.push(' ');
    content.push_str("Metaheuristica: ");
    content.push_str("Busqueda Tabu");
    content.push_str(", ");
    content.push_str("Costo: ");
    content.push_str(&cost.to_string());
    content.push_str(&", ");
    content.push_str("Semilla: ");
    content.push_str(&seed.to_string());
    content.push_str(&", ");
    content.push_str("Interrupciones: ");
    content.push_str(&m.to_string());
    content.push_str(&", ");
    content.push_str("Tiempo: ");
    content.push_str(&time);
    content.push_str(&", ");
    content.push_str("Tiempo tabu: ");
    content.push_str(&tabu_time.to_string());
    content.push_str(&", ");
    content.push_str("Vecinos: ");
    content.push_str(&neighbors.to_string());
    content.push_str(&", ");
    content.push_str("Iteraciones: ");
    content.push_str(&iterations.to_string());
    get_log(log.clone());
    if !std::path::Path::new(LOG_PATH).is_file() {
        fs::File::create(LOG_PATH).expect("No se pudo crear un archivo");
        fs::write(LOG_PATH, content.as_bytes()).expect("No se pudó escribir un archivo");
    } else {
        let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(LOG_PATH)
        .unwrap();
        write!(file, "{}", content).expect("No se pudo escribir un archivo");
    }
}

pub fn write_svg(content: String, name: &str){
    if !std::path::Path::new(IMAGE_PATH).is_dir() {
        fs::create_dir(IMAGE_PATH).expect("Ocurrio error");
    }
    fs::write(&(IMAGE_PATH.to_owned() + name), content.as_bytes()).expect("No se pudó escribir un archivo");
}

/**
* Given a vector with the cost of best state in each iteration,
* create a file with the (x,y) coordinates.
* log: vector with the cost value.
*/
pub fn get_log(log : Vec<String>)  {
    let mut content = String::new();
    let mut pos = 0;
    for l in log {
        content.push_str(&pos.to_string());
        content.push(' ');
        content.push_str(&l);
        content.push('\n');
        pos += 20;
    }
    fs::File::create(HISTORY_PATH).expect("No se pudó crear un archivo");
    fs::write(HISTORY_PATH, content.as_bytes()).expect("No se pudó escribir un archivo");
}

pub fn read_random_prcpsp(filename: String) -> Vec<u32> {
    let contents = fs::read_to_string(filename)
            .expect("Ocurrio un error al leer el archivo");
    let mut params : Vec<u32> = vec![];
    let lines : Vec<&str> = contents.split("\n").collect();
    for c in lines {
        if c.len() == 0 {
            continue;
        }
        let param = c.replace("\r","");
        params.push(param.parse::<u32>().unwrap());
    }
    return params;
}

#[allow(dead_code)]
pub fn write_current(iteration: u32, planning: String, state: String, metaheuristic: String) {
    let mut dir_name = String::from("it-");
    dir_name.push_str(&iteration.to_string());
    dir_name.push('/');
    let mut name_file = String::new();
    name_file.push_str(&metaheuristic);
    name_file.push_str(&"-curr-it-");
    name_file.push_str(&iteration.to_string());
    name_file.push_str(&".svg");
    dir_name.push_str(&name_file);
    write_svg(planning, &dir_name);

    let mut dir_name = String::from("it-");
    dir_name.push_str(&iteration.to_string());
    dir_name.push('/');

    let mut name_file = String::new();
    name_file.push_str(&metaheuristic);
    name_file.push_str(&"-curr-it-");
    name_file.push_str(&iteration.to_string());
    name_file.push_str(&"-state.svg");
    dir_name.push_str(&name_file);
    write_svg(state, &dir_name);
}

#[allow(dead_code)]
pub fn write_neighbor(iteration: u32, n: u32, planning: String, state: String) {
    let mut dir_name = String::from("it-");
    dir_name.push_str(&iteration.to_string());
    dir_name.push('/');

    let mut name_file = String::new();
    name_file.push_str(&"it-");
    name_file.push_str(&iteration.to_string());
    name_file.push_str(&"-neig-");
    name_file.push_str(&(n+1).to_string());
    name_file.push_str(&".svg");

    dir_name.push_str(&name_file);
    write_svg(planning, &dir_name);
    name_file.clear();

    let mut dir_name = String::from("it-");
    dir_name.push_str(&iteration.to_string());
    dir_name.push('/');

    name_file.push_str(&"it-");
    name_file.push_str(&iteration.to_string());
    name_file.push_str(&"-neig-");
    name_file.push_str(&(n+1).to_string());
    name_file.push_str(&"-state.svg");
    dir_name.push_str(&name_file);
    write_svg(state, &dir_name);
}
