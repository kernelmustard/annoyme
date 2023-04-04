/*
    Author:         kernelmustard
    Description:    If you ever had the thought "I wish my computer was more difficult to use," then this crate is for you.
*/

use std::{
    env::consts,        // expose env::consts::OS
    thread,             // used to sleep thread
    time,               // ^
    process::Command,   // struct to spawn a subshell
};
use windows::{
    w,                                                  // macro for UTF-16 LE string (LPCWSTR datatype)
    Win32::{
        UI::WindowsAndMessaging::{
            GetSystemMetrics,                           // GetSystemMetrics()
            SYSTEM_METRICS_INDEX,                       // struct to select specific data for GetSystemMetrics
            SetProcessDPIAware,                         // SetProcessDPIAware()
            SetCursorPos,                               // SetCursorPos()
            //MoveWindow,                                 // MoveWindows()
            //SetWindowPos,                               // SetWindowPos()
            //SWP_ASYNCWINDOWPOS, HWND_BOTTOM,            // Structs as arugments for SetWindowPos()
            MessageBoxW,                                // MessageBoxW()
            MB_OK, MB_ICONQUESTION, MB_TOPMOST,         // Structs to set attributes of MessageBoxW
        },                     
        Foundation::{HWND, TRUE, FALSE},                // use HWND, TRUE datatypes
        System::{
            Diagnostics::Debug::{
                //MessageBeep,                              // MessageBeep() is asynchronous, may be used to set off a bunch of beeps later?
                Beep,                                       // Beep() to beep :) (humans can hear 20-20000Hz)
            },
        },
        UI::Input::KeyboardAndMouse::GetActiveWindow,   // GetActiveWindow() to return HWND to active window
    },
};
use rand::Rng;  // generate random numbers in a range

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

fn annoy_windows_cursor() { // randomly move the cursor
    unsafe { SetProcessDPIAware(); } // return TRUE if it worked
    let i32_monitor_x_max = unsafe { let unsafe_i32_monitor_x_max: i32 = GetSystemMetrics(SYSTEM_METRICS_INDEX(78)); unsafe_i32_monitor_x_max }; // get monitor x max
    let i32_monitor_y_max = unsafe { let unsafe_i32_monitor_y_max: i32 = GetSystemMetrics(SYSTEM_METRICS_INDEX(79)); unsafe_i32_monitor_y_max }; // get monitor y max
    println!("The width of your screen(s) is/are {:?} and the height of your screen(s) is/are {:?}.", i32_monitor_x_max, i32_monitor_y_max);

    for _n in 0..10 { // move cursor 10 times
        let monitor_x_random = rand::thread_rng().gen_range(0..i32_monitor_x_max);  // generate a pair of pseudorandom numbers for X and Y axis
        let monitor_y_random = rand::thread_rng().gen_range(0..i32_monitor_y_max);

        unsafe { SetCursorPos(monitor_x_random, monitor_y_random); }  // set cursor position to random numbers
        println!("Your cursor will be placed at {:?}, {:?}", monitor_x_random, monitor_y_random);

        let random_ms = time::Duration::from_millis(rand::thread_rng().gen_range(0..1000));
        unsafe { Beep(6000, 100); }
        thread::sleep(random_ms);
    }
}

fn annoy_windows_ok() {

    let message = w!("Word around the town is that you're a Silly Billy. Can you confirm these claims?");
    let title = w!("RUMOURS!?!");

    unsafe {
        Beep(1000, 1000);
        MessageBoxW(
            HWND(0),                                // [in, optional] HWND    hWnd,
            message,                                // [in, optional] LPCTSTR lpText,
            title,                                  // [in, optional] LPCTSTR lpCaption,
            MB_OK | MB_ICONQUESTION | MB_TOPMOST,   // [in]           UINT    uType
        );
    }
}

/*  THIS FUNCTION DOES NOT WORK

fn annoy_windows_resize() {
    // resize current window between 500x500 and max resolution

    // gather info
    let i32_min_x = 500;
    let i32_min_y = 500;
    let i32_max_x = unsafe { let unsafe_i32_max_x: i32 = GetSystemMetrics(SYSTEM_METRICS_INDEX(78)); unsafe_i32_max_x };
    let i32_max_y = unsafe { let unsafe_i32_max_y: i32 = GetSystemMetrics(SYSTEM_METRICS_INDEX(79)); unsafe_i32_max_y };
    let h_active_window = unsafe { let unsafe_h_active_window = GetActiveWindow(); unsafe_h_active_window };

    // generate random x and y between min and max values
    let i32_corner_x: i32 = 500;
    let i32_corner_y: i32 = 500;
    let i32_width:  i32 = rand::thread_rng().gen_range(i32_min_x..i32_max_x);
    let i32_height: i32 = rand::thread_rng().gen_range(i32_min_y..i32_max_y);

    // call MoveWindow to resize it
    println!("Your window will be resized to {:?}x{:?} and the top left will be placed at {:?}x{:?}.", i32_width, i32_height, i32_corner_x, i32_corner_y);
    unsafe {
        
        SetWindowPos(
            h_active_window,
            HWND_BOTTOM,
            i32_corner_x,       // x coordinate of top left
            i32_corner_y,       // y coordinate of top left
            i32_width,          // max width
            i32_height,         // max height
            SWP_ASYNCWINDOWPOS,
        );
        
        //MoveWindow(
        //    h_active_window,    // HWND of current window
        //    i32_corner_x,       // x coordinate of top left
        //    i32_corner_y,       // y coordinate of top left
        //    i32_width,          // max width
        //    i32_height,         // max height
        //    FALSE,               // BOOL to repaint current window
        //);
    }
}
*/

fn annoy_windows_camera() {

    // spawn Camera UWP application
    let _spawn_camera = Command::new("powershell")
        .args(["Start-Process(\"microsoft.windows.camera:\")"])
        .output()
        .expect("Failed to open camera.");

    // cheeky messagebox
    let message = w!("Say cheese :)");
    let title = w!("I am inside your walls");

    unsafe {
        Beep(1000, 1000);
        MessageBoxW(
            HWND(0),                                // [in, optional] HWND    hWnd,
            message,                                // [in, optional] LPCTSTR lpText,
            title,                                  // [in, optional] LPCTSTR lpCaption,
            MB_OK | MB_ICONQUESTION | MB_TOPMOST,   // [in]           UINT    uType
        );
    }
}

fn annoy_windows() -> ! {

    // i cannot figure out how to invoke random functions in an array
    

    // array of pointers to the functions
    let ptr_annoy_windows_cursor: fn() -> () = annoy_windows_cursor;
    let ptr_annoy_windows_ok: fn() -> () = annoy_windows_ok;
    let ptr_annoy_windows_camera: fn() -> () = annoy_windows_camera;
    //let ptr_annoy_windows_resize: fn() -> () = &annoy_windows_resize;

    let array_annoy_functions = [
       ptr_annoy_windows_cursor,
       ptr_annoy_windows_ok,
       ptr_annoy_windows_camera,
    ];

    // spawn a single child process that beeps randomly

    loop {
        // generate random index value in annoy_functions array
        let random_function = rand::thread_rng().gen_range(0..array_annoy_functions.len());
        println!("The index of your random function is {:?}", random_function);


        // invoke random function
        array_annoy_functions[random_function]();

        
        // sleep thread for random amount of time between 1 and 10 seconds
        //let random_ms = time::Duration::from_millis(rand::thread_rng().gen_range(1000..10000));    
        //thread::sleep(random_ms);
    }

    // ideas:
        // cycle between screen resolution
        // pop up dialog boxes with silly messages
        // pop up dialog boxes where you have to confirm to be a silly billy
        // make annoying beeps
        // spawn child process that beeps every 1-3 seconds
}

fn annoy_linux() {
    loop {
        // pass
    }
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
        annoy_windows();
    } else if let "linux" = info[0].as_str() {
        annoy_linux();
    }
}
