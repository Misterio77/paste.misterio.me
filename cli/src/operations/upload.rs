use crate::{Client, Paste, PathBuf, Result, Session, Url};

use anyhow::anyhow;
use atty::Stream;
use reqwest::StatusCode;
use serde::Serialize;

use std::fs::read_to_string as read_from_file;
use std::io::{self, Read};

#[derive(Serialize)]
struct Payload {
    title: Option<String>,
    description: Option<String>,
    content: String,
    unlisted: bool,
}

pub async fn upload(
    api: Url,
    file: Option<PathBuf>,
    title: Option<String>,
    description: Option<String>,
    unlisted: bool,
    link_only: bool,
) -> Result<()> {
    let session = Session::load()?;
    let client = Client::new();

    let url = api.join("/p")?;

    // Get file name
    let file_title = file
        .as_ref()
        .filter(|p| p.to_string_lossy() != "-")
        .and_then(|f| f.file_name().and_then(|s| s.to_str()))
        .map(|s| s.to_owned());
    // If a title was not specified, use file name (if exists)
    let title = title.or(file_title);

    let content = if let Some(f) = file {
        if f.as_os_str() == "-" {
            read_from_stdin()?
        } else {
            read_from_file(f)?
        }
    } else {
        read_from_stdin()?
    };

    let payload = Payload {
        title,
        description,
        content,
        unlisted,
    };

    let res = client
        .post(url.clone())
        .json(&payload)
        .bearer_auth(session.key())
        .send()
        .await?;

    if res.status() == StatusCode::UNAUTHORIZED {
        return Err(anyhow!(
            "Your API key is invalid or has been revoked. Try `pmis auth`"
        ));
    }

    let paste: Paste = res.error_for_status()?.json().await?;

    let paste_path = format!("/p/{}", paste.id);

    if !link_only && atty::is(Stream::Stdout) {
        println!("Upload successful.");
        println!("Your paste id is {}", paste.id);
        println!("{}", url.join(&paste_path)?);
    } else {
        println!("{}", url.join(&paste_path)?);
    }

    Ok(())
}

fn read_from_stdin() -> Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;
    Ok(buffer)
}
