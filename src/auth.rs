use anyhow;
use anyhow::{Context, Result};
use oauth2::basic::BasicClient;
use std::env::var;

pub fn get_twitter_client_id() -> Result<String> {
    Ok(var("TWITTER_CLIENT_ID").map_err(|_e| {
        anyhow::Error::msg(
            "Please include the TWITTER_CLIENT_ID variable, which is provided by the Twitter API",
        )
    })?)
}

pub fn get_twitter_client_secret() -> Result<String> {
    Ok(var("TWITTER_CLIENT_SECRET").with_context(|| {
        "Please include the TWITTER_CLIENT_SECRET variable, which is provided by the Twitter API"
    })?)
}

pub fn get_twitter_redirect_url() -> Result<String> {
    Ok(var("TWITTER_REDIRECT_URL").with_context(|| "Please include the TWITTER_REDIRECT_URL variable, which you must set in the Twitter API platform.")?)
}

/// Gets a Twitter OAuth client
pub fn client_from_env() -> anyhow::Result<BasicClient> {
    Ok(oauth2::basic::BasicClient::new(
        oauth2::ClientId::new(get_twitter_client_id()?),
        Some(oauth2::ClientSecret::new(get_twitter_client_secret()?)),
        oauth2::AuthUrl::new("https://twitter.com/i/oauth2/authorize".into())?,
        Some(oauth2::TokenUrl::new(
            "https://api.twitter.com/2/oauth2/token".into(),
        )?),
    )
    .set_redirect_uri(oauth2::RedirectUrl::new(get_twitter_redirect_url()?)?)
    .set_auth_type(oauth2::AuthType::BasicAuth))
}

// #[test]
// fn test_auth() {
//     let cfg = load_config("config.json").unwrap();
//     let token = get_token(cfg).unwrap();
// }
