use crate::cli::Commands;
use crate::commands;

pub fn handle_command(command: &Commands) {
    match command {
        Commands::Shell => commands::shell::execute(),
        Commands::Run { path } => commands::run::execute(path),
        Commands::Lint { extension } => commands::lint::execute(extension),
        Commands::Diff { emu_trace, rtl_trace } => commands::diff::execute(emu_trace, rtl_trace),
        Commands::Test => commands::test_cmd::execute(),
        Commands::Reset => commands::reset::execute(),
        Commands::Disasm { path } => commands::disasm::execute(path),
        Commands::Analyze { path } => {
            if let Err(e) = crate::commands::analyze::execute(&path) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }

        Commands::X86DecodeTest { vector_file } => {
            if let Err(e) = crate::commands::x86_decode_test::run(vector_file.clone()) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
        Commands::Cosim { binary, steps } => {
            if let Err(e) = crate::commands::cosim::execute(binary, *steps) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
        Commands::Fuzz { count } => commands::fuzz::execute(*count),
        Commands::Connect { slot } => {
            commands::connect::execute(slot);
        }
    }
}
