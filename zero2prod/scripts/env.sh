
export DB_USER=${POSTGRES_USER:=username}
export DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
export DB_NAME="${POSTGRES_DB:=zero2prod}"
export DB_PORT="${POSTGRES_PORT:=5432}"
export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}

export PGWEB_DATABASE_URL=$DATABASE_URL