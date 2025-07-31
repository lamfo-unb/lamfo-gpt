```
docker network create \
--driver overlay \
db-network

docker service create \
--name database \
--replicas 1 \
--publish published=5432,target=5432 \
--network db-network \
--mount type=bind,source=/root/postgres_data,destination=/var/lib/postgresql/data \
--secret source=postgres_user,target=postgres_user \
--secret source=postgres_password,target=postgres_password \
--env TZ=America/Sao_Paulo \
--env POSTGRES_USER_FILE=/run/secrets/postgres_user \
--env POSTGRES_PASSWORD_FILE=/run/secrets/postgres_password \
postgres:16

docker service create \
--name qdrant \
--replicas 1 \
--publish published=6333,target=6333 \
--publish published=6334,target=6334 \
--mount type=bind,source=/root/qdrant_data,destination=/qdrant/storage:z \
--network db-network \
qdrant/qdrant

docker service create \
--name lamfo-gpt \
--replicas 1 \
--publish published=3000,target=8000 \
--network db-network \
--env MODEL_CHAT_OA=gpt-3.5-turbo-0125 \
--env QDRANT_URL=http://qdrant:6334 \
--env LAMFO_GPT_DIR=./lamfo_gpt/files \
--env RUST_LOG=assist_lamfo=info \
--secret source=openai_api_key,target=openai_api_key \
--secret source=pg_dev_app_url,target=pg_dev_app_url \
--secret source=db_url,target=db_url \
--secret source=pg_dev_postgres_url,target=pg_dev_postgres_url \
dauid64/lamfo-gpt:latest
```