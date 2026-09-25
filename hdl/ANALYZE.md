This document outlines the validation logic performed by `gooney-tui analyze` and provides a checklist to ensure your Veryl HDL core  is ready for co-simulation as an oracle-verified RISC-V RV64I core.

## 1. Automated Checks (via `analyze` command)
When you run `cargo run --bin gooney-tui -- analyze`, the tool automatically verifies:
- [ ] **Workspace Integrity:** Confirms the `hdl/` directory exists.
- [ ] **Asset Detection:** Scans for `.veryl`, `.sv`, or `.v` files to confirm the presence of your RTL source code.
- [ ] **File Count:** Reports total HDL assets for configuration tracking.

## 2. HDL Verification Checklist
Before proceeding to co-simulation, ensure your Veryl implementation meets these requirements:

### Architectural Contract (RV64I)
- [ ] **Memory Operations:** Does the LSU correctly handle byte (`LB/LBU`), half-word (`LH/LHU`), and word (`LW/LWU`) accesses?
- [ ] **Sign-Extension:** Is sign-extension implemented correctly for all sub-word loads?
- [ ] **Hazard Handling:** Is the 1-cycle Load-Use stall logic correctly freezing the PC and stalling the pipeline?

### Pipeline & Data Hazards
- [ ] **Forwarding Paths:** Confirm MEM-to-EX and WB-to-EX forwarding is functioning without register file writeback.
- [ ] **Branch Logic:** Does the core flush speculative stages correctly on branch misprediction?

### Co-Simulation Readiness
- [ ] **Retirement Boundary:** Can your core signal when an instruction commits at the Writeback stage? (This is your key sync point with `gooney-emu`).
- [ ] **State Matching:** Does your HDL writeback destination (`wb_result_out`) match the software model's register commit?

## 3. How to Run the Analysis
To perform the static analysis of your HDL workspace, run:

```bash
cargo run --bin gooney-tui -- analyze
