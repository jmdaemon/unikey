#!/bin/bash

show_usage() {
    echo "Usage:    ./install.sh [path-to-layout] [layout-name]"
}

src="$1"
layout_name="$2"
dest=/usr/share/X11/xkb/symbols

if [[ -z $src || -z layout_name ]]; then
    show_usage
else
    sudo cp "$src/$layout_name" $dest

    # LST
    baselst=$(cat layouts/base.lst)
    sudo sed "/\! layout/a   ${layout}" $src/base.lst

    evdev=$(cat layouts/evdev.lst)
    sudo sed "/\! layout/a   ${layout}" $src/evdev.lst

    # XML
    escaped=$(cat layouts/evdev.xml |
        sed 's;<;\\<;g' |
        sed 's;>;\\>;g' |
        sed 's;/;\\/;g' |
        sed 's/!/\\!/g' | tr -d '\n'; printf "\n")

    regexp="<layout>"
    line=$(grep -n "$regexp" $src/evdev.xml | cut -d ":" -f 1)
    result=$(echo $line | cut -d " " -f 1)

    # Evdev
    sudo sed -i "${result} i $escaped" $src/evdev.xml
    xmllint --format $src/evdev.xml > temp
    sudo cat temp > $src/evdev.xml
    rm temp

    # Base
    sudo sed -i "${result} i $escaped" $src/base.xml
    xmllint --format $src/base.xml > temp
    sudo cat temp > $src/base.xml
    rm temp
fi
