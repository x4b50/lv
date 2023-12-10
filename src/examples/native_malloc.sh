#!/usr/bin/env sh
# this example demonstrates how allocating memory natively is different from in vm malloc
./lc src/examples/native_malloc.lv src/examples/native_malloc.lb &&
./lv src/examples/native_malloc.lb -b -m
