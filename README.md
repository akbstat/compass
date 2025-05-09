# Compass

> Service End for Project MobiusKit

## Local launch
```
$ make dev
```

## Build into Docker image
```
$ make build
```

## Run via Docker Compose

Docker compose file:
```yaml
services:
  app:
    image: mobius/compass:v0.1.0
    ports:
      - "2217:2217"
    volumes:
      - app_log:/data/app/log
    environment:
      - LOG_DIR=/data/app/log/compass
      - DATABASE_URL=<your database url>
      - SERVER_PORT=2217
    networks:
      - postgres_postgres_net
volumes:
  app_log:
    external: true
networks:
  postgres_postgres_net:
    external: true

```

```
$ docker compose up -d
```

## Review Rest Api via Swagger

Visit swagger ui in browser
```
http://localhost:2217/swagger-ui
```

## Compass Stands for...


>***C***linical​
>
>***​O***perational
>
>***M***etrics
>
>***P***rovenance
>
>***​A***nalytics​
>
>***S***urveillance
>
>***S***ystem