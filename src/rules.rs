use std::collections::HashMap;

pub fn read_type_content_map() -> HashMap<&'static str, Vec<&'static str>> {
    HashMap::from([
        ("spark", vec!["import org.apache.spark.sql.SparkSession"]),
        (
            "spring-boot",
            vec!["import org.springframework.web.bind.annotation.RestController"],
        ),
    ])
}

pub fn read_type_name_dir_map() -> HashMap<&'static str, Vec<&'static str>> {
    HashMap::from(
        [
            ("git", vec![".git"]),
            ("circleci", vec![".circleci"]),
            ("github", vec![".github"]),
        ]
    )
}

pub fn read_type_name_map() -> HashMap<&'static str, Vec<&'static str>> {
    HashMap::from([
        // Languages
        ("c", vec![".c"]),
        ("css", vec![".css"]),
        ("dart", vec![".dart"]),
        ("gherkin", vec![".feature"]),
        ("go", vec![".go"]),
        ("html", vec![".html"]),
        ("ini", vec![".properties"]),
        ("java", vec![".java"]),
        ("javascript", vec![".js"]),
        ("jupyter notebook", vec![".ipynb"]),
        ("kotlin", vec![".kt"]),
        ("makefile", vec!["Makefile"]),
        ("markdown", vec![".md"]),
        ("objective-c", vec![".m"]),
        ("python", vec![".py"]),
        ("rust", vec![".rs"]),
        ("scala", vec![".scala"]),
        ("shell", vec![".sh"]),
        ("swift", vec![".swift"]),
        ("typescript", vec![".ts"]),
        ("xml property list", vec![".plist"]),
        ("xml", vec![".xml"]),
        ("yaml", vec![".yaml", ".yml"]),

        // Data format
        ("avro", vec![".avro"]),
        ("csv", vec![".csv"]),
        ("json", vec![".json"]),
        ("parquet", vec![".parquet"]),
        ("tsv", vec![".tsv"]),

        // Image
        ("jpeg", vec!["jpeg"]),
        ("jpg", vec!["jpg"]),
        ("png", vec![".png"]),
        ("svg", vec![".svg"]),

        // dependency manager
        ("cargo", vec!["Cargo.toml"]),
        ("gradle", vec!["build.gradle"]),
        ("npm", vec!["package.json"]),
        ("yarn", vec!["yarn.lock"]),

        // Misc
        ("docker", vec!["Dockerfile"]),
        ("gitignore", vec![".gitignore"]),
        ("jenkins", vec!["Jenkinsfile"]),
        ("toml", vec![".toml"]),
    ])
}
