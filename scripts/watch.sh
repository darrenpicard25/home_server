#!/bin/bash

# npx tailwindcss -i ./input.css -o ./style/output.css --watch &&
#     cargo leptos watch

# Run the first command in the background
npx tailwindcss -i ./input.css -o ./style/output.css --watch &

# Get the process ID of the first command
pid=$!

# Wait for the first command to finish
wait $pid

# Run the second command
cargo leptos watch
