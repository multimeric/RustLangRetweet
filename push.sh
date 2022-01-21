#!/usr/bin/env bash
docker run \
  --platform linux/amd64 \
  --rm \
  --user "$(id -u)":"$(id -g)" \
  -v "${PWD}":/usr/src/myapp \
   -w /usr/src/myapp rust:latest \
  cargo build --release --target x86_64-unknown-linux-gnu
rm bundle.zip
zip bundle.zip bootstrap
aws lambda create-function --function-name retweet-bot \
  --handler doesnt.matter \
  --zip-file fileb://bundle.zip \
  --runtime provided.al2 \
  --role "${AWS_EXECUTION_ROLE}" \
  --environment 'Variables={RUST_BACKTRACE=1}' \
  --tracing-config Mode=Active || aws lambda update-function-code --function-name retweet-bot --zip-file fileb://bundle.zip