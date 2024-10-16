#!/bin/bash

cd "$(dirname "$0")" || {
  echo "cd error"
  exit 1
}

cargo run -- global-setup
cargo run -- uninstall --attributes-file .gitattributes
cargo run -- install --pattern 'testbed/*' --attributes-file .gitattributes
echo "XXX(tom)" >>testbed/test.txt

# e2e test
if git add testbed/test.txt; then
  echo "adding to index succeeded! testbed test failed! rolling back working tree..."
  git restore --staged testbed/test.txt
  git restore testbed/test.txt
  exit 1
else
  echo "testbed test passed"
  git restore testbed/test.txt
fi

# test clean command newline handling
foo=$(printf "foo" | cargo run -- clean | tr '\n' 'X')
[[ $foo != "foo" ]] && {
  echo "no-newline test failed"
  exit 1
}
echo "no-newline test passed"
fooX=$(printf "foo\n" | cargo run -- clean | tr '\n' 'X')
[[ $fooX != "fooX" ]] && {
  echo "trailing-newline test failed"
  exit 1
}
echo "trailing-newline test passed"
