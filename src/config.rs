use std::vec::IntoIter;

#[derive(Debug, PartialEq)]
pub struct Config {
    mode: Mode,
}

#[derive(Debug, PartialEq)]
enum Mode {
    Render {
        root_dir: String,
        max_depth: usize,
        dir_len_limit: Option<usize>,
        total_len_limit: Option<usize>,
    },
    Message(String),
    Error(String),
}

impl Config {
    pub fn new() -> Self {
        Self {
            mode: Mode::Render {
                root_dir: ".".to_string(),
                max_depth: 2,
                dir_len_limit: None,
                total_len_limit: None,
            },
        }
    }

    pub fn new_message(message: String) -> Self {
        Self {
            mode: Mode::Message(message),
        }
    }

    pub fn new_error(error: String) -> Self {
        Self {
            mode: Mode::Error(error),
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
        // --help, --version at args[1], return Config{mode: Mode::Message(...)}
        if let Some(arg1) = args.get(1) {
            match &arg1[..] {
                "--help" => return Self::new_message(Self::get_help_message()),
                "--version" => return Self::new_message(Self::get_version_message()),
                _ => (),
            }
        }

        let mut args = args.into_iter();
        args.next()
            .expect("Args should always contain at least one element.");

        let mut config = Self::new();

        // -D followed by a valid usize sets the max depth
        // -L followed by a valid usize sets the max length of any sub directory
        // -T followed by a valid usize sets the total length. How many times render::render_line() is called.
        // exactly one string not following a tag is the base directory.

        let mut directories_contained_in_args: usize = 0;

        while let Some(arg) = args.next() {
            let result = match &arg[..].starts_with("-") {
                false => {
                    directories_contained_in_args += 1;
                    config.set_root_dir(arg)
                }
                true => config.parse_tag_and_value(&arg[..], &mut args),
            };

            if let Err(error) = result {
                return Config::new_error(error);
            }

            if directories_contained_in_args > 1 {
                return Config::new_error(
                    "The arguments can only contain up to one base directory.".to_string(),
                );
            }
        }

        config
    }

    // I want to be able to parse `-D 10` for example and then insert 10 into the config.
    // ALSo new idea, coming right off the dome, add a third enum option in Mode, one that is Error,
    // so config can always return a value, instead of exiting for me.
    fn parse_tag_and_value(
        &mut self,
        tag: &str,
        args: &mut IntoIter<String>,
    ) -> Result<(), String> {
        // Check if the tags are valid. Done twice to ensure error heirarchy.
        match tag {
            "-D" | "-L" | "-T" => (),
            _ => return Err(format!("The tag `{tag}` is invalid.")),
        }

        let value = args
            .next()
            .ok_or_else(|| format!("No value after tag `{tag}`."))?;

        let value = value
            .parse::<usize>()
            .map_err(|_| format!("Invalid value `{value}` after tag `{tag}`"))?;

        match tag {
            "-D" => self.set_max_depth(value),
            "-L" => self.set_dir_len_limit(Some(value)),
            "-T" => self.set_total_len_limit(Some(value)),
            _ => Err(format!("The tag `{tag}` is invalid.")),
        }
    }

    pub fn get_root_dir(&self) -> Option<String> {
        if let Mode::Render { ref root_dir, .. } = self.mode {
            Some(root_dir.clone())
        } else {
            None
        }
    }

    pub fn set_root_dir(&mut self, new_root_dir: String) -> Result<(), String> {
        if let Err(_error) = std::fs::read_dir(&new_root_dir) {
            return Err(format!("The directory `{new_root_dir}` does not exist."));
        }

        if let Mode::Render {
            ref mut root_dir, ..
        } = self.mode
        {
            *root_dir = new_root_dir;
            Ok(())
        } else {
            Err(
                "Tried to set the root directory while the Config was not in Render mode."
                    .to_string(),
            )
        }
    }

    pub fn get_max_depth(&self) -> Option<usize> {
        if let Mode::Render { max_depth, .. } = self.mode {
            Some(max_depth)
        } else {
            None
        }
    }

    pub fn set_max_depth(&mut self, new_depth: usize) -> Result<(), String> {
        if let Mode::Render {
            ref mut max_depth, ..
        } = self.mode
        {
            *max_depth = new_depth;
            Ok(())
        } else {
            Err("Tried to set the max depth while the Config was not in Render mode.".to_string())
        }
    }

    pub fn get_dir_len_limit(&self) -> Option<usize> {
        if let Mode::Render { dir_len_limit, .. } = self.mode {
            dir_len_limit
        } else {
            None
        }
    }

    pub fn set_dir_len_limit(&mut self, new_dir_len_limit: Option<usize>) -> Result<(), String> {
        if let Mode::Render {
            ref mut dir_len_limit,
            ..
        } = self.mode
        {
            *dir_len_limit = new_dir_len_limit;
            Ok(())
        } else {
            Err("Tried to set the max depth while the Config was not in Render mode.".to_string())
        }
    }

    pub fn get_total_len_limit(&self) -> Option<usize> {
        if let Mode::Render {
            total_len_limit, ..
        } = self.mode
        {
            total_len_limit
        } else {
            None
        }
    }

    pub fn set_total_len_limit(
        &mut self,
        new_total_len_limit: Option<usize>,
    ) -> Result<(), String> {
        if let Mode::Render {
            ref mut total_len_limit,
            ..
        } = self.mode
        {
            *total_len_limit = new_total_len_limit;
            Ok(())
        } else {
            Err("Tried to set the max depth while the Config was not in Render mode.".to_string())
        }
    }

    pub fn get_message(&self) -> Option<String> {
        if let Mode::Message(ref message) = self.mode {
            Some(message.clone())
        } else {
            None
        }
    }

    pub fn get_error(&self) -> Option<String> {
        if let Mode::Error(ref error) = self.mode {
            Some(error.clone())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args_no_arguments() {
        let args: Vec<String> = vec!["mtree".to_string()];
        let config = Config::from(args);
        assert_eq!(config, Config::new());
    }

    #[test]
    fn test_parse_args_help() {
        let args: Vec<String> = vec!["mtree".to_string(), "--help".to_string()];
        let config = Config::from(args);
        assert!(config.get_message().is_some());
    }

    #[test]
    fn test_parse_args_version() {
        let args: Vec<String> = vec!["mtree".to_string(), "--version".to_string()];
        let config = Config::from(args);
        assert!(config.get_message().is_some());
    }

    #[test]
    fn test_parse_args_with_valid_base_dir() {
        let args: Vec<String> = vec!["mtree".to_string(), "C:/Windows".to_string()];
        let config = Config::from(args);
        assert_eq!(config.get_root_dir(), Some("C:/Windows".to_string()));
        assert_eq!(config.get_max_depth(), Some(2));
    }

    #[test]
    fn test_parse_args_with_invalid_base_dir() {
        let args: Vec<String> = vec!["mtree".to_string(), "C:/awoooo0ooogaaaa".to_string()];
        let config = Config::from(args);
        assert!(config.get_error().is_some());
    }

    #[test]
    fn test_parse_args_with_max_depth() {
        let args: Vec<String> = vec![
            "mtree".to_string(),
            "-D".to_string(),
            "5".to_string(),
            "C:/Windows".to_string(),
        ];
        let config = Config::from(args);
        assert_eq!(config.get_max_depth(), Some(5));
        assert_eq!(config.get_root_dir(), Some("C:/Windows".to_string()));
    }

    #[test]
    fn test_parse_args_with_dir_len_limit() {
        let args: Vec<String> = vec![
            "mtree".to_string(),
            "-L".to_string(),
            "10".to_string(),
            "C:/Windows".to_string(),
        ];
        let config = Config::from(args);
        assert_eq!(config.get_dir_len_limit(), Some(10));
        assert_eq!(config.get_root_dir(), Some("C:/Windows".to_string()));
    }

    #[test]
    fn test_parse_args_with_total_len_limit() {
        let args: Vec<String> = vec![
            "mtree".to_string(),
            "-T".to_string(),
            "100".to_string(),
            "C:/Windows".to_string(),
        ];
        let config = Config::from(args);
        assert_eq!(config.get_total_len_limit(), Some(100));
        assert_eq!(config.get_root_dir(), Some("C:/Windows".to_string()));
    }

    #[test]
    fn test_parse_args_with_multiple_flags() {
        let args: Vec<String> = vec![
            "mtree".to_string(),
            "-D".to_string(),
            "3".to_string(),
            "-L".to_string(),
            "10".to_string(),
            "-T".to_string(),
            "50".to_string(),
            "C:/Windows".to_string(),
        ];
        let config = Config::from(args);
        assert_eq!(config.get_max_depth(), Some(3));
        assert_eq!(config.get_dir_len_limit(), Some(10));
        assert_eq!(config.get_total_len_limit(), Some(50));
        assert_eq!(config.get_root_dir(), Some("C:/Windows".to_string()));
    }

    #[test]
    fn test_parse_args_missing_value_for_depth() {
        let args: Vec<String> = vec!["mtree".to_string(), "-D".to_string()];
        let config = Config::from(args);
        assert!(config.get_error().is_some());
    }

    #[test]
    fn test_parse_args_invalid_depth_value() {
        let args: Vec<String> = vec![
            "mtree".to_string(),
            "-D".to_string(),
            "invalid".to_string(),
            "C:/Windows".to_string(),
        ];
        let config = Config::from(args);
        assert!(config.get_error().is_some());
    }

    #[test]
    fn test_parse_args_invalid_tag() {
        let args: Vec<String> = vec![
            "mtree".to_string(),
            "-X".to_string(),
            "10".to_string(),
            "C:/Windows".to_string(),
        ];
        let config = Config::from(args);
        assert!(config.get_error().is_some());
    }

    #[test]
    fn test_parse_args_multiple_base_directories() {
        let args: Vec<String> = vec![
            "mtree".to_string(),
            "first_dir".to_string(),
            "-D".to_string(),
            "3".to_string(),
            "second_dir".to_string(),
        ];
        let config = Config::from(args);
        assert!(config.get_error().is_some());
    }
}
