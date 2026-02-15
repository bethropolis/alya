# Alya VM

A 24-bit stack-based virtual machine with a custom assembler and runtime.

## Quick Start

### Build & Run

```bash
# Build the project
cargo build

# Assemble and run a program
cargo run -- assemble examples/hello.alya examples/hello.bin && cargo run -- run examples/hello.bin
```

### Assemble Only

```bash
cargo run -- assemble examples/hello.alya examples/hello.bin
```

### Run Only

```bash
cargo run -- run examples/hello.bin
```

## 🛠️ Development

### Project Structure

```
src/
├── assembler/      # Assembler (lexer, parser, codegen)
├── core/           # Core VM components (CPU, memory, registers)
├── execution/      # Instruction handlers and execution engine
├── instruction/    # Instruction encoding and decoding
└── main.rs         # CLI entry point
```


## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
