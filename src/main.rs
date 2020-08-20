pub mod leetcode;
pub mod solution;

use leetcode::query::Query;
use serde_json;
use std::env;
use std::fs;
use std::path::Path;

fn fetch_problem(title_slug: String) {
    let response = Query::question_data(title_slug.clone()).invoke();

    match &response["data"].as_object().unwrap()["question"] {
        serde_json::Value::Object(question) => {
            let id: i32 = question["questionId"]
                .as_str()
                .unwrap()
                .parse::<i32>()
                .unwrap();

            let path: String = format!(
                "src/solution/s{:04}_{}.rs",
                id,
                title_slug
                    .chars()
                    .map(|c| if c == '-' { '_' } else { c })
                    .collect::<String>()
            );

            let source: String = serde_json::from_str::<serde_json::Value>(
                question["codeDefinition"].as_str().unwrap(),
            )
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .filter(|def| def["value"].as_str().unwrap() == "rust")
            .map(|def| def["defaultCode"].as_str().unwrap().to_string())
            .next()
            .unwrap();

            let source = vec![
                "pub struct solution {}",
                "",
                source.as_str(),
                "",
                "#[cfg(test)]",
                "mod tests {",
                "    use super::Solution;",
                "",
                "    #[test]",
                format!("    fn test_{:04}() {}", id, "{").as_str(),
                "    }",
                "}",
            ]
            .join("\n");

            if Path::new(&path).exists() {
                return println!("[💥] Cannot create file `{}` (already exists).", path);
            }

            fs::write(path.clone(), source).expect("Unable to write file");

            println!("[🌟] File `{}` created.", path);
        }

        _ => {
            println!("[💥] Cannot fetch the question (may not exist).");
        }
    };
}

fn show_help() {
    println!("Usage:");
    println!("  `cargo run fetch [slug]`: fetch a new problem.");
}

pub fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 2 && args[0] == String::from("fetch") {
        return fetch_problem(args[1].clone());
    }

    show_help();
}
