use std::{env, io};

use crate::scripts::UnicodeBuilder;

mod scripts;

fn main() -> io::Result<()> {
    let Ok(profile) = env::var("PROFILE") else {
        panic!("env PROFILE error")
    };

    if profile == "release" {
        UnicodeBuilder::generate_tables()?;
    }

    Ok(())
}
