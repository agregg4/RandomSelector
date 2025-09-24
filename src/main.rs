use rand::Rng;
use std::io;
use std::collections::HashMap;

// PROGRAM OBJECTIVES
// Allow user to enter a set of strings that a winner will randomly be selected from
// Currently based on a fixed set of entries in the random system the highest value will be selected
// Currently by default ties are allowed (I could make an options menu to disable this.)
// In the future I would like to allow setting the tie bias (max_random)
// FIXED - You currently can use the same word multiple times to win. (Name collisions need to be prevented, its creating multi entries increasing win bias)
// ---------------------------------
// CLEANUP TASKS
// Find and Reduce Repitition (IE make functions to reduce reused code)
// Readability abstractions

fn main() {
    println!("Please enter a list of options, then type -q to quit or -f to finish:");
    //decalre input variable word_entry
    let mut word_entry = String::new(); //Assign new String
    //Declare the list for words that are input
    let mut words_list = Vec::new(); //Assign new Vec (vector)

    //Command Structure Loop
    loop {
        io::stdin().read_line(&mut word_entry).expect("failed to get input");

        let trimmed = word_entry.trim();
        let mut exists = false;

        if trimmed  == "q" {
            println!("Exiting");
            break;
        }

        if trimmed == "f" {
            println!("Moving to random");
            run_logic(words_list.clone());
            break; //I would rather use exit(0) but I dont know the syntax
        }
        else {
            for word in &words_list {
                if *word == word_entry {
                    exists = true;
                }
            }
            if !exists {
                words_list.push(word_entry.clone()); // push the last entered word into the word list
            }
            if exists {
                println!("Word already exists, please enter another word.")
            }
            
            println!("Another Word?:"); //As for another word
            exists = false;
        }
        word_entry.clear();
    }

}

fn run_logic(wordlist: Vec<String>) {
    let mut counter: i32 = 0;
    let mut rand_counts = HashMap::new();
    
    //Generate the hashmap
    //This is populating the hashmap with 0 values for each word in the word list.
    for n in 0..wordlist.len() {
        rand_counts.insert(&wordlist[n],0); 
    }
    
    
    //Can I clean this up and put it in a function? fn gen_rand
    //Tie bias (Max Random Attempts), I am hardcoding the setting max_rand. The point is to disallow faulty bias because when the random count is too low it ties too often.
    // I am hard coding the value because i dont want to create a configuration menu right now.
    while counter < 300{
        
        //Random selection function goes here.
        let num: usize = rand::rng().random_range(0..wordlist.len()).try_into().unwrap();

        //Evaluating how many times each has been selected.
        if let Some(value) = rand_counts.get_mut(&wordlist[num]) { // a reference
            *value += 1 // Dereferenceing and operating on the value itself.
        }
        counter+=1; //Operating on the owned value
    }

    //fn determine_winner (&mut rand_counts)
    let mut winner = ""; //Mutable empty string
    let mut tie = ""; //Mutable empty string
    let mut top:u64 = 0; //Mutable unsigned int, Value should not be negative 64 bit to give too much rope.
    let mut tieval:u64 = 0; //Mutable unsigned int, Value should not be negative. 64 bit to give too much rope.
    
    //Create a header for the results.
    println!("-- Name : Votes --");

    for (key, value) in rand_counts { //Creating new ownership of rand_counts for iterating
        
        //Figure out what the winner is.
        if value > top { //Reference
            winner = key; //Reference
            top = value; //Reference
        }

        //Evaluate if a tie has occured.
        if value == top && winner != key {//Reference
            tie = key; //Reference
            tieval = value; //Reference
        }

        //Print the results of the word selection
        println!(" {} : {}",key.trim(), value); //References
    }

    //Print out that a tie has occured
    if tieval >= top && winner != tie { //References
        println!("TIED: {} and {} \n", winner.trim(), tie.trim()); //References
    }
    println!("---------------");

    // if there is no tie print the winner
    if top > tieval {
        println!("Heres the Winner: {}", winner); //Reference
    }
}