use clap::{Parser, Subcommand};
mod regression;
mod trace;
mod reset_test;
mod checklist;
mod crv;

#[derive(Parser)]
#[command(name = "gooney-fpga")]
#[command(about = "Pre-silicon DV and hardware validation utility for Gooney Emu", version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run automated regression test suite against golden reference
    Regression {
        /// Path to the directory containing test binaries
        #[arg(short, long, default_value = "tests/compliance")]
        dir: String,
    },
    /// Generate cycle-by-cycle execution trace logs for debugging
    Trace {
        /// Path to the firmware binary
        #[arg(short, long)]
        firmware: String,
    },
    /// Run reset and initialization stress tests
    ResetTest {
        /// Number of random async/sync reset injection cycles
        #[arg(short, long, default_value_t = 100)]
        cycles: usize,
    },
    Checklist,
    Crv,

}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Regression { dir } => {
            if let Err(e) = regression::run_regression(dir) {    
                eprintln!("[Error] Regression suite failed to execute: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Trace { firmware } => {
           if let Err(e) = trace::generate_trace(firmware, None) {
              eprintln!("[Error] Failed to generate trace: {}", e);
              std::process::exit(1);
        }
    }
        Commands::ResetTest { cycles } => {
        if let Err(e) = reset_test::run_reset_test(*cycles) {
        eprintln!("[Error] Reset test failed: {}", e);
        std::process::exit(1);
      }
    }
    Commands::Checklist => {
        if let Err(e) = checklist::print_checklist() {
            eprintln!("[Error] Failed to print checklist: {}", e);
            std::process::exit(1);
        }
    }

    Commands::Crv => {
    if let Err(e) = crv::run_cocotb_crv() {
        eprintln!("[Error] CRV execution failed: {}", e);
        std::process::exit(1);
        }
      }
    }
}
