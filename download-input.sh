#!/bin/sh

YEAR=2021
DAY=$(date '+%-d')
DAY_0=$(date '+%d')

TRIGGER_TIMESTAMP="${YEAR}-12-${DAY_0}T05:00:00+00:00"
check_time() {
  [[ "$(date -Is -u)" < "${TRIGGER_TIMESTAMP}" ]]
}
if check_time; then
  echo "waiting until $TRIGGER_TIMESTAMP"
  while check_time; do sleep 1; done
fi

curl --cookie "session=${SESSION_COOKIE}" "https://adventofcode.com/${YEAR}/day/${DAY}/input" | tee "inputs/day${DAY_0}.in"
