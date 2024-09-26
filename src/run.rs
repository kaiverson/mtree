use super::config::Config;

#[allow(unreachable_code)]
pub fn run(config: Config) {
    if let Some(message) = config.get_message() {
        println!("{message}");
    } else if let Some(error) = config.get_error() {
        println!("Error: {error}");
    } else {
        super::render::render_directory(config);
    }
}
