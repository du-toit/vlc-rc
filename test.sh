#!/usr/bin/bash

# The testing address we use to host VLC's built-in server.
export TEST_ADDR=127.0.0.1:9090

# Path to the audio sample.
SAMPLE=samples/audio.mp3

# Runs the given unit test with cargo.
test() {
    cargo t $1;
}

# Start VLC without a GUI and play the audio sample - piping away all VLC's output.
cvlc --rc-host $TEST_ADDR $SAMPLE &> /dev/null &

wait &

# Run the doc tests.
cargo t --doc;

# Run the parsing tests.
test "track_from_parts_none";
test "track_from_parts_some";
test "subtitle_from_parts_none";
test "subtitle_from_parts_some";

# Run the client tests.
test "get_and_set_volume";
test "play_and_stop";
test "forward";
test "rewind";

# Kill the VLC background process.
trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT
