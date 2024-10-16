#!/bin/bash

echo "XXX(tom)" >> testbed/test.txt

if git add testbed/test.txt; then
  echo "adding to index succeeded! test failed! rolling back"
  git restore --staged testbed/test.txt
  git restore testbed/test.txt
  exit 1
else
  echo "testbed test passed!"
  git restore testbed/test.txt
fi

foo=$(printf "foo" | tr '\n' "X" | ./clean.sh)
[[ $foo != "foo" ]] && { echo "foo no-newline test failed"; exit 1; }
echo "foo no-newline test passed"
fooX=$(printf "foo\n" | tr '\n' "X" | ./clean.sh)
[[ $fooX != "fooX" ]] && { echo "foo trailing-newline test failed"; exit 1; }
echo "foo trailing-newline test passed"
