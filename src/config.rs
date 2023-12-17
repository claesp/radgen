use clap::Parser;

#[derive(Parser)]
pub struct Config {
    pub server_type: String,
}

pub fn compare_config_set(config_a: Config, config_b: &mut Config) {
    if config_a.server_type != config_b.server_type {
        config_b.server_type = config_a.server_type;
    }
}

pub fn load_config_file(global_config: &mut Config) {
    let file_args = Config {
        server_type: String::from("unknown"),
    };

    /* TODO: Load from external file. */

    compare_config_set(file_args, global_config);
}

pub fn load_config_defaults() -> Config {
    Config {
        server_type: String::from("unknown"),
    }
}

pub fn load_config_args(global_config: &mut Config) {
    let args = Config::parse();
    compare_config_set(args, global_config);
}
