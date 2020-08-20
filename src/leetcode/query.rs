use reqwest::blocking::Client;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

const API_URL: &str = "https://leetcode.com/graphql";

#[derive(Serialize)]
pub struct Query {
    operation_name: String,
    variables: HashMap<String, String>,
    query: String,
}

impl Query {
    pub fn question_data(title_slug: String) -> Query {
        Query {
            operation_name: String::from("questionData"),
            variables: [(String::from("titleSlug"), title_slug)]
                .iter()
                .cloned()
                .collect::<HashMap<String, String>>(),
            query: String::from(
                r#"
                      query questionData($titleSlug: String!) {
                        question(titleSlug: $titleSlug) {
                          questionId
                          titleSlug
                          content
                          stats
                          codeDefinition
                          sampleTestCase
                          metaData
                        }
                      }
                    "#,
            ),
        }
    }

    pub fn invoke(&self) -> Value {
        Client::new()
            .post(API_URL)
            .json(&self)
            .send()
            .unwrap()
            .json::<Value>()
            .unwrap()
    }
}
