// basic idea: depth first search the files in root_dir
use super::config::Config;
use super::utils::Limit;
use std::fs;
use std::path::PathBuf;
use std::time;

pub struct Renderer {
    draw_layer_table: Vec<bool>,
    dir_depth_limit: Limit,
    dir_len_limit: Limit,
    total_len_limit: Limit,
    start_time: time::Instant,
}

impl Renderer {
    fn new(
        draw_layer_table: Vec<bool>,
        dir_depth_limit: Limit,
        dir_len_limit: Limit,
        total_len_limit: Limit,
    ) -> Self {
        Self {
            draw_layer_table,
            dir_depth_limit,
            dir_len_limit,
            total_len_limit,
            start_time: time::Instant::now(),
        }
    }

    pub fn render_directory(config: Config) {
        let mut path: PathBuf = PathBuf::from(
            config
                .get_root_dir()
                .expect("Config should be in render mode."),
        );

        // Create three limits to easily track the bounds of the tree.
        let dir_depth_limit: Limit = Limit::new(config.get_max_depth());
        let dir_len_limit: Limit = Limit::new(config.get_dir_len_limit());
        let total_len_limit: Limit = Limit::new(config.get_total_len_limit());

        // Remembers the past to determine if we should draw:
        // │   ├── file_name
        // or
        //     ├── file_name
        let draw_layer_table = vec![true; dir_depth_limit.get_limit().unwrap()];

        let mut renderer = Renderer::new(
            draw_layer_table,
            dir_depth_limit,
            dir_len_limit,
            total_len_limit,
        );

        // Print the root of the tree.
        println!(
            "{}",
            path.to_str().expect("Config should be in render mode.")
        );

        let rendered_full_dir = renderer.scan_directory(&mut path);

        if !rendered_full_dir {
            renderer.render_limit_reached();
        }

        println!(
            "\n{} files and directories displayed in {:.2} seconds",
            renderer.total_len_limit.get_count(),
            renderer.render_time(),
        )
    }

    #[allow(clippy::needless_borrows_for_generic_args)]
    fn scan_directory(&mut self, path: &mut PathBuf) -> bool {
        // Get a list of files and sub directoris at the directory at path.
        let entries = match fs::read_dir(&path) {
            Ok(read_dir) => read_dir
                .collect::<Result<Vec<_>, std::io::Error>>()
                .unwrap(),
            Err(_) => {
                let r = self.render_line("[[RESTRICTED]]", true, false);
                return r;
            }
        };

        let entries_len = entries.len();

        if entries_len == 0 {
            return true;
        }

        let last_entry_index = entries_len - 1;

        for (index, entry) in entries.iter().enumerate() {
            let file_type = entry.file_type().unwrap();
            let file_name = &entry.file_name().into_string().unwrap()[..];
            let is_last_in_dir = last_entry_index == index;

            self.draw_layer_table[self.dir_depth_limit.get_count()] = !is_last_in_dir;
            // if is_last_in_dir {
            // self.draw_layer_table[self.dir_depth_limit.get_count()] = false;
            // Logically this shouldn't work, but it seems to work perfectly!
            // }

            let is_dir = file_type.is_dir();

            let continue_render = self.render_line(file_name, is_last_in_dir, is_dir);
            if !continue_render {
                return false;
            }

            if !is_dir {
                continue;
            }

            self.dir_depth_limit.increment();
            if self.dir_depth_limit.is_under_limit() {
                path.push(file_name);
                self.scan_directory(path);
                path.pop();
            }
            self.dir_depth_limit.decrement();
        }
        self.dir_len_limit.reset_count();

        true
    }

    fn render_line(&mut self, file_name: &str, is_last_in_dir: bool, is_dir: bool) -> bool {
        if !self.total_len_limit.is_under_limit() {
            return false;
        }

        self.total_len_limit.increment();

        assert!(
            self.dir_depth_limit
                .get_limit()
                .expect("Tree should have a depth limit.")
                <= self.draw_layer_table.len()
        );

        let mut print_buffer: String = String::new();

        for &render_layer in self
            .draw_layer_table
            .iter()
            .take(self.dir_depth_limit.get_count())
        {
            if render_layer {
                print_buffer.push_str("│   ");
            } else {
                print_buffer.push_str("    ");
            }
        }

        if is_last_in_dir {
            print_buffer.push_str("└── ");
        } else {
            print_buffer.push_str("├── ");
        }

        print_buffer.push_str(file_name);

        if is_dir && self.dir_depth_limit.is_at_limit() {
            print_buffer.push_str(" ...");
        }

        println!("{print_buffer}");

        true
    }

    pub fn render_limit_reached(&mut self) {
        self.total_len_limit.decrement();

        for &render_layer in self
            .draw_layer_table
            .iter()
            .take(self.total_len_limit.get_count())
        {
            if render_layer {
                print!("... ");
            } else {
                print!("    ");
            }
        }
    }

    pub fn render_time(&self) -> f32 {
        self.start_time.elapsed().as_secs_f32()
    }
}
