mod database;
mod machine;
mod ui;

use clap::{Parser, Subcommand};

use crate::{database::Database, machine::Machine};

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
            let total_bet = bet * lines * count;

            // todo, display this somehow?
            let mut database = Database::load()?;
            database.balance -= total_bet;
            database.total_spins += count;

            // simulate machine spins to get results prior to animating them
            let mut machines = Machine::create_n_machines(*count);
            for machine in &mut machines {
                machine.spin();
            }

            // todo: move the total lines calculation here and clear out the right amount of space
            ui.start()?;

            // the sauce is going to primarily be in the animations
            // we need to start it one reel at a time, n ms between, and stop them one at a time
            // with m ms between
            //
            // upon that finishes, we should then display
            ui.run_spin_animation(machines)?;

            // todo: add a step here to display winning lines in a loop
            // we can either let that run for Y ms, or even better would be add a
            // press any button to continue to pause here before moving onto .finish()

            // todo: move the cleanup step of run spin animation here, passing total lines
            ui.finish()?;

            database.save()?;

            println!("You won {} credits!", 5);
        }
    }

    Ok(())
}
