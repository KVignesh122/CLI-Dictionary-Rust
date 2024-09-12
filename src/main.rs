use reqwest::get;
use serde::{Deserialize, Serialize};
// use clap::Parser;

#[derive(Debug, Serialize, Deserialize)]
struct Definition {
    definition: String,
    synonyms: Vec<String>,
    antonyms: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Meaning {
    partOfSpeech: String,
    definitions: Vec<Definition>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    word: String,
    meanings: Vec<Meaning>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let args = Cli::parse();
    let word = "hello";

    // Call the API
    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);
    let response = get(&url).await?;
    
    if response.status().is_success() {
        let api_response: Vec<ApiResponse> = response.json().await?;
        for entry in api_response {
            println!("Word: {}\n", entry.word);
            for meaning in entry.meanings {
                println!("Part of Speech: {}", meaning.partOfSpeech);
                for def in meaning.definitions {
                    println!("Definition: {}", def.definition);
                    if !def.synonyms.is_empty() {
                        println!("Synonyms: {:?}", def.synonyms);
                    }
                    if !def.antonyms.is_empty() {
                        println!("Antonyms: {:?}", def.antonyms);
                    }
                }
                println!();
            }
        }
    } else {
        println!("Word not found");
    }

    Ok(())
}
