use reqwest::get;
use serde::{Deserialize, Serialize};
use colored::*;
use text_to_ascii_art::to_art;
// use clap::Parser;

#[derive(Debug, Serialize, Deserialize)]
struct Definition {
    definition: String,
    synonyms: Vec<String>,
    antonyms: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Meaning {
    #[serde(rename = "partOfSpeech")] 
    part_of_speech: String,
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
        
        // Print word as ASCII Art
        println!();
        match to_art(word.to_string(),"standard", 0, 0, 0) {
            Ok(word) => println!("{}", word),
            Err(err) => println!("Error generating ASCII art: {}", err),
        }
        println!();

        for entry in api_response {
            for meaning in entry.meanings {
                // Mention word type
                println!("===============================");
                println!("{}", format!("\tAs a {}", meaning.part_of_speech.to_uppercase()).bold().cyan());
                println!("-------------------------------");
                
                // List all definitions for word used as that type
                for (index, def) in meaning.definitions.iter().enumerate() {
                    println!("  {}) {}", (index + 1).to_string().green(), def.definition);
                }
                
                // List all synonyms and antonyms for word used as that type
                println!();
                if !meaning.synonyms.is_empty() {
                    println!("Synonyms: {}", meaning.synonyms.join(", "));
                }
                if !meaning.antonyms.is_empty() {
                    println!("Antonyms: {}", meaning.antonyms.join(", "));
                }

                println!();
            }
        }
    } else {
        println!("ERROR: Word not found or no internet connection.");
    }

    Ok(())
}
