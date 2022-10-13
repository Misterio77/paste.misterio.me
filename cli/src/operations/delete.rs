use crate::{Client, Result, Session, Url, Uuid};
use anyhow::anyhow;
use reqwest::StatusCode;

pub async fn delete(api: Url, id: Uuid) -> Result<()> {
    let session = Session::load()?;
    let client = Client::new();

    let url = api.join(&format!("/p/{}", id))?;
    let res = client.delete(url).bearer_auth(session.key()).send().await?;

    if res.status() == StatusCode::NOT_FOUND {
        return Err(anyhow!("The requested paste does not exist"));
    } else if res.status() == StatusCode::FORBIDDEN {
        return Err(anyhow!("You can only delete your own pastes"));
    } else if res.status() == StatusCode::UNAUTHORIZED {
        return Err(anyhow!(
            "Your API key is invalid or has been revoked. Try `pmis auth`"
        ));
    }
    res.error_for_status()?;

    println!("Paste {} successfully deleted.", id);

    Ok(())
}
