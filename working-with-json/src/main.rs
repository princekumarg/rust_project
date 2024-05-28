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
  "article": "how to work with json in Rust",
  "author": "akhil",
  "paragraph": [
    {
      "name": "starting sentences"
    },
    {
      "name": "body of the paragraph"
    },
    {
      "name": "end of the paragraph"
    }
  ]
}
"#;
    let article: Article = Article {
        article: String::from("how to work with json in Rust"),
        author: String::from("prince"),
        paragraph: vec![
            Paragraph {
                name: String::from("first sentence"),
            },
            Paragraph {
                name: String::from("second sentence"),
            },
            Paragraph {
                name: String::from("third sentence"),
            },
        ],
    };
    let parsed: Article = read_json_typed(json);
    let jsons = serde_json::to_string(&article).unwrap();
    println!(
        "\n\n The name of the first paragraph is: {}",
        parsed.paragraph[0].name
    );
    println!("the json is :{}", jsons);
}

fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed;
}
