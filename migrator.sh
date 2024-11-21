#!/bin/bash

# Run the migrator container with the current user's UID and GID
docker run --name migrator -v $(pwd):/tmp --user $(id -u):$(id -g) devkramer/migrator $@

# Remove the migrator container
docker rm migrator > /dev/null 2>&1