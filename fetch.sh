#!/bin/bash

# Fetches the input, prints to stdout and copies to clipboard.
# This gives both a quick overview of what the input looks like
# and makes it available on ctrl+v for use in the challenge.

if [ -z "$1" ]; then
  echo "Please provide a year number."
  echo "usage: $0 YEAR DAY"
  exit 1
fi

if [ -z "$2" ]; then
  echo "Please provide a day number."
  echo "usage: $0 YEAR DAY"
  exit 1
fi

session=`cat .session`

mkdir input/$1
URL="https://adventofcode.com/$1/day/$((10#$2))/input"
curl $URL --cookie "session=$session" -s > input/$1/$2
