#!/bin/sh

# Start the first process
/smoke_test --port 9000 2>&1 | sed -e 's/^/[smoke_test] /' &
  
# Start the second process
/prime_time --port 9001 2>&1 | sed -e 's/^/[prime_time] /' &

# Start the second process
/means_to_an_end --port 9002 2>&1 | sed -e 's/^/[means_to_an_end] /' &
# Wait for any process to exit
wait -n
  
# Exit with status of process that exited first
exit $?