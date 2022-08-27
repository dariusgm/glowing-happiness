use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::io::Read;
use std::iter::{Filter, FilterMap};
use std::path::PathBuf;

use walkdir::{DirEntry, Error, IntoIter, WalkDir};

fn by_name(a: &PathBuf, predicate: &str) -> bool {
    match fs::read_to_string(a) {
        Ok(content) => content.contains(predicate),
        Err(_) => false,
    }
}

fn by_content(a: &PathBuf, predicate: HashMap<&str, &str>) -> Vec<String> {
    let mut result = Vec::new();
    match fs::read_to_string(a) {
        Ok(content) => {
            for (app, predicate) in predicate.iter() {
                if content.contains(predicate) {
                    result.push(String::from(*app));
                }
            }
        }
        Err(_) => {}
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

fn walk(root: &str) -> Vec<DirEntry> {
    let mut all_files = Vec::new();
    for p in WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        all_files.push(p);
    }
    all_files
}

fn main() {
    let type_content_map = read_type_content_map();
    let type_name_map = read_type_name_map();
    let root = ".";
    let files = walk(root);

    let mut result = HashMap::<PathBuf, Vec<String>>::new();
    for file in files {
        let key = file.path().to_path_buf();
        for (app, predicate) in type_name_map.iter() {
            if by_name(&key, predicate) {
                match result.get_mut(&key) {
                    Some(v) => v.push(String::from(*app)),
                    None => {
                        result.insert(key.clone(), vec![String::from(*app)]);
                    }
                };
            }
        }

        for app in by_content(&key, type_content_map.clone()) {
            match result.get_mut(&key) {
                Some(v) => v.push(app),
                None => {
                    result.insert(key.clone(), vec![app]);
                }
            }
        }
    }

    println!("{:?}", &result);
    println!("Hello, world!");
}
