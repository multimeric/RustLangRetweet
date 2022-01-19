use anyhow;
use oauth2::{
    AuthorizationCode,
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    PkceCodeChallenge,
    RedirectUrl,
    Scope,
    TokenResponse,
    TokenUrl,
};
use oauth2::basic::{BasicClient, BasicTokenResponse};
use oauth2::reqwest::{async_http_client, http_client};
use std::env::var;
use anyhow::Result;
use oauth2::AuthType::BasicAuth;
use crate::config::load_config;

pub(crate) async fn get_token(config: crate::config::Config) -> Result<BasicTokenResponse> {
    let client =
        BasicClient::new(
            ClientId::new(config.client_id.clone()),
            Some(ClientSecret::new(config.client_secret)),
            AuthUrl::new("https://twitter.com/i/oauth2/authorize".into())?,
            Some(TokenUrl::new("https://api.twitter.com/2/oauth2/token".into())?),
        )
            // Set the URL the user will be redirected to after the authorization process.
            .set_redirect_uri(RedirectUrl::new(config.redirect_url)?).set_auth_type(BasicAuth);

// Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

// Generate the full authorization URL.
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("tweet.write".into()))
        .add_scope(Scope::new("offline.access".into()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

// This is the URL you should redirect the user to, in order to trigger the authorization
// process.
    println!("Browse to: {}", auth_url);
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    line = line.trim().into();
    dbg!(&line);

// Once the user has been redirected to the redirect URL, you'll have access to the
// authorization code. For security reasons, your code should verify that the `state`
// parameter returned by the server matches `csrf_state`.

// Now you can trade it for an access token.
    Ok(client
        .exchange_code(AuthorizationCode::new(line))
        .add_extra_param("client_id", config.client_id)
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client).await?
    )
}

#[test]
fn test_auth() {
    let cfg = load_config("config.json").unwrap();
    let token = get_token(cfg).unwrap();
}