#! /bin/bash

fichier=$(cat ressources/processus.txt)

for line in $fichier
do
   xterm -hold -e "cargo run --release "$line"" &
done

echo "Press ^C to kill all terminals"
wait
trap "pkill -f xterm " SIGINT