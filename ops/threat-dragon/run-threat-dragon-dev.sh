#!/usr/bin/env bash

docker run --rm -d -p 8085:3000 -v "$(pwd)"/.env:/app/.env owasp/threat-dragon:stable