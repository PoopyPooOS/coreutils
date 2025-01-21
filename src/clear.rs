use std::io::{self, stdout, Write};

pub fn clear() -> io::Result<()> {
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush()?;

    Ok(())
}
