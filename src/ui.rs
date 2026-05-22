use crossterm::{
    cursor,
    event::{self, Event},
    execute,
    style::Stylize,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::collections::HashSet;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use crate::{
    animation_state::{
        AnimationState, AnimationType, FRAMES_PER_PAYLINE, LEVER_PULL_FRAME_TIME,
        total_animation_frames,
    },
    machine::{Machine, calc_reel_starting_points, get_visible_symbols_for_reel},
    paylines::Paylines,
};

// TODO: make this configurable (with a "fast mode")
// TODO: add some debug prints/flag
// TODO: add a session tracker, need to think through this
const FRAME_MS: u64 = 80;
const LEFT_PADDING: usize = 5;
const REEL_WIDTH: usize = 3;

pub struct TerminalUI {
    stdout: io::Stdout,
}

impl TerminalUI {
    pub fn new() -> Self {
        Self {
            stdout: io::stdout(),
        }
    }

    pub fn start(&mut self, total_lines: i32) -> io::Result<()> {
        enable_raw_mode()?;
        execute!(self.stdout, cursor::Hide)?;

        // Set the stage
        // We print the lines once to push the prompt up and establish our space.
        for _ in 0..total_lines {
            writeln!(self.stdout)?;
        }

        Ok(())
    }

    pub fn wait_for_keypress(&mut self) -> io::Result<()> {
        loop {
            if let Event::Key(_) = event::read()? {
                break;
            }
        }

        Ok(())
    }

    pub fn finish(&mut self, total_lines: i32) -> io::Result<()> {
        execute!(self.stdout, cursor::Show)?;
        disable_raw_mode()?;

        // Cleanup
        // Move back up one last time and wipe the stage clean.
        execute!(
            self.stdout,
            cursor::MoveUp(total_lines as u16),
            Clear(ClearType::FromCursorDown)
        )?;

        Ok(())
    }

    pub fn run_spin_animation(
        &mut self,
        machines: Vec<Machine>,
        total_lines: i32,
    ) -> io::Result<()> {
        let max_paylines = machines.iter().map(|m| m.paylines.len()).max().unwrap_or(0);
        let total_machines = machines.len();

        let mut machine_animations: Vec<(Machine, AnimationState)> = machines
            .iter()
            .enumerate()
            .map(|(i, machine)| {
                (
                    machine.clone(),
                    AnimationState::new(i, total_machines, max_paylines),
                )
            })
            .collect();

        let total_frames = total_animation_frames(total_machines, max_paylines);
        for frame in 0..total_frames {
            // Move to the top of the stage
            execute!(self.stdout, cursor::MoveUp(total_lines as u16))?;
            writeln!(self.stdout)?;

            for (machine, animation_state) in &mut machine_animations {
                self.render_machine_inline(machine, animation_state, frame)?;
            }

            // Total winnings line
            let all_stopped = machine_animations
                .iter()
                .all(|(_, state)| matches!(state.animation_type, AnimationType::Stopped));

            execute!(
                self.stdout,
                cursor::MoveToColumn(0),
                Clear(ClearType::UntilNewLine)
            )?;
            if all_stopped {
                let total_winnings: i32 = machine_animations
                    .iter()
                    .map(|(m, _)| m.paylines.iter().map(|p| p.get_payout(m.bet)).sum::<i32>())
                    .sum();
                writeln!(
                    self.stdout,
                    "{}You won {} credits!",
                    " ".repeat(LEFT_PADDING),
                    total_winnings
                )?;
            } else {
                writeln!(self.stdout)?;
            }

            // Press any key line
            execute!(
                self.stdout,
                cursor::MoveToColumn(0),
                Clear(ClearType::UntilNewLine)
            )?;
            if all_stopped {
                writeln!(
                    self.stdout,
                    "{}{}",
                    " ".repeat(LEFT_PADDING),
                    "Press any key to continue...".dark_grey()
                )?;
            } else {
                writeln!(self.stdout)?;
            }

            self.stdout.flush()?;
            thread::sleep(Duration::from_millis(FRAME_MS));
        }

        Ok(())
    }

    fn render_machine_inline(
        &mut self,
        machine: &mut Machine,
        animation_state: &mut AnimationState,
        frame: usize,
    ) -> io::Result<()> {
        // Go to top
        execute!(
            self.stdout,
            cursor::MoveToColumn(0),
            Clear(ClearType::UntilNewLine)
        )?;

        // Header
        writeln!(
            self.stdout,
            "{} {}",
            " ".repeat(11),
            (*machine.name).yellow()
        )?;

        // Make top of machine
        // │ ┌ ┐ └ ┘ ┬ ┴ ─
        execute!(
            self.stdout,
            cursor::MoveToColumn(0),
            Clear(ClearType::UntilNewLine)
        )?;
        writeln!(
            self.stdout,
            "{}┌{}┬{}┬{}┬{}┬{}┐",
            " ".repeat(LEFT_PADDING),
            "─".repeat(REEL_WIDTH),
            "─".repeat(REEL_WIDTH),
            "─".repeat(REEL_WIDTH),
            "─".repeat(REEL_WIDTH),
            "─".repeat(REEL_WIDTH),
        )?;

        let reel_mid_overrides = calc_reel_starting_points(machine);

        let info_line: String = match animation_state.animation_type {
            AnimationType::Wait => {
                let visible_symbols =
                    get_visible_symbols_for_reel(&machine.reels, Some(&reel_mid_overrides));
                for (row_idx, row) in visible_symbols.iter().enumerate() {
                    execute!(
                        self.stdout,
                        cursor::MoveToColumn(0),
                        Clear(ClearType::UntilNewLine)
                    )?;

                    let lever = if row_idx == 0 { " O" } else { "  " };
                    writeln!(
                        self.stdout,
                        "{}│ {} │ {} │ {} │ {} │ {} │ {}",
                        " ".repeat(LEFT_PADDING),
                        row[0],
                        row[1],
                        row[2],
                        row[3],
                        row[4],
                        lever
                    )?;
                }
                String::new()
            }
            AnimationType::LeverPull => {
                let lever_frame =
                    (LEVER_PULL_FRAME_TIME - 1).saturating_sub(animation_state.frames_remaining);

                let lever_suffix = |row_idx: usize| -> &'static str {
                    match (lever_frame, row_idx) {
                        (0, 0) | (4, 0) => " O", // handle at top
                        (1, 0) | (3, 0) => " |", // shaft at top, handle mid
                        (1, 1) | (3, 1) => " O", // handle at middle
                        (2, 0) | (2, 1) => " |", // shaft trailing
                        (2, 2) => " O",          // handle at bottom (fully pulled)
                        _ => "  ",               // nothing
                    }
                };

                let visible_symbols =
                    get_visible_symbols_for_reel(&machine.reels, Some(&reel_mid_overrides));
                for (row_idx, row) in visible_symbols.iter().enumerate() {
                    execute!(
                        self.stdout,
                        cursor::MoveToColumn(0),
                        Clear(ClearType::UntilNewLine)
                    )?;
                    writeln!(
                        self.stdout,
                        "{}│ {} │ {} │ {} │ {} │ {} │ {}",
                        " ".repeat(LEFT_PADDING),
                        row[0],
                        row[1],
                        row[2],
                        row[3],
                        row[4],
                        lever_suffix(row_idx)
                    )?;
                }
                String::new()
            }
            AnimationType::Spinning => {
                let spinning_mid_override = frame % 20;
                let visible_symbols = get_visible_symbols_for_reel(
                    &machine.reels,
                    Some(&[
                        spinning_mid_override,
                        spinning_mid_override,
                        spinning_mid_override,
                        spinning_mid_override,
                        spinning_mid_override,
                    ]),
                );
                for (row_idx, row) in visible_symbols.iter().enumerate() {
                    execute!(
                        self.stdout,
                        cursor::MoveToColumn(0),
                        Clear(ClearType::UntilNewLine)
                    )?;

                    let lever = if row_idx == 0 { " O" } else { "  " };
                    writeln!(
                        self.stdout,
                        "{}│ {} │ {} │ {} │ {} │ {} │ {}",
                        " ".repeat(LEFT_PADDING),
                        row[0],
                        row[1],
                        row[2],
                        row[3],
                        row[4],
                        lever,
                    )?;
                }
                String::new()
            }
            AnimationType::ShowWinnings => {
                let visible_symbols = get_visible_symbols_for_reel(&machine.reels, None);

                if machine.paylines.is_empty() {
                    for (row_idx, row) in visible_symbols.iter().enumerate() {
                        execute!(
                            self.stdout,
                            cursor::MoveToColumn(0),
                            Clear(ClearType::UntilNewLine)
                        )?;

                        let lever = if row_idx == 0 { " O" } else { "  " };
                        writeln!(
                            self.stdout,
                            "{}│ {} │ {} │ {} │ {} │ {} │ {}",
                            " ".repeat(LEFT_PADDING),
                            format!("{}", row[0]).dark_grey(),
                            format!("{}", row[1]).dark_grey(),
                            format!("{}", row[2]).dark_grey(),
                            format!("{}", row[3]).dark_grey(),
                            format!("{}", row[4]).dark_grey(),
                            lever,
                        )?;
                    }
                    format!("{}{}", " ".repeat(LEFT_PADDING), "0 lines paid 0 credits")
                } else {
                    let elapsed = animation_state
                        .show_winnings_duration
                        .saturating_sub(animation_state.frames_remaining);
                    let current_idx = (elapsed / FRAMES_PER_PAYLINE) % machine.paylines.len();
                    let current_payline = &machine.paylines[current_idx];

                    let winning_positions: HashSet<(usize, usize)> =
                        current_payline.positions().iter().cloned().collect();

                    for (row_idx, row) in visible_symbols.iter().enumerate() {
                        execute!(
                            self.stdout,
                            cursor::MoveToColumn(0),
                            Clear(ClearType::UntilNewLine)
                        )?;

                        let lever = if row_idx == 0 { " O" } else { "  " };

                        let styled: Vec<String> = row
                            .iter()
                            .enumerate()
                            .map(|(col_idx, sym)| {
                                let text = format!("{}", sym);
                                if winning_positions.contains(&(row_idx, col_idx)) {
                                    style_winning_symbol(current_payline, text).to_string()
                                } else {
                                    format!("{}", text.dark_grey())
                                }
                            })
                            .collect();

                        writeln!(
                            self.stdout,
                            "{}│ {} │ {} │ {} │ {} │ {} │ {}",
                            " ".repeat(LEFT_PADDING),
                            styled[0],
                            styled[1],
                            styled[2],
                            styled[3],
                            styled[4],
                            lever,
                        )?;
                    }

                    let payout = current_payline.get_payout(machine.bet);
                    format!(
                        "{}{} ({}) - {} credits",
                        " ".repeat(LEFT_PADDING),
                        current_payline.display_name(),
                        current_payline.symbol(),
                        payout
                    )
                }
            }
            AnimationType::Stopped => {
                let visible_symbols = get_visible_symbols_for_reel(&machine.reels, None);
                for (row_idx, row) in visible_symbols.iter().enumerate() {
                    execute!(
                        self.stdout,
                        cursor::MoveToColumn(0),
                        Clear(ClearType::UntilNewLine)
                    )?;

                    let lever = if row_idx == 0 { " O" } else { "  " };
                    writeln!(
                        self.stdout,
                        "{}│ {} │ {} │ {} │ {} │ {} │ {}",
                        " ".repeat(LEFT_PADDING),
                        format!("{}", row[0]).dark_grey(),
                        format!("{}", row[1]).dark_grey(),
                        format!("{}", row[2]).dark_grey(),
                        format!("{}", row[3]).dark_grey(),
                        format!("{}", row[4]).dark_grey(),
                        lever,
                    )?;
                }

                let total_payout: i32 = machine
                    .paylines
                    .iter()
                    .map(|p| p.get_payout(machine.bet))
                    .sum();
                let line_count = machine.paylines.len();
                format!(
                    "{}{} lines paid {} credits",
                    " ".repeat(LEFT_PADDING),
                    line_count,
                    total_payout
                )
            }
        };

        // Make bottom of machine
        // │ ┌ ┐ └ ┘ ┬ ┴ ─
        execute!(
            self.stdout,
            cursor::MoveToColumn(0),
            Clear(ClearType::UntilNewLine)
        )?;
        writeln!(
            self.stdout,
            "{}└{}┴{}┴{}┴{}┴{}┘",
            " ".repeat(LEFT_PADDING),
            "─".repeat(REEL_WIDTH),
            "─".repeat(REEL_WIDTH),
            "─".repeat(REEL_WIDTH),
            "─".repeat(REEL_WIDTH),
            "─".repeat(REEL_WIDTH),
        )?;

        // Info line
        execute!(
            self.stdout,
            cursor::MoveToColumn(0),
            Clear(ClearType::UntilNewLine)
        )?;
        writeln!(self.stdout, "{}", info_line)?;

        // Spacer
        execute!(
            self.stdout,
            cursor::MoveToColumn(0),
            Clear(ClearType::UntilNewLine)
        )?;
        writeln!(self.stdout)?;

        animation_state.tick();
        Ok(())
    }
}

fn style_winning_symbol(payline: &Paylines, text: String) -> String {
    let styled = match payline {
        Paylines::HorSM(..) => text.yellow().bold(),
        Paylines::AboveSM(..) => text.cyan().bold(),
        Paylines::BelowSM(..) => text.blue().bold(),
        Paylines::ZigSM(..) => text.magenta().bold(),
        Paylines::ZagSM(..) => text.dark_magenta().bold(),
        Paylines::HorXL(..) => text.green().bold(),
        Paylines::Zig(..) => text.dark_yellow().bold(),
        Paylines::Zag(..) => text.red().bold(),
        Paylines::Above(..) => text.dark_cyan().bold(),
        Paylines::Below(..) => text.dark_blue().bold(),
        Paylines::Eye(..) => text.white().bold(),
    };
    format!("{}", styled)
}
