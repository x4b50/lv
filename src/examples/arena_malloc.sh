#!/usr/bin/env sh
# this example demonstrates how the resizing of the arena works in action
./lc src/examples/arena_malloc.lv src/examples/arena_malloc.lb &&
./lv src/examples/arena_malloc.lb -A
