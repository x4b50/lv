#!/usr/bin/env sh
# this example demonstrates how the resizing of the arena works in action
./lc src/examples/resize.lv src/examples/resize.lb &&
echo "With arena resizing:"
./lv src/examples/resize.lb -A -R
echo
echo "Without arena resizing:"
./lv src/examples/resize.lb -A
