use anyhow::{Context, Result};
use std::env::var;

pub fn get_twitter_client_id() -> Result<String> {
    Ok(var("TWITTER_CLIENT_ID").with_context(|| {
        "Please export the TWITTER_CLIENT_ID variable, which is provided by the Twitter API"
    })?)
}

pub fn get_twitter_client_secret() -> Result<String> {
    Ok(var("TWITTER_CLIENT_SECRET").with_context(|| {
        "Please export the TWITTER_CLIENT_SECRET variable, which is provided by the Twitter API"
    })?)
}

pub fn get_twitter_redirect_url() -> Result<String> {
    Ok(var("TWITTER_REDIRECT_URL").with_context(|| "Please export the TWITTER_REDIRECT_URL variable, which you must set in the Twitter API platform.")?)
}

pub fn get_twitter_query() -> Result<String> {
    Ok(var("TWITTER_QUERY").with_context(|| "Please export the TWITTER_QUERY variable, which is the search term for Tweets you want to retweet.")?)
}
