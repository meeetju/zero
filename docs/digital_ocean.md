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

## Application deployed

