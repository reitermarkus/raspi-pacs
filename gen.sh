#!/usr/bin/env bash

set -euo pipefail

print_usage() {
    echo 'Usage: gen.sh MCU_NAME' >&2
}

if [ $# -ne 1 ]; then
    print_usage
    exit 1
fi

mcu_name=$1

pushd peripherals
mkdir -p svd/gen
pipenv install -r requirements.txt
pipenv run python ./gen_svd.py
popd

mkdir -p "crates/${mcu_name}-lpa"
pushd "crates/${mcu_name}-lpa"

svd2rust -i "../../peripherals/svd/gen/${mcu_name}_lpa.svd" --target none \
    --atomics --impl_debug

rm -rf src/
form -i lib.rs -o src/
rm lib.rs
cargo fmt

popd
