use clap::Parser;

#[derive(Parser)]
struct Config {
    server_type: String
}

fn generate_data() {
}

fn write_to_file() {
}

fn parse_args() {
    let args = Config::parse();

    println!("server_type: {:?}", args.server_type)
}

fn read_definition() {
}

fn send_to_remote() {
}

fn main() {
    parse_args();
    read_definition();
    generate_data();
    write_to_file();
    send_to_remote();
}