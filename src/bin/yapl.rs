use std::env;
use std::error::Error;
use std::process::exit;
use yapl::compile;

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).unwrap_or_else(|| {
        println!("must include filepath as single arg");
        exit(1);
    });

    compile(path.as_str())?;

    Ok(())
}
