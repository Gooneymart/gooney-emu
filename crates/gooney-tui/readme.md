<div align="center">

# Gooney FPGA DV Utility

### `gooney-fpga`

**Standalone Design Verification Utility for the Gooney RISC-V Co-Simulation Ecosystem**

[![Rust](https://img.shields.io/badge/built%20with-Rust-orange.svg?logo=rust)](https://www.rust-lang.org/)
[![Architecture](https://img.shields.io/badge/ISA-RV64I-blue.svg)](https://riscv.org/technical/specifications/)
[![Verification](https://img.shields.io/badge/focus-FPGA%20DV-purple.svg)](#)
[![License](https://img.shields.io/badge/license-MPL--2.0-green.svg)](LICENSE)

</div>

---

## Overview

`gooney-fpga` is a standalone command-line **Design Verification (DV) utility** for the **Gooney RISC-V co-simulation ecosystem**.

It bridges the gap between software reference models and hardware RTL verification by providing automated regression testing, execution trace generation, and reset/initialization stress testing for **Veryl/SystemVerilog-based RISC-V implementations**.

The utility is designed to validate RTL behavior **before FPGA synthesis and hardware deployment**, while remaining independent from the interactive Gooney TUI.

### Verification Flow

```text
                 ┌─────────────────────┐
                 │   RISC-V Firmware   │
                 │       / Tests       │
                 └──────────┬──────────┘
                            │
                            ▼
                 ┌─────────────────────┐
                 │    gooney-fpga      │
                 │   DV Verification   │
                 └──────────┬──────────┘
                            │
              ┌─────────────┼─────────────┐
              ▼             ▼             ▼
       ┌────────────┐ ┌────────────┐ ┌────────────┐
       │ Regression │ │   Trace    │ │   Reset    │
       │   Tests    │ │ Generation │ │   Stress   │
       └─────┬──────┘ └─────┬──────┘ └─────┬──────┘
             │              │              │
             └──────────────┼──────────────┘
                            ▼
                 ┌─────────────────────┐
                 │   RTL / FPGA Core   │
                 │ Veryl / SystemVerilog│
                 └─────────────────────┘
```

---

## Key Features

### Automated Regression Suite

Scans a test directory for compiled RISC-V test binaries and executes them against the verification oracle.

Useful for:

- ISA compliance testing
- Arithmetic and logic verification
- Control-flow verification
- Pipeline hazard testing
- Regression testing after RTL changes

---

### Cycle-by-Cycle Execution Traces

Generates structured execution traces containing information such as:

- Cycle number
- Program Counter (`PC`)
- Instruction encoding
- Register write-backs
- Committed data values

Trace output is written to CSV, making it suitable for:

- Software/hardware comparison
- Waveform correlation
- Pipeline debugging
- Regression analysis
- Automated trace diffing

---

### Reset & Initialization Stress Testing

Exercises reset behavior across repeated reset assertions and deassertions.

The reset tests are intended to validate:

- Boot-vector initialization
- Pipeline register clearing
- Register-file initialization
- Reset sequencing
- Synchronous reset behavior
- Asynchronous reset behavior
- Zero-state integrity
- Unexpected state/X-propagation detection

The expected Gooney boot vector is:

```text
0x80000000
```

---

### CI/CD Ready

`gooney-fpga` is intentionally implemented as an independent command-line utility.

It does not require the interactive Gooney TUI, making it suitable for:

- GitHub Actions
- CI regression pipelines
- Pre-commit verification
- RTL development workflows
- Automated FPGA verification environments

---

## Project Relationship

`gooney-fpga` is part of the broader **Gooney RISC-V verification ecosystem**.

```text
┌─────────────────────────────────────────────┐
│              Gooney Ecosystem               │
├─────────────────────────────────────────────┤
│                                             │
│  gooney-emu                                 │
│  └── Software reference / golden model      │
│                                             │
│  gooney-tui                                 │
│  └── Interactive emulator interface         │
│                                             │
│  gooney-fpga                                │
│  └── Hardware DV / regression utility       │
│                                             │
│  FPGA / RTL                                 │
│  ├── Veryl                                   │
│  └── SystemVerilog                           │
│                                             │
└─────────────────────────────────────────────┘
```

The intended workflow is to use the Gooney software model as a reference while validating the corresponding RTL implementation.

---

# Installation & Build

`gooney-fpga` is maintained inside the `gooney-emu` Cargo workspace.

From the repository root:

### Check Compilation

```bash
cargo check --bin gooney-fpga
```

### Build Release Binary

```bash
cargo build --release --bin gooney-fpga
```

The optimized executable will be generated under:

```text
target/release/gooney-fpga
```

---

# Command Reference

The utility is organized around three primary verification workflows:

| Command | Purpose |
|---|---|
| `regression` | Execute automated RISC-V regression tests |
| `trace` | Generate cycle-by-cycle execution traces |
| `reset-test` | Stress reset and initialization behavior |

---

## 1. Regression Testing

The `regression` command scans a directory for compiled RISC-V test binaries and executes them against the verification oracle.

### Usage

```bash
cargo run --bin gooney-fpga -- regression --dir tests/compliance
```

### Options

| Option | Description | Default |
|---|---|---|
| `-d, --dir <PATH>` | Directory containing test binaries | `tests/compliance` |

### Example

```bash
cargo run --bin gooney-fpga -- \
    regression \
    --dir tests/compliance
```

A typical regression directory may contain:

```text
tests/
└── compliance/
    ├── arith_test.bin
    ├── branch_test.bin
    ├── hazard_test.bin
    └── load_store_test.bin
```

The regression command is intended to provide a repeatable pass/fail verification workflow suitable for both local development and CI environments.

---

# 2. Execution Trace Generation

The `trace` command executes a firmware image while recording its execution behavior.

### Usage

```bash
cargo run --bin gooney-fpga -- trace \
    --firmware tests/firmware.bin
```

### Options

| Option | Description |
|---|---|
| `-f, --firmware <PATH>` | **Required.** Path to the firmware binary |

### Output

The command generates:

```text
trace_output.csv
```

The trace contains structured execution information such as:

```text
cycle,pc,instruction,register_write,commit_value
```

Example:

```text
0,0x80000000,0x00000013,x0,0x00000000
1,0x80000004,0x00100093,x1,0x00000001
2,0x80000008,0x00208113,x2,0x00000003
```

The generated trace can be compared against RTL simulation output to identify the first point at which software and hardware execution diverge.

---

# 3. Reset & Initialization Stress Testing

The `reset-test` command repeatedly exercises reset behavior to detect initialization and state-retention problems.

### Usage

```bash
cargo run --bin gooney-fpga -- reset-test --cycles 200
```

### Options

| Option | Description | Default |
|---|---|---|
| `-c, --cycles <NUMBER>` | Number of reset injection cycles | `100` |

### Example

```bash
cargo run --bin gooney-fpga -- \
    reset-test \
    --cycles 200
```

The reset test is intended to exercise both asynchronous and synchronous reset scenarios and verify that the processor consistently returns to a known initial state.

The expected boot Program Counter is:

```text
0x80000000
```

---

# DV Coverage

The utility currently targets the following verification areas:

| DV Area | Command | Validation Target |
|---|---|---|
| **RISC-V ISA Compliance** | `regression` | RV64I instruction execution |
| **Arithmetic & Logic** | `regression` | ALU and datapath behavior |
| **Control Flow** | `regression` | Branch and jump behavior |
| **Pipeline Hazards** | `regression` / `trace` | Hazard detection and forwarding |
| **Execution Ordering** | `trace` | Cycle-by-cycle architectural behavior |
| **Register Write-back** | `trace` | Destination register and commit values |
| **Reset Behavior** | `reset-test` | Reset sequencing and state clearing |
| **Boot Behavior** | `reset-test` | PC initialization to `0x80000000` |
| **State Integrity** | `reset-test` | Zero-state and initialization checks |
| **CI Regression** | All | Repeatable command-line verification |

---

# Recommended Verification Workflow

A typical RTL development cycle can use `gooney-fpga` as follows:

```text
       RTL Change
           │
           ▼
   ┌─────────────────┐
   │ cargo check     │
   │ gooney-fpga     │
   └────────┬────────┘
            │
            ▼
   ┌─────────────────┐
   │ Regression Test │
   └────────┬────────┘
            │
       ┌────┴────┐
       │         │
      PASS      FAIL
       │         │
       ▼         ▼
 ┌──────────┐  Debug RTL
 │ Trace    │       │
 │ Compare  │◄──────┘
 └────┬─────┘
      │
      ▼
 ┌──────────────┐
 │ Reset Stress │
 └──────┬───────┘
        │
        ▼
   RTL DV Ready
        │
        ▼
   Vivado / FPGA
```

### Recommended sequence

```bash
# 1. Compile verification utility
cargo check --bin gooney-fpga

# 2. Run ISA / regression tests
cargo run --bin gooney-fpga -- \
    regression \
    --dir tests/compliance

# 3. Generate a reference execution trace
cargo run --bin gooney-fpga -- \
    trace \
    --firmware tests/firmware.bin

# 4. Stress reset behavior
cargo run --bin gooney-fpga -- \
    reset-test \
    --cycles 200
```

---

# CI/CD Integration

Because `gooney-fpga` is a standalone command-line application, it can be integrated directly into automated verification pipelines.

A minimal CI workflow can follow this pattern:

```bash
cargo check --bin gooney-fpga

cargo run --bin gooney-fpga -- \
    regression \
    --dir tests/compliance

cargo run --bin gooney-fpga -- \
    reset-test \
    --cycles 100
```

A non-zero process exit status can be used by the CI system to mark the verification job as failed.

This makes `gooney-fpga` suitable for regression gates before:

- RTL merges
- FPGA synthesis
- Bitstream generation
- Hardware deployment

---

# Directory Layout

A typical workspace layout may look like:

```text
gooney-emu/
├── crates/
│   ├── gooney-core/
│   └── gooney-tui/
│
├── tests/
│   ├── compliance/
│   │   ├── *.bin
│   │   └── ...
│   └── firmware.bin
│
├── gooney-fpga/
│   └── ...
│
├── Cargo.toml
└── LICENSE
```

The exact workspace structure may vary as the Gooney verification infrastructure evolves.

---

# Design Goals

`gooney-fpga` is designed around several principles:

### Deterministic Verification

Tests should be repeatable and suitable for automated regression environments.

### Software as a Reference

The Gooney software model provides an architectural reference against which RTL behavior can be compared.

### Hardware-Oriented Diagnostics

Execution traces and reset testing expose failures that may not be obvious from final program output alone.

### Toolchain Independence

The verification utility should remain usable without requiring the interactive Gooney TUI.

### Pre-Synthesis Confidence

The goal is to identify architectural, pipeline, reset, and initialization problems before they reach FPGA synthesis and hardware bring-up.

---

# Future Verification Targets

Potential future extensions include:

- [ ] Automated RTL-vs-EMU trace comparison
- [ ] Waveform (`VCD` / `FST`) correlation
- [ ] Differential testing
- [ ] Randomized instruction streams
- [ ] Constrained-random pipeline testing
- [ ] Memory-system verification
- [ ] Interrupt verification
- [ ] CSR verification
- [ ] Custom RISC-V extension verification
- [ ] Coverage reporting
- [ ] JUnit-compatible CI reports
- [ ] Vivado simulation integration
- [ ] Verilator simulation integration

---

# License

This project is licensed under the **Mozilla Public License 2.0 (MPL-2.0)**.

See [`LICENSE`](LICENSE) for the complete license text.

---

<div align="center">

**Gooney RISC-V Verification Infrastructure**

*Software reference models → RTL simulation → FPGA Design Verification*

</div>
