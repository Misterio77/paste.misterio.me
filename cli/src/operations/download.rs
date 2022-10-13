use crate::{Client, Paste, Result, Url, Uuid};

use anyhow::anyhow;
use atty::Stream;
use bat::{Input, PagingMode, PrettyPrinter};
use chrono_humanize::HumanTime;
use reqwest::StatusCode;

pub async fn download(api: Url, id: Uuid, raw: bool) -> Result<()> {
    let client = Client::new();

    let url = api.join(&format!("/p/{}", id))?;
    let res = client.get(url).send().await?;

    if res.status() == StatusCode::NOT_FOUND {
        return Err(anyhow!("The requested paste does not exist"));
    }

    let paste: Paste = res.error_for_status()?.json().await?;

    if !raw && atty::is(Stream::Stdout) {
        let title = paste.title.as_deref().unwrap_or("Untitled");
        let creator = paste.creator;
        let time = HumanTime::from(paste.creation);
        let visibility = if paste.unlisted { "Unlisted" } else { "Public" };

        PrettyPrinter::new()
            .input(
                Input::from_bytes(paste.content.as_bytes())
                    .name(title)
                    .title(format!(
                        "{} // by u/{}, {} - {}",
                        title, creator, time, visibility
                    )),
            )
            .line_numbers(true)
            .header(true)
            .grid(true)
            .paging_mode(PagingMode::QuitIfOneScreen)
            .theme("base16")
            .print()?;
    } else {
        println!("{}", paste.content.replace("\r", ""));
    }

    Ok(())
}
