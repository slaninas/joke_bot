#!/bin/bash

docker build -t joke_bot . && \
    docker run --rm -ti --name joke_bot joke_bot
