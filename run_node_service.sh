#!/bin/bash

rm ~/.config/linera/wallet.* -rf

options="N:"

while getopts $options opt; do
  case ${opt} in
    N) NETWORK_TYPE=${OPTARG} ;;
  esac
done

case $NETWORK_TYPE in
  localnet)
    faucet_url=http://localhost:40080
    ;;
  testnet-archimedes)
    faucet_url=https://faucet.testnet-archimedes.linera.net
    ;;
  devnet|*)
    faucet_url=https://faucet.devnet-2024-09-04.linera.net
    ;;
esac

linera wallet init --faucet $faucet_url
linera wallet show

function run_service () {
  linera --max-retries 100 --retry-delay-ms 10 service --port 30080 --listener-skip-process-inbox
}

run_service &
sleep 10
socat TCP4-LISTEN:30081 TCP4:localhost:30080 &

while true; do
  sleep 1000000
done

