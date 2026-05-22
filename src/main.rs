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

    let mut database = Database::load()?;

    match cli.command {
        Commands::Balance => {
            println!("Your balance is: {}", database.balance);
        }
        Commands::Cheat { amount } => {
            database.balance += amount;
            database.save()?;
            println!(
                "Added {} credits. New balance: {}",
                amount, database.balance
            );
        }
        Commands::Play { bet, count } => {
            let total_bet = bet * count;
            database.balance -= total_bet;
            database.total_spins += count;

            let mut machines = Machine::create_n_machines(count, bet);
            let mut total_winnings: i32 = 0;
            for machine in &mut machines {
                machine.spin();
                machine.get_all_paylines();

                for payline in &machine.paylines {
                    total_winnings += payline.get_payout(bet);
                }
            }

            let lines_per_machine = 8; // Header + top border + 3 rows + bottom border + info line + spacer
            let total_lines = (count * lines_per_machine) + 3; // +1 top + 1 total winnings + 1 press any key

            ui.start(total_lines)?;

            ui.run_spin_animation(machines, total_lines)?;
            ui.wait_for_keypress()?;

            ui.finish(total_lines)?;

            database.balance += total_winnings;
            database.save()?;
        }
    }

    Ok(())
}
