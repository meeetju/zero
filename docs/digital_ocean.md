# DigitalOcean

## References
- [CLI](https://docs.digitalocean.com/reference/doctl/)

## Doctl

Create application

```
doctl apps create --spec spec.yaml
```

List applications

```
doctl apps list
```

```
ID                                      Spec Name    Default Ingress                          Active Deployment ID                    
a2bbf450-9394-4181-8731-6ebdfed8d2fd    zero         https://zero-omio2.ondigitalocean.app    98242613-2990-40a9-8b5a-1e43090ff86b
```

Update application

```
doctl apps update a2bbf450-9394-4181-8731-6ebdfed8d2fd --spec=spec.yaml
```

Delete application

```
doctl apps delete a2bbf450-9394-4181-8731-6ebdfed8d2fd --force
```

## Local DO database migration

Install deps

```
   apt-get update && apt-get install -y postgresql-client
```

Export password

```
export PGPASSWORD=''
```

Start migration

```
psql "postgres://newsletter:<password>@app-ba3e31d5-c0a4-43e4-abd1-000533621b3f-do-user-29001877-0.f.db.ondigitalocean.com:25060/newsletter?sslmode=require"
```

Paste

```
   CREATE TABLE subscriptions(
       id uuid NOT NULL,
       PRIMARY KEY (id),
       email TEXT NOT NULL UNIQUE,
       name TEXT NOT NULL,
       subscribed_at timestamptz NOT NULL
   );
```

Query

```
psql postgresql://newsletter:<password>@app-fb60500b-a554-4c18-be08-bd933ea91efe-do-user-29001877-0.g.db.ondigitalocean.com:25060/newsletter?sslmode=require
```

## Local DB query

```
apt-get update && apt-get install -y postgresql-client
```

```
psql postgresql://newsletter:<password>@app-ba3e31d5-c0a4-43e4-abd1-000533621b3f-do-user-29001877-0.f.db.ondigitalocean.com:25060/newsletter?sslmode=require -c SELECT * FROM subscriptions;
```