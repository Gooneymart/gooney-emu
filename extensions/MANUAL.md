# Gooney Emu Extension Manual

This manual explains how to build, connect, test, and reset custom RISC-V extensions using the **Gooney Emu** command-line interface.

Gooney Emu provides four extension slots:

| Slot   | Name       | RISC-V Custom Opcode |
| ------ | ---------- | -------------------: |
| Slot 0 | `custom-0` |               `0x0B` |
| Slot 1 | `custom-1` |               `0x2B` |
| Slot 2 | `custom-2` |               `0x5B` |
| Slot 3 | `custom-3` |               `0x7B` |

The `gooney-tui` CLI manages the extension lifecycle so that developers do not need to manually copy compiled libraries or maintain registration files.

---

# 1. Extension Lifecycle

A custom extension follows this general lifecycle:

```text
┌──────────────┐
│ Create       │
│ Extension    │
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ Implement    │
│ ISA Hooks    │
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ Lint          │
│ Extension    │
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ Build         │
│ --release     │
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ Connect       │
│ Extension     │
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ Run / Test    │
│ Instructions  │
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ Reset         │
│ if required   │
└──────────────┘
```

The recommended workflow is:

```bash
cargo run --bin gooney-tui -- connect <slot_name>
```

followed by:

```bash
cargo run --bin gooney-tui -- run <test_binary>
```

---

# 2. Before Connecting an Extension

Before an extension can be connected, make sure that:

1. The extension exists in one of the supported slots.
2. Its `Cargo.toml` is valid.
3. It references the correct `gooney-core`.
4. Required ISA CPU hooks have been implemented.
5. The extension compiles successfully.
6. The extension follows the expected ABI/API provided by `gooney-core`.

A typical extension layout is:

```text
gooney-emu/
└── extensions/
    ├── custom-0/
    │   └── my_extension/
    │       ├── Cargo.toml
    │       ├── src/
    │       │   └── lib.rs
    │       └── test/
    │           └── test_binary
    │
    ├── custom-1/
    ├── custom-2/
    └── custom-3/
```

---

# 3. Implementing the ISA CPU Hooks

Custom extensions must implement the CPU hooks required by the `gooney-core` ISA compliance API.

These hooks provide the interface between the extension and the emulator CPU.

The exact hook set depends on the version of the `gooney-core` API being used. The extension should therefore use the API definitions provided by the current repository rather than copying an interface from another version.

At minimum, your implementation should ensure that:

* Required hook functions exist.
* Function signatures match the `gooney-core` API.
* Register access follows the CPU API.
* Program-counter behavior is correct.
* Execution results use the types expected by `gooney-core`.
* Unsupported instructions are handled according to the extension API.

The extension should not directly modify internal CPU structures unless the API explicitly permits it.

For instruction-specific implementation details, see:

```text
EXTENSION.md
```

---

# 4. Connecting Your Extension

Once the extension has been implemented, use `gooney-tui` to validate, compile, package, and register it.

## Command

```bash
cargo run --bin gooney-tui -- connect <slot_name>
```

For example:

```bash
cargo run --bin gooney-tui -- connect custom-0
```

---

# 5. Valid Slot Names

The following slot names are supported:

```text
custom-0
custom-1
custom-2
custom-3
```

The slot determines which custom RISC-V opcode is assigned to the extension.

| Slot       | Opcode |
| ---------- | -----: |
| `custom-0` | `0x0B` |
| `custom-1` | `0x2B` |
| `custom-2` | `0x5B` |
| `custom-3` | `0x7B` |

For example:

```bash
cargo run --bin gooney-tui -- connect custom-0
```

connects the extension in the `custom-0` slot.

---

# 6. What `connect` Does

The `connect` command performs the complete extension registration workflow.

Conceptually:

```text
connect custom-0
       │
       ├── 1. Validate
       │
       ├── 2. Build
       │
       ├── 3. Package
       │
       └── 4. Register
```

## 6.1 Lint

The extension is checked before it is built.

The linter validates important extension requirements, including:

* `Cargo.toml`
* Required project structure
* `gooney-core` dependency
* ISA hook presence
* ISA hook signatures
* Extension configuration

If the extension fails validation, the connection process stops.

Fix the reported errors before attempting to connect again.

---

## 6.2 Build

After validation succeeds, the extension is compiled in release mode.

Conceptually, the build step is equivalent to:

```bash
cargo build --release
```

Building in release mode ensures that the plugin used by the emulator is an optimized shared library.

---

## 6.3 Package

After a successful build, Gooney Emu locates the generated shared-library artifact.

The resulting plugin is placed into:

```text
crates/gooney-core/plugins/
```

The plugin is assigned a slot-specific filename:

```text
slot_0.so
slot_1.so
slot_2.so
slot_3.so
```

The mapping is:

```text
custom-0 → slot_0.so
custom-1 → slot_1.so
custom-2 → slot_2.so
custom-3 → slot_3.so
```

For example:

```text
crates/gooney-core/plugins/
├── slot_0.so
├── slot_1.so
├── slot_2.so
└── slot_3.so
```

Only connected extensions need to have a corresponding plugin artifact.

---

## 6.4 Register

Finally, the extension is marked as connected.

A `.connected` marker is created in the extension directory.

For example:

```text
extensions/custom-0/my_extension/
├── Cargo.toml
├── src/
│   └── lib.rs
├── test/
│   └── test_binary
└── .connected
```

The marker allows Gooney Emu to track the connection state of the extension.

---

# 7. Connecting an Extension: Complete Example

Assume the following project:

```text
extensions/
└── custom-0/
    └── my_extension/
        ├── Cargo.toml
        ├── src/
        │   └── lib.rs
        └── test/
            └── add42
```

Connect it with:

```bash
cargo run --bin gooney-tui -- connect custom-0
```

The expected lifecycle is:

```text
custom-0
   │
   ▼
Validate Cargo.toml and ISA hooks
   │
   ▼
Build release library
   │
   ▼
Create slot_0.so
   │
   ▼
Install into:
crates/gooney-core/plugins/
   │
   ▼
Create .connected
```

The extension is now available through the `custom-0` slot.

---

# 8. Testing an Extension

After connecting an extension, use the `run` command to execute a test binary containing your custom instructions.

## Command

```bash
cargo run --bin gooney-tui -- run <test_binary>
```

For an extension located in `custom-0`:

```bash
cargo run --bin gooney-tui -- run extensions/custom-0/<your-project>/test/<test_binary>
```

For example:

```bash
cargo run --bin gooney-tui -- run extensions/custom-0/my_extension/test/add42
```

---

# 9. Test Program Requirements

A test binary should contain instructions that exercise the custom extension.

For example, a test program might:

```text
1. Initialize input registers
2. Execute the custom instruction
3. Store the result
4. Exit
```

A useful test should verify:

* Instruction decoding.
* Source-register values.
* Destination-register values.
* Immediate operands.
* Arithmetic behavior.
* Edge cases.
* Program-counter advancement.
* Interaction with normal RISC-V instructions.

---

# 10. Testing Multiple Instructions

An extension can provide multiple instructions within the same custom opcode space.

For example:

```text
CUSTOM-0 / 0x0B

funct7 = 0x01 → ADD42
funct7 = 0x02 → MUL
funct7 = 0x03 → CRC32
```

A test binary can therefore exercise multiple instructions from the same extension:

```text
ADD42
   ↓
MUL
   ↓
CRC32
   ↓
Normal RISC-V instruction
```

This is useful for verifying that custom instructions interact correctly with the normal CPU execution pipeline.

---

# 11. Verifying the Connected Plugin

After connecting an extension, the generated plugin should exist under:

```text
crates/gooney-core/plugins/
```

For `custom-0`, check:

```text
crates/gooney-core/plugins/slot_0.so
```

For `custom-1`:

```text
crates/gooney-core/plugins/slot_1.so
```

and so on.

The extension directory should also contain:

```text
.connected
```

These two artifacts indicate that the extension has been built and registered.

---

# 12. Resetting the Extension System

To remove all connected extensions and return Gooney Emu to the vanilla ISA configuration, use:

```bash
cargo run --bin gooney-tui -- reset
```

The reset operation is intended to clear the active extension state.

Conceptually:

```text
Connected Extensions
       │
       ▼
     reset
       │
       ▼
Remove extension registration
       │
       ▼
Vanilla ISA
```

After resetting, custom instructions should no longer be available through the connected extension slots.

---

# 13. Reconnecting an Extension

If an extension has been modified after it was connected, reconnect it:

```bash
cargo run --bin gooney-tui -- connect custom-0
```

This ensures that the updated source is:

1. Validated.
2. Recompiled.
3. Packaged.
4. Registered again.

A typical development loop is therefore:

```bash
# Modify extension
$EDITOR extensions/custom-0/my_extension/src/lib.rs

# Reconnect
cargo run --bin gooney-tui -- connect custom-0

# Test
cargo run --bin gooney-tui -- run extensions/custom-0/my_extension/test/add42
```

---

# 14. Troubleshooting

## 14.1 Extension Fails Linting

If `connect` fails during validation, inspect the reported error first.

Common causes include:

```text
Invalid Cargo.toml
Missing gooney-core dependency
Missing ISA hook
Incorrect hook signature
Invalid extension structure
```

Do not attempt to bypass the linter. The validation step exists to prevent incompatible plugins from being installed.

---

## 14.2 Extension Fails to Build

If validation succeeds but compilation fails, run the extension's Cargo build directly:

```bash
cargo build --release
```

This can provide more detailed Rust compiler diagnostics.

Check:

* Rust compiler errors.
* Dependency versions.
* `gooney-core` API changes.
* Incorrect imports.
* Incorrect hook signatures.
* Platform-specific dependencies.

---

## 14.3 Plugin Is Missing

If the extension connects successfully but the expected plugin cannot be found, check:

```text
crates/gooney-core/plugins/
```

and verify the appropriate slot artifact exists:

```text
slot_0.so
slot_1.so
slot_2.so
slot_3.so
```

The slot must correspond to the extension being connected.

---

## 14.4 Test Instruction Does Not Execute

Check the following:

1. The extension is connected.
2. The correct slot was selected.
3. The test binary contains the expected custom opcode.
4. The instruction encoding matches the extension implementation.
5. The CPU hooks are implemented correctly.
6. The generated plugin corresponds to the latest source.
7. The extension has been reconnected after changes.

A common mistake is modifying an extension and immediately running the old plugin without reconnecting it.

---

# 15. Slot Management

Each custom slot is independent.

```text
custom-0 → slot_0.so → 0x0B
custom-1 → slot_1.so → 0x2B
custom-2 → slot_2.so → 0x5B
custom-3 → slot_3.so → 0x7B
```

Avoid assigning two different extensions to the same slot.

If you need to experiment with another implementation, reconnect the desired implementation to the slot.

---

# 16. Recommended Development Workflow

For normal extension development, use:

```bash
# 1. Implement your extension
$EDITOR extensions/custom-0/my_extension/src/lib.rs

# 2. Run tests for the extension
cargo test

# 3. Connect the extension
cargo run --bin gooney-tui -- connect custom-0

# 4. Run an instruction-level test
cargo run --bin gooney-tui -- run \
    extensions/custom-0/my_extension/test/add42

# 5. Modify the extension and repeat
```

When switching back to the standard emulator:

```bash
cargo run --bin gooney-tui -- reset
```

---

# 17. Command Reference

| Command                                          | Purpose                     |
| ------------------------------------------------ | --------------------------- |
| `cargo run --bin gooney-tui -- connect custom-0` | Connect `CUSTOM-0`          |
| `cargo run --bin gooney-tui -- connect custom-1` | Connect `CUSTOM-1`          |
| `cargo run --bin gooney-tui -- connect custom-2` | Connect `CUSTOM-2`          |
| `cargo run --bin gooney-tui -- connect custom-3` | Connect `CUSTOM-3`          |
| `cargo run --bin gooney-tui -- run <binary>`     | Execute a test binary       |
| `cargo run --bin gooney-tui -- reset`            | Remove connected extensions |

---

# 18. Extension State

A connected extension can be thought of as having three states:

```text
                    ┌─────────────┐
                    │   Created   │
                    └──────┬──────┘
                           │
                         connect
                           │
                           ▼
                    ┌─────────────┐
                    │  Connected  │
                    └──────┬──────┘
                           │
                          reset
                           │
                           ▼
                    ┌─────────────┐
                    │    Clean    │
                    │ Vanilla ISA │
                    └─────────────┘
```

When source code changes, reconnect the extension to rebuild and register the updated implementation.

---

# 19. Safety and Compatibility

Custom extensions execute as native code through the plugin system.

Only connect extensions that you trust.

A custom extension can potentially:

* Consume CPU resources.
* Access process resources available to native code.
* Crash the emulator.
* Produce invalid CPU state.
* Cause incorrect execution if its ISA hooks are implemented incorrectly.

For this reason, extensions should be treated as native executable components rather than ordinary configuration files.

When developing third-party extensions, inspect the source code and dependencies before connecting them.

---

# 20. Quick Start

The shortest path from an implemented extension to a running test is:

### 1. Implement the CPU hooks

```text
extensions/custom-0/<your-project>/
└── src/
    └── lib.rs
```

### 2. Connect the extension

```bash
cargo run --bin gooney-tui -- connect custom-0
```

### 3. Run the test binary

```bash
cargo run --bin gooney-tui -- run \
    extensions/custom-0/<your-project>/test/<test_binary>
```

### 4. Reset when finished

```bash
cargo run --bin gooney-tui -- reset
```

---

# 21. Related Documentation

Use the following documentation for the corresponding part of the extension workflow:

```text
EXTENSION.md
```

**`EXTENSION.md`** — Extension development guide. Covers custom opcode slots, instruction encoding, `execute()` implementations, CPU state access, registration, and extension design.

```text
MANUAL.md
```

**`MANUAL.md`** — Extension management guide. Covers the `gooney-tui` workflow for linting, building, connecting, running, and resetting extensions.

Together:

```text
EXTENSION.md
     │
     │  How to BUILD an extension
     ▼
Custom Extension
     │
     │  How to CONNECT and RUN it
     ▼
MANUAL.md
```

---

# 22. Summary

Gooney Emu provides a complete workflow for developing and managing custom RISC-V extensions.

The core commands are:

```bash
# Connect an extension
cargo run --bin gooney-tui -- connect custom-0

# Run an extension test
cargo run --bin gooney-tui -- run \
    extensions/custom-0/<your-project>/test/<test_binary>

# Return to vanilla ISA
cargo run --bin gooney-tui -- reset
```

The `connect` command handles the extension lifecycle:

```text
Lint
  ↓
Build
  ↓
Package
  ↓
Register
```

The resulting plugin is placed into:

```text
crates/gooney-core/plugins/
```

with the appropriate slot mapping:

```text
custom-0 → slot_0.so → 0x0B
custom-1 → slot_1.so → 0x2B
custom-2 → slot_2.so → 0x5B
custom-3 → slot_3.so → 0x7B
```

This keeps extension management centralized while allowing extension developers to focus on implementing and testing their custom ISA behavior.

