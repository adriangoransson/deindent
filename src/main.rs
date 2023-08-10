use deindent::Deindenter;
use std::io::{self, Read, Result};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let Some(deindenter) = Deindenter::new(&input) else {
        return Ok(());
    };

    let mut out = io::stdout().lock();
    deindenter.to_writer(&mut out)
}
