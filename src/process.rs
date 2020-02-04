use std::io;
use std::io::BufRead;
use std::process::{Command, ExitStatus};
use std::string::FromUtf8Error;

use anyhow::{Context, Result};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Process didn't succeed: {0}")]
    ExitStatus(ExitStatus),
    #[error("Process wrote to stderr: {0}")]
    Stderr(String),
}

pub fn run_cmd<F, T>(c: &mut Command, f: F) -> Result<T>
where
    F: FnOnce(Vec<u8>) -> T,
{
    let output = c.output().map_err(Box::new)?;

    if !output.status.success() {
        return Err(
            CommandError::ExitStatus(output.status).context(format!("Process {} failed.", c))
        );
    }

    if !output.stderr.is_empty() {
        return Err(CommandError::Stderr(
            String::from_utf8(output.stderr).with_context(|| {
                format!(
                    "Failed to decode stderr of {} as utf-8. Stderr (lossily decoded): {}",
                    c,
                    String::from_utf8_lossy(output.stderr)
                )
            })?,
        ));
    }

    Ok(f(output.stdout))
}

pub fn run_cmd_stdout(c: &mut Command) -> Result<String> {
    run_cmd(c, String::from_utf8)
        .with_context(|| format!("Failed to decode stdout of {} as utf-8.", c))?
}
