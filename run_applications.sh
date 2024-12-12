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
unset TMPDIR
cargo build --release --target wasm32-unknown-unknown

PROJECT_ROOT=$HOME/linera-project
WALLET_BASE=$PROJECT_ROOT/linera/respeer

mkdir -p $PROJECT_ROOT
SERVICE_LOG_FILE=$PROJECT_ROOT/service_8080.log

function _run_service() {
  linera --max-retries 100 --retry-delay-ms 10 --with-wallet $1 service --port $2 > $LOG_FILE 2>&1
}

function run_new_service() {
  BASE_PORT=9080
  port=`expr $BASE_PORT + $1`

  export LINERA_WALLET_$1=$WALLET_BASE/wallet_$1.json
  export LINERA_STORAGE_$1=rocksdb:$WALLET_BASE/client_$1.db

  print $'\U01f499' $LIGHTGREEN " Wallet of $port service ..."
  linera --with-wallet $1 wallet show
  print $'\U01f499' $LIGHTGREEN " Run $port service ..."
  LOG_FILE=`echo $SERVICE_LOG_FILE | sed "s/8080/$port/g"`
  _run_service $1 $port &
  BASE_CAT_PORT=20201
  cat_port=`expr $BASE_CAT_PORT + $1`
  socat TCP4-LISTEN:$cat_port TCP4:localhost:$port &
}

run_new_service 1

trap cleanup INT
read -p "  Press any key to exit"
print $'\U01f499' $LIGHTGREEN " Exit ..."

cleanup

