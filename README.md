# Rust Web

## Requirements

```
$ rustup install nightly
```

## Operation

```
$ make run

$ curl -i -X POST localhost:9090/item/create -d 'code=1234'
HTTP/1.1 200

id: 42, name: John
```
