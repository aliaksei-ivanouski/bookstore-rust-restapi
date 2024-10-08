## Rust REST API service example

### Build following Udemy tutorial
Creating an API server in Rust with Rocket, SeaORM and JWT
https://www.udemy.com/course/rest-api-server-rust-rocket-seaorm-jwt

### Initiate migration tool
```shell
sea migrate init -d ./src/migrator 
```

### Generate entities from database tables
```shell
sea generate entity -o src/entities -u mysql://root:12345678@localhost:13306/rust-rocket-restapi
```

## Run application
- start full application (all containers)
```shell
docker-compose -f docker-compose.local.yml up -d app
```

## Address
```shell
curl http://localhost:18000
```

## Prometheus dashboard
* Available here http://localhost:9090

## Grafana dashboard
* Available here http://localhost:3000
* login: admin
* password: admin