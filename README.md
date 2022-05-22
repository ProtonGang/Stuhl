# Stuhl
## Initial setup
```shell
sudo docker pull postgres
```

## Running
```
sudo docker run --rm --detach --name postgres --env POSTGRES_USER=test --env POSTGRES_PASSWORD=123 --publish 127.0.0.1:1234:5432 postgres
diesel migration run
cargo run
```
