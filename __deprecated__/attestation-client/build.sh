#!/bin/bash

docker build . --tag local
id=$(docker run -it -d --rm local bash)
rm -rf target
docker cp "$id":/app/target target/
