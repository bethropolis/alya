use std::env;
use std::fs;
use std::io::Write;
use std::process;
use alya_vm::assembler;
use alya_vm::instruction::{Instruction, Program};
use alya_vm::execution::VM;
use alya_vm::error::VmError;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print_usage();
        process::exit(1);
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "assemble" => {
            // Usage: alya assemble input.alya [output.bin]
            let output_file = if args.len() >= 4 { &args[3] } else { "out.bin" };
            assemble_file(filename, output_file);
        }
        "run" => {
            // Usage: alya run program.bin
            run_binary(filename);
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            print_usage();
            process::exit(1);
        }
    }
}

fn print_usage() {
    eprintln!("Alya VM Toolchain");
    eprintln!("Usage:");
    eprintln!("  alya assemble <source.alya> [output.bin]  Compile text to binary");
    eprintln!("  alya run <program.bin>                    Execute binary file");
}

fn assemble_file(input_path: &str, output_path: &str) {
    let source = fs::read_to_string(input_path).unwrap_or_else(|e| {
        eprintln!("Error reading file '{}': {}", input_path, e);
        process::exit(1);
    });

    println!("Assembling '{}'...", input_path);
    let program = assembler::assemble(&source, input_path).unwrap_or_else(|e| {
        eprintln!("Assembly error: {}", e);
        process::exit(1);
    });

    // Serialize all instructions to bytes
    let mut code_bytes = Vec::new();
    for instr in &program.instructions {
        code_bytes.extend_from_slice(&instr.encode());
    }

    // Write to file: [Code Size u64] [Code] [Data]
    let mut file = fs::File::create(output_path).unwrap();
    
    let code_size = code_bytes.len() as u64;
    file.write_all(&code_size.to_le_bytes()).unwrap();
    file.write_all(&code_bytes).unwrap();
    file.write_all(&program.data).unwrap();

    println!("Successfully wrote {} code bytes and {} data bytes to '{}'", 
             code_size, program.data.len(), output_path);
}

fn run_binary(input_path: &str) {
    let raw_bytes = fs::read(input_path).unwrap_or_else(|e| {
        eprintln!("Error reading binary '{}': {}", input_path, e);
        process::exit(1);
    });

    if raw_bytes.len() < 8 {
        eprintln!("Binary too short (missing header)");
        process::exit(1);
    }

    // Read code size
    let mut size_bytes = [0u8; 8];
    size_bytes.copy_from_slice(&raw_bytes[0..8]);
    let code_size = u64::from_le_bytes(size_bytes) as usize;

    if 8 + code_size > raw_bytes.len() {
         eprintln!("Binary truncated (code size mismatch)");
         process::exit(1);
    }

    let code_slice = &raw_bytes[8..8+code_size];
    let data_slice = &raw_bytes[8+code_size..];

    // Decode instructions
    let mut instructions = Vec::new();
    let mut pc = 0;
    while pc < code_slice.len() {
        match Instruction::decode(&code_slice[pc..]) {
            Ok((instr, len)) => {
                instructions.push(instr);
                pc += len;
            }
            Err(e) => {
                eprintln!("Corrupt binary at offset {}: {}", pc, e);
                process::exit(1);
            }
        }
    }

    let program = Program::with_data(input_path, instructions, data_slice.to_vec());
    let mut vm = VM::new();
    
    if let Err(e) = vm.run(&program) {
        match e {
            VmError::Halted => {}, 
            _ => eprintln!("Runtime Error: {}", e),
        }
    }
}
