cat << EOS
----------------------------------------------------------------------------------------------------
Demo 6
thread: 2, connection: 2

request:
  - write/heavy
  - read/light

connection 1 will be blocked by first request, but second request will process soon by connection 2.
----------------------------------------------------------------------------------------------------
EOS

target/release/rust-web --thread 2 --connection 2 &
server=$!

sleep 0.2

curl -sS -X POST localhost:9090/write/heavy -d 'code=1234' > /dev/null &
curl1=$!

sleep 0.2

curl -sS localhost:9090/read/light > /dev/null &
curl2=$!

wait $curl1
wait $curl2

kill $server
