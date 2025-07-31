#!/bin/sh
export DB_URL=$(cat /run/secrets/db_url)
export OPENAI_API_KEY=$(cat /run/secrets/openai_api_key)
export PG_DEV_APP_URL=$(cat /run/secrets/pg_dev_app_url)
export PG_DEV_POSTGRES_URL=$(cat /run/secrets/pg_dev_postgres_url)

exec /assist-lamfo
