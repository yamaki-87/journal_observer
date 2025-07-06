#!/bin/bash

messages=(
    "This is a test message."
    "System is running smoothly."
    "No issues detected."
    "Warning: Disk usage high."
    "Alert: CPU temperature rising."
    "Heartbeat OK."
    "New connection established."
    "Error: Failed to load module."
    "All systems nominal."
    "Debug: Checking service state."
)

while true; do
    # ランダムに1つ選択
    msg=${messages[$RANDOM % ${#messages[@]}]}
    echo "[Test Service] $(date +%Y-%m-%dT%H:%M:%S) - $msg"
    sleep 1
done