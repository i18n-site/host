#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}

$DIR/$1/test.sh
