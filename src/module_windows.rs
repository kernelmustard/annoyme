/*
	Author:			kernelmustard
	Description:	Library of functions used to annoy Windows x86_64 users
*/

use std::{
    thread,             // used to sleep thread
    time,               // define amount of time for sleep
    process::Command,   // struct to spawn a subshell
};
use rand::Rng;  // generate random numbers in a range
use windows::{
    w,                                                  // macro for UTF-16 LE string (LPCWSTR datatype)
    Win32::{
        UI::WindowsAndMessaging::{
            GetSystemMetrics,                           // GetSystemMetrics()
            SYSTEM_METRICS_INDEX,                       // struct to select specific data for GetSystemMetrics
            SetProcessDPIAware,                         // SetProcessDPIAware()
            SetCursorPos,                               // SetCursorPos()
            MessageBoxW,                                // MessageBoxW()
            MB_OK, MB_ICONQUESTION, MB_TOPMOST,         // Structs to set attributes of MessageBoxW
        },                     
        Foundation::HWND,                               // use HWND datatype
        System::{
            Diagnostics::Debug::{
                //MessageBeep,                              // MessageBeep() is asynchronous, may be used to set off a bunch of beeps later?
                Beep,                                       // Beep() to beep :) (humans can hear 20-20000Hz)
            },
        },
    },
};

fn annoy_windows_cursor() {
    // Set DPI awareness to ensure accurate screen size
    unsafe { SetProcessDPIAware(); }

    // Get the maximum X and Y coordinates of the monitor(s)
    let monitor_x_max = unsafe { GetSystemMetrics(SYSTEM_METRICS_INDEX(78)) };
    let monitor_y_max = unsafe { GetSystemMetrics(SYSTEM_METRICS_INDEX(79)) };
    println!("The width of your screen(s) is/are {} and the height of your screen(s) is/are {}.", monitor_x_max, monitor_y_max);

    // Move the cursor randomly 10 times
    for _n in 0..10 {
        let monitor_x_random = rand::thread_rng().gen_range(0..monitor_x_max);
        let monitor_y_random = rand::thread_rng().gen_range(0..monitor_y_max);

        unsafe { SetCursorPos(monitor_x_random, monitor_y_random); }
        println!("Your cursor will be placed at {}, {}.", monitor_x_random, monitor_y_random);

        let random_ms = time::Duration::from_millis(rand::thread_rng().gen_range(0..1000));
        unsafe { Beep(6000, 100); }
        thread::sleep(random_ms);
    }
}

fn annoy_windows_ok() {
    // Set the message and title of the message box
    let message = w!("Word around the town is that you're a Silly Billy. Can you confirm these claims?");
    let title = w!("RUMOURS!?!");

    // Show a message box with the message and title
    let mb_type = MB_OK | MB_ICONQUESTION | MB_TOPMOST;
    unsafe {
        Beep(1000, 1000);
        MessageBoxW(HWND(0), message, title, mb_type);
    }
}

fn annoy_windows_camera() {
    // Open the Camera UWP application
    let _spawn_camera = Command::new("powershell")
        .arg("Start-Process")
        .arg("microsoft.windows.camera:")
        .spawn()
        .expect("Failed to open camera.");

    // Show a message box with a message and a title
    let message = w!("Say cheese :)");
    let title = w!("I am inside your walls");
    let mb_type = MB_OK | MB_ICONQUESTION | MB_TOPMOST;

    unsafe {
        Beep(1000, 1000);
        MessageBoxW(HWND(0), message, title, mb_type);
    }
}

/*
pub fn annoy_windows() -> ! {
    // Define an array of functions to randomly select from
    let functions = [
        annoy_windows_cursor as fn(),
        annoy_windows_ok as fn(),
        annoy_windows_camera as fn(),
    ];

    loop {
        // Generate a random index to select a function from the array
        let random_index = rand::thread_rng().gen_range(0..functions.len());
        println!("The index of your random function is {}.", random_index);

        // Invoke the randomly selected function
        functions[random_index]();
    }

    // ideas:
        // cycle between screen resolution
        // pop up dialog boxes with silly messages
        // pop up dialog boxes where you have to confirm to be a silly billy
        // make annoying beeps
        // spawn child process that beeps every 1-3 seconds
}*/

pub fn annoy_windows(num_loops: u32) {
    // Define an array of functions to randomly select from
    let functions = [
        annoy_windows_cursor,
        annoy_windows_ok,
        annoy_windows_camera,
    ];

    if num_loops == 0 {
        let mut loop_count = 0;
        loop {
            // Generate a random index to select a function from the array
            let random_index = rand::thread_rng().gen_range(0..functions.len());
            println!("Loop {}: The index of your random function is {}.", loop_count, random_index);

            // Invoke the randomly selected function
            functions[random_index]();

            loop_count += 1;
        }
    } else {
        for n in 0..num_loops {
            // Generate a random index to select a function from the array
            let random_index = rand::thread_rng().gen_range(0..functions.len());
            println!("Loop {}: The index of your random function is {}.", n, random_index);

            // Invoke the randomly selected function
            functions[random_index]();
        }
    }
}