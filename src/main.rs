mod config;

fn generate_data() {}

fn read_definition() {}

fn send_to_remote() {}

fn write_to_file() {}

fn main() {
    let mut config = config::load_config_defaults();
    config::load_config_file(&mut config);
    config::load_config_args(&mut config);
    println!("server_type: {:?}", config.server_type);

    read_definition();
    generate_data();
    write_to_file();
    send_to_remote();
}
