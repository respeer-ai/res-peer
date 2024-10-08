#!/bin/bash

service_pids=`ps -ef | grep "linera " | grep -v "amount\|net" | awk '{print $2}'`
kill -15 $service_pids > /dev/null 2>&1

BLUE='\033[1;34m'
YELLOW='\033[1;33m'
LIGHTGREEN='\033[1;32m'
NC='\033[0m'

function print() {
  echo -e $1$2$3$NC
}

options="f:"
faucet_url=https://faucet.devnet-2024-05-07.linera.net

while getopts $options opt; do
  case ${opt} in
    f) faucet_url=${OPTARG} ;;
  esac
done

PROJECT_ROOT=/home/test
NODE_LOG_FILE=$PROJECT_ROOT/linera-project/linera.log
SERVICE_LOG_FILE=$PROJECT_ROOT/linera-project/service_8080.log
WALLET_NUMBER=4
EXTRA_WALLET_NUMBER=`expr $WALLET_NUMBER - 1`
WALLET_BASEDIR=$PROJECT_ROOT/.config/linera

print $'\U01F4AB' $YELLOW " Running lienra net, log in $NODE_LOG_FILE ..."

for i in `seq 0 $EXTRA_WALLET_NUMBER`; do
  print $'\U01f499' $LIGHTGREEN " Initialize wallet $i"
  export LINERA_WALLET_$i="$WALLET_BASEDIR/wallet_$i.json"
  export LINERA_STORAGE_$i="rocksdb:$WALLET_BASEDIR/wallet_$i.db"
  linera --with-wallet $i wallet init --with-new-chain --faucet $faucet_url
done

print $'\U01F4AB' $YELLOW " Deploying Credit bytecode ..."
credit_bid=`linera --with-wallet 0 publish-bytecode ./target/wasm32-unknown-unknown/release/credit_{contract,service}.wasm`
print $'\U01F4AB' $YELLOW " Creating Credit application ..."
credit_appid=`linera --with-wallet 0 create-application $credit_bid --json-argument '{"initial_supply":"99999999999999.0","amount_alive_ms":600000}'`
print $'\U01f499' $LIGHTGREEN " Credit application deployed"
echo -e "    Bytecode ID:    $BLUE$credit_bid$NC"
echo -e "    Application ID: $BLUE$credit_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Foundation application ..."
foundation_bid=`linera --with-wallet 0 publish-bytecode ./target/wasm32-unknown-unknown/release/foundation_{contract,service}.wasm`
foundation_appid=`linera --with-wallet 0 create-application $foundation_bid --json-argument '{"review_reward_percent":20,"review_reward_factor":20,"author_reward_percent":40,"author_reward_factor":20,"activity_reward_percent":10}'`
print $'\U01f499' $LIGHTGREEN " Foundation application deployed"
echo -e "    Bytecode ID:    $BLUE$foundation_bid$NC"
echo -e "    Application ID: $BLUE$foundation_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Feed application ..."
feed_bid=`linera --with-wallet 0 publish-bytecode ./target/wasm32-unknown-unknown/release/feed_{contract,service}.wasm`
feed_appid=`linera --with-wallet 0 create-application $feed_bid --json-argument '{"react_interval_ms":60000}' --json-parameters "{\"credit_app_id\":\"$credit_appid\",\"foundation_app_id\":\"$foundation_appid\"}" --required-application-ids $credit_appid --required-application-ids $foundation_appid`
print $'\U01f499' $LIGHTGREEN " Feed application deployed"
echo -e "    Bytecode ID:    $BLUE$feed_bid$NC"
echo -e "    Application ID: $BLUE$feed_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Market application ..."
market_bid=`linera --with-wallet 0 publish-bytecode ./target/wasm32-unknown-unknown/release/market_{contract,service}.wasm`
market_appid=`linera --with-wallet 0 create-application $market_bid --json-argument '{"credits_per_linera":"30","max_credits_percent":30,"trade_fee_percent":3}' --json-parameters "{\"credit_app_id\":\"$credit_appid\",\"foundation_app_id\":\"$foundation_appid\"}" --required-application-ids $credit_appid --required-application-ids $foundation_appid`
print $'\U01f499' $LIGHTGREEN " Market application deployed"
echo -e "    Bytecode ID:    $BLUE$market_bid$NC"
echo -e "    Application ID: $BLUE$market_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Review application ..."
review_bid=`linera --with-wallet 0 publish-bytecode ./target/wasm32-unknown-unknown/release/review_{contract,service}.wasm`
review_appid=`linera --with-wallet 0 create-application $review_bid --json-argument '{"content_approved_threshold":3,"content_rejected_threshold":2,"asset_approved_threshold":2,"asset_rejected_threshold":2,"reviewer_approved_threshold":2,"reviewer_rejected_threshold":2,"activity_approved_threshold":2,"activity_rejected_threshold":2}' --json-parameters "{\"feed_app_id\":\"$feed_appid\",\"credit_app_id\":\"$credit_appid\",\"foundation_app_id\":\"$foundation_appid\",\"market_app_id\":\"$market_appid\"}" --required-application-ids $feed_appid --required-application-ids $credit_appid --required-application-ids $foundation_appid --required-application-ids $market_appid`
print $'\U01f499' $LIGHTGREEN " Review application deployed"
echo -e "    Bytecode ID:    $BLUE$review_bid$NC"
echo -e "    Application ID: $BLUE$review_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Activity application ..."
activity_bid=`linera --with-wallet 0 publish-bytecode ./target/wasm32-unknown-unknown/release/activity_{contract,service}.wasm`
activity_appid=`linera --with-wallet 0 create-application $activity_bid --json-parameters "{\"review_app_id\":\"$review_appid\",\"foundation_app_id\":\"$foundation_appid\",\"feed_app_id\":\"$feed_appid\"}" --required-application-ids $review_appid --required-application-ids $foundation_appid --required-application-ids $feed_appid`
print $'\U01f499' $LIGHTGREEN " Activity application deployed"
echo -e "    Bytecode ID:    $BLUE$activity_bid$NC"
echo -e "    Application ID: $BLUE$activity_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying BlobGateway application ..."
blob_gateway_bid=`linera --with-wallet 0 publish-bytecode ./target/wasm32-unknown-unknown/release/blob_gateway_{contract,service}.wasm`
blob_gateway_appid=`linera --with-wallet 0 create-application $blob_gateway_bid`
print $'\U01f499' $LIGHTGREEN " BlobGateway application deployed"
echo -e "    Bytecode ID:    $BLUE$blob_gateway_bid$NC"
echo -e "    Application ID: $BLUE$blob_gateway_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying CPRegistry application ..."
cp_registry_bid=`linera --with-wallet 0 publish-bytecode ./target/wasm32-unknown-unknown/release/cp_registry_{contract,service}.wasm`
cp_registry_appid=`linera --with-wallet 0 create-application $cp_registry_bid`
print $'\U01f499' $LIGHTGREEN " CPRegistry application deployed"
echo -e "    Bytecode ID:    $BLUE$cp_registry_bid$NC"
echo -e "    Application ID: $BLUE$cp_registry_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Copilot CPU application ..."
copilot_cpu_bid=`linera --with-wallet 0 publish-bytecode ./target/wasm32-unknown-unknown/release/copilot_{contract,service}.wasm`
copilot_cpu_appid=`linera --with-wallet 0 create-application $copilot_cpu_bid --json-parameters "{\"cp_registry_app_id\":\"$cp_registry_appid\"}" --required-application-ids $cp_registry_appid`
print $'\U01f499' $LIGHTGREEN " Copilot CPU application deployed"
echo -e "    Bytecode ID:    $BLUE$copilot_cpu_bid$NC"
echo -e "    Application ID: $BLUE$copilot_cpu_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Copilot GPU application ..."
copilot_fetch_server_url="http://localhost:9071/?prompt="
copilot_gpu_bid=`linera --with-wallet 0 publish-bytecode ./target/wasm32-unknown-unknown/release/copilot_{contract,service}.wasm`
copilot_gpu_appid=`linera --with-wallet 0 create-application $copilot_gpu_bid --json-parameters "{\"cp_registry_app_id\":\"$cp_registry_appid\"}" --required-application-ids $cp_registry_appid`
print $'\U01f499' $LIGHTGREEN " Copilot GPU application deployed"
echo -e "    Bytecode ID:    $BLUE$copilot_gpu_bid$NC"
echo -e "    Application ID: $BLUE$copilot_gpu_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Illustrator CPU application ..."
illustrator_cpu_bid=`linera --with-wallet 0 publish-bytecode ./target/wasm32-unknown-unknown/release/illustrator_{contract,service}.wasm`
illustrator_cpu_appid=`linera --with-wallet 0 create-application $illustrator_cpu_bid --json-parameters "{\"cp_registry_app_id\":\"$cp_registry_appid\"}" --required-application-ids $cp_registry_appid`
print $'\U01f499' $LIGHTGREEN " Illustrator CPU application deployed"
echo -e "    Bytecode ID:    $BLUE$illustrator_cpu_bid$NC"
echo -e "    Application ID: $BLUE$illustrator_cpu_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Illustrator GPU application ..."
illustrator_fetch_server_url="http://localhost:9072/?prompt="
illustrator_gpu_bid=`linera --with-wallet 0 publish-bytecode ./target/wasm32-unknown-unknown/release/illustrator_{contract,service}.wasm`
illustrator_gpu_appid=`linera --with-wallet 0 create-application $illustrator_gpu_bid --json-parameters "{\"cp_registry_app_id\":\"$cp_registry_appid\"}" --required-application-ids $cp_registry_appid`
print $'\U01f499' $LIGHTGREEN " Illustrator GPU application deployed"
echo -e "    Bytecode ID:    $BLUE$illustrator_gpu_bid$NC"
echo -e "    Application ID: $BLUE$illustrator_gpu_appid$NC"


app_deploy_chain=`linera --with-wallet 1 wallet show | grep "Public Key" | awk '{print $2}'`
app_deploy_owner=`linera --with-wallet 1 wallet show | grep "Owner" | awk '{print $4}'`

sed -i "s/feedApp =.*/feedApp = '$feed_appid',/g" webui/src/const/index.ts
sed -i "s/creditApp =.*/creditApp = '$credit_appid',/g" webui/src/const/index.ts
sed -i "s/marketApp =.*/marketApp = '$market_appid',/g" webui/src/const/index.ts
sed -i "s/reviewApp =.*/reviewApp = '$review_appid',/g" webui/src/const/index.ts
sed -i "s/foundationApp =.*/foundationApp = '$foundation_appid'/g," webui/src/const/index.ts
sed -i "s/activityApp =.*/activityApp = '$activity_appid',/g" webui/src/const/index.ts
sed -i "s/blobGatewayApp =.*/blobGatewayApp = '$blob_gateway_appid',/g" webui/src/const/index.ts
sed -i "s/cpRegistryApp =.*/cpRegistryApp = '$cp_registry_appid',/g" webui/src/const/index.ts
sed -i "s/copilotCpuApp =.*/copilotCpuApp = '$copilot_cpu_appid',/g" webui/src/const/index.ts
sed -i "s/copilotGpuApp =.*/copilotGpuApp = '$copilot_gpu_appid',/g" webui/src/const/index.ts
sed -i "s/illustratorCpuApp =.*/illustratorCpuApp = '$illustrator_cpu_appid',/g" webui/src/const/index.ts
sed -i "s/illustratorGpuApp =.*/illustratorGpuApp = '$illustrator_gpu_appid'/g" webui/src/const/index.ts
sed -i "s/export const appDeployChain =.*/export const appDeployChain = '$app_deploy_chain'/g" webui/src/const/index.ts
sed -i "s/export const appDeployOwner =.*/export const appDeployOwner = '$app_deploy_owner'/g" webui/src/const/index.ts

function run_new_service() {
  BASE_PORT=9080
  port=`expr $BASE_PORT + $1`
  print $'\U01f499' $LIGHTGREEN " Wallet of $port service ..."
  linera --with-wallet $1 wallet show
  print $'\U01f499' $LIGHTGREEN " Run $port service ..."
  LOG_FILE=`echo $SERVICE_LOG_FILE | sed "s/8080/$port/g"`
  linera --with-wallet $1 service --external-signing false --port $port > $LOG_FILE 2>&1 &
}

for i in `seq 0 $EXTRA_WALLET_NUMBER`; do
  run_new_service $i
done

function cleanup() {
  service_pids=`ps -ef | grep "linera " | grep -v "amount\|net" | awk '{print $2}'`
  kill -15 $service_pids > /dev/null 2>&1
}

trap cleanup INT
read -p "  Press any key to exit"
print $'\U01f499' $LIGHTGREEN " Exit ..."

cleanup

