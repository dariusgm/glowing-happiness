use std::collections::HashMap;

pub fn read_type_content_map() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("spark", "import org.apache.spark.sql.SparkSession"),
        (
            "spring-boot",
            "import org.springframework.web.bind.annotation.RestController",
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
        ("dart", vec![".dart"]),
        ("ini", vec![".properties"]),
        ("jupyter notebook", vec![".ipynb"]),
        ("makefile", vec!["Makefile"]),
        ("objective-c", vec![".m"]),
        ("typescript", vec![".ts"]),
        ("xml property list", vec![".plist"]),
        ("c", vec![".c"]),
        ("css", vec![".css"]),
        ("go", vec![".go"]),
        ("html", vec![".html"]),
        ("java", vec![".java"]),
        ("javascript", vec![".js"]),
        ("kotlin", vec![".kt"]),
        ("markdown", vec![".md"]),
        ("python", vec![".py"]),
        ("rust", vec![".rs"]),
        ("scala", vec![".scala"]),
        ("shell", vec![".sh"]),
        ("swift", vec![".swift"]),
        ("xml", vec![".xml"]),
        ("yaml", vec![".yaml", ".yml"]),

        // Data format
        ("avro", vec![".avro"]),
        ("csv", vec![".csv"]),
        ("json", vec![".json"]),
        ("parquet", vec![".parquet"]),
        ("tsv", vec![".tsv"]),

        // Image
        ("jpg", vec!["jpeg"]),
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
