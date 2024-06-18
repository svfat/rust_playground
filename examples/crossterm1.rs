use std::{thread, time};
use std::io::{self, Write};
use crossterm::{ execute, queue, style::{self, Stylize}, cursor, terminal};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, cursor::Hide)?;
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    for n in 0..=64 {
        for y in 0..=32 {
            queue!(stdout, cursor::MoveTo(n,y), style::PrintStyledContent("█".cyan()))?;
        }
        stdout.flush()?;
        thread::sleep(time::Duration::from_millis(200));
    }
    Ok(())
}
