cat << EOS
----------------------------------------------------------------------------------------------------
Demo 2
thread: 1, connection: 1

request:
  - write/heavy
  - read/light

thread will be blocked by first request, and second request will start 3 seconds later.
----------------------------------------------------------------------------------------------------
EOS

target/release/rust-web --thread 1 --connection 1 &
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
