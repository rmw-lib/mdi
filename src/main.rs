use std::{env::current_dir, path::PathBuf};

use anyhow::Result;
use clap::{arg, command};
use mdi::parse;
use walkdir::WalkDir;

pub const DIR: &str = "dir";

pub fn main() -> Result<()> {
  let matches = command!()
    .propagate_version(true)
    .arg(arg!([dir]).value_parser(clap::value_parser!(PathBuf)))
    .get_matches();

  let dir = match matches.get_one::<PathBuf>(DIR) {
    None => {
      let pwd = current_dir()?;
      let mut dir = pwd.clone();
      loop {
        let git = dir.join(".git");
        if git.exists() {
          break;
        }
        if !dir.pop() {
          dir = pwd;
          break;
        }
      }
      dir
    }
    Some(dir) => dir.clone(),
  };

  let li = WalkDir::new(&dir).follow_links(true).into_iter();

  let ignore = dir.join(".gitignore");
  if ignore.exists() {
    if let Ok(ignore) = gitignore::File::new(&ignore) {
      return parse(
        &dir,
        li.filter_entry(|e| {
          let p = e.path();
          if let Ok(i) = ignore.is_excluded(p) {
            if p != dir && i {
              return false;
            }
          }
          true
        }),
      );
    }
  };

  parse(&dir, li)
}
