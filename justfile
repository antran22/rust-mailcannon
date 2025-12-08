set dotenv-load
set unstable

postgres_host := "127.0.0.1" # can be overridden
postgres_user := env("POSTGRES_USER")
postgres_password := env("POSTGRES_PASSWORD")
postgres_port := env("POSTGRES_PORT")
postgres_db := env("POSTGRES_DB")
  
export DATABASE_URL := "postgres://" + postgres_user + ":" + postgres_password + "@" + postgres_host + ":" + postgres_port + "/" + postgres_db


start:
  cargo run

start-watch:
  bacon run

test:
  cargo test

test-watch:
  bacon test

test-coverage:
  cargo tarpaulin --verbose --workspace 

test-log:
  TEST_LOG=true cargo test | bunyan

dev-dep-start:
  docker compose -f docker-compose.dev.yml up -d

dev-dep-stop:
  docker compose -f docker-compose.dev.yml down

dev-dep-clean:
  docker compose -f docker-compose.dev.yml down -v

dev-dep-log:
  docker compose -f docker-compose.dev.yml logs -f

create-db:
  sqlx database create

migration +args:
  sqlx migrate {{args}}

db-schema-update: _db-schema-update-code _db-schema-update-test

_db-schema-update-code:
  cargo sqlx prepare

_db-schema-update-test:
  cargo sqlx prepare -- --tests


# Linting

lint:
  cargo clippy

unused-deps:
 ::{init_subscriber, make_tracing_subscriber}, cargo +nightly udeps
