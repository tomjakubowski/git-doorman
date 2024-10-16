#!/bin/bash

REPO=${1:-$PWD}

gitattributes="${REPO}/.git/info/attributes"
echo "installing git-doorman to $gitattributes"
if [[ ! -d "${REPO}/.git/info" ]]; then
  echo "${REPO}.git/info not found or not a directory.  Is ${REPO} a git repository?"
  exit 1
fi
cat << 'EOF' >> "$gitattributes"
* filter=doorman
EOF
