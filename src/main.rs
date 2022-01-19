mod auth;
mod config;

use anyhow::Result;
use chrono::prelude::*;
use egg_mode::search::{ResultType, SearchResult};
use egg_mode::{search, Response, Token};
use lambda_runtime::{handler_fn, Context, Error};
use serde_json::{json, Value};
use std::future::Future;
use std::io;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct TwitterSearchResponse {
    data: Vec<Tweet>,
    meta: TwitterMeta
}

#[derive(Deserialize, Debug)]
struct TwitterMeta {
    newest_id: String,
    oldest_id: String,
    result_count: usize,
    next_token: Option<String>
}

#[derive(Deserialize, Debug)]
struct Tweet{
    id: String,
    text: String
}

// trait TweetUrl {
//     fn get_url(&self) -> Result<String>;
// }
//
// impl TweetUrl for Tweet {
//     fn get_url(&self) -> Result<String> {
//         let user = self.user.as_ref().ok_or(anyhow::Error::msg("No user!"))?;
//         Ok(format!(
//             "https://twitter.com/{screen_name}/status/{id}",
//             screen_name = user.screen_name,
//             id = self.id
//         ))
//     }
// }

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // let func = handler_fn(func);
    // let result = lambda_runtime::run(func).await?;
    // Ok(())
    let cfg = crate::config::load_config("config.json").unwrap();
    let token = crate::auth::get_token(cfg).await.unwrap();
    dbg!(&token);
    Ok(())
}

async fn func(event: Value, _: Context) -> Result<Value> {
    Ok(json!({ "message": "Foo" }))
}


async fn scrape_tweets(time: chrono::Duration, token: Token) -> Result<()> {
    // let now = Utc::now();
    // let earliest = now - time;
    // let mut results = Vec::<Tweet>::new();
    //
    // let client = reqwest::Client::new();
    // let res = client
    //     .get("https://api.twitter.com/2/tweets/search/recent")
    //     .query(&[
    //         ("query", "#rustlang -is:retweet"),
    //         ("start_time", &earliest.to_rfc3339()),
    //     ])
    //     .bearer_auth(token)
    //     .send()
    //     .await?
    //     .json::<TwitterSearchResponse>()
    //     .await?;
    // dbg!(&res);

    Ok(())


    //
    // let body = reqwest::get("https://api.twitter.com/2/tweets/search/recent")
    //     .await?
    //     .text()
    //     .await?;
    //
    // // We are using the API v1 here, so we use the old search operators:
    // // https://developer.twitter.com/en/docs/twitter-api/v1/rules-and-filtering/search-operators
    // let mut search = search::search("#rustlang -filter:retweets")
    //     .result_type(ResultType::Recent)
    //     .call(&token)
    //     .await?;

    // We loop forever until either there are no more tweets, or we find a tweet that is too old
    // loop {
    //     for tweet in search.response.statuses.iter() {
    //         if tweet.created_at < earliest {
    //             return Ok(results);
    //         }
    //         results.push(tweet.clone());
    //     }
    //     search = match search.response.older(&token).await {
    //         Err(e) => return Ok(results),
    //         Ok(ok) => ok,
    //     }
    // }
}

mod test {
    use super::*;
    use chrono::Duration;

    #[tokio::test]
    async fn test_login() {
        // login().await.unwrap();
    }

    #[tokio::test]
    async fn test_scrape() {
        // let token = login().await.unwrap();
        // let tweets = scrape_tweets(Duration::minutes(360), token).await.unwrap();
        // let urls: Result<Vec<_>> = tweets
        //     .iter()
        //     .map(|tweet| -> Result<String> { tweet.get_url() })
        //     .collect();
        // dbg!(urls);
        // assert!(!tweets.is_empty());
    }
}
