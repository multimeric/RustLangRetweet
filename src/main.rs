use lambda_runtime::{handler_fn, Context, Error};
use serde_json::{json, Value};
use anyhow::Result;
use std::env::var;
use std::future::Future;
use std::io;
use egg_mode::{Response, search, Token};
use egg_mode::search::{ResultType, SearchResult};
use egg_mode::tweet::Tweet;
use chrono::prelude::*;

trait TweetUrl {
    fn get_url(&self) -> Result<String>;
}

impl TweetUrl for Tweet {
    fn get_url(&self) -> Result<String> {
        let user = self.user.as_ref().ok_or(anyhow::Error::msg("No user!"))?;
        Ok(format!("https://twitter.com/{screen_name}/status/{id}", screen_name = user.screen_name, id = self.id))
    }
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let func = handler_fn(func);
    let result = lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: Value, _: Context) -> Result<Value> {
    Ok(json!({ "message": "Foo" }))
}

async fn login() -> anyhow::Result<Token> {
    // Get the API token from the environment
    let con_token = egg_mode::KeyPair::new(var("TWITTER_API_KEY")?, var("TWITTER_API_SECRET")?);
    let request_token = egg_mode::auth::request_token(&con_token, "oob").await.unwrap();

    // Get the user to 2FA
    Ok(egg_mode::auth::bearer_token(&con_token).await?)
}


async fn scrape_tweets(time: chrono::Duration, token: Token) -> Result<Vec<Tweet>> {
    let now = Utc::now();
    let earliest = now - time;
    let mut results = Vec::<Tweet>::new();

    // We are using the API v1 here, so we use the old search operators:
    // https://developer.twitter.com/en/docs/twitter-api/v1/rules-and-filtering/search-operators
    let mut search = search::search("#rustlang -filter:retweets")
        .result_type(ResultType::Recent)
        .call(&token)
        .await?;

    // We loop forever until either there are no more tweets, or we find a tweet that is too old
    loop {
        for tweet in search.response.statuses.iter() {
            if tweet.created_at < earliest {
                return Ok(results);
            }
            results.push(tweet.clone());
        }
        search = match search.response.older(&token).await {
            Err(e) => return Ok(results),
            Ok(ok) => ok
        }
    }
}

mod test {
    use chrono::Duration;
    use super::*;

    #[tokio::test]
    async fn test_login() {
        login().await.unwrap();
    }


    #[tokio::test]
    async fn test_scrape() {
        let token = login().await.unwrap();
        let tweets = scrape_tweets(Duration::minutes(120), token).await.unwrap();
        let urls: Result<Vec<_>> = tweets.iter().map(|tweet| -> Result<String> {
            tweet.get_url()
        }).collect();
        dbg!(urls);
        assert!(!tweets.is_empty());
    }
}