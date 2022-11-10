use std::collections::HashMap;
use std::fs::{File, read_dir};
use std::io::{BufWriter, Write};
use std::io::Result;

use itertools::Itertools;

mod behavioral_patterns;
mod creational_patterns;
mod structural_patterns;

type Modules = HashMap<String, Vec<Module>>;

#[derive(Debug)]
struct Module {
    path: String,
    pattern: String,
    name: String,
}

impl Module {
    fn new(path: &str) -> Self {
        let parts = path.split('/').collect_vec();
        Self { path: path.to_string(), pattern: parts[1].to_string(), name: parts[2].to_string() }
    }
}

struct Output {
    lines: Vec<String>,
}

impl Output {
    fn new() -> Self {
        Self { lines: vec![] }
    }

    fn add(&mut self, line: &str) {
        self.lines.push(line.to_string());
    }

    fn write(&self, path: &str) -> Result<()> {
        let mut file = File::create(path)?;
        file.write_all(format!("{}\n", self.lines.join("\n")).as_bytes())
    }
}

fn find_mods() -> Result<Modules> {
    fn read_dirs(path: &str) -> Result<Vec<String>> {
        let mut found = vec![];
        for item in read_dir(path)? {
            let item = item?;
            if item.file_type()?.is_dir() {
                found.push(item.path().to_string_lossy().into_owned());
            }
        }
        found.sort();
        Ok(found)
    }

    let mut found = vec![];
    for pattern_dir in read_dirs("src")? {
        for mod_dir in read_dirs(&pattern_dir)? {
            found.push(Module::new(&mod_dir));
        }
    }
    Ok(found.into_iter().into_group_map_by(|x| x.pattern.clone()))
}

fn update_readme(modules: &Modules) -> Result<()> {
    let mut readme = Output::new();
    readme.add("# Rust Design Pattern ( ver 1 )");
    readme.add("## Versions");
    readme.add("- [ver 1](https://github.com/suzuki-hoge/rust-design-pattern/tree/master)");
    readme.add("## Implementations");

    for (pattern, modules) in modules {
        readme.add(&format!("### {}", pattern));
        for module in modules {
            readme.add(&format!("- [{}]({})", module.name, module.path));
        }
    }

    readme.write("README.md")
}

fn update_mods(modules: &Modules) -> Result<()> {
    for (pattern, modules) in modules {
        let mut deps = Output::new();

        for module in modules {
            deps.add(&format!("pub mod {};", module.name));
        }

        deps.write(&format!("src/{}.rs", pattern)).expect("some errors occurred")
    }
    Ok(())
}

fn main() {
    let modules = find_mods().unwrap();
    update_mods(&modules);
    update_readme(&modules);
}
