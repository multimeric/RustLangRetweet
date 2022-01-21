use crate::environment::{
    get_twitter_client_id, get_twitter_client_secret, get_twitter_redirect_url,
};
use anyhow;
use oauth2::basic::BasicClient;

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
