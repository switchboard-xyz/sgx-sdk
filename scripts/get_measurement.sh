#!/bin/bash

set -eo pipefail

IMG_NAME="${1}"
if [ -z "$IMG_NAME" ]; then
    echo "No docker image tag provided to get measurement from"
    exit 1
fi

id=$(docker run -it -d --rm "$IMG_NAME" bash)
docker cp "$id":/measurement.txt measurement.txt
docker kill "$id"

echo "MR_ENCLAVE: $(cat measurement.txt)"
