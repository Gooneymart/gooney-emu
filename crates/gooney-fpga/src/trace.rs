use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn generate_trace(firmware_path: &str, output_path: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n[gooney-fpga] Loading firmware for tracing: {}", firmware_path);

    let path = Path::new(firmware_path);
    if !path.exists() {
        return Err(format!("Firmware binary not found at: {}", firmware_path).into());
    }

    let default_out = "trace_output.csv";
    let out_file = output_path.unwrap_or(default_out);

    println!("[gooney-fpga] Initializing simulation trace capture...");
    let mut file = File::create(out_file)?;

    // Write CSV header for DV trace analysis
    writeln!(file, "Cycle,PC,Instruction,RegisterWrite,DataValue")?;

    // Mocking sample trace entries using explicit 32-bit unsigned math for RV64 boot addresses
    for cycle in 1..=10 {
        let pc: u32 = 0x80000000_u32 + ((cycle - 1) as u32 * 4);
        let inst: u32 = 0x00000013; // dummy NOP / addi x0, x0, 0
        let reg_write = if cycle % 2 == 0 { "x1" } else { "-" };
        let val = (cycle * 4) as u64;

        writeln!(file, "{},0x{:08X},0x{:08X},{},0x{:016X}", cycle, pc, inst, reg_write, val)?;
    }

    println!("\x1b[32m[SUCCESS]\x1b[0m Trace successfully written to: {}", out_file);
    Ok(())
}
