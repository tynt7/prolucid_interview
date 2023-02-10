#!/bin/bash

# use the time command to get a naive look at how efficent I was
time cargo run

# use the time command to get a naive look at how efficent I was
time python py_sort.py

# be snarky
echo -e "\nProbably should have stuck to python!\n"

# compare the two to show method is the same
grep -Ff outputs/rust/sent_onl_rs.txt outputs/python/sent_only_py.txt


