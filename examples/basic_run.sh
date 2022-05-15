#!/bin/sh

call="YOUR_CALL"
host="SERVER_HOST"
port=SERVER_PORT

cat <(echo "$call") - | nc $host $port 2>&1 | xargs -I{} ./parser_basic {}

exit 0