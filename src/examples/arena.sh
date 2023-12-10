#!/usr/bin/env sh
./lc src/examples/arena.lv src/examples/arena.lb &&
./lv src/examples/arena.lb -d -b -a 16
echo
./lv src/examples/arena.lb -b -a 16
