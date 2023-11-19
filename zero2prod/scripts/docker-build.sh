#!/bin/bash

set -e

artifacts=../target/debug/zero2prod*

cp $artifacts .
docker build --tag zero2prod --file docker/Dockerfile .
rm zero2prod*