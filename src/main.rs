use std::env;
use std::fs;
use std::io::Write;
use std::process;
use alya_vm::assembler;
use alya_vm::instruction::{Instruction, Program};
use alya_vm::execution::{VM, debugger::Debugger};
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
        "disassemble" | "disasm" => {
            // Usage: alya disassemble program.bin
            disassemble_binary(filename);
        }
        "debug" => {
            // Usage: alya debug program.bin
            run_debugger(filename);
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
    eprintln!("  alya disassemble <program.bin>            Convert binary back to assembly");
    eprintln!("  alya debug <program.bin>                  Start interactive debugger");
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

    // Write to file with header and debug info
    let mut file = fs::File::create(output_path).unwrap();
    
    // Header
    file.write_all(b"ALYA").unwrap();
    file.write_all(&1u16.to_le_bytes()).unwrap(); // Version 1
    
    // Code Section
    let code_size = code_bytes.len() as u64;
    file.write_all(&code_size.to_le_bytes()).unwrap();
    file.write_all(&code_bytes).unwrap();
    
    // Data Section
    let data_size = program.data.len() as u64;
    file.write_all(&data_size.to_le_bytes()).unwrap();
    file.write_all(&program.data).unwrap();

    // Debug Section: Line Table
    let line_count = program.line_table.len() as u64;
    file.write_all(&line_count.to_le_bytes()).unwrap();
    for &line in &program.line_table {
        file.write_all(&(line as u64).to_le_bytes()).unwrap();
    }

    println!("Successfully wrote {} code bytes, {} data bytes, and {} debug entries to '{}'", 
             code_size, data_size, line_count, output_path);
}

fn run_binary(input_path: &str) {
    let raw_bytes = fs::read(input_path).unwrap_or_else(|e| {
        eprintln!("Error reading binary '{}': {}", input_path, e);
        process::exit(1);
    });

    if raw_bytes.len() < 6 {
        eprintln!("Binary too short (missing header)");
        process::exit(1);
    }

    // Check magic
    if &raw_bytes[0..4] != b"ALYA" {
        // Fallback or legacy check? 
        // For now, let's enforce the new format.
        eprintln!("Invalid binary format (missing ALYA header)");
        process::exit(1);
    }

    let version = u16::from_le_bytes([raw_bytes[4], raw_bytes[5]]);
    if version != 1 {
        eprintln!("Unsupported binary version: {}", version);
        process::exit(1);
    }

    let mut cursor = 6;

    // Read code size
    if cursor + 8 > raw_bytes.len() { process::exit(1); }
    let code_size = u64::from_le_bytes(raw_bytes[cursor..cursor+8].try_into().unwrap()) as usize;
    cursor += 8;

    if cursor + code_size > raw_bytes.len() { process::exit(1); }
    let code_slice = &raw_bytes[cursor..cursor+code_size];
    cursor += code_size;

    // Read data size
    if cursor + 8 > raw_bytes.len() { process::exit(1); }
    let data_size = u64::from_le_bytes(raw_bytes[cursor..cursor+8].try_into().unwrap()) as usize;
    cursor += 8;

    if cursor + data_size > raw_bytes.len() { process::exit(1); }
    let data_slice = &raw_bytes[cursor..cursor+data_size];
    cursor += data_size;

    // Read line table
    let mut line_table = Vec::new();
    if cursor + 8 <= raw_bytes.len() {
        let line_count = u64::from_le_bytes(raw_bytes[cursor..cursor+8].try_into().unwrap()) as usize;
        cursor += 8;
        
        for _ in 0..line_count {
            if cursor + 8 > raw_bytes.len() { break; }
            let line = u64::from_le_bytes(raw_bytes[cursor..cursor+8].try_into().unwrap()) as usize;
            line_table.push(line);
            cursor += 8;
        }
    }

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

    let mut program = Program::with_data(input_path, instructions, data_slice.to_vec());
    program.line_table = line_table;
    let mut vm = VM::new();
    
    if let Err(e) = vm.run(&program) {
        match e {
            VmError::Halted => {}, 
            _ => eprintln!("Runtime Error: {}", e),
        }
    }
}

fn disassemble_binary(input_path: &str) {
    let raw_bytes = fs::read(input_path).unwrap_or_else(|e| {
        eprintln!("Error reading binary '{}': {}", input_path, e);
        process::exit(1);
    });

    if raw_bytes.len() < 6 {
        eprintln!("Binary too short (missing header)");
        process::exit(1);
    }

    if &raw_bytes[0..4] != b"ALYA" {
        eprintln!("Invalid binary format");
        process::exit(1);
    }

    let mut cursor = 6;

    // Read code size
    if cursor + 8 > raw_bytes.len() { process::exit(1); }
    let code_size = u64::from_le_bytes(raw_bytes[cursor..cursor+8].try_into().unwrap()) as usize;
    cursor += 8;

    let code_slice = &raw_bytes[cursor..cursor+code_size];
    cursor += code_size;

    // Skip data
    if cursor + 8 > raw_bytes.len() { process::exit(1); }
    let data_size = u64::from_le_bytes(raw_bytes[cursor..cursor+8].try_into().unwrap()) as usize;
    cursor += 8 + data_size;

    // Read line table
    let mut line_table = Vec::new();
    if cursor + 8 <= raw_bytes.len() {
        let line_count = u64::from_le_bytes(raw_bytes[cursor..cursor+8].try_into().unwrap()) as usize;
        cursor += 8;
        for _ in 0..line_count {
            if cursor + 8 > raw_bytes.len() { break; }
            let line = u64::from_le_bytes(raw_bytes[cursor..cursor+8].try_into().unwrap()) as usize;
            line_table.push(line);
            cursor += 8;
        }
    }
    
    println!("; Disassembly of '{}'", input_path);
    println!("; Code size: {} bytes", code_size);
    println!("");

    let mut pc = 0;
    let mut instr_idx = 0;
    while pc < code_slice.len() {
        match Instruction::decode(&code_slice[pc..]) {
            Ok((instr, len)) => {
                let line_info = if let Some(&line) = line_table.get(instr_idx) {
                    format!("; line {}", line)
                } else {
                    "".to_string()
                };
                println!("{:04x}:  {:<30} {}", instr_idx, instr.to_assembly(), line_info);
                pc += len;
                instr_idx += 1;
            }
            Err(e) => {
                eprintln!("Corrupt binary at offset {}: {}", pc, e);
                process::exit(1);
            }
        }
    }
}

fn run_debugger(input_path: &str) {
    let raw_bytes = fs::read(input_path).unwrap_or_else(|e| {
        eprintln!("Error reading binary '{}': {}", input_path, e);
        process::exit(1);
    });

    if raw_bytes.len() < 6 {
        eprintln!("Binary too short");
        process::exit(1);
    }

    // New format parsing
    if &raw_bytes[0..4] != b"ALYA" {
        eprintln!("Invalid binary format");
        process::exit(1);
    }

    let mut cursor = 6;

    // Code
    if cursor + 8 > raw_bytes.len() { process::exit(1); }
    let code_size = u64::from_le_bytes(raw_bytes[cursor..cursor+8].try_into().unwrap()) as usize;
    cursor += 8;

    if cursor + code_size > raw_bytes.len() { process::exit(1); }
    let code_slice = &raw_bytes[cursor..cursor+code_size];
    cursor += code_size;

    // Data
    if cursor + 8 > raw_bytes.len() { process::exit(1); }
    let data_size = u64::from_le_bytes(raw_bytes[cursor..cursor+8].try_into().unwrap()) as usize;
    cursor += 8;

    if cursor + data_size > raw_bytes.len() { process::exit(1); }
    let data_slice = &raw_bytes[cursor..cursor+data_size];
    cursor += data_size;

    // Line Table
    let mut line_table = Vec::new();
    if cursor + 8 <= raw_bytes.len() {
        let line_count = u64::from_le_bytes(raw_bytes[cursor..cursor+8].try_into().unwrap()) as usize;
        cursor += 8;
        for _ in 0..line_count {
            if cursor + 8 > raw_bytes.len() { break; }
            let line = u64::from_le_bytes(raw_bytes[cursor..cursor+8].try_into().unwrap()) as usize;
            line_table.push(line);
            cursor += 8;
        }
    }

    let mut instructions = Vec::new();
    let mut pc = 0;
    while pc < code_slice.len() {
        match Instruction::decode(&code_slice[pc..]) {
            Ok((instr, len)) => {
                instructions.push(instr);
                pc += len;
            }
            Err(e) => {
                eprintln!("Corrupt binary: {}", e);
                process::exit(1);
            }
        }
    }

    let mut program = Program::with_data(input_path, instructions, data_slice.to_vec());
    program.line_table = line_table;
    
    let vm = VM::new();
    let mut dbg = Debugger::new(vm);
    
    if let Err(e) = dbg.run(&program) {
        eprintln!("Debugger Error: {}", e);
    }
}
