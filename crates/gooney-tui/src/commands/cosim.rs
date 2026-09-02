// crates/gooney-tui/src/commands/cosim.rs
use crate::bridge::{ActiveSimCore, SimCore};
use std::path::Path;
#[allow(unused_mut)]
pub fn execute(firmware_path: &str, steps: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!("==================================================================");
    println!("🚀 GOONEY-EMU / VERILATOR NATIVE CO-SIMULATION (EXIT & CUSTOM MONITOR)");
    println!("📦 Target Firmware: \"{}\"", firmware_path);
    println!("⏱ Simulation Budget: {} steps", steps);
    println!("==================================================================\n");

    if !Path::new(firmware_path).exists() {
        return Err(format!("Firmware binary not found at '{}'", firmware_path).into());
    }

    let core = ActiveSimCore;
    unsafe {
        core.init(firmware_path);
    }

    let mut commit_count: usize = 0;
    let mut custom_hits: usize = 0;
    let mut divergence_count: usize = 0; // 👈 Track divergences here
    let mut exit_detected = false;
    let mut final_status: u64 = 0;

    println!("🔍 Running co-simulation trace...");

    for step_num in 1..=steps {
        unsafe {
            core.step(false, 0, false, false);

            if core.get_sim_done() {
                exit_detected = true;
                println!("🏁 Simulation finished successfully at step {} (Exit address hit).", step_num);
                break;
            }

            let pc = core.get_pc();
            let inst = core.get_inst();
            let committed = core.get_wb_commit();
            let custom_active = core.get_custom_active();
            let mem_wen = core.get_mem_wen();
            let mem_addr = core.get_mem_addr();
            let mem_wdata = core.get_mem_wdata();
            let wb_rd = core.get_wb_rd();
            let wb_result = core.get_wb_result();

            // ==========================================================
            // 🔍 DIVERGENCE CHECK HOOK
            // ==========================================================
            // TODO: If you have a software reference model or expected state, 
            // compare it against `pc`, `wb_result`, etc., here. 
            // Example structure:
            // let expected_pc = get_reference_pc(step_num);
            // if committed && pc != expected_pc {
            //     divergence_count += 1;
            //     println!("❌ DIVERGENCE DETECTED at step {}: HW PC (0x{:08X}) != REF PC (0x{:08X})", step_num, pc, expected_pc);
            // }
            // ==========================================================

            // Check if register a0 (x10) is being updated
            if committed && wb_rd == 10 {
                final_status = wb_result;
            }

            // Print telemetry for interesting events or early steps
            let opcode = inst & 0x7F;
            let is_custom = custom_active || opcode == 0x0B;

            if step_num <= 10 || committed || is_custom || mem_wen {
                println!(
                    "[{:3}] PC: 0x{:08X} | Inst: 0x{:08X} | Commit: {} | Custom: {} | MemWrite[0x{:08X}] = 0x{:016X}",
                    step_num, pc, inst, committed, is_custom, mem_addr, mem_wdata
                );
            }

            if committed {
                commit_count += 1;

                if is_custom {
                    custom_hits += 1;
                    println!("⚡ >>> CUSTOM-0 EXTENSION TRIGGERED AT STEP {} (PC: 0x{:08X}, Inst: 0x{:08X})! <<<", step_num, pc, inst);
                }
            }

            // Check for exit trigger write
            if mem_wen {
                exit_detected = true;
            }
        }
    }

    unsafe {
        core.destroy();
    }

    println!("\n==================================================================");
    println!("📊 CO-SIMULATION SUMMARY REPORT");
    println!("    • Total Instructions Committed: {}", commit_count);
    println!("    • Custom-0 Extensions Triggered: {} ⚡", custom_hits);
    println!("    • Divergences Detected: {}", divergence_count); // 👈 Displayed in summary
    println!("    • Final Status in a0 (x10): 0x{:X}", final_status);
    println!("    • Exit Triggered via Memory: {}", if exit_detected { "YES ✅" } else { "NO ❌" });
    println!("==================================================================");

    Ok(())
}
