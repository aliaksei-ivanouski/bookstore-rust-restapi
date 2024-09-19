### Initiate migration tool
```shell
sea migrate init -d ./src/migrator 
```

### Generate entities from database tables
```shell
sea generate entity -o src/entities -u mysql://root:12345678@localhost:13306/bookstore
```