mod logger;
fn main() -> std::io::Result<()> {
    logger::log_utc("Info", "test.log")?;
    logger::warn_utc("Warning!", "test.log")?;
    logger::err_utc("Error!", "test.log")?;

    logger::log_local("Info", "test.log")?;
    logger::warn_local("Warning!", "test.log")?;
    logger::err_local("Error!", "test.log")?;
    Ok(())
}
