cat << EOS
----------------------------------------------------------------------------------------------------
Demo 1
thread: 1, connection: 1

request:
  - read/light
  - read/light

it will be processed by same thread and same connection.
----------------------------------------------------------------------------------------------------
EOS

target/release/rust-web --thread 1 --connection 1 &
server=$!

sleep 0.2

curl -sS localhost:9090/read/light > /dev/null &
curl1=$!

sleep 0.2

curl -sS localhost:9090/read/light > /dev/null &
curl2=$!

wait $curl1
wait $curl2

kill $server
