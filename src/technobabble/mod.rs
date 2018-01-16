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
                return Err("Désolé chéri, mon intégrité structurelle est cassée : mes fonctionnalités liées au technobabble ne fonctionnent pas.");
            }
        };

        Ok(Technobabble { source })
    }

    pub fn generate(&self) -> String {
        let mut rng = rand::thread_rng();

        let suffixes_or_prefixes_rng = Range::new(0, 2);
        let suffixes_or_prefixes = suffixes_or_prefixes_rng.ind_sample(&mut rng);
        
        // random noun
        let noun_rng = Range::new(0, self.source.fr.nouns.len());
        let noun_idx = noun_rng.ind_sample(&mut rng);
        let noun = &self.source.fr.nouns[noun_idx];

        // prefix + noun
        if suffixes_or_prefixes == 0 {
            let prefixes_rng = Range::new(0, self.source.fr.prefixes.len());
            let prefixes_idx = prefixes_rng.ind_sample(&mut rng);
            let prefix = &self.source.fr.prefixes[prefixes_idx];

            let prefix_noun = prefix.to_string() + &' '.to_string() + &noun.to_string();

            return String::from(prefix_noun);
        }

        // noun + suffix
        let suffixes_rng = Range::new(0, self.source.fr.suffixes.len());
        let suffixes_idx = suffixes_rng.ind_sample(&mut rng);
        let suffix = &self.source.fr.suffixes[suffixes_idx];

        let noun_suffix = noun.to_string() + &' '.to_string() + &suffix.to_string();

        return String::from(noun_suffix);
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