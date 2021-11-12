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
        time.push_str(" hh:mm:ss");
        return time;
}

pub fn write_log(state: String, cost: u32, iterations: u32, temperature: f32, epsilon: f32, decrement: f32, seed: u64, log: Vec<String>, time: String){
    let mut content  = String::new();
    content.push_str("\n >>>>>>>>>>> Ejemplar: \n");
    content.push_str(&state);
    content.push('\n');
    content.push_str("Costo: ");
    content.push_str(&cost.to_string());
    content.push(' ');
    content.push_str("Semilla: ");
    content.push_str(&seed.to_string());
    content.push(' ');
    content.push_str("Tiempo: ");
    content.push_str(&time);
    content.push(' ');
    content.push_str("Iteraciones: ");
    content.push_str(&iterations.to_string());
    content.push(' ');
    content.push_str("Temperatura: ");
    content.push_str(&temperature.to_string());
    content.push(' ');
    content.push_str("Epsilon: ");
    content.push_str(&epsilon.to_string());
    content.push(' ');
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
