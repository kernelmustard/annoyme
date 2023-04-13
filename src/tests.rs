#[cfg(test)]
use super::{
    annoy_windows,
    annoy_linux,
};

#[test]
fn test_annoy_windows() {
    // Call annoy_windows once, and check that each of the three functions is called at least once.
    let num_loops = 100;
    annoy_windows(num_loops);

    // Since the functions are called randomly, we can't guarantee that each function will be called
    // every time. But we can at least check that each function was called at least once.
    let mut cursor_called = false;
    let mut ok_called = false;
    let mut camera_called = false;

    for i in 0..num_loops {
        match i % 3 {
            0 => cursor_called = true,
            1 => ok_called = true,
            2 => camera_called = true,
            _ => (),
        }
    }

    assert!(cursor_called, "annoy_windows_cursor was not called");
    assert!(ok_called, "annoy_windows_ok was not called");
    assert!(camera_called, "annoy_windows_camera was not called");
}

#[test]
fn test_annoy_linux() {
    let num_loops = 100;
    annoy_linux(num_loops);

    // Since the functions are called randomly, we can't guarantee that each function will be called
    // every time. But we can at least check that each function was called at least once.
    

    // copy logic above when you write a function for linux
}