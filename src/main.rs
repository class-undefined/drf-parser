use std::{env, process};

use drf_parser::{parse_drf_file, parse_layermap_file};

fn main() {
    let mut args = env::args().skip(1);
    let kind = args.next().unwrap_or_else(|| usage_and_exit());
    let path = args.next().unwrap_or_else(|| usage_and_exit());

    if args.next().is_some() {
        usage_and_exit();
    }

    let output = match kind.as_str() {
        "drf" => parse_drf_file(&path),
        "layermap" => parse_layermap_file(&path),
        _ => usage_and_exit(),
    };

    match output {
        Ok(json) => {
            println!("{}", json);
        }
        Err(err) => {
            eprintln!("parse error: {}", err);
            process::exit(1);
        }
    }
}

fn usage_and_exit() -> ! {
    eprintln!("usage: drf-parser <drf|layermap> <file>");
    process::exit(2);
}
