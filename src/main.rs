/*
    Author:         kernelmustard
    Description:    If you ever had the thought "I wish my computer was more difficult to use," then this crate is for you.
*/

mod module_windows;
mod module_linux;
mod tests;

use module_windows::annoy_windows;  // use functions in module_windows.rs
use module_linux::annoy_linux;      // use functions in module_linux.rs
use std::env::consts;               // expose env::consts::OS

fn supported_os() -> [String; 2] {
    // all OSes supported by std::env::consts::OS are: 
    // android, dragonfly, freebsd, ios, linux, macos, netbsd, openbsd, solaris, windows
    // this program currently supports: "windows" | "linux"

    let os = String::from(consts::OS);
    let is_supported = match os.as_str() {
        "windows" | "linux" => true,
        _ => false,
    };

    let info_array: [String; 2] = [
        os, 
        if is_supported { String::from("yes") } else { String::from("no") },
    ];

    info_array
}


fn main() {

    let info = supported_os(); // return a tuple with relevant info regarding current OS

    println!("Your OS is {:?}", info[0]);
    if info[1] == "yes" {
        println!("Your OS is supported :)");
        println!("Prepare to be annoyed :)");
    } else {
        println!("Your OS is not supported :(");
        println!("Unfortunately I am unable to annoy you at this time :(");
        return;
    }

    // Testing Area

    //

    if let "windows" = info[0].as_str() {
        annoy_windows(0);
    } else if let "linux" = info[0].as_str() {
        annoy_linux(0);
    }
}
