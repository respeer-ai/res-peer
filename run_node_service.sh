#!/bin/bash

rm ~/.config/linera/wallet.* -rf
linera wallet init --faucet http://localhost:8080
linera service --port 30080
