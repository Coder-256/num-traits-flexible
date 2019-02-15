#!/bin/sh

iif() {
    if [ "$1" -eq 0 ]; then
        echo "$2"
    else
        echo "$3"
    fi
}

run() {
    echo
    echo "Testing with Rust $1..."
    echo
    BUILD_RESULT=0

    rustup run "$1" cargo build --verbose
    BUILD_RESULT="$(iif "$?" "$BUILD_RESULT" 1)"

    rustup run "$1" cargo test --verbose
    BUILD_RESULT="$(iif "$?" "$BUILD_RESULT" 1)"

    rustup run "$1" cargo build --no-default-features --verbose
    BUILD_RESULT="$(iif "$?" "$BUILD_RESULT" 1)"

    rustup run "$1" cargo test --no-default-features --verbose
    BUILD_RESULT="$(iif "$?" "$BUILD_RESULT" 1)"

    return "$BUILD_RESULT"
}

run stable
STABLE_RESULT="$?"
run beta
BETA_RESULT="$?"
run nightly
NIGHTLY_RESULT="$?"

SUCCESS_STRING='\033[0;32mSUCCESS\033[0m'
FAIL_STRING='\033[0;31FAIL\033[0m'

echo
echo
echo "Result:"
echo "Stable : $(iif "$STABLE_RESULT" "$SUCCESS_STRING" "$FAIL_STRING")"
echo "Beta   : $(iif "$BETA_RESULT" "$SUCCESS_STRING" "$FAIL_STRING")"
echo "Nightly: $(iif "$NIGHTLY_RESULT" "$SUCCESS_STRING" "$FAIL_STRING")"

if [ "$STABLE_RESULT" -eq 0 ] && [ "$BETA_RESULT" -eq 0 ]; then
    exit 0
else
    exit 1
fi
