#!/bin/bash

echo "XXX(tom)" >> testbed/test.txt

if git add testbed/test.txt; then
  echo "adding to index succeeded! test failed! rolling back"
  git restore --staged testbed/test.txt
  git restore testbed/test.txt
  exit 1
else
  echo "test passed!"
  git restore testbed/test.txt
fi
