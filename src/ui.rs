use crossterm::{
    cursor, execute,
    style::Stylize,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub struct TerminalUI {
    stdout: io::Stdout,
}

impl TerminalUI {
    pub fn new() -> Self {
        Self {
            stdout: io::stdout(),
        }
    }

    pub fn start(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        execute!(self.stdout, cursor::Hide)?;
        Ok(())
    }

    pub fn finish(&mut self) -> io::Result<()> {
        execute!(self.stdout, cursor::Show)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn run_spin_animation(&mut self, machine_count: i32) -> io::Result<()> {
        let lines_per_machine = 5; // Header + 3 rows + Spacer
        let total_lines = (machine_count * lines_per_machine) as u16 + 1;

        // Step 1: Create the "Stage"
        // We print the lines once to push the prompt up and establish our space.
        for _ in 0..total_lines {
            writeln!(self.stdout)?;
        }

        // Step 2: The Animation Loop
        for frame in 0..20 {
            // Move back to the top of our "Stage"
            execute!(self.stdout, cursor::MoveUp(total_lines))?;
            writeln!(self.stdout)?;

            for i in 0..machine_count {
                self.render_machine_inline(i, frame)?;
            }

            self.stdout.flush()?;
            thread::sleep(Duration::from_millis(80)); // Slightly faster for smoothness
        }

        // Step 3: Cleanup (The "Dissolve")
        // Move back up one last time and wipe the stage clean.
        execute!(
            self.stdout,
            cursor::MoveUp(total_lines),
            Clear(ClearType::FromCursorDown)
        )?;

        Ok(())
    }

    fn render_machine_inline(&mut self, id: i32, frame: i32) -> io::Result<()> {
        // Header
        execute!(
            self.stdout,
            cursor::MoveToColumn(0),
            Clear(ClearType::UntilNewLine)
        )?;
        writeln!(
            self.stdout,
            "{}{} {}",
            " ".repeat(12),
            "Machine".bold(),
            (id + 1).to_string().yellow()
        )?;

        // Slot machine rows
        for _row in 0..3 {
            execute!(
                self.stdout,
                cursor::MoveToColumn(0),
                Clear(ClearType::UntilNewLine)
            )?;

            // example of the animations
            // we probably want to generate "partial" versions, will revisit complex animations
            // later
            writeln!(
                self.stdout,
                "  [ {} ] [ {} ] [ {} ] [ {} ] [ {} ]",
                frame % 9,
                frame % 9,
                frame % 9,
                frame % 9,
                frame % 9
            )?;
        }

        // Spacer
        execute!(
            self.stdout,
            cursor::MoveToColumn(0),
            Clear(ClearType::UntilNewLine)
        )?;
        writeln!(self.stdout)?;

        Ok(())
    }
}

