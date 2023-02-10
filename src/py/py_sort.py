# use python tools to remove punctuation in the way
from string import punctuation
# use the natural language toolkit for more precise sentence tokenizing
import nltk
import re


# initialize the list we'll keep the finished data in
sent = []
# open the file pointer
file = open('inputs/ShortStory.txt')
# read the whole thing
data = file.read()
# let smarter people handle finding sentences
sent_raw = nltk.tokenize.sent_tokenize(data, language='english')
# we have to do a little cleaning because nothing is perfect
for se in sent_raw:
    # so lets get rid of any new lines inside a sentence from visual formatting
    for s in se.split('\n'):
        # then lets do that same check to makes sure our sentences say something and are not a line of hyphens signifying a break
        if len(s) > 3 and not re.match(r'^-+$', s):
            # add the cleaned sentence to our list, with leading whitespace and punctuation removed, and any whitespace after any punctuation to be safe
            sent.append(s.strip().strip(punctuation).strip())

# sort the sentences ignoring case
sent.sort(key=str.casefold)
#write it to an output file joining it with newlines for readability
out_file = open('outputs/python/sent_only_py.txt', 'w+')
out_file.write('\n'.join(sent))
