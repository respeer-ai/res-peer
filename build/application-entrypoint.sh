#!/bin/bash

PROJECT_DIR=/opt/linera-project
cd $PROJECT_DIR

[ ! -d res-peer ] && git clone https://github.com/respeer-ai/res-peer
cd res-peer
git pull
sed -i "/read -p*/c sleep 100000000" ./deploy-applications.sh
sed -i "/read -p*/c sleep 100000000" ./run_applications.sh

source ~/.cargo/env
export PATH=/root/.cargo/bin/:$PATH
if [ -f /root/linera-project/linera/respeer/wallet_1.json ]; then
  ./run_applications.sh -N testnet-archimedes -n 5
else
  ./deploy-applications.sh -N testnet-archimedes -n 5
fi
