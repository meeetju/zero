# About the project

# Testing and local development

## Local

### Initialise database

Initialise the db in the docker

    ./scripts/init_db.sh

### Start application

#### From binary

Start the server locally

    cargo run --bin zero | bunyan

#### In docker container

Build container

    docker build --tag zero --file Dockerfile .

Run container

    docker run -p 8000:8000 -e APP_DATABASE__HOST=172.17.0.1 -e APP_DATABASE__REQUIRE_SSL=false zero

Note: 
- The database host is overridden to `172.17.0.1` (Docker bridge gateway IP) so the container can access the database running on the host.
- SSL is disabled since the local database doesn't support TLS.

### Manual Test

Healthcheck

    curl -v http://127.0.0.1:8000/health_check

Subscribe

    curl -X POST http://127.0.0.1:8000/subscriptions -H "Content-Type: application/x-www-form-urlencoded" -d "name=Tom&email=thomas_mann%40hotmail.com"

Read the database

    PGPASSWORD=password psql -h localhost -U postgres -p 5432 -d newsletter -c "SELECT * FROM subscriptions;"






