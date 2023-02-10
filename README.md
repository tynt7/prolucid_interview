# Prolucid Interview
### Tyler Thomson

For this technical interview I was tasked to arrange a short story's sentences in alphabetic order. Some initial assumptions I have made are that we want to preserve the sentence structure. Additionally I have assumed as sentence is anything concluding in;
- .
- ?
- !
- \n
- ;

I was able to accomplish the rust program in a naive approach 08/02/23 after starting 07/02/23. I chose rust because it's a language I want to get better at and thought it would prove a good test for me. ~~I also figured why not shoot for the moon. So my hope is to perform the assigned task in C and python as well, then move my rust program into the alph.rs file, call all three using sub processes and compare their general efficiency and display the results using rust through a standard output. Stretch goal for sure but I think it sounds cool and I want you to think I am cool. Maybe if that doesn't crush me completely I will wrap it in a pretty (ugly) GUI! With Charts! Diagrams! Snarky comments!~~


As of 09/02/23 I have taken stock of realistic and accomplish-able work for the time allotted, so sadly I am reducing scope to just the rust and python programs already completed, and a simple bash script that can be run for some (really) naive measurements. I admittedly can get a bit carried away but I like to think myself realistic at the end of the day, so with the loose deadline of within the week approaching I wanted to be sure I was covered in the event of unforeseen circumstances tomorrow 10/02/23. I hope that you like what I've been able to put together, I had a blast writing it and really wanted to do more but at a certain point bells and whistles are just noise. I also will be the first to admit the strictness of some of my data cleaning, I had intended to do a fun little experiment across the three languages to learn more about concurrency in rust, though now with my sights brought back down to earth and that no longer being the case it leaves my solution in a poor position to receive any new stories should they not follow a similar formatting to the provided story. Additionally because I wouldn't have access to the same libraries across all three languages I tried to minimize which ones I used to provide a greater comparability in my experiment. I include this all to give you the full picture of what I was thinking.

Okay one more time, because it was bugging me not using existing tools, I have done it again in python but used the nltk library so it wasn't as cheap. I really am enjoying this project.

Additionally, I enjoyed rereading that story for whatever nerd points that nets! What is, Isaac Asimov's 'The Last Question'? I recently got a copy of the first Foundation book and am looking forward to it!

## General Setup
* Pull the repository to your workspace, I will assume a Linux environment.

## How to run Naive Experiment
* Provide ```run_naive.sh``` with run* permissions.
* Run the test with ./run_naive.sh > results.txt and you will see the results from ```time``` on each command, as well as the results of their outputs compared through grep in a txt file. It doesn't do the full justice of running the command yourself to stdio, and this part is a bit of a rush job, so I encourage you to run the command yourself!

## How to run Tool Experiment
* Set up a python venv using the method of your choice, I used a simple approach of;
    * ```python3 -m venv env```
    * ```source env/bin/activate```
    * ```pip3 install -r requirements.txt```
* The rest are the same as Naive, but this time we will use ```run_tool.sh```

\* sorry I just realized I had a brain fart
