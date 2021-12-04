#!/bin/bash

src="$1"
dest=/usr/share/X11/xkb/symbols
sudo cp $src $dest

# LST
baselst=$(cat layouts/base.lst)
sed "/\! layout/a   ${layout}" test/base.lst

evdev=$(cat layouts/evdev.lst)
sed "/\! layout/a   ${layout}" test/evdev.lst

# XML

escaped=$(cat layouts/evdev.xml |
    sed 's;<;\\<;g' |
    sed 's;>;\\>;g' |
    sed 's;/;\\/;g' |
    sed 's/!/\\!/g' | tr -d '\n'; printf "\n")

regexp="<layout>"
line=$(grep -n "$regexp" test/evdev.xml | cut -d ":" -f 1)
result=$(echo $line | cut -d " " -f 1)

# Evdev
sed -i "${result} i $escaped" test/evdev.xml
xmllint --format test/evdev.xml > temp
cat temp > test/evdev.xml
rm temp

# Base
sed -i "${result} i $escaped" test/base.xml
xmllint --format test/base.xml > temp
cat temp > test/base.xml
rm temp
