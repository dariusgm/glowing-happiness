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
    #[clap(long, multiple_values = false)]
    pub input: String,

    #[clap(long)]
    pub mode: Option<String>,

    #[clap(long, value_parser)]
    pub output: Option<String>,
}

pub fn run_by_option(options: &ApplicationOptions) -> Result<(), Box<dyn Error>> {
    let root = options.input.as_str();
    let files = walk(root);
    match &options.mode {
        Some(s) => {
            if s == "list" {
                let path_by_tool = collect_by_path(files);
                let counted_by_tool = count_by_path(&path_by_tool);
                let tools = Vec::from_iter(counted_by_tool.into_keys());
                match serde_json::to_string(&tools) {
                    Ok(json_string) => println!("{}", json_string),
                    Err(err) => panic!("{:?}", err)
                }
            }
        }
        None => {
            let path_by_tool = collect_by_path(files);
            let counted_by_tool = count_by_path(&path_by_tool);
                match serde_json::to_string(&counted_by_tool) {
                    Ok(json_string) => println!("{}", json_string),
                    Err(err) => panic!("{:?}", err)
                }
        }
    }
    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let options: ApplicationOptions = arg_parse();
    run_by_option(&options)
}

fn by_dir(a: &Path, predicates: &Vec<&str>) -> bool {
    for predicate in predicates {
        if a.is_dir() && a.ends_with(predicate) {
            return true;
        }
    }
    false
}

fn by_name(a: &PathBuf, predicates: &Vec<&str>) -> bool {
    for x in a {
        for predicate in predicates {
            if x.to_string_lossy().contains(predicate) {
                return true;
            }
        }
    }
    false
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

pub fn walk(root: &str) -> Vec<DirEntry> {
    let mut all_files = Vec::new();
    for p in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        all_files.push(p);
    }
    all_files
}

fn process_file(file: &DirEntry) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let type_content_map = read_type_content_map();
    let type_name_map = read_type_name_map();
    let type_name_dir_map = read_type_name_dir_map();

    let key = file.path().to_path_buf();
    for (&app, predicates) in type_name_map.iter() {
        if by_name(&key, predicates) && (!result.contains(&app.to_string())) {
            result.push(app.to_string())
        }
    }

    for (&app, predicates) in type_name_dir_map.iter() {
        if by_dir(&key, predicates) && (!result.contains(&app.to_string())) {
            result.push(app.to_string());
        }
    }

    for app in by_content(&key, type_content_map) {
        if !result.contains(&app.to_string()) {
            result.push(app)
        }
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
