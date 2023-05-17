#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail

readonly SCRIPT_FULL_PATH="$(realpath "$0")"
readonly PROJECT_DIR="$(dirname "$(dirname "$SCRIPT_FULL_PATH")")"

(command -V cargo 2> /dev/null > /dev/null) || source "${HOME}/.cargo/env"

cargo build

"$PROJECT_DIR/target/debug/hello_world" &

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
get_url_expect_200 "/1/p" "Welcome p, user_id 1!"
