# Alya VM Project Roadmap

This document outlines the development journey of the Alya VM, from its core foundation to advanced architectural enhancements and future goals.

## 🏁 Milestone 1: Core Foundation (v0.1) - [COMPLETED]
The goal of this milestone was to build a functional, modular Virtual Machine and an Assembler pipeline.

- [x] **Core CPU Architectue**: Registers (GP & Special), Flags (Z, N, C, V), and Error handling.
- [x] **Memory & Stack**: 64KB unified memory with downward-growing stack.
- [x] **Instruction Set**: ~40 instructions for arithmetic, logic, and control flow.
- [x] **Assembler Pipeline**: Lexer, Parser, and multi-pass Codegen with named variable resolution.
- [x] **Modular Execution**: decoupled instruction handlers and a main VM facade.
- [x] **Basic I/O**: Initial `print` and `input` instructions.

## 🚀 Milestone 2: Stability & Architecture (v0.2) - [COMPLETED]
This milestone focused on correcting critical logic flaws and modernizing the architecture for a real-world toolchain.

- [x] **Advanced Control Flow**: 
    - Full support for Signed vs. Unsigned comparisons using Carry/Zero flags.
    - Infinite Recursion Protection via `MAX_STACK_DEPTH`.
- [x] **Modern I/O System**: Replaced specific opcodes with a generic **Syscall Architecture** (ID-based).
- [x] **CLI Toolchain**: 
    - Separated `assemble` (Build) and `run` (Execute) steps.
    - Implemented binary serialization/deserialization for `.bin` files.
- [x] **String Support**: String literal tokenization, data section embedding, and `PrintString` syscall.
- [x] **Stability & Optimization**:
    - Project-wide standardized Error enums (`MemoryError`, `StackError`).
    - `unsafe` fast-path for 64-bit memory access (performance optimization).

## 🛠️ Milestone 3: Advanced Logic (Planned)
Focusing on code generation efficiency and system robustness.

- [ ] **Register Spilling**: Automatically offload variables to memory when the 16 GP registers are exhausted.
- [ ] **Standard Library**: Create an `std.alya` included by the assembler for common tasks (math, buffer management).
- [ ] **Enhanced Debugger**: A standalone CLI debugger with step-by-step execution and memory inspection.

## 🔭 Future Horizons
Long-term vision for the Alya ecosystem.

- [ ] **JIT Compilation**: Dynamic translation of Alya bytecode to host machine code (x86/ARM).
- [ ] **Plugin System**: Allow native C/Rust extensions to be called via special Syscalls.
- [ ] **Web Inspector**: A WASM-based browser tool to visualize VM state in real-time.
