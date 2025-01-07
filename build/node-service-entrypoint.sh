#!/bin/bash

PROJECT_DIR=/opt/linera-project
cd $PROJECT_DIR

[ ! -d res-peer ] && git clone https://github.com/respeer-ai/res-peer
cd res-peer
git pull

mkdir -p /root/.config
source ~/.cargo/env
export PATH=/root/.cargo/bin/:$PATH

if [ -f /root/.config/linera/wallet.json ]; then
  linera --max-retries 100 --retry-delay-ms 10 service --port 30080 --listener-skip-process-inbox
else
  ./run_node_service.sh -N testnet-archimedes
fi
