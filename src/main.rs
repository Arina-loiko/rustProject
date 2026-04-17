use std::env;
use std::process;

mod error;
mod graph;
mod manifest;
mod registry;
mod resolver;

use crate::manifest::Manifest;
use crate::registry::Registry;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Использование: {} <project.toml> <registry.json>", args[0]);
        process::exit(2);
    }

    let manifest = match Manifest::from_file(&args[1]) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Ошибка чтения манифеста: {}", e);
            process::exit(1);
        }
    };

    let registry = match Registry::from_file(&args[2]) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Ошибка чтения реестра: {}", e);
            process::exit(1);
        }
    };

    match resolver::resolve(&manifest, &registry) {
        Ok(order) => {
            println!("Порядок установки:");
            for (i, p) in order.iter().enumerate() {
                println!("  {}. {} {}", i + 1, p.name, p.version);
            }
        }
        Err(e) => {
            eprintln!("Ошибка: {}", e);
            process::exit(1);
        }
    }
}
