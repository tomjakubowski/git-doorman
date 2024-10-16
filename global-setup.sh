#!/bin/bash

DOORMAN_DIR=${DOORMAN_DIR:-$PWD}

echo "installing global git config for doorman filter at ${DOORMAN_DIR}"
git config --global filter.doorman.clean '"'"${DOORMAN_DIR}"'"/clean.sh %f'
git config --global filter.doorman.smudge cat
git config --global filter.doorman.required true
