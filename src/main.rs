use clap::Parser;

#[derive(Parser)]
struct Config {
    server_type: String,
}

fn generate_data() {}

fn load_config_file(global_config: &mut Config) {
    let file_args = Config {
        server_type: String::from("unknown"),
    };

    if file_args.server_type != global_config.server_type {
        global_config.server_type = file_args.server_type;
    }
}

fn load_config_defaults() -> Config {
    Config {
        server_type: String::from("unknown"),
    }
}

fn load_config_args(global_config: &mut Config) {
    let args = Config::parse();

    if args.server_type != global_config.server_type {
        global_config.server_type = args.server_type;
    }
}

fn read_definition() {}

fn send_to_remote() {}

fn write_to_file() {}

fn main() {
    let mut config = load_config_defaults();
    load_config_file(&mut config);
    load_config_args(&mut config);
    println!("server_type: {:?}", config.server_type);

    read_definition();
    generate_data();
    write_to_file();
    send_to_remote();
}
