#!/bin/bash

set -e
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error: psql is not installed"
    exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error sqlx is not installed."
    echo >&2 "Use:"
    echo >&2 " cargo install --version='~0.6' sqlx-cli --no-default-features --features rustls,postgres to install it"
    exit 1
fi

DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD=${POSTGRES_PASSWORD:=password}
DB_NAME=${POSTGRES_DB:=spotter}
DB_PORT=${POSTGRES_PORT:=5432}

if [[ -z "${SKIP_DOCKER}" ]]
then
    docker run \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \
        -e POSTGRES_DB=${DB_NAME} \
        -p "${DB_PORT}":5432 \
        -d postgres \
        postgres -N 1000

    # Make sure that Postgres is available before creating it
    echo >&2 "Giving postgres time to start up..."
    sleep 10
fi


echo >&2 "Postgres is up and running on on port ${DB_PORT}!"

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}

export DATABASE_URL

sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated and ready to go!"
