use std::env;
use std::io::Write;

use dust_dds::generate_dust_dds_configuration_schema;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Usage: {} <filepath> ", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];

    let json_schema_str_pretty = generate_dust_dds_configuration_schema().unwrap();

    let mut file = std::fs::File::create(file_path).unwrap();
    file.write_all(json_schema_str_pretty.as_bytes()).unwrap();

    println!("Schema written to: {}", file_path);
}