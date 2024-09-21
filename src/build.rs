use std::env;
use std::fs;

fn main() {
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let build_time = chrono::offset::Utc::new().to_string();
    let output = format!(
        r#"mtree (mini tree) {}
        compiled on {}
        This is free software: you are free to change and redistribute it.
        This is NO WARRANTY, to the extent permitted by law.
        Written by Kai A Iverson from Alaska, USA. 
        See <https://github.com/kaiverson> for more awesome projects. "#,
        version, build_time
    );

    fs::write("VERSION_MESSAGE.txt", output).unwrap();
}
