/*
    Author:         kernelmustard
    Description:    If you ever had the thought "I wish my computer was more difficult to use," then this crate is for you.
*/

mod module_windows;
mod module_linux;
mod tests;

use module_windows::annoy_windows;
use module_linux::annoy_linux;
use std::env::consts;        // expose env::consts::OS

fn supported_os() -> [String; 2] {
    // all OSes supported by std::env::consts::OS are: android, dragonfly, freebsd, ios, linux, macos, netbsd, openbsd, solaris, windows
    // this program currently supports: ("windows", "linux")

    let mut info_array: [String; 2] = [
        String::from(consts::OS), // OS
        String::from("no")        // whether that OS is supported
    ];
    
    let supported_os = ["windows", "linux"];

    for element in supported_os {
        if String::from(element) == info_array[0] {
            info_array[1] = String::from("yes");
            break;
        }
    }
    return info_array;
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

    if let "windows" = info[0].as_str() as &str {
        annoy_windows(0);
    } else if let "linux" = info[0].as_str() {
        annoy_linux();
    }
}
