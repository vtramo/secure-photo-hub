#!/usr/bin/env bash

dockerfile_context=${1:-.}

docker build -t secure-photo-hub "${dockerfile_context}"
docker tag secure-photo-hub localhost:5001/secure-photo-hub
docker push localhost:5001/secure-photo-hub