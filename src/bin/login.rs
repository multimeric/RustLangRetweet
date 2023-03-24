use anyhow::Result;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, Scope, TokenResponse};

use rust_lang_retweet::auth::client_from_env;
use rust_lang_retweet::dynamo::update_database;
use rust_lang_retweet::environment::get_twitter_client_id;

/// Executes the auth flow for a
pub(crate) async fn get_token(client: BasicClient) -> Result<String> {
    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("tweet.write".into()))
        .add_scope(Scope::new("tweet.read".into()))
        .add_scope(Scope::new("users.read".into()))
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

    // Once the user has been redirected to the redirect URL, you'll have access to the
    // authorization code. For security reasons, your code should verify that the `state`
    // parameter returned by the server matches `csrf_state`.

    let client_id = get_twitter_client_id()?;
    // Now you can trade it for an access token.
    let req = client
        .exchange_code(AuthorizationCode::new(line))
        .add_extra_param("client_id", client_id)
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce_verifier);
    dbg!(&req);
    let token = req.request_async(async_http_client)
        .await
        .map_err(|err| anyhow::Error::msg(format!("{:?}", err)))?;

    Ok(token
        .refresh_token()
        .ok_or(anyhow::Error::msg("No refresh token provided"))?
        .secret()
        .to_owned())
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let client = client_from_env()?;
    let token = get_token(client).await?;
    update_database(token).await?;
    Ok(())
}
