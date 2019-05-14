extern crate comrak;

use comrak::{markdown_to_html, ComrakOptions};
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

fn get_md_paths(dir: &Path) -> io::Result<Vec<PathBuf>> {
  let mut entries: Vec<PathBuf> = Vec::new();
  if dir.is_dir() {
    for entry in fs::read_dir(dir)? {
      let entry = entry?;
      let path = entry.path();
      if path.is_dir() {
        get_md_paths(&path).unwrap();
      } else {
        entries.push(path);
      }
    }
  }
  Ok(entries)
}

fn main() -> io::Result<()> {
  let content_path = Path::new("./content");
  let post_paths = get_md_paths(&content_path).unwrap();
  for path in post_paths {
    let mut file = File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let html = markdown_to_html(&contents, &ComrakOptions::default());
    println!("{:?}", html);
  }
  Ok(())
}
