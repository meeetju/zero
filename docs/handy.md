# Handy

## Run application with nice logging

```
cargo run | bunyan
```

## Curl

## Health check
```
curl -v http://127.0.0.1:8000/health_check
```

## Subscribe

```
curl -X POST http://127.0.0.1:8000/subscriptions -H "Content-Type: application/x-www-form-urlencoded" -d "name=Tom&email=thomas_mann%40hotmail.com"
```

or

```
curl --request POST --data 'name=le%20guin&email=ursula_le_guin%40gmail.com' 127.0.0.1:8000/subscriptions --verbose
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

## Docker

Build

```
docker build --tag zero --file Dockerfile .
```

Check size
```
docker images zero
```

Run
```
docker run -p 8000:8000 zero
```