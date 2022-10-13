use crate::{Result, Session, Url};

use atty::Stream;

use std::io;

pub async fn auth(api: Url) -> Result<()> {
    if atty::is(Stream::Stdout) {
        println!("Get a token from {} and paste it here:", api.join("/keys")?);
    }
    let mut key = String::new();
    io::stdin().read_line(&mut key)?;

    Session::save(key)?;

    Ok(())
}
