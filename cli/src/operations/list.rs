use crate::{Client, Paste, Result, Session, Url};

use anyhow::anyhow;
use chrono_humanize::HumanTime;
use reqwest::StatusCode;

pub async fn list(api: Url, owner: Option<String>, ids_only: bool) -> Result<()> {
    let session = Session::load().ok();
    let client = Client::new_with_redir();

    let url = api.join(&format!("/u/{}/pastes", owner.unwrap_or_default()))?;
    let res = if let Some(s) = session {
        client.get(url).bearer_auth(s.key())
    } else {
        client.get(url)
    }
    .send()
    .await?;

    if res.status() == StatusCode::UNAUTHORIZED {
        return Err(anyhow!("You must either specify which user to show or be authenticated. Try specifying an user or authenticating with `pmis auth`"));
    }

    let pastes: Vec<Paste> = res.error_for_status()?.json().await?;

    if !ids_only {
        if pastes.is_empty() {
            println!("No pastes found");
        }
        for paste in pastes.into_iter() {
            let title = paste.title.as_deref().unwrap_or("Untitled");
            let time = HumanTime::from(paste.creation);
            let visibility = if paste.unlisted { "Unlisted" } else { "Public" };

            println!("{} - {} // {} - {}", paste.id, title, time, visibility);
        }
    } else {
        for paste in pastes.into_iter() {
            println!("{}", paste.id);
        }
    }

    Ok(())
}
