extern crate core;

use core::result::Result;
use core::result::Result::{Err, Ok};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Mutex;
use walkdir::DirEntry;

use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use clap::Parser;
use crate::parsing::arg_parse;

mod parsing;

#[derive(Parser)]
#[clap(name = "Glowing-Happiness")]
#[clap(version = "0.1")]
#[clap(about = "Analyse what kind of files you have in a directory", long_about = None)]
pub struct ApplicationOptions {
    #[clap(long, multiple_values = false)]
    pub input: String,

    #[clap(long, value_parser)]
    pub output: Option<String>,
}

pub fn run_by_option(options: &ApplicationOptions) -> Result<(), Box<dyn Error>> {
    let root = options.input.as_str();
    let files = walk(root);
    let path_by_tool = collect_by_path(files);
    println!("{:?}", &path_by_tool);

    let counted_by_tool = count_by_path(&path_by_tool);
    println!("{:?}", &counted_by_tool);
    let tools = Vec::from_iter(counted_by_tool.into_keys());
    println!("{:?}", &tools);
    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let options: ApplicationOptions = arg_parse();
    run_by_option(&options)
}

fn by_dir(a: &Path, predicate: &str) -> bool {
    a.is_dir() && a.ends_with(predicate)
}

fn by_name(a: &PathBuf, predicate: &str) -> bool {
    match fs::read_to_string(a) {
        Ok(content) => content.contains(predicate),
        Err(_) => false,
    }
}

fn by_content(a: &PathBuf, predicate: HashMap<&str, &str>) -> Vec<String> {
    let mut result = Vec::new();
    if let Ok(content) = fs::read_to_string(a) {
        for (app, predicate) in predicate.iter() {
            if content.contains(predicate) {
                result.push(String::from(*app));
            }
        }
    }
    result
}

fn read_type_content_map() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("spark", "import org.apache.spark.sql.SparkSession"),
        (
            "spring-boot",
            "import org.springframework.web.bind.annotation.RestController",
        ),
    ])
}

fn read_type_name_dir_map() -> HashMap<&'static str, &'static str> {
    HashMap::from(
        [
            ("git", ".git"),
            ("circleci", ".circleci"),
            ("github", ".github"),
        ]
    )
}

fn read_type_name_map() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        // Languages
        ("Dart", ".dart"),
        ("Ini", ".properties"),
        ("Jupyter Notebook", ".ipynb"),
        ("Makefile", "Makefile"),
        ("Objective-C", ".m"),
        ("TypeScript", ".ts"),
        ("XML Property List", ".plist"),
        ("c", ".c"),
        ("css", ".css"),
        ("go", ".go"),
        ("html",".html"),
        ("java", ".java"),
        ("javascript", ".js"),
        ("json", ".json"),
        ("kotlin", ".kt"),
        ("markdown", ".md"),
        ("python", ".py"),
        ("rust", ".rs"),
        ("scala", ".scala"),
        ("shell", ".sh"),
        ("swift", ".swift"),
        ("xml", ".xml"),
        ("yaml", ".yaml"),
        ("yaml", ".yml"),

        // Image
        ("png", ".png"),
        ("jpg", "jpg"),
        ("jpg", "jpeg"),
        ("svg",".svg"),

        // dependency manager
        ("gradle", "build.gradle"),
        ("yarn", "yarn.lock"),
        ("npm", "package.json"),
        ("cargo", "Cargo.toml"),

        // Misc
        ("docker", "Dockerfile"),
        ("gitignore", ".gitignore"),
        ("jenkins", "Jenkinsfile"),
        ("toml", ".toml"),
        ("test", "tests"),
    ])
}

pub fn walk(root: &str) -> Vec<DirEntry> {
    let mut all_files = Vec::new();
    for p in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        all_files.push(p);
    }
    all_files
}

fn process_file(file: &DirEntry) -> Vec<String> {
    let mut result = Vec::new();
    let type_content_map = read_type_content_map();
    let type_name_map = read_type_name_map();
    let type_name_dir_map = read_type_name_dir_map();

    let key = file.path().to_path_buf();
    for (app, predicate) in type_name_map.iter() {
        if by_name(&key, predicate) {
            result.push(String::from(*app))
        }
    }

    for (app, predicate) in type_name_dir_map.iter() {
        if by_dir(&key, predicate) {
            result.push(String::from(*app));
        }
    }

    for app in by_content(&key, type_content_map) {
        result.push(app)
    }
    result
}

pub fn collect_by_path(files: Vec<DirEntry>) -> HashMap<PathBuf, Vec<String>> {
    let result = Mutex::new(HashMap::<PathBuf, Vec<String>>::new());
    files.par_iter().for_each(|a| {
        let r = process_file(a);
        if !r.is_empty() {
            match result.lock() {
                Ok(x) => x,
                Err(_) => panic!(),
            }
            .insert(a.clone().into_path(), r);
        }
    });

    let real_result = match result.lock() {
        Ok(unboxed) => unboxed.to_owned(),
        Err(_) => panic!(),
    };
    real_result
}

pub fn count_by_path(hashmap: &HashMap<PathBuf, Vec<String>>) -> HashMap<String, usize> {
    let mut result: HashMap<String, usize> = HashMap::new();
    for (_path, tools) in hashmap.iter() {
        for tool in tools {
            match result.get(tool) {
                Some(&t) => result.insert(tool.to_string(), t + 1),
                _ => result.insert(tool.to_string(), 1),
            };
        }
    }

    result
}
