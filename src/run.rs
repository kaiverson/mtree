use super::config::Config;

pub fn run(config: Config) {
    if let Some(message) = config.get_message() {
        println!("{message}");
        return;
    }

    super::render::render_directory(config);
}
