use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let json = r#"
    {
        "article": "How to work with JSON in Rust?",
        "author": "Udit Yadav",
        "paragraph": [
            {
                "name": "starting sentence"
            },
            {
                "name": "body of the paragraph"
            },
            {
                "name": "end of the paragraph"
            }
        ]
    }"#;

    let parsed_article: Article = serde_json::from_str(json).unwrap();
    println!("\nThe name of the first paragraph is: {}", parsed_article.paragraph[0].name);
}