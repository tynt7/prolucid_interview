#!/bin/bash

# use the time command to get a naive look at how efficent I was
time python py_sort.py

time python py_sort_naive.py

# compare the two to see how bad naive was
grep -Ff outputs/python/sent_only_py.txt outputs/python/sent_only_py_naive.txt


