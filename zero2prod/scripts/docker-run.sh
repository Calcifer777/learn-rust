#!/bin/bash

set -e

docker run  \
  -p 8080:8080  \
  --privileged  \
  --security-opt="seccomp=unconfined" zero2prod
