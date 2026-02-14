# Alya Assembly Examples

This directory contains example `.alya` programs demonstrating various features of the Alya VM.

## Basic Examples (Getting Started)

### 01_hello.alya
**Difficulty**: Beginner  
**Concepts**: Load immediate, print, halt  
The simplest possible program - prints a single number.

### 02_arithmetic.alya
**Difficulty**: Beginner  
**Concepts**: Basic arithmetic operations (+, -, *, /, %)  
Demonstrates all basic arithmetic operations with different operands.

### 03_compound_ops.alya
**Difficulty**: Beginner  
**Concepts**: Compound assignment (+=, -=, *=, /=)  
Shows how to use compound assignment operators for cleaner code.

### 04_bitwise.alya
**Difficulty**: Beginner  
**Concepts**: Bitwise operations (&, |, ^, ~, <<, >>)  
Demonstrates bit manipulation operations and binary literals.

### 05_stack.alya
**Difficulty**: Beginner  
**Concepts**: Stack operations (push, pop, peek)  
Shows how to use the stack for temporary storage.

## Control Flow Examples

### 06_simple_loop.alya
**Difficulty**: Beginner  
**Concepts**: Labels, jumps, conditionals  
A simple counting loop from 0 to 4.

### 07_conditionals.alya
**Difficulty**: Intermediate  
**Concepts**: Conditional jumps, branching logic  
Demonstrates if-else logic using conditional jumps.

### 08_functions.alya
**Difficulty**: Intermediate  
**Concepts**: Function calls, return, calling conventions  
Shows how to define and call functions with parameters.

## Advanced Control Flow

### 09_factorial.alya
**Difficulty**: Intermediate  
**Concepts**: Recursion, stack usage  
Recursive implementation of factorial function.

### 10_fibonacci.alya
**Difficulty**: Intermediate  
**Concepts**: Recursion (advanced)  
Recursive Fibonacci - demonstrates complex stack management.

### 11_fibonacci_iterative.alya
**Difficulty**: Intermediate  
**Concepts**: Iteration, state management  
Iterative Fibonacci - more efficient than recursive version.

## Memory and Arrays

### 12_memory.alya
**Difficulty**: Intermediate  
**Concepts**: Load, store, memory addressing  
Basic memory read/write operations.

### 13_arrays.alya
**Difficulty**: Intermediate  
**Concepts**: Array indexing, memory as array  
Using indexed memory access to work with arrays.

### 14_swap.alya
**Difficulty**: Beginner  
**Concepts**: Swap operation, debug printing  
Demonstrates the swap operator for exchanging register values.

## Algorithms

### 15_bubble_sort.alya
**Difficulty**: Advanced  
**Concepts**: Nested loops, array manipulation, algorithms  
Complete bubble sort implementation.

### 16_gcd.alya
**Difficulty**: Intermediate  
**Concepts**: Euclidean algorithm, modulo operation  
Greatest Common Divisor using Euclidean algorithm.

### 17_prime_check.alya
**Difficulty**: Intermediate  
**Concepts**: Prime testing, optimization  
Checks if a number is prime using trial division.

### 18_power.alya
**Difficulty**: Intermediate  
**Concepts**: Exponentiation, loops  
Computes base^exponent using repeated multiplication.

### 19_array_sum.alya
**Difficulty**: Intermediate  
**Concepts**: Array traversal, accumulation  
Sums all elements in an array and computes average.

### 20_max_min.alya
**Difficulty**: Intermediate  
**Concepts**: Array searching, comparison  
Finds maximum and minimum values in an array.

## Learning Path

**Day 1-2: Basics**
1. 01_hello.alya
2. 02_arithmetic.alya
3. 03_compound_ops.alya
4. 05_stack.alya

**Day 3-4: Control Flow**
1. 06_simple_loop.alya
2. 07_conditionals.alya
3. 08_functions.alya

**Day 5-6: Advanced**
1. 09_factorial.alya
2. 11_fibonacci_iterative.alya
3. 13_arrays.alya

**Week 2: Algorithms**
1. 16_gcd.alya
2. 17_prime_check.alya
3. 19_array_sum.alya
4. 15_bubble_sort.alya

## Running Examples

Once the assembler is implemented:

```bash
# Assemble to bytecode
alya assemble 01_hello.alya -o hello.bin

# Run bytecode
alya run hello.bin

# Or do both in one step
alya run 01_hello.alya
```

## Testing Examples

These examples are also useful for testing the VM:

- **Unit tests**: Use 01-05 for basic instruction testing
- **Integration tests**: Use 06-11 for control flow testing
- **Stress tests**: Use 15-20 for complex algorithm testing

## Expected Output

Each file includes comments showing expected output. For example:

```asm
print @r0      ; Should print 42
```

Use these comments to verify your VM implementation is correct.

## Creating Your Own Examples

When creating new examples:

1. Start with a comment block explaining what it does
2. Use meaningful register names via comments
3. Include expected output as comments
4. Test edge cases
5. Follow the assembly syntax conventions

## Assembly Syntax Quick Reference

```asm
; Comments start with semicolon

; Load immediate
@r0 := 42

; Arithmetic
@r2 := @r0 + @r1
@r0 += 5

; Conditionals
if @r0 > @r1 goto label

; Functions
call function_name
return

; Stack
push @r0
@r1 := pop
@r2 := peek

; Memory
store @r0 at @r1
@r2 := load @r1

; Arrays
@array[5] := @r0
@r1 := @array[5]

; Labels
label_name:
    ; code here
```

## Contributing

Feel free to add more examples! Particularly useful would be:

- String manipulation (when implemented)
- More sorting algorithms (quicksort, mergesort)
- Data structures (linked lists, trees)
- Mathematical functions (sqrt, trigonometry)
- Graphics/simulation demos

Happy coding! 🚀
