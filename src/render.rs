// basic idea: depth first search the files in root_dir
use super::config::Config;
use std::fs;
use std::path::PathBuf;

pub fn render_directory(config: Config) {
    let max_depth = config
        .get_max_depth()
        .expect("Config should be in render mode.");

    let mut path: PathBuf = PathBuf::from(
        config
            .get_root_dir()
            .expect("Config should be in render mode."),
    );

    // The state table is necessary to prevent branches to nowhere.
    // If a branch is over in a previous layer, we don't need to draw that branch.
    let mut state_table = vec![true; max_depth];

    search_directory(&mut path, 0, max_depth, &mut state_table);
}

#[allow(clippy::needless_borrows_for_generic_args)]
fn search_directory(path: &mut PathBuf, depth: usize, max_depth: usize, state_table: &mut [bool]) {
    let entries = fs::read_dir(&path)
        .expect("Path should valid and for a dir!")
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap();

    let last_entry_index = entries.len() - 1;

    for (index, entry) in entries.iter().enumerate() {
        let file_type = entry.file_type().unwrap();
        let file_name = &entry.file_name().into_string().unwrap()[..];
        let is_last_in_dir = last_entry_index == index;
        // state_table[depth] = is_last_in_dir;
        if is_last_in_dir {
            state_table[depth] = false; // Logically this shouldn't work, but it seems to work perfectly!
        }

        render_line(file_name, depth, state_table, is_last_in_dir);

        let is_dir = file_type.is_dir();
        if is_dir && depth + 1 < max_depth {
            path.push(file_name);
            search_directory(path, depth + 1, max_depth, state_table);
            path.pop();
        }
    }
}

fn render_line(file_name: &str, depth: usize, state_table: &[bool], is_last_in_dir: bool) {
    assert!(depth <= state_table.len());

    let mut print_buffer: String = String::new();

    for &render_layer in state_table.iter().take(depth) {
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

    println!("{print_buffer}");
}
