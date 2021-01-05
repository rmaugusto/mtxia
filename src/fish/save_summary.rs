use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    path::Path,
};

use crate::shared::config::Config;

use super::Fish;

pub fn save_summary(generation: i32, config: &Config, f: &Fish) {
    let result_dir = format!("{}{}/result.csv", config.ai.state_path, f.brain.get_name());
    let f_path = Path::new(&result_dir);
    let write_header: bool = f_path.exists();

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(f_path)
        .unwrap();

    if !write_header {
        writeln!(file, "generation;energy;age;distance;fitness;").unwrap();
    }
    writeln!(
        file,
        "{};{};{};{};{};",
        generation,
        format_number(f.energy),
        format_number(f.age()),
        format_number(f.distance),
        format_number(f.fitness())
    )
    .unwrap();
}

fn format_number(num: f32) -> String {
    let str = format!("{:.6}", num);
    str.replace(".", ",")
}
