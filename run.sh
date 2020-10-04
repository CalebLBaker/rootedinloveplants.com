#!/bin/bash

docker kill ril
docker build . --rm --tag rootedinlovedev
docker run --detach --rm --publish 8080:8080 rootedinlovedev:latest --name ril

