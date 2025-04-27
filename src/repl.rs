// Copyright 2025 Benjamin Fryxell

// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.


use crate::consts;
use crate::web_client;

use std::io::{stdin, stdout, Write};
use std::process;

fn usage() {
    println!("{}", consts::TEXT_DECORATION);
    println!("Welcome to LibriSync! Options:");
    println!("s <title>: search");
    println!("l: list");
    println!("p <id>: play");
    println!("q: quit");
    println!("{}\n", consts::TEXT_DECORATION);
}

fn is_valid_command(command: &str) -> bool {
    if command.trim().len() == 0 {
        usage();
        return false;
    }
    true
}

fn parse(option: &str, command: &str) {
    //Parse to see if we should just list, or if we should use the command to search for or play a given audiobook.
    match option.trim() {
        "s" => {
            if is_valid_command(command) {
                //Execute the search command.
                println!("{}", web_client::get_json(command));
            }
        },
        "p" => {
            if is_valid_command(command) {
                //Execute the play command.
            }
        },
        "l" => {
            //List. TODO decide if we should allow it to proceed if command exists.
        },
        "q" => {
            println!("{}", consts::TEXT_DECORATION);
            println!("Thank you for using {}. Have a great day!", consts::APP_NAME);
            println!("{}", consts::TEXT_DECORATION);
            process::exit(0);
        }
        _ => usage()
    }
}

pub fn repl() {
    usage();
    loop {
        print!("> ");
        let _ = stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line.");
        let (option, command) = input.split_at_checked(2).unwrap();
        parse(option, command); //TODO make parse return an option based upon how it cascades through action(s).
    }
    
}