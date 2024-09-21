pub struct Config {
    mode: Mode,
}

enum Mode {
    Render { root_dir: String, max_depth: usize },
    Message(String),
}

impl Config {
    pub fn new() -> Self {
        Self {
            mode: Mode::Render {
                root_dir: ".".to_string(),
                max_depth: 2,
            },
        }
    }

    pub fn new_message(message: String) -> Self {
        Self {
            mode: Mode::Message(message),
        }
    }

    pub fn from(args: Vec<String>) -> Self {
        Self::parse_config(args)
    }

    /// Here is how the command line arguments should work:
    /// --help, --version at args[1], return Config{mode: Mode::Message(...)}
    /// -D followed by a valid usize sets the max depth
    /// -L followed by a valid usize sets the max length of any sub directory
    /// -T followed by a valid usize sets the total length. How many times render::render_line() is called.
    /// exactly one string not following a tag is the base directory.
    /// We start with the default Config::new() and fill in values as we get them from the args.
    ///
    ///
    /// Some examples
    /// `mtree .` s
    fn parse_config(args: Vec<String>) -> Self {
        if let Some(arg1) = args.get(1) {
            match &arg1[..] {
                "--help" => return Self::new_message(Self::get_help_message()),
                "--version" => return Self::new_message(Self::get_version_message()),
                _ => (),
            }
        }

        Self::new()
    }

    pub fn get_max_depth(&self) -> Option<usize> {
        if let Mode::Render { max_depth, .. } = self.mode {
            Some(max_depth)
        } else {
            None
        }
    }

    pub fn get_root_dir(&self) -> Option<String> {
        if let Mode::Render { ref root_dir, .. } = self.mode {
            Some(root_dir.clone())
        } else {
            None
        }
    }

    pub fn get_message(&self) -> Option<String> {
        if let Mode::Message(ref message) = self.mode {
            Some(message.clone())
        } else {
            None
        }
    }

    fn get_help_message() -> String {
        std::fs::read_to_string("src/messages/help.txt")
            .unwrap_or_else(|_| "Error help version message.".to_string())
    }

    fn get_version_message() -> String {
        let version = &std::env::var("CARGO_PKG_VERSION").unwrap()[..];
        let build_time = &chrono::offset::Utc::now().date_naive().to_string()[..];
        let message = std::fs::read_to_string("src/messages/version.txt")
            .unwrap_or_else(|_| "Error reading version message.".to_string());

        message
            .replace("<version>", version)
            .replace("<build_time>", build_time)
    }
}
