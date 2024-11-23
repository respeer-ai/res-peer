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
SERVICE_LOG_FILE=$PROJECT_ROOT/service_8080.log
FAUCET_LOG_FILE=$PROJECT_ROOT/faucet_8080.log
WALLET_NUMBER=5
EXTRA_WALLET_NUMBER=`expr $WALLET_NUMBER - 1`
SERVICE_WALLET_NUMBER=`expr $EXTRA_WALLET_NUMBER - 1`

print $'\U01F4AB' $YELLOW " Running linera net, log in $NODE_LOG_FILE ..."
lineradir=`whereis linera | awk '{print $2}'`
lineradir=`dirname $lineradir`
cd $lineradir
linera net up --initial-amount 100000000 --extra-wallets $EXTRA_WALLET_NUMBER 2>&1 | sh -c 'exec cat' > $NODE_LOG_FILE &
cd -

for i in `seq 0 $EXTRA_WALLET_NUMBER`; do
  while true; do
    [ ! -f $NODE_LOG_FILE ] && sleep 3 && continue
    LINERA_WALLET_ENV=`grep "export LINERA_WALLET_$i" $NODE_LOG_FILE | sed 's/"//g'`
    LINERA_STORAGE_ENV=`grep "export LINERA_STORAGE_$i" $NODE_LOG_FILE | sed 's/"//g'`
    print $'\U01F411' $LIGHTGREEN " Waiting linera net $i ..."
    [ -z "$LINERA_WALLET_ENV" -o -z "$LINERA_STORAGE_ENV" ] && sleep 3 && continue
    print $'\U01F411' $LIGHTGREEN " Linera net up $i ..."
    break
  done

  print $'\U01F4AB' $YELLOW " $LINERA_WALLET_ENV"
  $LINERA_WALLET_ENV
  print $'\U01F4AB' $YELLOW " $LINERA_STORAGE_ENV"
  $LINERA_STORAGE_ENV

  while true; do
    LINERA_WALLET_NAME="LINERA_WALLET_$i"
    print $'\U01F411' $LIGHTGREEN " Waiting linera database `dirname ${!LINERA_WALLET_NAME}` ..."
    [ ! -f ${!LINERA_WALLET_NAME} ] && sleep 3 && continue
    break
  done
done

print $'\U01F4AB' $YELLOW " Deploying Credit application ..."
credit_bid=`linera --with-wallet 1 publish-bytecode ./target/wasm32-unknown-unknown/release/credit_{contract,service}.wasm`
credit_appid=`linera --with-wallet 1 create-application $credit_bid --json-argument '{"initial_supply":"99999999999999.0","amount_alive_ms":600000}'`
print $'\U01f499' $LIGHTGREEN " Credit application deployed"
echo -e "    Bytecode ID:    $BLUE$credit_bid$NC"
echo -e "    Application ID: $BLUE$credit_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Foundation application ..."
foundation_bid=`linera --with-wallet 1 publish-bytecode ./target/wasm32-unknown-unknown/release/foundation_{contract,service}.wasm`
foundation_appid=`linera --with-wallet 1 create-application $foundation_bid --json-argument '{"review_reward_percent":20,"review_reward_factor":20,"author_reward_percent":40,"author_reward_factor":20,"activity_reward_percent":10}'`
print $'\U01f499' $LIGHTGREEN " Foundation application deployed"
echo -e "    Bytecode ID:    $BLUE$foundation_bid$NC"
echo -e "    Application ID: $BLUE$foundation_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Feed application ..."
feed_bid=`linera --with-wallet 1 publish-bytecode ./target/wasm32-unknown-unknown/release/feed_{contract,service}.wasm`
feed_appid=`linera --with-wallet 1 create-application $feed_bid --json-argument '{"react_interval_ms":60000}' --json-parameters "{\"credit_app_id\":\"$credit_appid\",\"foundation_app_id\":\"$foundation_appid\"}" --required-application-ids $credit_appid --required-application-ids $foundation_appid`
print $'\U01f499' $LIGHTGREEN " Feed application deployed"
echo -e "    Bytecode ID:    $BLUE$feed_bid$NC"
echo -e "    Application ID: $BLUE$feed_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Market application ..."
market_bid=`linera --with-wallet 1 publish-bytecode ./target/wasm32-unknown-unknown/release/market_{contract,service}.wasm`
market_appid=`linera --with-wallet 1 create-application $market_bid --json-argument '{"credits_per_linera":"30","max_credits_percent":30,"trade_fee_percent":3}' --json-parameters "{\"credit_app_id\":\"$credit_appid\",\"foundation_app_id\":\"$foundation_appid\"}" --required-application-ids $credit_appid --required-application-ids $foundation_appid`
print $'\U01f499' $LIGHTGREEN " Market application deployed"
echo -e "    Bytecode ID:    $BLUE$market_bid$NC"
echo -e "    Application ID: $BLUE$market_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Review application ..."
review_bid=`linera --with-wallet 1 publish-bytecode ./target/wasm32-unknown-unknown/release/review_{contract,service}.wasm`
review_appid=`linera --with-wallet 1 create-application $review_bid --json-argument '{"content_approved_threshold":3,"content_rejected_threshold":2,"asset_approved_threshold":2,"asset_rejected_threshold":2,"reviewer_approved_threshold":2,"reviewer_rejected_threshold":2,"activity_approved_threshold":2,"activity_rejected_threshold":2}' --json-parameters "{\"feed_app_id\":\"$feed_appid\",\"credit_app_id\":\"$credit_appid\",\"foundation_app_id\":\"$foundation_appid\",\"market_app_id\":\"$market_appid\"}" --required-application-ids $feed_appid --required-application-ids $credit_appid --required-application-ids $foundation_appid --required-application-ids $market_appid`
print $'\U01f499' $LIGHTGREEN " Review application deployed"
echo -e "    Bytecode ID:    $BLUE$review_bid$NC"
echo -e "    Application ID: $BLUE$review_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Activity application ..."
activity_bid=`linera --with-wallet 1 publish-bytecode ./target/wasm32-unknown-unknown/release/activity_{contract,service}.wasm`
activity_appid=`linera --with-wallet 1 create-application $activity_bid --json-parameters "{\"review_app_id\":\"$review_appid\",\"foundation_app_id\":\"$foundation_appid\",\"feed_app_id\":\"$feed_appid\"}" --required-application-ids $review_appid --required-application-ids $foundation_appid --required-application-ids $feed_appid`
print $'\U01f499' $LIGHTGREEN " Activity application deployed"
echo -e "    Bytecode ID:    $BLUE$activity_bid$NC"
echo -e "    Application ID: $BLUE$activity_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying BlobGateway application ..."
blob_gateway_bid=`linera --with-wallet 1 publish-bytecode ./target/wasm32-unknown-unknown/release/blob_gateway_{contract,service}.wasm`
blob_gateway_appid=`linera --with-wallet 1 create-application $blob_gateway_bid`
print $'\U01f499' $LIGHTGREEN " BlobGateway application deployed"
echo -e "    Bytecode ID:    $BLUE$blob_gateway_bid$NC"
echo -e "    Application ID: $BLUE$blob_gateway_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying CPRegistry application ..."
cp_registry_bid=`linera --with-wallet 1 publish-bytecode ./target/wasm32-unknown-unknown/release/cp_registry_{contract,service}.wasm`
cp_registry_appid=`linera --with-wallet 1 create-application $cp_registry_bid`
print $'\U01f499' $LIGHTGREEN " CPRegistry application deployed"
echo -e "    Bytecode ID:    $BLUE$cp_registry_bid$NC"
echo -e "    Application ID: $BLUE$cp_registry_appid$NC"

app_deploy_chain=`linera --with-wallet 1 wallet show | grep "Public Key" | awk '{print $2}'`
app_deploy_owner=`linera --with-wallet 1 wallet show | grep "Owner" | awk '{print $4}'`

print $'\U01F4AB' $YELLOW " Deploying Copilot CPU application ..."
copilot_cpu_bid=`linera --with-wallet 1 publish-bytecode ./target/wasm32-unknown-unknown/release/copilot_{contract,service}.wasm`
copilot_cpu_appid=`linera --with-wallet 1 create-application $copilot_cpu_bid --json-argument "{\"node_id\":\"d7a776b018fefbd45d533d3031c101bb64c29d52423beb6e4d5cf84e322ef429\",\"brand_logo\":\"https://github.com/respeer-ai/res-peer/blob/master/webui/public/favicon.png?raw=true\",\"brand_name\":\"respeer.ai\",\"link_base\":\"http://172.16.31.73:9081\",\"resource_type\":\"CPU\",\"device_model\":\"Intel(R) Xeon(R) Silver 4214R CPU @ 2.40GHz\",\"cpu_model\":\"Intel(R) Xeon(R) Silver 4214R CPU @ 2.40GHz\",\"storage_type\":\"NVME\",\"storage_bytes\":100000000000,\"memory_bytes\":256000000000,\"free_quota\":3,\"price_quota\":1,\"quota_price\":\"0.003\",\"supported_task_types\":[\"FixGrammar\",\"RewriteEasierUnderstand\",\"Paraphrase\",\"WriteFormally\",\"WriteMoreNeutral\"],\"payment_chain_id\":\"$app_deploy_chain\",\"ai_model\":\"CoEDiT T5\",\"ai_model_url\":\"https://huggingface.co/jbochi/candle-coedit-quantized\"}" --json-parameters "{\"cp_registry_app_id\":\"$cp_registry_appid\"}" --required-application-ids $cp_registry_appid`
print $'\U01f499' $LIGHTGREEN " Copilot CPU application deployed"
echo -e "    Bytecode ID:    $BLUE$copilot_cpu_bid$NC"
echo -e "    Application ID: $BLUE$copilot_cpu_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Copilot GPU application ..."
copilot_fetch_server_url="http://localhost:9071/?prompt="
copilot_gpu_bid=`linera --with-wallet 1 publish-bytecode ./target/wasm32-unknown-unknown/release/copilot_{contract,service}.wasm`
copilot_gpu_appid=`linera --with-wallet 1 create-application $copilot_gpu_bid --json-argument "{\"node_id\":\"d7a776b018fefbd45d533d3031c101bb64c29d52423beb6e4d5cf84e322ef429\",\"brand_logo\":\"https://github.com/respeer-ai/res-peer/blob/master/webui/public/favicon.png?raw=true\",\"brand_name\":\"respeer.ai\",\"link_base\":\"http://172.16.31.73:9081\",\"resource_type\":\"GPU\",\"device_model\":\"NVIDIA GeForce RTX 3090\",\"cpu_model\":\"Intel(R) Xeon(R) Silver 4214R CPU @ 2.40GHz\",\"storage_type\":\"NVME\",\"storage_bytes\":100000000000,\"memory_bytes\":256000000000,\"free_quota\":3,\"price_quota\":1,\"quota_price\":\"0.003\",\"supported_task_types\":[\"FixGrammar\",\"RewriteEasierUnderstand\",\"Paraphrase\",\"WriteFormally\",\"WriteMoreNeutral\"],\"payment_chain_id\":\"$app_deploy_chain\",\"fetch_server_url\":\"$copilot_fetch_server_url\",\"ai_model\":\"CoEDiT T5\",\"ai_model_url\":\"https://huggingface.co/jbochi/candle-coedit-quantized\"}" --json-parameters "{\"cp_registry_app_id\":\"$cp_registry_appid\"}" --required-application-ids $cp_registry_appid`
print $'\U01f499' $LIGHTGREEN " Copilot GPU application deployed"
echo -e "    Bytecode ID:    $BLUE$copilot_gpu_bid$NC"
echo -e "    Application ID: $BLUE$copilot_gpu_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Illustrator CPU application ..."
illustrator_cpu_bid=`linera --with-wallet 1 publish-bytecode ./target/wasm32-unknown-unknown/release/illustrator_{contract,service}.wasm`
illustrator_cpu_appid=`linera --with-wallet 1 create-application $illustrator_cpu_bid --json-argument "{\"node_id\":\"d7a776b018fefbd45d533d3031c101bb64c29d52423beb6e4d5cf84e322ef429\",\"brand_logo\":\"https://github.com/respeer-ai/res-peer/blob/master/webui/public/favicon.png?raw=true\",\"brand_name\":\"respeer.ai\",\"link_base\":\"http://172.16.31.73:9081\",\"resource_type\":\"CPU\",\"device_model\":\"Intel(R) Xeon(R) Silver 4214R CPU @ 2.40GHz\",\"cpu_model\":\"Intel(R) Xeon(R) Silver 4214R CPU @ 2.40GHz\",\"storage_type\":\"NVME\",\"storage_bytes\":100000000000,\"memory_bytes\":256000000000,\"free_quota\":3,\"price_quota\":1,\"quota_price\":\"0.03\",\"supported_task_types\":[\"GenerateIllustrate\"],\"payment_chain_id\":\"$app_deploy_chain\",\"ai_model\":\"Tiny Stable Diffusion\",\"ai_model_url\":\"https://huggingface.co/segmind/tiny-sd\"}" --json-parameters "{\"cp_registry_app_id\":\"$cp_registry_appid\"}" --required-application-ids $cp_registry_appid`
print $'\U01f499' $LIGHTGREEN " Illustrator CPU application deployed"
echo -e "    Bytecode ID:    $BLUE$illustrator_cpu_bid$NC"
echo -e "    Application ID: $BLUE$illustrator_cpu_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Illustrator GPU application ..."
illustrator_fetch_server_url="http://localhost:9072/?prompt="
illustrator_gpu_bid=`linera --with-wallet 1 publish-bytecode ./target/wasm32-unknown-unknown/release/illustrator_{contract,service}.wasm`
illustrator_gpu_appid=`linera --with-wallet 1 create-application $illustrator_gpu_bid --json-argument "{\"node_id\":\"d7a776b018fefbd45d533d3031c101bb64c29d52423beb6e4d5cf84e322ef429\",\"brand_logo\":\"https://github.com/respeer-ai/res-peer/blob/master/webui/public/favicon.png?raw=true\",\"brand_name\":\"respeer.ai\",\"link_base\":\"http://172.16.31.73:9081\",\"resource_type\":\"GPU\",\"device_model\":\"NVIDIA GeForce RTX 3090\",\"cpu_model\":\"Intel(R) Xeon(R) Silver 4214R CPU @ 2.40GHz\",\"storage_type\":\"NVME\",\"storage_bytes\":100000000000,\"memory_bytes\":256000000000,\"free_quota\":3,\"price_quota\":1,\"quota_price\":\"0.03\",\"supported_task_types\":[\"GenerateIllustrate\"],\"payment_chain_id\":\"$app_deploy_chain\",\"fetch_server_url\":\"$illustrator_fetch_server_url\",\"ai_model\":\"Tiny Stable Diffusion\",\"ai_model_url\":\"https://huggingface.co/segmind/tiny-sd\"}" --json-parameters "{\"cp_registry_app_id\":\"$cp_registry_appid\"}" --required-application-ids $cp_registry_appid`
print $'\U01f499' $LIGHTGREEN " Illustrator GPU application deployed"
echo -e "    Bytecode ID:    $BLUE$illustrator_gpu_bid$NC"
echo -e "    Application ID: $BLUE$illustrator_gpu_appid$NC"

sed -i "s/feedApp =.*/feedApp = '$feed_appid',/g" webui/src/const/index.ts
sed -i "s/creditApp =.*/creditApp = '$credit_appid',/g" webui/src/const/index.ts
sed -i "s/marketApp =.*/marketApp = '$market_appid',/g" webui/src/const/index.ts
sed -i "s/reviewApp =.*/reviewApp = '$review_appid',/g" webui/src/const/index.ts
sed -i "s/foundationApp =.*/foundationApp = '$foundation_appid',/g" webui/src/const/index.ts
sed -i "s/activityApp =.*/activityApp = '$activity_appid',/g" webui/src/const/index.ts
sed -i "s/blobGatewayApp =.*/blobGatewayApp = '$blob_gateway_appid',/g" webui/src/const/index.ts
sed -i "s/cpRegistryApp =.*/cpRegistryApp = '$cp_registry_appid',/g" webui/src/const/index.ts
sed -i "s/copilotCpuApp =.*/copilotCpuApp = '$copilot_cpu_appid',/g" webui/src/const/index.ts
sed -i "s/copilotGpuApp =.*/copilotGpuApp = '$copilot_gpu_appid',/g" webui/src/const/index.ts
sed -i "s/illustratorCpuApp =.*/illustratorCpuApp = '$illustrator_cpu_appid',/g" webui/src/const/index.ts
sed -i "s/illustratorGpuApp =.*/illustratorGpuApp = '$illustrator_gpu_appid'/g" webui/src/const/index.ts

sed -i "s/export const appDeployChain =.*/export const appDeployChain = '$app_deploy_chain'/g" webui/src/const/index.ts
sed -i "s/export const appDeployOwner =.*/export const appDeployOwner = '$app_deploy_owner'/g" webui/src/const/index.ts

function _run_service() {
  linera --with-wallet $1 service --port $2 > $LOG_FILE 2>&1
}

function run_new_service() {
  BASE_PORT=9080
  port=`expr $BASE_PORT + $1`
  print $'\U01f499' $LIGHTGREEN " Wallet of $port service ..."
  linera --with-wallet $1 wallet show
  print $'\U01f499' $LIGHTGREEN " Run $port service ..."
  LOG_FILE=`echo $SERVICE_LOG_FILE | sed "s/8080/$port/g"`
  _run_service $1 $port &
  BASE_CAT_PORT=20201
  cat_port=`expr $BASE_CAT_PORT + $1`
  socat TCP4-LISTEN:$cat_port TCP4:localhost:$port &
}

for i in `seq 1 $SERVICE_WALLET_NUMBER`; do
  run_new_service $i
done

print $'\U01f499' $LIGHTGREEN " Wallet of faucet ..."
linera --with-wallet 0 wallet show
linera --with-wallet 0 faucet --port 40080 --amount "100.0" e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65 > $FAUCET_LOG_FILE 2>&1 &

trap cleanup INT
read -p "  Press any key to exit"
print $'\U01f499' $LIGHTGREEN " Exit ..."

cleanup

