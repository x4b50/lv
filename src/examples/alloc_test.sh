#!/usr/bin/env sh
# this example demonstrates how allocating memory natively is different from in vm malloc
./lc src/examples/alloc_test.lv src/examples/alloc_test.lb &&
./lv src/examples/alloc_test.lb -b -m -d
