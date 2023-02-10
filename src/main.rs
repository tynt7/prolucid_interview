use std::fs;
use std::path::Path;
use regex::Regex;

fn main() {
    //a path to the input from the root directory, may need to change later
    let in_path = Path::new("inputs/ShortStory.txt");
    //read the whole input into a big string so we can use String trait functionality
    //for easy formating and cleaning human language
    //expect to handle the result from the IO in worst case
    let contents = fs::read_to_string(in_path).expect("couldn't read data");
    //initalize a new vector to store the sentences into later when we finish cleaning
    let mut sent_only = Vec::new();
    //initalize a regex to grab the lines used by this formatting to signify breaks
    let clean_hyph = Regex::new(r"^-+$").unwrap();

    //split across all sentences, because I'm not a linguist and I'm cheeky like this
    //I'm going to say a sentence is anything that ends in these characters ;)
    for s in  contents.split(['\n', '.', '?', '!', ';']){
        //ignore nothing sentences
        if s.len() > 0 {
            //if our regex hasn't triggered and the sentence is at least 2 characters
            //we could tweak this tolerance i guess but it seems fine here
            if !clean_hyph.is_match(s) && s.len() > 2{
                //lets add that sentence!
                sent_only.push(s
                    //but lets get rid of that white space first
                    .trim()
                    //and then actually lets get rid of quotes cuz we want the first letter
                    .trim_start_matches("\"")
                    //but seriously no more white space!!!!
                    .trim()
                    //also I want the two sentences starting with these
                    //to play nice it's a little hacky but it gives me closure
                    .trim_start_matches("(")
                    .trim_start_matches("--")
                );
            }
        }
    }
    //lets do the actual sorting by making rust happy and letting it do the work 
    sent_only.sort_by(
        //pass our custom function to determine ordering using the cmp utility
        //make them lower case for this comparison to ignore case
        |a, b| a.to_lowercase().cmp(&b.to_lowercase())
    );
    //our output path, same idea as before
    let out_path = Path::new("outputs/rust/sent_onl_rs.txt");
    //write the vec to a file at that path, joining with new lines for readablity
    //and expect the worst case
    fs::write(out_path, sent_only.join("\n")).expect("Unable to write data");
}
