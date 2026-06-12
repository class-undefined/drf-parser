use std::{env, process};

use drf_parser::{parse_drf_file, parse_layermap_file, write_lyp_outputs};

fn main() {
    let mut args = env::args().skip(1);
    let kind = args.next().unwrap_or_else(|| usage_and_exit());
    let path = args.next().unwrap_or_else(|| usage_and_exit());

    if kind == "lyp" {
        let output_dir = args.next().unwrap_or_else(|| String::from("."));
        if args.next().is_some() {
            usage_and_exit();
        }

        match write_lyp_outputs(&path, &output_dir) {
            Ok(()) => {
                eprintln!("wrote {}/display.drf.json", output_dir);
                eprintln!("wrote {}/layermap.json", output_dir);
            }
            Err(err) => {
                eprintln!("parse error: {}", err);
                process::exit(1);
            }
        }
        return;
    }

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
    eprintln!("       drf-parser lyp <file> [output-dir]");
    process::exit(2);
}
