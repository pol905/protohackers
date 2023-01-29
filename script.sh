#!/bin/sh

# Start the first process
/smoke_test --port 9000 2>&1 | sed -e 's/^/[smoke_test] /' &
  
# Start the second process
/prime_time --port 9001 2>&1 | sed -e 's/^/[prime_time] /' &

# Start the third process
/means_to_an_end --port 9002 2>&1 | sed -e 's/^/[means_to_an_end] /' &

# Start the fourth process
/budget_chat --port 9003 2>&1 | sed -e 's/^/[budget_chat] /' &

# Wait for any process to exit
wait -n
  
# Exit with status of process that exited first
exit $?