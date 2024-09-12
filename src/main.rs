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
    synonyms: Vec<String>,
    antonyms: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    word: String,
    meanings: Vec<Meaning>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let args = Cli::parse();
    let word = "love";

    // Call the API
    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);
    let response = get(&url).await?;
    
    if response.status().is_success() {
        let api_response: Vec<ApiResponse> = response.json().await?;
        
        for entry in api_response {
            for meaning in entry.meanings {
                // Mention word type
                println!("As a {}:", meaning.partOfSpeech);
                
                // List all definitions for word used as that type
                for (index, def) in meaning.definitions.iter().enumerate() {
                    println!("{}) {}", index+1, def.definition);
                }
                
                // List all synonyms and antonyms for word used as that type
                if !meaning.synonyms.is_empty() {
                    println!("Synonyms: {:?}", meaning.synonyms);
                }
                if !meaning.antonyms.is_empty() {
                    println!("Antonyms: {:?}", meaning.antonyms);
                }

                println!();
            }
        }
    } else {
        println!("ERROR: Word not found or no internet connection.");
    }

    Ok(())
}
