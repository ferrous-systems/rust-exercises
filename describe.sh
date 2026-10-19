#!/bin/bash

# Turns a Github ref into a usable string
#
# * Merge queues look like "refs/heads/gh-readonly-queue/main/pr-408-a0e4fb7cfffac5066645846f25d833ef87851b49"
#   and become "gh-readonly-queue-main-pr-408-a0e4fb7cfffac5066645846f25d833ef87851b49-abcdefg"
# * Tags look like "refs/tags/v1.2.3" and become "v1.2.3"
# * PRs look like "refs/pull/408/merge" and become "pr-408-abcdefg"

set -euo pipefail

GIVEN_REF=$1

case "${GIVEN_REF}" in
  refs/heads/*)
    slug="$(git branch --show)-$(git rev-parse --short HEAD)"
    ;;
  refs/tags/*)
    slug="$(echo "${GIVEN_REF}" | awk '{split($0,a,"/"); print a[3]}')"
    ;;
  refs/pull/*/merge)
    slug="pr-$(echo "${GIVEN_REF}" | awk '{split($0,a,"/"); print a[3]}')-$(git rev-parse --short HEAD)"
    ;;
  *)
    echo "Bad input"
    exit 1
    ;;
esac

# Replace every (//) forward slash (escaped, as \/) with (/) underscore (-)
echo "${slug//\//-}"
