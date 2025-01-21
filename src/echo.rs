use std::io;

pub fn echo(args: impl Iterator<Item = String>) -> io::Result<()> {
    let message: String = args.skip(1).intersperse(" ".to_string()).collect();
    println!("{message}");

    Ok(())
}
