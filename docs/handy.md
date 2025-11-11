# Handy

## Run application with nice logging

```
cargo run | bunyan
```

## Curl

```
curl -X POST http://127.0.0.1:8000/subscriptions \                                                                           mati@mati-Precision-3551
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "name=Tom&email=thomas_mann%40hotmail.com"
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
