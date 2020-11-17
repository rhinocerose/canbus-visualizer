use config_struct::{Error, StructOptions};

fn main() -> Result<(), Error> {
    config_struct::create_config(
        "config.toml",
        "src/config.rs",
        &StructOptions::default())
}
