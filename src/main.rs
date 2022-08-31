use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use walkdir::{DirEntry, WalkDir};

fn by_dir(a: &PathBuf, predicate: &str) -> bool {
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

fn read_type_name_dir_map() -> HashMap<&'static str, &'static str> {
    HashMap::from([("git", ".git")])
}

fn read_type_name_map() -> HashMap<&'static str, &'static str> {
    HashMap::from([
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
    for p in WalkDir::new(root).into_iter().filter_map(Result::ok)
    //.filter(|e| !e.file_type().is_dir())
    //.filter(|e| !e.path().starts_with("./.git"))
    {
        all_files.push(p);
    }
    all_files
}

fn collect_by_path(files: Vec<DirEntry>) -> HashMap<PathBuf, Vec<String>> {
    let type_content_map = read_type_content_map();
    let type_name_map = read_type_name_map();
    let type_name_dir_map = read_type_name_dir_map();

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

        for (app, predicate) in type_name_dir_map.iter() {
            if by_dir(&key, predicate) {
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
    result
}

fn count_by_path(hashmap: &HashMap<PathBuf, Vec<String>>) -> HashMap<String, usize> {
    let mut result: HashMap<String, usize> = HashMap::new();
    for (_path, tools) in hashmap.iter() {
        for tool in tools {
            match result.get(tool) {
                Some(&t) => {
                    result.insert(tool.to_string(), t + 1)
                },
                None => result.insert(tool.to_string(), 1)
            };
        }
    }

    result
}

fn main() {
    let root = ".";
    let files = walk(root);
    let path_by_tool = collect_by_path(files);
    println!("{:?}", &path_by_tool);

    let counted_by_tool = count_by_path(&path_by_tool);
    println!("{:?}", &counted_by_tool);
    let tools = Vec::from_iter(counted_by_tool.into_keys());
    println!("{:?}", &tools);
}
