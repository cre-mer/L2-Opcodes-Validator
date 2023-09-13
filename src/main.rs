use std::{fs::File, io::{Error, BufReader, ErrorKind, BufRead}};
use clap::Parser;
use opcodes_handler::opcode_to_string;
use serde_json::{from_str, Value};
use crate::json_validator::validate_json;
use evm_disassembler::disassemble_str;
extern crate colored;
use colored::*;

pub mod json_validator;
pub mod opcodes_handler;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to json file
    #[arg(short, long)]
    path: String,

    /// Illegal opcodes
    #[arg(short, long, num_args = 1.., default_value = "SELFDESTRUCT")]
    illegal_upcodes: Vec<String>,
}

fn main() {
    let args = Args::parse();

    // import JSON file
    let file = match File::open(&args.path) {
        Ok(it) => it,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => panic!("File does not exist"),
            ErrorKind::PermissionDenied => panic!("Permission denied"),
            _ => panic!("Error opening file: {:?}", err),
        },
    };
    let reader = BufReader::new(file);

    // Read the JSON contents of the file
    let file: Value = match from_str(&reader.lines().collect::<Result<String, Error>>().unwrap()) {
        Ok(it) => it,
        Err(err) => panic!("Error parsing file: {:?}", err),
    };

    let bytecode = validate_json(file.clone());

    match bytecode {
        Ok(it) => {
            let bytecode = it.as_str();
            let instructions = disassemble_str(bytecode).unwrap();
            
            // convert instructions into a list of opcodes
            let mut opcodes: Vec<String> = Vec::new();
            for instruction in instructions {
                opcodes.push(opcode_to_string(instruction.opcode));
            }


            println!("Looking for the following illegal opcodes: {:#?}\n", args.illegal_upcodes);
            
            let illegal_upcodes: Vec<String> = args.illegal_upcodes
                .iter()
                .map(|s| return s.to_string())
                .collect();
        
        let mut illegal_opcodes_found = Vec::new();
        for illegal_opcode in illegal_upcodes {
            if opcodes.contains(&illegal_opcode) {
                illegal_opcodes_found.push(illegal_opcode);
            }
        }
            // panic if any of the illegal opcodes were found in the list of opcodes
            if illegal_opcodes_found.len() > 0 {
                panic!("Illegal opcodes found: {:?}", illegal_opcodes_found);
            }
        },
        Err(err) => panic!("Error parsing file: {:?}", err),
    }

    println!("{}", "No illegal opcodes found ☺️".green());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_json() {
        let json = r#"{
            "bytecode": {
                "object": "608060405234801561001057600080fd5b5061012f806100206000396000f3fe60806040526004361061004c576000357c0
            }
        }"#;

        let data: Value = from_str(json).unwrap();
        let bytecode = validate_json(data).unwrap();
        assert_eq!(bytecode, "608060405260043610610104c576000357c0");
    }

    #[test]
    fn test_opcode_to_string() {
        let opcode = evm_disassembler::Opcode::ADD;
        let opcode_string = opcode_to_string(opcode);
        assert_eq!(opcode_string, "ADD");
    }

    #[test]
    // test illegal upcodes
    fn test_illegal_upcodes() {
        let json = r#"{
            "bytecode": {
                "object": "608060405234801561001057600080fd5b5061012f806100206000396000f3fe60806040526004361061004c576000357c0
            }
        }"#;

        let data: Value = from_str(json).unwrap();
        let bytecode = validate_json(data).unwrap();
        assert_eq!(bytecode, "608060405260043610610104c576000357c0");
    }
}
