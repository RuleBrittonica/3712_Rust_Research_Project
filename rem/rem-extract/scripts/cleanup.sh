#!/bin/bash

rm -rf ./input
rm -rf ./output
rm -rf ./correct_output

mkdir -p ./input
mkdir -p ./output
mkdir -p ./correct_output

python ./scripts/fixup_cargotoml.py