#!/bin/bash

function cleanup() {
  kill -15 `ps | grep linera-proxy | awk '{print $1}'` > /dev/null 2>&1
  kill -15 `ps | grep linera-server | awk '{print $1}'` > /dev/null 2>&1
  kill -15 `ps | grep linera | awk '{print $1}'` > /dev/null 2>&1
  kill -9 `ps | grep socat | awk '{print $1}'` > /dev/null 2>&1
}

cleanup

BLUE='\033[1;34m'
YELLOW='\033[1;33m'
LIGHTGREEN='\033[1;32m'
RED='\033[1;31m'
NC='\033[0m'

function print() {
  echo -e $1$2$3$NC
}

unset RUSTFLAGS
export TMPDIR=
cargo build --release --target wasm32-unknown-unknown

PROJECT_ROOT=$HOME/linera-project

mkdir -p $PROJECT_ROOT
NODE_LOG_FILE=$PROJECT_ROOT/linera.log
FAUCET_LOG_FILE=$PROJECT_ROOT/faucet.log

linera --version > /dev/null 2>&1
if [ ! $? -eq 0 ]; then
  echo
  print $'\U01F4AB' $RED " Please install linera toolchain firstly..."
  print $'\U01F4AB' $LIGHTGREEN "   cargo install --path linera-service --features storage-service"
  print $'\U01F4AB' $LIGHTGREEN "   cargo install --path linera-storage-service --features storage-service"
  echo
  exit 1
fi

print $'\U01F4AB' $YELLOW " Running linera net, log in $NODE_LOG_FILE ..."
linera net up --initial-amount 100000000 2>&1 | sh -c 'exec cat' > $NODE_LOG_FILE &

function wait_for_genesis() {
  while true; do
    [ ! -f $NODE_LOG_FILE ] && sleep 3 && continue
    LINERA_WALLET_ENV=`grep "export LINERA_WALLET" $NODE_LOG_FILE | sed 's/"//g'`
    LINERA_STORAGE_ENV=`grep "export LINERA_STORAGE" $NODE_LOG_FILE | sed 's/"//g'`
    print $'\U01F411' $LIGHTGREEN " Waiting linera net ..."
    [ -z "$LINERA_WALLET_ENV" -o -z "$LINERA_STORAGE_ENV" ] && sleep 3 && continue
    print $'\U01F411' $LIGHTGREEN " Linera net up ..."
    break
  done

  print $'\U01F4AB' $YELLOW " $LINERA_WALLET_ENV"
  $LINERA_WALLET_ENV
  print $'\U01F4AB' $YELLOW " $LINERA_STORAGE_ENV"
  $LINERA_STORAGE_ENV

  while true; do
    LINERA_WALLET_NAME="LINERA_WALLET"
    print $'\U01F411' $LIGHTGREEN " Waiting linera database `dirname ${!LINERA_WALLET_NAME}` ..."
    [ ! -f ${!LINERA_WALLET_NAME} ] && sleep 3 && continue
    break
  done
}

wait_for_genesis

linera wallet show
print $'\U01f499' $LIGHTGREEN " Wallet of faucet ... http://localhost:40080"
linera faucet --port 40080 --amount "100.0" e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65 > $FAUCET_LOG_FILE 2>&1 &

trap cleanup INT
read -p "  Press any key to exit"
print $'\U01f499' $LIGHTGREEN " Exit ..."

cleanup

