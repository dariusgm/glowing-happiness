use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::iter::{Filter, FilterMap};
use std::path::PathBuf;
use walkdir::{DirEntry, Error, IntoIter, WalkDir};


fn by_names(a:PathBuf, predicates: Vec<&str>) -> Vec<&str> {
    match a.file_name() {
        Some(t) => {
            let mut r = Vec::new();
            for mut p in predicates {
                match t.to_str() {
                    None => { println!("WARN: Can't convert filepath to str") }
                    Some(path) => if path.contains(p) {
                        r.push(p);
                    },
                }
            }
            r
        },
        None => Vec::new()
    }
}

fn by_name(a:PathBuf, predicate: &str) -> bool {
    match a.file_name()   {
        Some(t) => {
            println!("{:?}", t);
            true
        },
        None => { false }
    }
}

fn by_content(a:PathBuf, predicate: &str) ->bool {
    match fs::read_to_string(a) {
        Ok(content) => {
            content.contains(predicate)
        }
        Err(_) => { false }
    }
}

fn main() {
    let root = ".";
    let mut all_files = Vec::new();
    for p in WalkDir::new(root)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir()) {

        all_files.push(p);
    }

    for x in all_files {
        println!("{:?}", x)
    }
    println!("Hello, world!");
}
