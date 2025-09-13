extern crate core;

use core::result::Result;
use core::result::Result::{Err, Ok};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use clap::Parser;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use walkdir::DirEntry;
use walkdir::WalkDir;


use crate::parsing::arg_parse;
use crate::rules::{read_type_content_map, read_type_name_dir_map, read_type_name_map};

mod parsing;
mod rules;

#[derive(Parser)]
#[clap(name = "Glowing-Happiness")]
#[clap(version = "0.1")]
#[clap(about = "Analyse what kind of files you have in a directory", long_about = None)]
pub struct ApplicationOptions {
    #[clap(long)]
    pub input: String,

    #[clap(long)]
    pub config: Option<String>,

    #[clap(long)]
    pub mode: Option<String>,

    #[clap(long, value_parser)]
    pub output: Option<String>,
}

#[derive(Deserialize)]
pub struct Config {
    pub name_map: HashMap<String, Vec<String>>,
    pub dir_map: HashMap<String, Vec<String>>,
    pub content_map: HashMap<String, Vec<String>>
}

impl Default for Config {
    fn default() -> Self {
        Config {
            name_map: read_type_name_map()
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.into_iter().map(|s| s.to_string()).collect()))
                .collect(),
            dir_map: read_type_name_dir_map()
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.into_iter().map(|s| s.to_string()).collect()))
                .collect(),
            content_map: read_type_content_map()
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.into_iter().map(|s| s.to_string()).collect()))
                .collect()
        }
    }
}


fn read_config(path_opt: &Option<String>) -> Result<Config, Box<dyn Error>> {
    if let Some(path_str) = path_opt {
        let path = Path::new(path_str);
        if !path.exists() {
            return Err(format!("Konfigurationsdatei nicht gefunden: {}", path_str).into());
        }
        let yaml = fs::read_to_string(path)?;
        let cfg: Config = serde_yaml::from_str(&yaml)
            .map_err(|e| format!("YAML kann nicht geparst werden: {e}"))?;
        Ok(cfg)
    } else {
        Ok(Config::default())
    }
}

fn write_json<T>(value: &T)
where
    T: ?Sized + Serialize
{
    match serde_json::to_string(&value) {
        Ok(json_string) => println!("{}", json_string),
        Err(err) => panic!("{:?}", err)
    }
}

pub fn run_by_option(options: &ApplicationOptions) -> Result<(), Box<dyn Error>> {
    let root = options.input.as_str();
    let files = walk(root);

    let mode = match &options.mode {
        Some(s) => s,
        None => "count_by_tool"
    };

    let config = read_config(&options.config)?;
    let path_by_tool = collect_by_path(&files, &config);

    if mode == "list_by_file" {
        write_json(&path_by_tool);
    } else {
        let counted_by_tool = count_by_path(&path_by_tool);

        if mode == "list" {
            let tools = Vec::from_iter(counted_by_tool.into_keys());
            write_json(&tools);
        } else {
            write_json(&counted_by_tool);
        }
    }
    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let options: ApplicationOptions = arg_parse();
    run_by_option(&options)
}

fn by_dir(a: &Path, predicates: &Vec<String>) -> bool {
    for predicate in predicates {
        if a.is_dir() && a.ends_with(predicate) {
            return true;
        }
    }
    false
}

fn by_name(a: &PathBuf, predicates: &Vec<String>) -> bool {
    for x in a {
        for predicate in predicates {
            let path_as_string = x.to_string_lossy().to_string();
            if path_as_string.ends_with(predicate) {
                return true;
            }
        }
    }
    false
}

fn by_content(a: &PathBuf, predicate: HashMap<String, Vec<String>>) -> Vec<String> {
    let mut result = Vec::new();
    if let Ok(content) = fs::read_to_string(a) {
        for (app, predicates) in predicate.iter() {
            for predicate in predicates {
                if content.contains(predicate) {
                    result.push(String::from(app));
                }
            }
        }
    }
    result
}

pub fn walk(root: &str) -> Vec<DirEntry> {
    let mut all_files = Vec::new();
    for p in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        all_files.push(p);
    }
    all_files
}

fn process_file(file: &DirEntry, config: &Config) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let type_content_map = config.content_map.clone();
    let type_name_map = config.name_map.clone();
    let type_name_dir_map = config.dir_map.clone();

    let key = file.path().to_path_buf();
    for (app_name_map, predicates) in type_name_map.iter() {
        if by_name(&key, predicates) && (!result.contains(&app_name_map)) {
            result.push(app_name_map.clone())
        }
    }

    for (app_name_dir, predicates) in type_name_dir_map.iter() {
        if by_dir(&key, predicates) && (!result.contains(&app_name_dir)) {
            result.push(app_name_dir.clone());
        }
    }

    for app in by_content(&key, type_content_map) {
        if !result.contains(&app.to_string()) {
            result.push(app)
        }
    }

    result
}

pub fn collect_by_path(files: &Vec<DirEntry>, config: &Config) -> HashMap<PathBuf, Vec<String>> {
    let result = Mutex::new(HashMap::<PathBuf, Vec<String>>::new());
    files.par_iter().for_each(|directory_entry| {
        let r = process_file(directory_entry, config);
        if !r.is_empty() {
            match result.lock() {
                Ok(x) => x,
                Err(_) => panic!(),
            }
                .insert(directory_entry.clone().into_path(), r);
        }
    });

    let real_result = match result.lock() {
        Ok(unboxed) => unboxed.to_owned(),
        Err(_) => panic!(),
    };
    real_result
}

fn count_by_path(hashmap: &HashMap<PathBuf, Vec<String>>) -> HashMap<String, usize> {
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

#[test]
fn test_count_by_path() {
    let path_buf_1 = PathBuf::from("/tmp/main.rs");
    let tools_2 = vec!["readme".to_owned()];
    let path_buf_2 = PathBuf::from("/tmp/README.md");
    let tools_1 = vec!["rust".to_owned(), "readme".to_owned()];
    let hashmap = HashMap::from([(path_buf_1, tools_1), (path_buf_2, tools_2)]);
    let result = count_by_path(&hashmap);
    assert_eq!(result.get("readme").unwrap().to_owned(), 2_usize);
    assert_eq!(result.get("rust").unwrap().to_owned(), 1_usize);
}

#[test]
fn test_by_name_1() {
    let path = PathBuf::from("somefile.txt");
    let predicates = vec![String::from(".txt")];
    assert!(by_name(&path, &predicates))
}

#[test]
fn test_by_name_2() {
    let path = PathBuf::from("hello.cpp");
    let predicates = vec![String::from(".c")];
    assert!(!by_name(&path, &predicates))
}