use crate::{Result, Session, Url};

use atty::Stream;

use std::fs::read_to_string as read_from_file;
use std::io::{self, BufRead};
use std::path::PathBuf;

pub async fn auth(api: Url, file: Option<PathBuf>) -> Result<()> {
    let key = if let Some(f) = file {
        if f.as_os_str() == "-" {
            read_from_stdin()?
        } else {
            read_from_file(f)?
        }
    } else {
        if atty::is(Stream::Stdout) {
            println!("Get a token from {} and paste it here:", api.join("/keys")?);
        }
        read_from_stdin()?
    };

    Session::save(key)?;

    Ok(())
}

fn read_from_stdin() -> Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_line(&mut buffer)?;
    Ok(buffer)
}
