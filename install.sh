#!/bin/bash

src="$1"
dest=/usr/share/X11/xkb/symbols
sudo cp $src $dest

# LST
baselst=$(cat layouts/base.lst)
sed "/\! layout/a   ${layout}" test/base.lst

evdev=$(cat layouts/evdev.lst)
sed "/\! layout/a   ${layout}" test/evdev.lst
