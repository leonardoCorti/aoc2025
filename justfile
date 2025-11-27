# justfile


# day := `if [ "$(date +%Y%m%d)" -gt "20251225" ]; then echo 25; else date +%d; fi`
day := `today=$(date +%Y%m%d); if [ "$today" -lt "20251201" ]; then echo 00; elif [ "$today" -gt "20251225" ]; then echo 25; else date +%d; fi`

table +DAYS:
  #!/bin/env bash
  echo "| day | part | time |"
  cargo r -q -- {{DAYS}} | grep 'Day' | while read -r line; do
  day=$(echo "$line" | sed -E 's/Day ([0-9]+)  Part.*/\1/')
  part=$(echo "$line" | sed -E 's/.*Part ([0-9]+).*/\1/')
  time=$(echo "$line" | sed -E 's/.*\((.*)\)/\1/')
  echo "| $day | $part | $time |"
  done

