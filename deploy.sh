#!/bin/sh

# moves to the directory of this script
cd $(dirname $0)

# builds the contract
./build.sh

if [ $? -ne 0 ]; then
  echo ">> Error building contract"
  exit 1
fi

echo ">> Deploying contract"

# deploys the contract as a dev account and returns the account ID in the folder neardev
near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/carbonite_nt_nft.wasm

