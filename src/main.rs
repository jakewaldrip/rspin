mod animation_state;
mod database;
mod machine;
mod paylines;
mod symbols;
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
        #[arg(default_value_t = 1)]
        count: i32,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut ui = ui::TerminalUI::new();

    // todo, display this somehow?
    let mut database = Database::load()?;

    match cli.command {
        Commands::Balance => {
            println!("Your balance is: {}", database.balance);
        }
        Commands::Cheat { amount } => {
            println!("Added {} credits.", amount);
            database.balance += amount;
            database.save()?;
        }
        Commands::Play { bet, count } => {
            println!("Playing with bet={}, count={}", bet, count);
            let total_bet = bet * count;
            database.balance -= total_bet;
            database.total_spins += count;

            // simulate machine spins to get results prior to animating them
            let mut machines = Machine::create_n_machines(count);
            let mut total_winnings: i32 = 0;
            for machine in &mut machines {
                machine.spin();
                machine.get_all_paylines();
                println!("Machine {} paylines: {:?}", machine.name, machine.paylines);

                for payline in &machine.paylines {
                    total_winnings += payline.get_payout(bet);
                }
            }

            let lines_per_machine = 7; // Header + 3 rows + Spacer
            let total_lines = (count * lines_per_machine) + 1;

            // println!("Machines: {machines:?}");

            // begin animations
            ui.start(total_lines)?;

            // the sauce is going to primarily be in the animations
            // we need to start it one reel at a time, n ms between, and stop them one at a time
            // with m ms between
            //
            // upon that finishes, we should then display
            ui.run_spin_animation(machines, total_lines)?;

            // todo: add a step here to display winning lines in a loop
            // we can either let that run for Y ms, or even better would be add a
            // press any button to continue to pause here before moving onto .finish()

            ui.finish(total_lines)?;

            database.balance += total_winnings;
            database.save()?;

            println!("You won {} credits!", total_winnings);
        }
    }

    Ok(())
}
