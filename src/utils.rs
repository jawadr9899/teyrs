use crate::SysDir;
use chrono::{DateTime, Utc};
use crossterm::cursor::MoveTo;
use crossterm::style::Attribute;
use crossterm::style::ContentStyle;
use crossterm::style::Stylize;
use crossterm::QueueableCommand;
use crossterm::{
    style::{PrintStyledContent, StyledContent},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io;
use std::io::Error;
use std::io::Write;
use std::process::exit;
use std::process::Command;
use std::process::Stdio;
use std::time::SystemTime;
const COL_WIDTH: usize = 15;

pub fn show_dirs_normal(dirs: &Vec<String>) {
    print!("====> ");
    for (i, j) in dirs.iter().enumerate() {
        print!("({i}) '{j}'");
        if (i >= 5) && (i % 5 == 0) {
            println!();
            print!("====> ");
        }
    }
}

fn calculate_columns(total_items: usize, max_columns: usize) -> usize {
    if total_items == 0 {
        return 1;
    }
    let cols_based_on_items = (total_items + max_columns - 1) / max_columns;
    (cols_based_on_items).min(max_columns)
}

pub fn show_dirs(dir: &SysDir) -> Result<(), std::io::Error> {
    let dirs = dir.get_as_vecstr()?;
    let size = crossterm::terminal::size().unwrap_or_else(|_| exit(1));
    let it_width = 10 as usize;
    let cols: usize = calculate_columns(dirs.len(), size.0 as usize / it_width);

    print_colorfully(
        "\n- [DIRS]:\n",
        ContentStyle::new().green(),
        Attribute::Bold,
    )?;
    let rows = (dirs.len() + cols - 1) / cols;
    for row in 0..rows {
        print!("--");
        let mut line = String::new();
        for col in 0..cols {
            let idx = row + col * rows;
            if idx < dirs.len() {
                let d = &dirs[idx];
                let cell = format!(
                    "|{idx}| {d:<fmt$}\t",
                    fmt = (COL_WIDTH - (idx.to_string().len()) - 2)
                );
                line.push_str(cell.as_str());
            }
        }
        print_colorfully(
            format!("\t{}\n", line.trim_start()).as_str(),
            ContentStyle::new().dark_yellow(),
            Attribute::Encircled,
        )?;
    }
    println!();
    Ok(())
}
pub fn to_readable_time(t: SystemTime) -> Result<DateTime<Utc>, Error> {
    match t.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => {
            let datetime: DateTime<Utc> = DateTime::from(SystemTime::UNIX_EPOCH + duration);
            Ok(datetime)
        }
        Err(e) => {
            println!("Error converting system time: {:?}", e);
            Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
        }
    }
}

pub fn show_metadata(dir: &SysDir, file_name: &str) -> Result<(), std::io::Error> {
    let d = dir.get_metadata(file_name)?;
    if let Some(metadata) = d {
        let f = crate::sysdir::FileInfo::new(metadata);
        print_colorfully(
            format!("- [{}]:\n", file_name).as_str(),
            ContentStyle::new().green(),
            Attribute::Bold,
        )?;
        let timings: (DateTime<Utc>, DateTime<Utc>, DateTime<Utc>) =
            (f.creation_time()?, f.last_modified()?, f.last_accessed()?);

        print_colorfully(
            format!(
                "\t- Created At: {}\n",
                timings.0.format("('%d-%m-%Y | %I:%M:%S %p')").to_string()
            )
            .as_str(),
            ContentStyle::new().cyan(),
            Attribute::Italic,
        )?;
        print_colorfully(
            format!(
                "\t- Modified At: {}\n",
                timings.0.format("('%d-%m-%Y | %I:%M:%S %p')").to_string()
            )
            .as_str(),
            ContentStyle::new().cyan(),
            Attribute::Italic,
        )?;
        print_colorfully(
            format!(
                "\t- Accessed At: {}\n",
                timings.0.format("('%d-%m-%Y | %I:%M:%S %p')").to_string()
            )
            .as_str(),
            ContentStyle::new().cyan(),
            Attribute::Italic,
        )?;
        print_colorfully(
            format!("\t- Is File: {}\n", f.is_file()).as_str(),
            ContentStyle::new().cyan(),
            Attribute::Italic,
        )?;
        print_colorfully(
            format!("\t- Is Dir: {}\n", f.is_dir()).as_str(),
            ContentStyle::new().cyan(),
            Attribute::Italic,
        )?;
        print_colorfully(
            format!("\t- Is Readonly: {}\n", f.is_readonly()).as_str(),
            ContentStyle::new().cyan(),
            Attribute::Italic,
        )?;
        print_colorfully(
            format!("\t- Is Symlink: {}\n", f.is_symlink()).as_str(),
            ContentStyle::new().cyan(),
            Attribute::Italic,
        )?;
        if f.is_dir() {
            print_colorfully(
                format!("\t- Size : ___ bytes\n").as_str(),
                ContentStyle::new().green(),
                Attribute::Italic,
            )?;
        } else {
            print_colorfully(
                format!("\t- Size : {} bytes\n", f.size_in_bytes()).as_str(),
                ContentStyle::new().blue(),
                Attribute::Italic,
            )?;
        }
    } else {
        print_error_colorfully("File not found in this scope")?;
    }
    println!("\n");
    Ok(())
}

pub fn print_error_colorfully(e: &str) -> Result<(), Error> {
    print_colorfully(
        format!("- [E] {}\n", e.to_string()).as_str(),
        ContentStyle::new().red(),
        Attribute::Bold,
    )
}

pub fn print_colorfully(text: &str, styl: ContentStyle, attr: Attribute) -> io::Result<()> {
    let mut stdout = io::stdout();
    let text = StyledContent::new(styl, text).attribute(attr);
    stdout.execute(PrintStyledContent(text))?;
    stdout.flush()?;
    Ok(())
}

pub fn clear_terminal() -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(Clear(ClearType::All))?;
    stdout.queue(MoveTo(0, 0))?;
    Ok(())
}

pub fn run_command(command: &str, args: &Vec<&str>) -> Result<(), io::Error> {
    for (i, arg) in args.iter().enumerate() {
        let process = Command::new(command)
            .arg(arg)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let output = process.wait_with_output()?;

        if output.status.success() {
            print_colorfully(
                format!("- [OUTPUT {}]:\n", i + 1).as_str(),
                ContentStyle::new().green(),
                Attribute::Bold,
            )?;

            let stdout = &output.stdout;
            let mut stdout_str = String::with_capacity(stdout.len());
            for chunk in stdout.chunks(1024) {
                stdout_str.push_str(&String::from_utf8_lossy(chunk));
            }

            let indented_output = stdout_str
                .lines()
                .map(|line| format!("-- \t{}", line))
                .collect::<Vec<String>>()
                .join("\n");

            print_colorfully(&indented_output, ContentStyle::new().grey(), Attribute::Dim)?;
            println!("\n");
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            print_error_colorfully(&stderr)?;
            return Err(io::Error::new(io::ErrorKind::Other, "Command failed"));
        }
    }
    Ok(())
}
