use super::config::Config;
use super::render::Renderer;

#[allow(unreachable_code)]
pub fn run(config: Config) {
    if let Some(message) = config.get_message() {
        println!("{message}");
    } else if let Some(error) = config.get_error() {
        println!("Error: {error}");
    } else {
        Renderer::render_directory(config);
    }
}
