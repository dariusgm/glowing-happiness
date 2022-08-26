use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::hash::Hash;
use std::io::Read;
use std::iter::{Filter, FilterMap};
use std::path::PathBuf;
use walkdir::{DirEntry, Error, IntoIter, WalkDir};

fn by_names(a: PathBuf, predicates: Vec<&str>) -> Vec<&str> {
    match a.file_name() {
        Some(t) => {
            let mut r = Vec::new();
            for mut p in predicates {
                match t.to_str() {
                    None => {
                        println!("WARN: Can't convert filepath to str")
                    }
                    Some(path) => {
                        if path.contains(p) {
                            r.push(p);
                        }
                    }
                }
            }
            r
        }
        None => Vec::new(),
    }
}

fn by_name(a: PathBuf, predicate: &str) -> bool {
    match a.file_name() {
        Some(t) => {
            println!("{:?}", t);
            true
        }
        None => false,
    }
}

fn by_content(a: &PathBuf, predicate: &str) -> bool {
    match fs::read_to_string(a) {
        Ok(content) => content.contains(predicate),
        Err(_) => false,
    }
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

fn read_type_name_map() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("git", ".git"),
        ("cdk", "cdk"),
        ("gradle", "build.gradle"),
        ("docker", "Dockerfile"),
        ("java", ".java"),
        ("scala", ".scala"),
        ("python", ".py"),
        ("kotlin", ".kt"),
        ("jenkins", "Jenkinsfile"),
        ("go", ".go"),
        ("npm", "package.json"),
        ("yarn", "yarn.lock"),
    ])
}

fn main() {
    // let mut type_content_map = read_type_content_map();
    let mut type_name_map = read_type_name_map();
    let root = ".";
    let mut all_files = Vec::new();
    for p in WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        all_files.push(p);
    }

    let mut result = HashMap::<PathBuf, Vec<&str>>::new();
    for x in all_files {
        for (app, predicate) in type_name_map.iter() {
            let key = x.path().to_path_buf();
            if by_content(&key, predicate) {
                if result.contains_key(&key) {
                    let mut values = result.get_mut(&key);
                    match values {
                        Some(v) => v.push(&app),
                        _ => {}
                    };
                } else {
                    result.insert(key, vec![&app]);
                }
            }
        }
    }

    println!("{:?}", result);
    println!("Hello, world!");
}
