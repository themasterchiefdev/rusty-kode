use std::{env, io};

fn main() -> io::Result<()> {
    let mut output = io::stdout();

    rusty_kode::run(
        env::args_os().skip(1),
        |_| {
            println!("Hello, world!");
            Ok(())
        },
        &mut output,
    )
}
