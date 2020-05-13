#[macro_use]
extern crate shell;

use shell::ShellError;

fn main() -> Result<(), ShellError> {
    match cmd!("cargo test").run() {
        Ok(_) => {
            cmd!("git add -A").run()?;
            cmd!("git commit -m \"working\"").run()?;
        },
        Err(_) => {
            cmd!("git reset --hard").run()?;
        }
    }
    Ok(())
}
