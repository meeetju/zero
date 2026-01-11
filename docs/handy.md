# Handy

## Run application with nice logging

```
cargo run | bunyan
```

## Curl

### Health check
```
curl -v http://127.0.0.1:8000/health_check
```

```
curl -v https://zero-omio2.ondigitalocean.app/health_check
```

### Subscribe

```
curl -X POST http://127.0.0.1:8000/subscriptions -H "Content-Type: application/x-www-form-urlencoded" -d "name=Tom&email=thomas_mann%40hotmail.com"
```

or

```
curl --request POST --data 'name=le%20guin&email=ursula_le_guin%40gmail.com' 127.0.0.1:8000/subscriptions --verbose
```

```
curl --request POST --data 'name=le%20guin&email=ursula_le_guin%40gmail.com' https://zero-omio2.ondigitalocean.app/subscriptions --verbose
```

## Runing tests

### No logs

```
cargo test health_check_works
```

### Logs

```
TEST_LOG=true cargo test health_check_works
```

## Database

### Connection string

```
DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
```

Example

```
DATABASE_URL=postgresql://newsletter:Password@app-ba3e31d5-c0a4-43e4-abd1-000533621b3f-do-user-29001877-0.f.db.ondigitalocean.com:25060/newsletter?sslmode=require sqlx migrate run

```