use std::{
    fs::{read_to_string, write, File},
    process::Command,
};

fn main() {
    if let Ok(output) = Command::new("target/debug/partun")
        .arg("--help")
        .output()
        {
            let helptext = String::from_utf8_lossy(&output.stdout);
            let template = read_to_string("readme_template")
                .unwrap()
                .replace("{HELPMSG}", &helptext);
            write("README.md", template).unwrap();
        }
}
