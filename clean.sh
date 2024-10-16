#!/bin/bash

IFS= read -rd '' content

if grep -q "XXX(tom)" <<<"$content"; then
  echo "error: Forbidden string 'XXX(tom)' detected in ${1:-<stdin>}" >&2
  exit 1
else
  printf "%s" "$content"
fi
