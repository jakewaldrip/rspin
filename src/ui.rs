use crossterm::{
    cursor, execute,
    style::Stylize,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use crate::{
    animation_state::{
        ANIMATION_TIME_FLOOR, ANIMATION_TIME_MACHINE_FACTOR, AnimationState, AnimationType,
        LEVER_PULL_FRAME_TIME,
    },
    machine::{Machine, get_visible_symbols_for_reel},
};

const FRAME_MS: u64 = 80;

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
        let mut machine_animations: Vec<(Machine, AnimationState)> = machines
            .iter()
            .enumerate()
            .map(|(i, machine)| (machine.clone(), AnimationState::new(i)))
            .collect();

        let total_frames: usize =
            ANIMATION_TIME_FLOOR + (ANIMATION_TIME_MACHINE_FACTOR * machines.len());
        for frame in 0..total_frames {
            // Move to the top of the stage
            execute!(self.stdout, cursor::MoveUp(total_lines as u16))?;
            writeln!(self.stdout)?;

            for (machine, animation_state) in &mut machine_animations {
                self.render_machine_inline(machine, animation_state, frame)?;
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

        // TODO: add box around the machine (vimscape has the relevant glyphs)
        // TODO: add state for showing pay lines, might need to refactor paylines to include the
        // positions that contributed to the pay so we can translate them here
        // Change color of the relevant positions
        // Make it blink as well
        // TODO: Calculate starting position for each reel
        match animation_state.animation_type {
            AnimationType::Wait => {
                let visible_symbols = get_visible_symbols_for_reel(&machine.reels, Some(0));
                for (row_idx, row) in visible_symbols.iter().enumerate() {
                    // Cursor back to the front
                    execute!(
                        self.stdout,
                        cursor::MoveToColumn(0),
                        Clear(ClearType::UntilNewLine)
                    )?;

                    // Draw row here, lever resting at top
                    let lever = if row_idx == 0 { " O" } else { "  " };
                    writeln!(
                        self.stdout,
                        "{}| {} | {} | {} | {} | {} |{}",
                        " ".repeat(5),
                        row[0],
                        row[1],
                        row[2],
                        row[3],
                        row[4],
                        lever
                    )?;
                }
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

                let visible_symbols = get_visible_symbols_for_reel(&machine.reels, Some(0));
                for (row_idx, row) in visible_symbols.iter().enumerate() {
                    execute!(
                        self.stdout,
                        cursor::MoveToColumn(0),
                        Clear(ClearType::UntilNewLine)
                    )?;
                    writeln!(
                        self.stdout,
                        "{}| {} | {} | {} | {} | {} |{}",
                        " ".repeat(5),
                        row[0],
                        row[1],
                        row[2],
                        row[3],
                        row[4],
                        lever_suffix(row_idx)
                    )?;
                }
            }
            AnimationType::Spinning => {
                let visible_symbols =
                    get_visible_symbols_for_reel(&machine.reels, Some(frame as usize % 20));
                for (row_idx, row) in visible_symbols.iter().enumerate() {
                    // Cursor back to the front
                    execute!(
                        self.stdout,
                        cursor::MoveToColumn(0),
                        Clear(ClearType::UntilNewLine)
                    )?;

                    // Draw row here
                    let lever = if row_idx == 0 { " O" } else { "  " };
                    writeln!(
                        self.stdout,
                        "{}| {} | {} | {} | {} | {} |{}",
                        " ".repeat(5),
                        row[0],
                        row[1],
                        row[2],
                        row[3],
                        row[4],
                        lever,
                    )?;
                }
            }
            AnimationType::Stopped => {
                let visible_symbols = get_visible_symbols_for_reel(&machine.reels, None);
                for (row_idx, row) in visible_symbols.iter().enumerate() {
                    // Cursor back to the front
                    execute!(
                        self.stdout,
                        cursor::MoveToColumn(0),
                        Clear(ClearType::UntilNewLine)
                    )?;

                    // Draw row here, lever resting at top
                    let lever = if row_idx == 0 { " O" } else { "  " };
                    writeln!(
                        self.stdout,
                        "{}| {} | {} | {} | {} | {} |{}",
                        " ".repeat(5),
                        row[0],
                        row[1],
                        row[2],
                        row[3],
                        row[4],
                        lever
                    )?;
                }
            }
        }

        // Slot machine rows

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
