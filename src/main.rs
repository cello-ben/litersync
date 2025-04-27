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
