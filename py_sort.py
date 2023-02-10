import re
# initialize the list we'll keep the finished data in
sent = []
# read the input line by line
with open('inputs/ShortStory.txt') as in_file:
    for line in in_file:
        # split each line at sentences
        sent_raw = re.split(r'\.|\!|\?|\;', line)
        for s in sent_raw:
            # go through the raw data and ignore empty or break lines
            if len(s) > 2 and not re.match(r'^-+$', s):
                # do some trimming of the data to make the sort smoother
                sent.append(s.strip().removeprefix('"').removeprefix('--').strip().removeprefix('('))
# sort the sentences ignoring case
sent.sort(key=str.casefold)
#write it to an output file joining it with newlines for readability
out_file = open('outputs/python/sent_only_py.txt', 'w+')
out_file.write('\n'.join(sent))
