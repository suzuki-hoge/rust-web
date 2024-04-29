cat << EOS
----------------------------------------------------------------------------------------------------
Demo 3
thread: 2, connection: 1

request:
  - thread/heavy
  - thread/light

thread 1 will be blocked by first request, but second request will start soon by thread 2.
----------------------------------------------------------------------------------------------------
EOS

target/release/rust-web --thread 2 --connection 1 &
server=$!

sleep 0.2

curl -sS localhost:9090/thread/heavy > /dev/null &
curl1=$!

sleep 0.2

curl -sS localhost:9090/thread/light > /dev/null &
curl2=$!

wait $curl1
wait $curl2

kill $server
