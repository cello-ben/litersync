// Copyright 2025 Benjamin Fryxell

// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.


mod consts;
mod repl;
mod web_client;

fn main() {
    println!("{}", consts::TEXT_DECORATION);
    println!("The goals of this application are as follows:");
    println!("\t- Allow searching through LibriVox for a title.");
    println!("\t- Get metadata for a book when selected, including audio URLs.");
    println!("\t- Cache the files for the entire book on the system. Show a progress bar when downloading.");
    println!("\t- Allow command line controls through a REPL on a separate thread from audio player.");
    println!("\t- When paused/stopped, save the user's place.");
    println!("{}\n", consts::TEXT_DECORATION);
    repl::repl();
}
