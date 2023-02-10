#!/bin/bash

# use the time command to get a naive look at how efficent I was
time cargo run

time python ./src/py/py_sort_naive.py

# compare the two naive to show method is the same
grep -Ff outputs/rust/sent_onl_rs.txt outputs/python/sent_only_py_naive.txt

