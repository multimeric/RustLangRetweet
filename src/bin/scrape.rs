use std::collections::HashMap;
use std::env::var;
use std::str::FromStr;

use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{AccessToken, RefreshToken, TokenResponse};
use serde::Deserialize;
use serde_json::{json, Value};

use rust_lang_retweet::auth::{client_from_env, get_twitter_client_id};
use rust_lang_retweet::dynamo::{get_key_name, get_table_name, update_database};

#[derive(Deserialize, Debug)]
struct TwitterApiResponse<DATA, META> {
    data: DATA,
    meta: Option<META>,
}

#[derive(Deserialize, Debug)]
struct User {
    id: String,
    name: String,
    username: String,
}

#[derive(Deserialize, Debug)]
struct Tweet {
    id: String,
    text: String,
}

#[derive(Deserialize, Debug)]
struct TwitterMeta {
    newest_id: Option<String>,
    oldest_id: Option<String>,
    result_count: Option<usize>,
    next_token: Option<String>,
}

pub(crate) async fn scrape_retweet(access_token: String, duration: Duration) -> Result<()> {
    let now = Utc::now();
    let earliest = now - duration;

    let http_client = reqwest::Client::new();
    let tok = AccessToken::new(access_token);
    let id = get_self(&http_client, &tok).await?;
    dbg!(&id);
    let tweets = scrape(&http_client, &tok, earliest).await?;
    retweet(id, &http_client, &tok, tweets).await?;

    Ok(())
}

async fn get_self(client: &reqwest::Client, token: &AccessToken) -> Result<String> {
    let user = client
        .get("https://api.twitter.com/2/users/me")
        .bearer_auth(token.secret())
        .send()
        .await?
        .json::<TwitterApiResponse<User, ()>>()
        .await?;
    Ok(user.data.id)
}

async fn scrape(
    client: &reqwest::Client,
    token: &AccessToken,
    earliest: DateTime<Utc>,
) -> Result<Vec<Tweet>> {
    let mut tweets = Vec::<Tweet>::new();
    let start_time = earliest.to_rfc3339();
    let mut query_params: HashMap<&str, String> = HashMap::from([
        ("query", "#rustlang -is:retweet".into()),
        ("start_time", start_time),
    ]);
    loop {
        let mut response = client
            .get("https://api.twitter.com/2/tweets/search/recent")
            .bearer_auth(token.secret())
            .query(
                &query_params
                    .iter()
                    .map(|(key, value)| (key.to_owned(), value.to_owned()))
                    .collect::<Vec<(&str, String)>>(),
            )
            .send()
            .await?
            .json::<TwitterApiResponse<Vec<Tweet>, TwitterMeta>>()
            .await?;
        tweets.append(&mut response.data);
        match response.meta {
            None => {
                // As soon as there are no more tweets, return
                return Ok(tweets);
            }
            Some(meta) => {
                match meta.next_token {
                    None => {
                        return Ok(tweets);
                    }
                    Some(token) => {
                        // If there are more tweets, rerun this using the new query
                        query_params.insert("next_token", token.clone());
                    }
                }
            }
        }
    }
}

async fn retweet(
    user_id: String,
    client: &reqwest::Client,
    token: &AccessToken,
    tweets: Vec<Tweet>,
) -> Result<()> {
    for tweet in tweets {
        let id = tweet.id;
        client
            .post(format!(
                "https://api.twitter.com/2/users/{user_id}/retweets"
            ))
            .json(&json!({ "tweet_id": id }))
            .bearer_auth(token.secret())
            .send()
            .await?;
    }
    Ok(())
}

async fn get_refresh_token() -> Result<String> {
    let client = rust_lang_retweet::dynamo::get_client().await;
    let ret = client
        .get_item()
        .table_name(get_table_name()?)
        .key("id", get_key_name())
        .send()
        .await?;
    Ok(ret
        .item
        .ok_or(anyhow::Error::msg("Failed to download key"))?
        .get("value")
        .ok_or(anyhow::Error::msg(
            "Refresh token was not present in the database",
        ))?
        .as_s()
        .map_err(|_e| anyhow::Error::msg("The refresh token was not a string type"))?
        .to_owned())
}

/// Assuming there is an existing refresh token in the database, updates the refresh token and
/// returns an access token
async fn get_access_token() -> Result<String> {
    let oauth_client = client_from_env()?;
    let refresh = get_refresh_token().await?;

    let res = oauth_client
        .exchange_refresh_token(&RefreshToken::new(refresh))
        .add_extra_param("client_id", get_twitter_client_id()?)
        .request_async(async_http_client)
        .await
        .map_err(|err| anyhow::Error::msg(format!("{:?}", err)))?;

    // We may be given a new refresh token, in which case we must store it
    match res.refresh_token() {
        None => {}
        Some(token) => {
            update_database(token.secret().to_owned()).await?;
        }
    }

    Ok(res.access_token().secret().to_owned())
}

#[tokio::main]
async fn main() -> std::result::Result<(), lambda_runtime::Error> {
    let func = lambda_runtime::handler_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(
    _event: Value,
    _ctx: lambda_runtime::Context,
) -> std::result::Result<Value, lambda_runtime::Error> {
    main_tokio().await?;
    Ok(json!(true))
}

async fn main_tokio() -> Result<()> {
    let access = get_access_token().await?;
    scrape_retweet(
        access,
        Duration::minutes(i64::from_str(
            &var("TWITTER_SCRAPE_INTERVAL").with_context(|| "Please include the TWITTER_SCRAPE_INTERVAL variable, which should be a number indicating the number of minutes between executions")?,
        )?),
    )
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_scrape() {
    // Test only the scraping part
    let earliest = Utc::now() - Duration::days(1);
    let access = get_access_token().await.unwrap();
    let http_client = reqwest::Client::new();
    let tweets = scrape(&http_client, &AccessToken::new(access), earliest)
        .await
        .unwrap();
    dbg!(&tweets);
    assert!(tweets.len() > 0)
}

#[tokio::test]
async fn test_self() {
    let http_client = reqwest::Client::new();
    let access = get_access_token().await.unwrap();
    let id = get_self(&http_client, &AccessToken::new(access))
        .await
        .unwrap();
    assert!(id.len() > 0);
}

#[tokio::test]
async fn test_all() {
    // Tests everything, including retweeting. Use with caution.
    main_tokio().await.unwrap();
}
