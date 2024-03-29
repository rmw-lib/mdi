use std::{
  fs::{self, File},
  io::{BufRead, BufReader, BufWriter, Write},
  iter::Iterator,
  path::PathBuf,
};

use anyhow::Result;
use pathdiff::diff_paths;
use phf::phf_map;
use walkdir::DirEntry;

pub const PREFIX_PWD: &str = "> ./";
pub const PREFIX_HOME: &str = "> ~/";

pub const EXT_LANG: phf::Map<&'static str, &'static str> = phf_map! {
  "rs" => "rust",
};

pub fn replace_all(
  txt: impl Into<String>,
  begin: impl AsRef<str>,
  end: impl AsRef<str>,
  replace: impl Fn(&str) -> String,
) -> String {
  let begin = begin.as_ref();
  let end = end.as_ref();
  let txt = txt.into();
  let mut n = 0;
  let len = txt.len();

  let mut li = vec![];

  while n < len {
    if let Some(pos) = txt[n..].find(begin) {
      let t = n + pos;
      li.push(txt[n..t].to_string());
      n = t;
      let t = n + begin.len();
      if let Some(pos) = txt[t..].find(end) {
        let t = t + pos + end.len();
        li.push(replace(txt[n..t].into()));
        n = t;
        continue;
      }
    }
    if n > 0 {
      li.push(txt[n..].into());
    }
    break;
  }
  if li.is_empty() {
    txt
  } else {
    li.join("")
  }
}

pub fn block(
  outdir: &PathBuf,
  out: &mut impl Write,
  line: &str,
  i: &String,
  indent: &Option<String>,
  root: &PathBuf,
  base: &PathBuf,
  prefix: &str,
) -> Result<bool> {
  if i.starts_with(prefix) {
    let t = &i[2..];
    let name = &t[2..];
    let fp = base.join(name);

    if fp.exists() {
      let ext = fp.extension();
      let is_md = if let Some(ext) = ext {
        ext == "md"
      } else {
        false
      };

      let n = line.len() - i.len();

      let space = if n > 0 {
        let t = &line[..n];
        Some(if let Some(i) = &indent {
          i.to_string() + t
        } else {
          String::from(&line[..n])
        })
      } else {
        indent.as_ref().map(|i| i.to_string())
      };

      if is_md {
        let md = BufReader::new(fs::File::open(&fp)?);
        out.flush()?;
        render(outdir, root, md, out, fp, space)?;
      } else {
        let mut link = format!(
          "[→ {}]({})\n\n",
          name,
          diff_paths(&fp, outdir).unwrap_or(PathBuf::new()).display()
        );

        if let Some(space) = &space {
          link = space.to_owned() + &link + space;
        }

        link += "```";

        out.write_all(link.as_bytes())?;

        if let Some(ext) = ext {
          if let Some(ext) = ext.to_str() {
            let lang = EXT_LANG.get(ext).unwrap_or(&ext);
            out.write_all(lang.as_bytes())?;
          }
        }

        out.write_all(b"\n")?;

        let infile = &mut fs::File::open(&fp)?;
        if let Some(space) = &space {
          let space = space.as_bytes();
          for i in BufReader::new(infile).lines().flatten() {
            out.write_all(space)?;
            out.write_all(i.as_bytes())?;
            out.write_all(b"\n")?;
          }
        } else {
          for i in BufReader::new(infile).lines().flatten() {
            out.write_all(i.as_bytes())?;
            out.write_all(b"\n")?;
          }
        }

        if let Some(space) = &space {
          out.write_all(space.as_bytes())?;
        }

        out.write_all(b"```\n\n")?;
      }
      return Ok(true);
    }
  }

  Ok(false)
}

pub fn inline(prefix: &str, base: &PathBuf, i: &str) -> String {
  let prefix = "`".to_owned() + prefix;
  replace_all(i, &prefix, "`", |file| {
    let len = file.len();
    let fp = base.join(&file[prefix.len()..len - 1]);
    if fp.exists() {
      if let Ok(s) = fs::read_to_string(fp) {
        return s.trim().into();
      }
    }
    file.into()
  })
}

pub fn render(
  outdir: &PathBuf,
  root: &PathBuf,
  md: impl BufRead,
  out: &mut impl Write,
  mut pwd: PathBuf,
  indent: Option<String>,
) -> Result<()> {
  out.write_all(format!("<!-- EDIT {} -->\n\n", pwd.display()).as_bytes())?;

  pwd.pop();

  'outer: for line in md.lines().flatten() {
    let mut i = line.trim_start().to_owned();
    let line = line.as_ref();

    let li = [(PREFIX_PWD, &pwd), (PREFIX_HOME, root)];
    for (prefix, dir) in li {
      if block(outdir, out, line, &i, &indent, root, dir, prefix)? {
        continue 'outer;
      }
    }
    if let Some(space) = &indent {
      out.write_all(space.as_bytes())?;
    }

    for (prefix, dir) in li {
      i = inline(prefix, dir, &i);
    }

    out.write_all(i.as_bytes())?;
    out.write_all(b"\n")?;
  }
  Ok(())
}

pub fn parse(
  root: &PathBuf,
  li: impl Iterator<Item = std::result::Result<DirEntry, walkdir::Error>>,
) -> Result<()> {
  for fp in li {
    if let Ok(fp) = err::ok!(fp) {
      let mut fp: PathBuf = fp.path().into();
      if let Some(name) = fp.file_name() {
        if let Some(name) = name.to_str() {
          if name.ends_with(".mdi.md") {
            let mut name = name.to_owned();
            name.truncate(name.len() - 5);
            name += "d";

            let md = fs::File::open(&fp)?;
            fp.set_file_name(name);
            let out = File::create(&fp)?;
            println!("{}", fp.display());

            let mut outdir = fp.clone();
            outdir.pop();

            render(
              &outdir,
              root,
              BufReader::new(md),
              &mut BufWriter::new(out),
              fp,
              None,
            )?;
          }
        }
      }
    }
  }
  Ok(())
}
