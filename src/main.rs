use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: alya_vm <file.alya>");
        eprintln!("  Runs an Alya VM assembly program.");
        process::exit(1);
    }

    let filename = &args[1];

    // Read the source file
    let source = match fs::read_to_string(filename) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            process::exit(1);
        }
    };

    // Assemble the source code into a program
    let program = match alya_vm::assembler::assemble(&source, filename) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Assembly error: {}", e);
            process::exit(1);
        }
    };

    // Run the program
    let mut vm = alya_vm::execution::VM::new();
    vm.print_immediately = true;

    if let Err(e) = vm.run(&program) {
        match e {
            alya_vm::VmError::Halted => {} // Normal termination
            _ => {
                eprintln!("Runtime error: {}", e);
                process::exit(1);
            }
        }
    }
}
