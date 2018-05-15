use std::error::Error;
use std::fs::File;
use std::path::Path;
use serde_json;
use rand;
use rand::distributions::{IndependentSample, Range};

#[derive(Debug)]
pub struct Technobabble {
    source: TechnobabbleSource
}

#[derive(Deserialize, Debug)]
struct TechnobabbleSource {
    fr: FrenchTechnobabble,
    en: EnglishTechnobabble,
}

#[derive(Deserialize, Debug)]
struct FrenchTechnobabble {
    prefixes: Vec<String>,
    suffixes: Vec<String>,
    nouns: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct EnglishTechnobabble {
    prefixes: Vec<String>,
    suffixes: Vec<String>,
    nouns: Vec<String>,
}

impl Technobabble {
    pub fn new() -> Result<Technobabble, &'static str> {
        let source: TechnobabbleSource = match extract("technobabble.json") {
            Ok(technobabble) => technobabble,
            Err(e) => {
                eprintln!("Fail to read the technobabble file : {}", e);
                return Err("Sorry darling, my structual integrity is broken : my technobabble features are not working.");
            }
        };

        Ok(Technobabble { source })
    }

    pub fn generate(&self) -> String {
        let mut rng = rand::thread_rng();

        // random noun
        let noun_rng = Range::new(0, self.source.en.nouns.len());
        let noun_idx = noun_rng.ind_sample(&mut rng);
        let noun = &self.source.en.nouns[noun_idx];

        // prefix + noun
        let prefixes_rng = Range::new(0, self.source.en.prefixes.len());
        let prefixes_idx = prefixes_rng.ind_sample(&mut rng);
        let prefix = &self.source.en.prefixes[prefixes_idx];

        let prefix_noun = prefix.to_string() + &' '.to_string() + &noun.to_string();

        return String::from(prefix_noun);
    }
}

fn extract<P: AsRef<Path>>(path: P) -> Result<TechnobabbleSource, Box<Error>> {
    // Open the file in read-only mode.
    let file = File::open(path)?;

    // Read the JSON contents of the file
    let technobabble: TechnobabbleSource = serde_json::from_reader(file)?;

    // Return the list.
    Ok(technobabble)
}