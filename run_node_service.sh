#!/bin/bash

rm ~/.config/linera/wallet.* -rf
linera wallet init --faucet http://localhost:8080 --with-new-chain
linera wallet show

function run_service () {
  linera service --port 30080 --external-signing false
  [ ! $? -eq 0 ] && linera service --port 30080
}

run_service &
sleep 10
socat TCP4-LISTEN:30081 TCP4:localhost:30080 &

sleep 1000000
