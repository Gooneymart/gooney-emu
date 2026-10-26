# Gooney Emu — Hardware Verification Guide

### `gooney-fpga` + Cocotb + Verilator

This guide documents the **pre-synthesis Design Verification (DV)** workflow for the **Gooney Emu RISC-V co-simulation ecosystem**.

The verification environment combines the Rust-based `gooney-fpga` CLI with **Cocotb** and **Verilator** to provide Python-driven **Constrained-Random Verification (CRV)** for the Gooney hardware implementation.

The objective is to identify RTL, pipeline, reset, and architectural issues **before FPGA synthesis and hardware bring-up**.

---

## Verification Architecture

The Gooney hardware verification environment is divided into three primary layers:

```text
┌──────────────────────────────────────────────────────────────┐
│                     Gooney DV Environment                    │
└──────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌──────────────────────────────────────────────────────────────┐
│                  gooney-fpga Rust CLI                        │
│                                                              │
│  • Verification orchestration                               │
│  • Argument routing                                           │
│  • Testbench scaffolding                                      │
│  • Environment/path configuration                             │
│  • Verilator/Cocotb invocation                               │
└──────────────────────────────┬───────────────────────────────┘
                               │
                               ▼
┌──────────────────────────────────────────────────────────────┐
│                    Cocotb Testbench                          │
│                         Python                               │
│                                                              │
│  • Clock/reset control                                       │
│  • Signal drivers                                            │
│  • Random stimulus generation                                │
│  • Constraint-based testing                                  │
│  • Verification assertions                                   │
└──────────────────────────────┬───────────────────────────────┘
                               │
                               ▼
┌──────────────────────────────────────────────────────────────┐
│                       Verilator                              │
│                                                              │
│  SystemVerilog / Veryl RTL                                   │
│          ↓                                                   │
│  Cycle-accurate C++ simulation model                         │
└──────────────────────────────┬───────────────────────────────┘
                               │
                               ▼
                    ┌────────────────────┐
                    │   Gooney RTL Core  │
                    │    / `core_top`    │
                    └────────────────────┘
```

### Verification Layers

| Layer | Technology | Responsibility |
|---|---|---|
| Orchestration | Rust / `gooney-fpga` | Runs and manages verification |
| Testbench | Python / Cocotb | Drives signals and generates stimuli |
| Simulator | Verilator | Cycle-accurate RTL simulation |
| DUT | Veryl / SystemVerilog | Hardware implementation under test |

---

# Prerequisites

Ensure the following tools are installed and available in your `PATH`.

## Required Toolchain

| Component | Requirement |
|---|---|
| Rust | Latest stable |
| Cargo | Included with Rust |
| Python | 3.10+ |
| Cocotb | 2.0+ |
| Verilator | 5.0+ |
| Make | Required for direct Makefile execution |

Verify the installation:

```bash
rustc --version
cargo --version
python3 --version
verilator --version
make --version
```

---

## Installing Cocotb

Install Cocotb using Python's package manager:

```bash
python3 -m pip install cocotb
```

For isolated development environments, using a virtual environment is recommended:

```bash
python3 -m venv .venv
source .venv/bin/activate

python3 -m pip install --upgrade pip
python3 -m pip install cocotb
```

Verify the installation:

```bash
python3 -c "import cocotb; print(cocotb.__version__)"
```

---

# Running Constrained-Random Verification

The recommended way to run the CRV regression suite is through the standalone `gooney-fpga` CLI.

From the root of the Gooney Emu repository:

```bash
cargo run --bin gooney-fpga -- crv
```

The CLI acts as the verification entry point and manages the underlying simulation environment.

---

## What Happens Behind the Scenes

When the `crv` command is executed, the verification workflow performs the following operations:

### 1. RTL Validation

The utility verifies that the expected RTL source is available:

```text
hdl/core_top.sv
```

If required, the verification environment can scaffold the expected test structure.

### 2. Verification Workspace Setup

The CRV environment is located under:

```text
crates/gooney-fpga/verification/
```

The expected structure is:

```text
crates/
└── gooney-fpga/
    └── verification/
        ├── Makefile
        └── test_crv.py
```

### 3. Simulation Path Binding

The verification environment configures the required RTL and simulation paths for the active workspace.

This allows the same verification infrastructure to be invoked without manually editing absolute paths.

### 4. Verilator Compilation

Verilator compiles the RTL into a high-performance simulation model.

```text
SystemVerilog / Veryl
        │
        ▼
    Verilator
        │
        ▼
 C++ Simulation Model
```

### 5. Cocotb Execution

The generated simulation model is driven by the Cocotb testbench:

```text
test_crv.py
     │
     ▼
Cocotb
     │
     ▼
Verilator
     │
     ▼
Gooney RTL
```

---

# Running the CRV Regression

The complete CRV regression can therefore be launched with:

```bash
cargo run --bin gooney-fpga -- crv
```

A successful run should produce Cocotb/Verilator test output indicating whether the randomized verification campaign passed or failed.

---

# Running the Testbench Manually

When developing or debugging the testbench, it can be useful to bypass the Rust CLI and execute the verification environment directly.

Navigate to:

```bash
cd crates/gooney-fpga/verification
```

Then run:

```bash
make
```

This invokes the verification Makefile directly.

---

## Cleaning Simulation Artifacts

To remove generated simulation binaries, logs, and intermediate artifacts:

```bash
make clean
```

This is useful when changing RTL, simulator configuration, or testbench parameters and you want to force a clean rebuild.

---

# CRV Testbench Structure

The primary Cocotb testbench is:

```text
crates/gooney-fpga/verification/test_crv.py
```

The testbench contains the core randomized verification driver:

```python
RISCVConstraintRandomDriver
```

The driver is responsible for generating and applying randomized stimuli to the RTL design under test.

---

## Stimulus Distribution

The current constrained-random strategy divides stimulus into three major categories:

| Stimulus Category | Weight | Purpose |
|---|---:|---|
| Instruction Randomization | 70% | Exercise instruction fields and datapath behavior |
| Pipeline Hazards | 20% | Stress hazard detection, stalls, and forwarding |
| Reset Glitching | 10% | Validate reset recovery and initialization |

### Instruction Randomization — 70%

Randomized instruction fields and opcode variations are generated to exercise datapath behavior across a broad range of input combinations.

This is intended to expose issues that deterministic directed tests may not encounter.

---

### Pipeline Hazards — 20%

The driver generates back-to-back instruction sequences designed to exercise potential data and control hazards.

Examples include:

```text
Instruction N
     │
     ├── writes register X
     │
     ▼
Instruction N+1
     │
     └── immediately consumes register X
```

This allows the verification environment to exercise:

- Data dependencies
- Pipeline stalls
- Forwarding paths
- Hazard detection
- Back-to-back execution

---

### Reset Glitching — 10%

The testbench periodically asserts and deasserts `rst_n` while the simulation is running.

This is intended to validate:

- Reset recovery
- Pipeline state clearing
- Register initialization
- Boot-vector behavior
- Unexpected state retention
- X-propagation behavior

The expected Gooney boot vector is:

```text
0x80000000
```

---

# Example Cocotb Test

A simplified CRV test follows this structure:

```python
@cocotb.test()
async def rv64i_crv_regression_test(dut):
    clock = Clock(dut.clk, 10, unit="ns")
    cocotb.start_soon(clock.start())

    driver = RISCVConstraintRandomDriver(dut)

    await driver.reset_dut()
    await driver.generate_random_stimulus(cycles=500)
```

The sequence is:

```text
Start Test
    │
    ▼
Start Clock
    │
    ▼
Create CRV Driver
    │
    ▼
Reset DUT
    │
    ▼
Generate Random Stimulus
    │
    ▼
Run Simulation
    │
    ▼
Validate DUT
    │
    ▼
PASS / FAIL
```

---

# Configuring the Simulation

When modifying the verification environment, the following components are the primary areas of interest:

```text
crates/gooney-fpga/
└── verification/
    ├── Makefile
    └── test_crv.py
```

### `Makefile`

Responsible for:

- Verilator configuration
- RTL source selection
- Cocotb invocation
- Simulation build targets
- Cleanup targets

### `test_crv.py`

Responsible for:

- Clock generation
- Reset handling
- Signal driving
- Random stimulus generation
- Verification checks
- Test reporting

---

# Interpreting Test Output

A successful Cocotb regression produces a summary similar to:

```text
********************************************************************************************
** TEST                                                STATUS   SIM TIME (ns)  REAL TIME (s) **
********************************************************************************************
** test_crv.rv64i_crv_regression_test                  PASS         5050.00           0.02 **
********************************************************************************************
** TESTS=1 PASS=1 FAIL=0 SKIP=0                             5050.00           0.02 **
********************************************************************************************
```

---

## Result Fields

### `STATUS`

Indicates whether the test completed successfully.

```text
PASS
```

means the verification checks completed successfully.

```text
FAIL
```

indicates that one or more verification conditions failed.

---

### `SIM TIME`

Represents the simulated hardware time.

For example:

```text
5050.00 ns
```

indicates that the simulation advanced through approximately 5.05 µs of simulated time.

---

### `REAL TIME`

Represents the actual wall-clock time required to execute the simulation.

For example:

```text
0.02 s
```

indicates that the simulation completed in approximately 20 milliseconds of real time.

The relationship between simulation time and wall-clock execution time provides a useful indication of simulator throughput.

---

# Debugging Failed Tests

When a CRV test fails, the recommended debugging sequence is:

```text
CRV Failure
     │
     ▼
Re-run Simulation
     │
     ▼
Inspect Cocotb Output
     │
     ▼
Identify First Failure
     │
     ▼
Inspect RTL Signal
     │
     ▼
Generate / Inspect Waveform
     │
     ▼
Compare Against Gooney Reference
     │
     ▼
Fix RTL / Testbench
     │
     ▼
Run CRV Again
```

When debugging a failure, focus on the **first divergence** rather than subsequent cascading failures.

Pipeline problems can propagate through multiple cycles, so the earliest incorrect state is generally the most useful diagnostic point.

---

# Recommended Verification Workflow

A typical development cycle should use directed tests and constrained-random verification together.

```text
              RTL Change
                  │
                  ▼
        ┌──────────────────┐
        │ Compile / Lint   │
        └────────┬─────────┘
                 │
                 ▼
        ┌──────────────────┐
        │ Directed Tests   │
        └────────┬─────────┘
                 │
                 ▼
        ┌──────────────────┐
        │ CRV Regression    │
        │ Cocotb + Verilator│
        └────────┬─────────┘
                 │
            ┌────┴────┐
            │         │
          PASS       FAIL
            │         │
            │         ▼
            │     Debug RTL
            │         │
            │         └──────┐
            │                │
            ▼                ▼
       Reset Stress     Waveform Analysis
            │                │
            └───────┬────────┘
                    ▼
              Re-run Tests
                    │
                    ▼
             FPGA Synthesis
```

---

# Verification Checklist

Before considering an RTL revision ready for synthesis, verify:

- [ ] RTL compiles successfully
- [ ] Directed ISA tests pass
- [ ] CRV regression passes
- [ ] Pipeline hazard scenarios pass
- [ ] Reset assertions are handled correctly
- [ ] Reset recovery returns to the expected boot state
- [ ] Boot PC initializes to `0x80000000`
- [ ] No unexpected X-propagation is observed
- [ ] Register write-back behavior is correct
- [ ] Simulation traces are consistent with the reference model
- [ ] Verification completes successfully in CI
- [ ] RTL is ready for synthesis

---

# CI/CD Integration

The CRV workflow is designed to run without an interactive TUI, making it suitable for automated CI environments.

A basic CI verification sequence can be:

```bash
# Validate the verification utility
cargo check --bin gooney-fpga

# Execute constrained-random verification
cargo run --bin gooney-fpga -- crv
```

For a direct simulation workflow:

```bash
cd crates/gooney-fpga/verification

make
```

A failed Cocotb test should cause the verification job to fail, preventing an unsuccessful RTL revision from progressing to later stages of the hardware pipeline.

---

# Directory Reference

The relevant verification components are organized as follows:

```text
gooney-emu/
│
├── hdl/
│   └── core_top.sv
│
├── crates/
│   └── gooney-fpga/
│       └── verification/
│           ├── Makefile
│           └── test_crv.py
│
├── Cargo.toml
└── LICENSE
```

The exact repository structure may evolve as the Gooney hardware verification infrastructure expands.

---

# Design Goals

The Gooney hardware DV environment is built around several principles.

### Reproducible Verification

Verification should be executable through deterministic commands and repeatable regression workflows.

### Randomized Coverage

Constrained-random stimulus complements directed tests by exploring input combinations that may not be explicitly covered by hand-written scenarios.

### Cycle-Accurate Validation

Verilator provides a high-performance simulation environment while preserving cycle-level RTL behavior.

### Software/Hardware Correlation

The Gooney software model can serve as an architectural reference when investigating RTL behavior.

### Pre-Synthesis Confidence

The primary goal is to identify functional, pipeline, reset, and initialization problems before FPGA synthesis and hardware deployment.

---

# Future Verification Extensions

Potential future improvements include:

- [ ] Automated RTL-vs-Gooney trace comparison
- [ ] Waveform generation and automated correlation
- [ ] VCD/FST waveform analysis
- [ ] Differential testing against the Gooney emulator
- [ ] Seed-controlled CRV reproduction
- [ ] Randomized instruction streams
- [ ] Expanded pipeline hazard scenarios
- [ ] Memory-system verification
- [ ] CSR verification
- [ ] Interrupt verification
- [ ] Custom RISC-V extension verification
- [ ] Functional coverage reporting
- [ ] Regression dashboards
- [ ] JUnit-compatible CI reports
- [ ] Verilator coverage integration
- [ ] Vivado simulator integration
- [ ] FPGA-in-the-loop verification

---

# Troubleshooting

## Verilator Not Found

If the CLI cannot locate Verilator:

```bash
verilator --version
```

Ensure the executable is installed and available through `PATH`.

---

## Cocotb Not Found

Check the Python installation:

```bash
python3 -c "import cocotb; print(cocotb.__version__)"
```

If the import fails:

```bash
python3 -m pip install cocotb
```

---

## Stale Simulation Artifacts

Clean the verification environment and rebuild:

```bash
cd crates/gooney-fpga/verification
make clean
make
```

---

## RTL Changes Not Reflected

When changing RTL or simulator configuration, perform a clean rebuild:

```bash
make clean
make
```

This ensures that stale generated simulation artifacts are removed.

---

# License

This verification guide and the associated `gooney-fpga` verification infrastructure are part of the Gooney Emu project.

The project is licensed under the **Mozilla Public License 2.0 (MPL-2.0)**.

See [`LICENSE`](LICENSE) for the complete license text.

---

<div align="center">

### Gooney RISC-V Hardware Verification

**Rust · Cocotb · Verilator · Veryl/SystemVerilog**

*From software reference models to pre-synthesis RTL verification.*

</div>
