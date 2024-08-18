pub mod explorer;
pub mod input;
pub mod sysdir;
pub mod utils;
use std::env;

use crossterm::style::{Attribute, Color, ContentStyle, Stylize};
use explorer::show_menu;
use sysdir::SysDir;
use utils::print_colorfully;
const TERMINAL: &str = "wsl";

fn init() -> Result<(), std::io::Error> {
    print_colorfully(
        "\n- TeyRS\n\n",
        ContentStyle::new().with(Color::AnsiValue(212u8)),
        Attribute::Bold,
    )?;
    match env::current_dir() {
        Ok(p) => {
            let mut dir = SysDir::new(p.to_str().unwrap().to_string());
            show_menu(&mut dir)?;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn main() -> Result<(), std::io::Error> {
    init()?;
    Ok(())
}
