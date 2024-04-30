# Rust Web

## Requirements

```
$ rustup install nightly
```

## Operation

Boot MySQL Container.

```
$ docker compose up --build --detach
```

Boot Web Server.

```
$ cargo run -- --thread 2 --connection 2
```

```
$ curl -i localhost:9090/read/light
HTTP/1.1 200

{"result":"success","content":[{"code":"8EEA","at":"2024-04-30 22:49:43"}]}
```

## Demo

Require release build.

```
$ cargo build --release
```

### 1 thread

```
$ bash demo/1-thread-1-connection-1.sh
----------------------------------------------------------------------------------------------------
Demo 1
thread: 1, connection: 1

request:
  - read/light
  - read/light

it will be processed by same thread and same connection.
----------------------------------------------------------------------------------------------------
2024/04/30 22:51:16 [thread main] socket start
2024/04/30 22:51:16 [thread main] create thread
2024/04/30 22:51:16 [thread main] init 1 threads
2024/04/30 22:51:16 [thread main] create connection
2024/04/30 22:51:16 [thread main] init 1 connections
2024/04/30 22:51:16 [thread 1] request : GET /read/light HTTP/1.1, [accept: */*, user-agent: curl/7.84.0, host: localhost],
2024/04/30 22:51:16 [thread 1] find from database [connection 17]
2024/04/30 22:51:16 [thread 1] response: HTTP/1.1 200 {"result":"success","content":[{"code":"8EEA","at":"2024-04-30 22:49:43"}]}
2024/04/30 22:51:16 [thread 1] request : GET /read/light HTTP/1.1, [user-agent: curl/7.84.0, host: localhost, accept: */*],
2024/04/30 22:51:16 [thread 1] find from database [connection 17]
2024/04/30 22:51:16 [thread 1] response: HTTP/1.1 200 {"result":"success","content":[{"code":"8EEA","at":"2024-04-30 22:49:43"}]}
```

### 1 blocked thread

```
$ bash demo/2-thread-1-connection-1-blocked-by-thread.sh
----------------------------------------------------------------------------------------------------
Demo 2
thread: 1, connection: 1

request:
  - write/heavy
  - read/light

thread will be blocked by first request, and second request will start 3 seconds later.
----------------------------------------------------------------------------------------------------
2024/04/30 22:52:01 [thread main] socket start
2024/04/30 22:52:01 [thread main] create thread
2024/04/30 22:52:01 [thread main] init 1 threads
2024/04/30 22:52:01 [thread main] create connection
2024/04/30 22:52:01 [thread main] init 1 connections
2024/04/30 22:52:01 [thread 1] request : POST /write/heavy HTTP/1.1, [content-length: 9, accept: */*, host: localhost, user-agent: curl/7.84.0, content-type: application/x-www-form-urlencoded], code=1234
2024/04/30 22:52:01 [thread 1] save into database [connection 11]
2024/04/30 22:52:04 [thread 1] response: HTTP/1.1 200 {"result":"success","content":"connection slept 3 seconds"}
2024/04/30 22:52:04 [thread 1] request : GET /read/light HTTP/1.1, [host: localhost, accept: */*, user-agent: curl/7.84.0],
2024/04/30 22:52:04 [thread 1] find from database [connection 11]
2024/04/30 22:52:04 [thread 1] response: HTTP/1.1 200 {"result":"success","content":[{"code":"8EEA","at":"2024-04-30 22:49:43"}]}
```

### 2 threads

```
$ bash demo/3-thread-2-connection-1.sh
----------------------------------------------------------------------------------------------------
Demo 3
thread: 2, connection: 1

request:
  - thread/heavy
  - thread/light

thread 1 will be blocked by first request, but second request will start soon by thread 2.
----------------------------------------------------------------------------------------------------
2024/04/30 22:52:11 [thread main] socket start
2024/04/30 22:52:11 [thread main] create thread
2024/04/30 22:52:11 [thread main] create thread
2024/04/30 22:52:11 [thread main] init 2 threads
2024/04/30 22:52:11 [thread main] create connection
2024/04/30 22:52:11 [thread main] init 1 connections
2024/04/30 22:52:11 [thread 1] request : GET /thread/heavy HTTP/1.1, [user-agent: curl/7.84.0, host: localhost, accept: */*],
2024/04/30 22:52:11 [thread 2] request : GET /thread/light HTTP/1.1, [host: localhost, user-agent: curl/7.84.0, accept: */*],
2024/04/30 22:52:11 [thread 2] response: HTTP/1.1 200 {"result":"success","content":"thread slept 0 seconds"}
2024/04/30 22:52:14 [thread 1] response: HTTP/1.1 200 {"result":"success","content":"thread slept 3 seconds"}
```

### 2 threads + 1 blocked connection

```
$ bash demo/4-thread-2-connection-1-blocked-by-connection.sh
----------------------------------------------------------------------------------------------------
Demo 4
thread: 2, connection: 1

request:
  - write/heavy
  - read/light

connection will be blocked by first request, and second request will process 3 seconds later.
----------------------------------------------------------------------------------------------------
2024/04/30 22:52:20 [thread main] socket start
2024/04/30 22:52:20 [thread main] create thread
2024/04/30 22:52:20 [thread main] create thread
2024/04/30 22:52:20 [thread main] init 2 threads
2024/04/30 22:52:20 [thread main] create connection
2024/04/30 22:52:20 [thread main] init 1 connections
2024/04/30 22:52:20 [thread 1] request : POST /write/heavy HTTP/1.1, [host: localhost, user-agent: curl/7.84.0, accept: */*, content-length: 9, content-type: application/x-www-form-urlencoded], code=1234
2024/04/30 22:52:20 [thread 1] save into database [connection 13]
2024/04/30 22:52:20 [thread 2] request : GET /read/light HTTP/1.1, [host: localhost, accept: */*, user-agent: curl/7.84.0],
2024/04/30 22:52:23 [thread 2] find from database [connection 13]
2024/04/30 22:52:23 [thread 1] response: HTTP/1.1 200 {"result":"success","content":"connection slept 3 seconds"}
2024/04/30 22:52:23 [thread 2] response: HTTP/1.1 200 {"result":"success","content":[{"code":"8EEA","at":"2024-04-30 22:49:43"}]}
```

### 2 threads + cache

```
$ bash demo/5-thread-2-connection-1-use-cache.sh
----------------------------------------------------------------------------------------------------
Demo 5
thread: 2, connection: 1

request:
  - read/cache  ( setup cache )
  - write/heavy ( first )
  - read/cache  ( second )

connection 1 will be blocked by first request, but second request will process soon by cache.
----------------------------------------------------------------------------------------------------
2024/04/30 22:52:10 [thread main] socket start
2024/04/30 22:52:10 [thread main] create thread
2024/04/30 22:52:10 [thread main] create thread
2024/04/30 22:52:10 [thread main] init 2 threads
2024/04/30 22:52:10 [thread main] create connection
2024/04/30 22:52:10 [thread main] init 1 connections
2024/04/30 22:52:10 [thread 1] request : GET /read/cache HTTP/1.1, [accept: */*, host: localhost, user-agent: curl/7.84.0],
2024/04/30 22:52:10 [thread 1] find from database [connection 8]
2024/04/30 22:52:10 [thread 1] save into cache
2024/04/30 22:52:10 [thread 1] response: HTTP/1.1 200 {"result":"success","content":[{"code":"8EEA","at":"2024-04-30 22:49:43"}]}
2024/04/30 22:52:10 [thread 2] request : POST /write/heavy HTTP/1.1, [accept: */*, host: localhost, content-length: 9, content-type: application/x-www-form-urlencoded, user-agent: curl/7.84.0], code=1234
2024/04/30 22:52:10 [thread 2] save into database [connection 8]
2024/04/30 22:52:10 [thread 1] request : GET /read/cache HTTP/1.1, [accept: */*, host: localhost, user-agent: curl/7.84.0],
2024/04/30 22:52:10 [thread 1] find from cache
2024/04/30 22:52:10 [thread 1] response: HTTP/1.1 200 {"result":"success","content":[{"code":"8EEA","at":"2024-04-30 22:49:43"}]}
2024/04/30 22:52:13 [thread 2] response: HTTP/1.1 200 {"result":"success","content":"connection slept 3 seconds"}
```

### 2 threads + 2 connections

```
$ bash demo/6-thread-2-connection-2.sh
----------------------------------------------------------------------------------------------------
Demo 6
thread: 2, connection: 2

request:
  - write/heavy
  - read/light

connection 1 will be blocked by first request, but second request will process soon by connection 2.
----------------------------------------------------------------------------------------------------
2024/04/30 22:52:38 [thread main] socket start
2024/04/30 22:52:38 [thread main] create thread
2024/04/30 22:52:38 [thread main] create thread
2024/04/30 22:52:38 [thread main] init 2 threads
2024/04/30 22:52:38 [thread main] create connection
2024/04/30 22:52:38 [thread main] create connection
2024/04/30 22:52:38 [thread main] init 2 connections
2024/04/30 22:52:38 [thread 1] request : POST /write/heavy HTTP/1.1, [host: localhost, content-length: 9, content-type: application/x-www-form-urlencoded, user-agent: curl/7.84.0, accept: */*], code=1234
2024/04/30 22:52:38 [thread 1] save into database [connection 15]
2024/04/30 22:52:38 [thread 2] request : GET /read/light HTTP/1.1, [host: localhost, accept: */*, user-agent: curl/7.84.0],
2024/04/30 22:52:38 [thread 2] find from database [connection 16]
2024/04/30 22:52:38 [thread 2] response: HTTP/1.1 200 {"result":"success","content":[{"code":"8EEA","at":"2024-04-30 22:49:43"}]}
2024/04/30 22:52:41 [thread 1] response: HTTP/1.1 200 {"result":"success","content":"connection slept 3 seconds"}
```
