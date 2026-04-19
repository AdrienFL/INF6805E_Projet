argos3 -z -c projet.argos 2>&1 \
| sed 's/\x1b\[[0-9;]*m//g' \
| sed -n 's/.*BUZZ:[ \t]*//p' \
| tr -d ' \t' \
| awk '
    /^MAP,/ { print > "experiment_map.csv"; next }
    { print > "experiment_data.csv" }
'