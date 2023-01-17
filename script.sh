#!/bin/sh

# Start the first process
# /smoke_test --port 9000 2>&1 | sed -e 's/^/[smoke_test] /' &
  
# Start the second process
/prime_time --port 9001
  
# Wait for any process to exit
wait -n
  
# Exit with status of process that exited first
exit $?