use chrono::{Local, Utc};
use std::fs::OpenOptions;
use std::io::Write;

// UTC TIME LOGS

fn template_utc(keyword: &str, text: &str, file: &str) -> std::io::Result<()> {
    let to_log: String = format!(
        "[{}] {} <{}>",
        keyword.to_uppercase(),
        text,
        Utc::now().format("%H:%M:%S")
    );
    println!("{}", to_log);
    let mut f = OpenOptions::new().append(true).create(true).open(file)?;
    writeln!(f, "{}", to_log)?;
    Ok(())
}

pub fn log_utc(text: &str, file: &str) -> std::io::Result<()> {
    template_utc("INFO", text, file)?;
    Ok(())
}

pub fn warn_utc(text: &str, file: &str) -> std::io::Result<()> {
    template_utc("WARN", text, file)?;
    Ok(())
}

pub fn err_utc(text: &str, file: &str) -> std::io::Result<()> {
    template_utc("ERROR", text, file)?;
    Ok(())
}

// LOCAL TIME LOGS

fn template_local(keyword: &str, text: &str, file: &str) -> std::io::Result<()> {
    let to_log: String = format!(
        "[{}] {} <{}>",
        keyword.to_uppercase(),
        text,
        Local::now().format("%H:%M:%S")
    );

    println!("{}", to_log);
    let mut f = OpenOptions::new().append(true).create(true).open(file)?;
    writeln!(f, "{}", to_log)?;
    Ok(())
}

pub fn log_local(text: &str, file: &str) -> std::io::Result<()> {
    template_local("INFO", text, file)?;
    Ok(())
}

pub fn warn_local(text: &str, file: &str) -> std::io::Result<()> {
    template_local("WARN", text, file)?;
    Ok(())
}

pub fn err_local(text: &str, file: &str) -> std::io::Result<()> {
    template_local("ERROR", text, file)?;
    Ok(())
}