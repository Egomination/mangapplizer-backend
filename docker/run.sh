#!/bin/sh

# TODO: Improve this script.

export POSTGRES_PW=1
export PGADMIN_EMAIL=t@d.com
export PGADMIN_PASS=ehe

if [ -z "$POSTGRES_PW" ]; then echo "EXPORT NECESSARY VARIABLES TO ENV"; exit 1; fi

# Start postgresql
docker run \
       --rm \
       -d \
       -p 5432:5432 \
       --name postgres \
       -e POSTGRES_PASSWORD=$POSTGRES_PW \
       -v $(pwd)/psql:/var/lib/postgresql/data \
       postgres:alpine

# Start pgadmin4
# https://www.pgadmin.org/docs/pgadmin4/latest/container_deployment.html
docker run \
       --rm \
       -d \
       -p 5433:80 \
       -e PGADMIN_DEFAULT_EMAIL=$PGADMIN_EMAIL \
       -e PGADMIN_DEFAULT_PASSWORD=$PGADMIN_PASS \
       -d dpage/pgadmin4
