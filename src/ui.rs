use crossterm::{
    cursor, execute,
    style::Stylize,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use crate::{
    animation_state::{AnimationState, AnimationType},
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

        for frame in 0..100 {
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

    // todo: add a little handle that gets pulled on frames x -> y for a static animation
    fn render_machine_inline(
        &mut self,
        machine: &mut Machine,
        animation_state: &mut AnimationState,
        frame: i32,
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

        match animation_state.animation_type {
            AnimationType::Wait => {
                let visible_symbols = get_visible_symbols_for_reel(&machine.reels, Some(0));
                for row in visible_symbols {
                    // Cursor back to the front
                    execute!(
                        self.stdout,
                        cursor::MoveToColumn(0),
                        Clear(ClearType::UntilNewLine)
                    )?;

                    // Draw row here
                    writeln!(
                        self.stdout,
                        "  [ {} ] [ {} ] [ {} ] [ {} ] [ {} ]",
                        row[0], row[1], row[2], row[3], row[4]
                    )?;
                }
            }
            AnimationType::Spinning => {
                let visible_symbols =
                    get_visible_symbols_for_reel(&machine.reels, Some(frame as usize % 20));
                for row in visible_symbols {
                    // Cursor back to the front
                    execute!(
                        self.stdout,
                        cursor::MoveToColumn(0),
                        Clear(ClearType::UntilNewLine)
                    )?;

                    // Draw row here
                    writeln!(
                        self.stdout,
                        "  [ {} ] [ {} ] [ {} ] [ {} ] [ {} ]",
                        row[0], row[1], row[2], row[3], row[4]
                    )?;
                }
            }
            AnimationType::Stopped => {
                let visible_symbols = get_visible_symbols_for_reel(&machine.reels, Some(0));
                for row in visible_symbols {
                    // Cursor back to the front
                    execute!(
                        self.stdout,
                        cursor::MoveToColumn(0),
                        Clear(ClearType::UntilNewLine)
                    )?;

                    // Draw row here
                    writeln!(
                        self.stdout,
                        "  [ {} ] [ {} ] [ {} ] [ {} ] [ {} ]",
                        row[0], row[1], row[2], row[3], row[4]
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
