cat << EOS
----------------------------------------------------------------------------------------------------
Demo 5
thread: 2, connection: 1

request:
  - write/heavy
  - read/cache

connection 1 will be blocked by first request, but second request will process soon by cache.
----------------------------------------------------------------------------------------------------
EOS

target/release/rust-web --thread 2 --connection 1 &
server=$!

sleep 0.2

curl -sS -X POST localhost:9090/write/heavy -d 'code=1234' > /dev/null &
curl1=$!

sleep 0.2

curl -sS localhost:9090/read/cache > /dev/null &
curl2=$!

wait $curl1
wait $curl2

kill $server
