use std::io::{stdin, stdout, Error, Write};

pub struct Input {
    pub input: String,
}

impl Input {
    pub fn get_string(s: &str) -> Result<Self, Error> {
        print!("{}", s);
        let mut s = String::new();
        stdout().flush().expect("failed to flush buffer!");
        stdin().read_line(&mut s)?;
        Ok(Input {
            input: s.trim().to_string(),
        })
    }
}
