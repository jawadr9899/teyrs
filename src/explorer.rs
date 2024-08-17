use std::env;
use std::io::Error;
use std::path::Path;
use std::process::exit;

use crate::input::Input;
use crate::sysdir::SysDir;
use crate::utils::{
    clear_terminal, print_colorfully, print_error_colorfully, run_command, show_dirs, show_metadata,
};
use crossterm::style::{Attribute, ContentStyle, Stylize};

pub fn show_menu(dir: &mut SysDir) -> Result<(), Error> {
    print_colorfully("- [CMDS]: ", ContentStyle::new().green(), Attribute::Bold)?;
    print_colorfully(
        "[mv path] [run cmd] [cls] [p] [inf name] [v] [exit]\n",
        ContentStyle::new().magenta(),
        Attribute::Bold,
    )?;

    let inp = Input::get_string("- ")?.input;
    process_cmd(inp, dir)?;
    Ok(())
}

pub fn handle_mv_cmd(args: &[&str], dir: &mut SysDir) -> Result<(), Error> {
    clear_terminal()?;
    // println!("Current Path: {:?}", dir.path);
    // println!("Command: {:?}", args.join(" "));

    let path = Path::new(&dir.path).join(args.join(" "));

    if path.try_exists()? {
        env::set_current_dir(&path)?;
        let cp = env::current_dir()?.to_str().unwrap().to_string();
        print_colorfully(
            format!("- [I] Moved to: {}\n\n", cp).as_str(),
            ContentStyle::new().blue(),
            Attribute::Italic,
        )?;
        dir.refresh(SysDir::from(path.to_str().unwrap().to_string()))?;
    } else {
        clear_terminal()?;
        print_error_colorfully("Invalid Cmd/Path!")?;
        dir.refresh(SysDir::from(dir.path.to_string()))?;
    }

    Ok(())
}

pub fn process_cmd(cmd_str: String, dir: &mut SysDir) -> Result<(), Error> {
    let mut cmd_parts = cmd_str.trim().split_whitespace().collect::<Vec<&str>>();
    if cmd_parts.is_empty() {
        clear_terminal()?;
        print_error_colorfully("No Command Entered!")?;
        dir.refresh(SysDir::from(dir.path.to_string()))?;
        return Ok(());
    }

    let cmd = cmd_parts.remove(0);

    match cmd {
        "mv" => {
            handle_mv_cmd(&cmd_parts, dir)?;
        }
        "run" => {
            print_colorfully(
                "- Processing...\n",
                ContentStyle::new().red(),
                Attribute::Bold,
            )?;
            if let Ok(_) = run_command("wsl", &cmd_parts) {
                dir.refresh(SysDir::from(dir.path.to_string()))?;
                println!();
            } else {
                clear_terminal()?;
                print_error_colorfully("Failed to run command!")?;
                dir.refresh(SysDir::from(dir.path.to_string()))?;
            }
        }
        "cls" => {
            clear_terminal()?;
            dir.refresh(SysDir::from(dir.path.to_string()))?;
        }
        "v" => {
            show_dirs(dir)?;
            dir.refresh(SysDir::from(dir.path.to_string()))?;
        }
        "p" => {
            let cp = env::current_dir()?.to_str().unwrap().to_string();
            print_colorfully(
                format!("\n- [I] Path: {}\n\n", cp).as_str(),
                ContentStyle::new().blue(),
                Attribute::Bold,
            )?;
            dir.refresh(SysDir::from(dir.path.to_string()))?;
        }
        "inf" => {
            show_metadata(&dir, cmd_parts[0])?;
            dir.refresh(SysDir::from(dir.path.to_string()))?;
        }
        "exit" => exit(0),
        _ => {
            clear_terminal()?;
            print_error_colorfully("Invalid Command!")?;
            dir.refresh(SysDir::from(dir.path.to_string()))?;
        }
    }
    Ok(())
}
