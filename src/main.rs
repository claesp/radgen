use clap::Parser;

#[derive(Parser)]
struct Config {
    server_type: String,
}

fn generate_data() {}

fn compare_config_set(config_a: Config, config_b: &mut Config) {
    if config_a.server_type != config_b.server_type {
        config_b.server_type = config_a.server_type;
    }
}

fn load_config_file(global_config: &mut Config) {
    let file_args = Config {
        server_type: String::from("unknown"),
    };

    /* TODO: Load from external file. */

    compare_config_set(file_args, global_config);
}

fn load_config_defaults() -> Config {
    Config {
        server_type: String::from("unknown"),
    }
}

fn load_config_args(global_config: &mut Config) {
    let args = Config::parse();
    compare_config_set(args, global_config);
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
