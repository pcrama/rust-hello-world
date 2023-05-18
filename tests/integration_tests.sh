#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail

readonly SCRIPT_FULL_PATH="$(realpath "$0")"
readonly PROJECT_DIR="$(dirname "$(dirname "$SCRIPT_FULL_PATH")")"

(command -V cargo 2> /dev/null > /dev/null) || source "${HOME}/.cargo/env"

cargo build
cargo test

(env RUST_BACKTRACE=1 "$PROJECT_DIR/target/debug/hello_world") &

readonly SERVER_PID=$!

trap "kill $SERVER_PID" EXIT

function get_url_expect_200 () {
    url="http://localhost:3000/hello-rust${1}"
    expected_text="${2}200"
    result="$(curl --silent --connect-timeout 1 "$url" --write-out '%{http_code}')"
    if [ "$result" == "$expected_text" ]; then
        echo "ok: GET $url" > /dev/stderr
    else
        echo "KO: GET $url -> $result" > /dev/stderr
        exit 1
    fi
}

get_url_expect_200 "/" "Hello, World!"
get_url_expect_200 "/1/p" '<!DOCTYPE html>
<html>
  <head>
    <title>Greeting</title>
    <link rel="apple-touch-icon" sizes="180x180" href="/hello-rust/assets/apple-touch-icon.png">
    <link rel="icon" type="image/png" sizes="32x32" href="/hello-rust/assets/favicon-32x32.png">
    <link rel="icon" type="image/png" sizes="16x16" href="/hello-rust/assets/favicon-16x16.png">
    <link rel="manifest" href="/hello-rust/assets/site.webmanifest">
  </head>
  <body>
    <h1>Hello, p!</h1>
    <p>Your user ID is 1.</p>
  </body>
</html>
'
