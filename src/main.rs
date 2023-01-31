use std::fs;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;

use color_eyre::eyre::Result;
use ansi_term::{Colour, Style};

fn print_dir_tree(dir: &Path, prefix: &str) -> Result<()> {

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = path.metadata()?;

        let style = if metadata.is_dir() {
            Style::new().fg(Colour::Blue).bold()
        } else if metadata.is_file() && metadata.permissions().mode() & 0o111 != 0 {
            Style::new().fg(Colour::Green).bold()
        } else {
            Style::new()
        };

        println!("{}├── {}", prefix, style.paint(entry.file_name().to_string_lossy()));

        if metadata.is_dir() {
            let sub_dir = path.as_path();
            let sub_prefix = format!("{prefix}|   ");
            print_dir_tree(sub_dir, &sub_prefix)?;
        }
    }

    Ok(())
}

fn main() -> Result<()> {

    print_dir_tree(Path::new("."), "")?;

    Ok(())
}
