# 🧠 iCPU

An educational CPU simulator written in Rust, created to explore computer architecture, digital logic, and low-level programming through hands-on implementation.

The goal of **iCPU** is to understand how a processor works by building one from the ground up — from registers and arithmetic operations to instruction execution and memory.

## 🎯 Project Goals

- Explore the fundamental concepts of computer architecture.
- Understand how CPUs execute instructions.
- Learn how registers, ALUs, flags, and memory work together.
- Understand the Fetch–Decode–Execute cycle.
- Practice low-level programming and systems development with Rust.
- Build a practical foundation for understanding hardware and processor design.

## 🏗️ Architecture

iCPU is designed around the fundamental components of a processor:

- **CPU** — coordinates instruction execution.
- **Registers** — store temporary values used by instructions.
- **ALU (Arithmetic Logic Unit)** — performs arithmetic and logical operations.
- **Flags** — represent conditions produced by CPU operations.
- **Program Counter (PC)** — tracks the position of instruction execution.
- **Instructions** — represent operations that the CPU can execute.

The project focuses on understanding how these components interact rather than simulating commercial hardware.

## 🦀 Why Rust?

Rust provides an interesting environment for exploring CPU simulation and systems programming.

It offers:

- Low-level control over data representation.
- Strong type safety.
- Explicit memory management concepts.
- Performance without a garbage collector.
- A practical way to explore systems programming concepts.

## 🚀 Getting Started

Make sure you have [Rust](https://www.rust-lang.org/tools/install) and Cargo installed.

```bash
git clone https://github.com/Ali127Dev/icpu.git
cd icpu
cargo run
```
