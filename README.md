# Rust Retweet Bot

This repo contains a rust project that can be used to create a Twitter retweet bot in AWS, using pure Rust.
The bot uses AWS Lambda and DynamoDB, so will likely cost you nothing.
The steps below will show you how to set up the bot.

## Setting it Up

### Twitter Setup
* Go to the [Twitter Developer Portal](https://developer.twitter.com/) and set up a new project
* Generate an OAuth 2.0 token, and note down the Client ID and Client Secret
* Do the same for the redirect URL. Note that you **do not** need it to point to a website you control. Feel free to use `localhost` or <https://en.wikipedia.org/>
* Export these variables for later (see below)
* Make a new Twitter user, and sign up for it the normal way. 
* Ensure you currently have this new Twitter user selected in Twitter

| Env Variable            | Value                     |
|-------------------------|---------------------------|
| `TWITTER_CLIENT_ID`     | Twitter app client ID     |
| `TWITTER_CLIENT_SECRET` | Twitter app client secret |
| `TWITTER_REDIRECT_URL`  | Twitter app redirect URL  |

### AWS Setup
* Make sure you have an AWS account, and you have a profile setup for that account
* Make a new DynamoDB table with the default settings, and store the name under `export AWS_TABLE_NAME=XXX`
* Create a new Role with Lambda execution and DynamoDB write permissions.
* Export the below variables:

| Env Variable     | Value                                    |
|------------------|------------------------------------------|
| `AWS_PROFILE`    | AWS Profile                              |
| `AWS_TABLE_NAME` | DynamoDB table name                      |
| `AWS_EXECUTION_ROLE` | ARN of the execution role for the lambda |

### Rust Setup

* First [make sure Rust is installed](https://www.rust-lang.org/tools/install)
* Clone the repo and `cd` into it

### Compile and Build Lambda

* Run `./push.sh`
  * Open the lambda in the AWS console, and set the following environment variables to the corresponding values on your local machine:
  * `AWS_TABLE_NAME`
  * `TWITTER_CLIENT_ID`
  * `TWITTER_CLIENT_SECRET`
  * `TWITTER_REDIRECT_URL`

### Authenticate Your Account

* Run `cargo run --bin login`
* The app will print out a Twitter URL, open this in your browser
* Approve the API request, and **make sure you are using your bot accout to do this** (see above)
* You will then be redirected to the redirect URL you have specified
* Inspect the URL, and extract the token which is the part of the URL after `code=`
* Paste the token into the console and press enter

### Test the Lambda
* Run `aws lambda invoke --function-name YOUR_LAMBDA_ARN - `
* Your account should then retweet the appropriate tweets

### Schedule the Lambda
* Open the lambda function in the console, and select "Add trigger"
* Add an EventBridge trigger, using a rate such as `rate(15 minutes)` to make it run regularly

## Technical Notes

### 

* Flow
  * User runs `login` locally
    * They are directed to the OAuth URL
    * The redirect URL is set to localhost
    * The redirect URL hits a webserver also run by `login`
    * This captures the refresh token, and inserts it into the dynamodb
  * Every 5 minutes, the `scrape` lambda is triggered
    * This uses the existing refresh token and requests an access token
    * Refreshes the refresh token in the db also
    * Scrapes tweets and retweets them