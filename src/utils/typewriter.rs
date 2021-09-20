use std::{
    io::{self, Write},
    thread, time,
};

// this function is used to add a typewriter effect to your print statements
pub fn write(st: &str, dur: u64, is_wait: bool, wait_times: i32) {
    for c in st.chars() {
        if c == ' ' {
            continue;
        } else {
            thread::sleep(time::Duration::from_millis(dur));
            print!("{}", c);
            io::stdout().flush().unwrap();
        }
    }
    if is_wait {
        wait(wait_times);
    }
}

pub fn wait(times: i32) {
    for _ in 0..times {
        let dots = "...";
        write(dots, 200, false, 0);
        print!("\u{8}\u{8}\u{8}   \u{8}\u{8}\u{8}");
    }
}
