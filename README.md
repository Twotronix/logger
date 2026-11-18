# Rust Logger Module
Lightweight logger built in Rust.

## Features

* **3 Log Types** `INFO`, `WARN` & `ERROR`
* **UTC timestamps** `log_utc()`, `warn_utc()`, `err_utc()`
* **Local timestamps** `log_local()`, `warn_local()`, `err_local()`

## Prerequisites
* Ensure the `chrono` crate has been added to `Cargo.toml`
```toml
[dependencies]
chrono = "0.4"
```
## How to use

#### UTC time
```Rust
mod logger;

fn main() -> std::io::Result<()> {
    log_utc("UTC Information Time", "x.log")?;
    warn_utc("UTC Warning Time", "x.log")?;
    err_utc("UTC Error Time", "x.log")?;
    Ok(())
}

```
#### Local time
```Rust
mod logger;

fn main() -> std::io::Result<()> {
    log_local("Local Time Information", "x.log")?;
    warn_local("Local Time Warning", "x.log")?;
    err_local("Local Time Error", "x.log")?;
    Ok(())
}
```
#### Output
```text
x.logs:
[INFO] Info <12:00:00>
[WARN] Warning! <12:00:00>
[ERROR] Error! <12:00:00>
```