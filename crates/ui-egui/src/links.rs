//! Community and project links.

pub const APP: &str = "tslight";
pub const WEBSITE: &str = "";
pub const APP_PAGE: &str = "";
pub const GITHUB: &str = "";
pub const HELP: &str = "";
pub const FEEDBACK: &str = "";
pub const DISCORD: &str = "";

pub const LINKS: &[(&str, &str, &str)] = &[
    ("app.help", "TSLight Help", HELP),
    ("app.feedback", "Send Feedback…", FEEDBACK),
];

pub fn url_of(cmd: &str) -> Option<&'static str> {
    LINKS.iter().find(|(id, _, _)| *id == cmd).map(|(_, _, u)| *u)
}

pub fn open(app: &mut crate::LightcraftApp, url: &str) -> Result<serde_json::Value, String> {
    if url.is_empty() {
        return Ok(serde_json::json!({ "url": "" }));
    }
    let open = app.services.open_url.as_mut().ok_or("can't open links here")?;
    open(url)?;
    Ok(serde_json::json!({ "url": url }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_structure() {
        assert_eq!(APP, "tslight");
    }
}
