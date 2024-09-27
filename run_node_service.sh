#!/bin/bash

rm ~/.config/linera/wallet.* -rf
linera wallet init --faucet http://localhost:10080
linera wallet show

function run_service () {
  linera service --port 30080 --external-signing true
  if [ ! $? -eq 0 ]; then
    echo "Run with official release"
    linera service --port 30080
  fi
}

run_service &
sleep 10
socat TCP4-LISTEN:30081 TCP4:localhost:30080 &

sleep 1000000
