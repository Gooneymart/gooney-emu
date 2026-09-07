<div align="center">

# Gooney RISC-V Emulator (`gooney-emu`)

[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)
[![Rust](https://img.shields.io/badge/built_with-Rust-orange.svg?logo=rust)](https://www.rust-lang.org/)
[![Architecture](https://img.shields.io/badge/ISA-RV64I-green.svg)](https://riscv.org/)

**A modular, lightweight RV64I RISC-V instruction-set simulator written in Rust.**

*Designed for education, transparency, experimentation, and hands-on instruction-level debugging.*

</div>

---

## 🎯 Overview

**Gooney** is a modular RISC-V 64-bit instruction-set simulator (ISS) written in Rust.

The project targets the **RV64I Base Integer Instruction Set**, with minimal support for the **Zicsr** extension. It is designed to make CPU instruction execution transparent and easy to inspect, while maintaining a clean architecture that can be extended with additional instructions and custom extensions.

Gooney can execute raw RISC-V binary workloads, manually execute individual machine instructions, inspect CPU state, and test instruction implementations through isolated unit tests.

### Design Goals

* 🧩 **Modular** — CPU state, memory, and instruction decoding are separated into dedicated components.
* 🔍 **Transparent** — Instruction execution can be inspected step-by-step.
* 🎓 **Educational** — Designed to make RISC-V concepts easier to understand and experiment with.
* 🛠️ **Extensible** — Decoder functionality is organized into modular instruction groups.
* ⚡ **Lightweight** — Built in Rust with minimal runtime dependencies.
* 🖥️ **Interactive** — Provides a terminal-based shell and TUI interface for experimentation.

---

## 🚀 Features

### RV64I Instruction Support

Gooney currently implements the major components of the RV64I Base Integer ISA, including:

* Integer arithmetic
* Immediate arithmetic
* Logical operations
* Shift operations
* Upper-immediate instructions
* Conditional branches
* `JAL` and `JALR`
* Memory loads and stores
* System instructions
* Basic `Zicsr` counter support
* Trap-related instructions such as `ECALL` and `EBREAK`
* `FENCE`

Memory instructions include:

```text
LB   LH   LW   LD
LBU  LHU  LWU
SB   SH   SW   SD
```

### Interactive Shell

The `gooney shell` interface provides a lightweight environment for manually executing instructions and inspecting CPU state.

Features include:

* Execute raw 32-bit machine instructions
* Inspect general-purpose registers
* Inspect the program counter
* Reset CPU state
* Experiment with instruction execution without creating a binary workload

### Workload Execution

Gooney can execute raw RISC-V binary workloads from the `workloads/` directory.

For example:

```bash
cargo run --bin gooney-tui -- run workloads/imm_logic_test.bin
```

This allows instruction implementations to be tested through the emulator's normal fetch, decode, and execution flow.

### Modular Decoder Architecture

Instruction decoding is separated into logical modules, making the implementation easier to understand and extend.

The decoder currently separates functionality into areas such as:

* Arithmetic instructions
* Control-flow instructions
* Memory instructions
* System instructions

---

## 📂 Project Structure

```text
gooney-emu/
├── Cargo.toml                 # Workspace configuration
├── Cargo.lock
├── LICENSE
├── PKGBUILD                   # Arch Linux packaging script
├── shell.nix                  # Nix development environment
├── CONTRIBUTING.md            # Contribution and fork guidelines
├── README.md                  # Project documentation
│
├── crates/
│   ├── gooney-core/           # Core emulator engine
│   │   └── src/
│   │       ├── cpu.rs         # CPU state and register management
│   │       └── decoder/       # Instruction decoding and execution
│   │           ├── arithmetic/    # Arithmetic and logical instructions
│   │           ├── control.rs     # Branches and jumps
│   │           ├── memory_instr.rs # Loads and stores
│   │           └── system.rs      # System instructions and Zicsr
│   │
│   └── gooney-tui/            # CLI, TUI, workload runner, and shell
│
├── docs/                      # Architecture, specifications, and guides
├── extensions/                # Custom RISC-V extension experiments
└── workloads/                 # Raw binary test workloads and helpers
    ├── *.bin
    └── decode.py
```

---

## 🛠️ Building

### Requirements

You will need:

* Rust toolchain
* Cargo
* A working Rust development environment

Clone the repository:

```bash
git clone https://github.com/soulking/gooney-emu.git
cd gooney-emu
```

Build the entire workspace:

```bash
cargo build --all
```

For an optimized release build:

```bash
cargo build --release
```

---

## 🕹️ Interactive Shell

Gooney provides an interactive shell for manually executing RISC-V instructions.

Start the shell with:

```bash
cargo run --bin gooney-tui -- shell
```

Once inside the shell, the following commands are available:

| Command      | Description                                  |
| ------------ | -------------------------------------------- |
| `regs`       | Display the general-purpose registers and PC |
| `exec <hex>` | Execute a raw 32-bit machine instruction     |
| `reset`      | Reset the CPU state                          |
| `exit`       | Exit the shell                               |

### Example

Execute an `ECALL` instruction:

```text
exec 0x00000073
```

This provides a convenient way to experiment with individual instructions without preparing a complete binary workload.

---

## 🚀 Running Workloads

Instruction-level functionality can be tested using the raw binary workloads located in `workloads/`.

For example:

```bash
cargo run --bin gooney-tui -- run workloads/imm_logic_test.bin
```

Other workloads can be executed using the same command:

```bash
cargo run --bin gooney-tui -- run workloads/arith_32_test.bin
```

```bash
cargo run --bin gooney-tui -- run workloads/shift_imm_test.bin
```

```bash
cargo run --bin gooney-tui -- run workloads/jump_branch_test.bin
```

```bash
cargo run --bin gooney-tui -- run workloads/illegal.bin
```

### Available Workload Categories

| Workload               | Purpose                                  |
| ---------------------- | ---------------------------------------- |
| `arith_32_test.bin`    | 32-bit arithmetic operations             |
| `reg_arith_test.bin`   | Register-based arithmetic                |
| `imm_logic_test.bin`   | Immediate logical operations             |
| `shift_imm_test.bin`   | Immediate shift operations               |
| `jump_branch_test.bin` | Branch and jump instructions             |
| `control_test.bin`     | Control-flow execution                   |
| `upper_test.bin`       | `LUI` and `AUIPC`                        |
| `illegal.bin`          | Invalid/unsupported instruction handling |

The workload approach provides an **end-to-end verification path** through the normal emulator execution flow.

---

## 🧪 Testing & Verification

Gooney uses two complementary testing approaches:

1. **Binary workloads** for instruction and execution-flow verification.
2. **Rust unit tests** for isolated implementation-level verification, particularly memory operations.

### Workload-Based Testing

Most instruction functionality is tested by executing binary workloads through the Gooney TUI:

```bash
cargo run --bin gooney-tui -- run workloads/imm_logic_test.bin
```

This exercises the emulator through its normal execution path rather than calling individual instruction handlers directly.

The workload approach is useful for verifying:

* Instruction decoding
* Instruction execution
* Register updates
* Control-flow behavior
* Program execution
* Invalid instruction handling

### Memory Unit Testing

Memory operations are tested separately using isolated Rust unit tests in:

```text
crates/gooney-core/src/decoder/memory_instr.rs
```

Run the core test suite with:

```bash
cargo test -p gooney-core
```

Or run the complete workspace test suite:

```bash
cargo test
```

These tests specifically verify memory instructions such as:

```text
LB   LH   LW   LD
LBU  LHU  LWU
SB   SH   SW   SD
```

The memory tests use a controlled CPU and memory state and directly exercise the decoder and execution pipeline.

They verify:

* Instruction encoding and decoding
* I-type and S-type immediate extraction
* Effective address calculation
* Memory read/write width
* Register updates
* Sign extension
* Zero extension

For example, `LB` must sign-extend a byte value such as `0xFF`, while `LBU` must zero-extend it.

This provides more precise coverage of memory semantics than relying exclusively on external binary workloads.

For a more detailed explanation of the testing methodology, see:

```text
docs/testing.md
```

---

## 🧱 Architecture

Gooney is organized around a small number of core components.

### `gooney-core`

The core crate contains the emulator implementation:

```text
CPU State
   │
   ├── Registers
   ├── Program Counter
   │
   ▼
Instruction Decoder
   │
   ├── Arithmetic
   ├── Control Flow
   ├── Memory
   └── System / Zicsr
   │
   ▼
Memory
```

The separation between CPU state, memory, and instruction decoding is intended to keep the emulator understandable and make individual instruction groups easier to develop and test.

### `gooney-tui`

The TUI crate provides the user-facing execution environment.

It is responsible for:

* Command-line handling
* Workload execution
* Interactive shell functionality
* Terminal-based interaction

---

## 🔬 Development Philosophy

Gooney is intentionally designed to favor **clarity and inspectability** over hiding CPU behavior behind complex abstractions.

The project is intended to be useful for:

* Learning the RISC-V ISA
* Understanding instruction encoding
* Experimenting with CPU emulation
* Studying instruction decoding
* Developing custom RISC-V extensions
* Building and testing experimental workloads
* Exploring emulator architecture in Rust

The codebase is structured so that individual instruction groups can be implemented, tested, and extended independently.

---

## 🧩 Extensions

The `extensions/` directory is reserved for experimental and custom RISC-V extension work.

This allows experimental functionality to be developed without unnecessarily coupling it to the core RV64I implementation.

---

## 📚 Documentation

Additional project documentation is maintained under:

```text
docs/
```

Current documentation includes:

* Architecture documentation
* Testing and verification guides
* Emulator specifications
* Development notes

---

## 📜 License

Gooney is open-source software licensed under the **Mozilla Public License 2.0 (MPL-2.0)**.

See [`LICENSE`](LICENSE) for the complete license text.
---

## 🤝 Contributing

Gooney is developed primarily as a part-time personal project.

The project currently follows a **fork-and-adapt development model**. Direct pull requests to the main repository are generally not accepted.

Before making changes, please read:

[`CONTRIBUTING.md`](CONTRIBUTING.md)

You are welcome to fork the project and adapt it for your own experiments, research, education, or RISC-V development work.

---

## 💼 Commercial Inquiries

Gooney is developed by **Gooneymart™**.

For corporate licensing, custom development, or support inquiries:

```text
gooneymart@gmail.com
```

---

<div align="center">

**Gooney RISC-V Emulator**

*Learn. Execute. Inspect. Extend.*

</div>

