#!/bin/bash

gas_pass_address="erd1qqqqqqqqqqqqqpgq5tlkn9qcza52v4dtfzsx99xjxrgtw2p74wzqnyc5ea"

mxpy contract deploy \
    --bytecode output/gass-pass-hack.wasm \
    --arguments ${gas_pass_address} \
    --recall-nonce \
    --gas-limit 60000000 \
    --pem="../wallet_shard0.pem" \
    --chain=D \
    --proxy https://devnet-gateway.multiversx.com \
    --send