mod ui;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rspin")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Balance,
    Cheat {
        amount: i32,
    },
    Play {
        bet: i32,
        lines: i32,
        #[arg(default_value_t = 1)]
        count: i32,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut ui = ui::TerminalUI::new();

    match &cli.command {
        Commands::Balance => {
            println!("Your balance is: 1000"); // Link to state.rs later
        }
        Commands::Cheat { amount } => {
            println!("Added {} credits.", amount);
        }
        Commands::Play { bet, lines, count } => {
            println!("Playing with bet={}, lines={}, count={}", bet, lines, count);
            ui.start()?;
            ui.run_spin_animation(*count)?;
            ui.finish()?;
            println!(" Final Result: You won {} credits!", 5);
        }
    }

    Ok(())
}
