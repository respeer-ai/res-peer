#!/bin/bash

rm ~/.config/linera/wallet.* -rf
linera wallet init --faucet http://localhost:40080
linera wallet show

function run_service () {
  linera service --port 30080 --listener-skip-process-inbox
}

run_service &
sleep 10
socat TCP4-LISTEN:30081 TCP4:localhost:30080 &

sleep 1000000
