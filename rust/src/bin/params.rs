use std::{env::args, fs, path::Path, process::exit};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct RawHmmParams {
    pub states: Vec<String>,
    pub inits: Vec<f64>,
    pub transitions: Vec<Vec<f64>>,
    pub emissions: Vec<Vec<f64>>,
}

fn main() {
    let args = args().collect::<Vec<String>>();

    if args.len() != 2 {
        eprintln!("Usage: {} <params.json>", args[0]);
        exit(1);
    }

    let hmm_params_path = Path::new(&args[1]);

    let hmm_params_string = match fs::exists(hmm_params_path) {
        Err(e) => {
            eprintln!("Error occurred while checking file existence: {}", e);
            exit(1);
        }
        Ok(exists) => {
            if !exists {
                eprintln!("File not found: {}", hmm_params_path.display());
                exit(1);
            }
            match fs::read_to_string(hmm_params_path) {
                Err(e) => {
                    eprintln!("Error occurred while reading file: {}", e);
                    exit(1);
                }
                Ok(content) => content,
            }
        }
    };

    let hmm_params: RawHmmParams = match serde_json::from_str(&hmm_params_string) {
        Err(e) => {
            eprintln!("Error occurred while parsing JSON: {}", e);
            exit(1);
        }
        Ok(params) => params,
    };
    
    //println!("HMM Params Representation:\n {:#?}", hmm_params);
    println!(
        "HMM Params JSON String:\n {}",
        serde_json::to_string_pretty(&hmm_params).unwrap()
    );
}
