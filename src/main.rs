use clap::Parser;

#[derive(Parser)]
struct Config {
    server_type: String
}

fn generate_data() {
}

fn load_config_file() {
}

fn load_defaults() {
}

fn parse_args() -> Config {
    let args = Config::parse();

    return args;
}

fn read_definition() {
}

fn send_to_remote() {
}

fn write_to_file() {
}

fn main() {
    load_defaults();
    load_config_file();

    let args = parse_args();
    println!("server_type: {:?}", args.server_type);

    read_definition();
    generate_data();
    write_to_file();
    send_to_remote();
}